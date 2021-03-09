use crate::*;
use std::ops::{Add, Div, Mul, Sub};

///////////////////////////////////////////////////////////////
/// The following impls add operators to Robj.
///
impl Robj {
    /// Do the equivalent of x$y
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    /// let env = Environment::from_pairs({
    ///    vec![("a".to_string(), r!(1)), ("b".to_string(), r!(2))]});
    /// assert_eq!(env.dollar("a").unwrap(), r!(1));
    /// assert_eq!(env.dollar("b").unwrap(), r!(2));
    /// }
    /// ```
    pub fn dollar<'a, T>(&self, symbol: T) -> Result<Robj>
    where
        Symbol<'a>: From<T>,
    {
        let symbol: Symbol = Symbol::from(symbol);
        call!("$", self, symbol)
    }

    /// Do the equivalent of `x[y]`
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.slice(2).unwrap(), r!(20));
    /// assert_eq!(vec.slice(2..=3).unwrap(), r!([20, 30]));
    /// }
    /// ```
    pub fn slice<T>(&self, rhs: T) -> Result<Robj>
    where
        T: Into<Robj>,
    {
        call!("[", self, rhs.into())
    }

    /// Do the equivalent of `x[[y]]`
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.index(2).unwrap(), r!(20));
    /// assert_eq!(vec.index(2..=3).is_err(), true);
    /// }
    /// ```
    pub fn index<T>(&self, rhs: T) -> Result<Robj>
    where
        T: Into<Robj>,
    {
        call!("[[", self, rhs.into())
    }

    /// Do the equivalent of x ~ y
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    /// let x = r!(Symbol("x"));
    /// let y = r!(Symbol("y"));
    /// let tilde = x.tilde(y).unwrap();
    /// assert_eq!(tilde, r!(Lang(&[r!(Symbol("~")), r!(Symbol("x")), r!(Symbol("y"))])));
    /// assert_eq!(tilde.inherits("formula"), true);
    /// }
    /// ```
    pub fn tilde<T>(&self, rhs: T) -> Result<Robj>
    where
        T: Into<Robj>,
    {
        call!("~", self, rhs.into())
    }

    /// Do the equivalent of x :: y
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    /// let base = r!(Symbol("base"));
    /// let env = r!(Symbol(".getNamespace"));
    /// let base_env = base.double_colon(env).unwrap();
    /// assert_eq!(base_env.is_function(), true);
    /// }
    /// ```
    pub fn double_colon<T>(&self, rhs: T) -> Result<Robj>
    where
        T: Into<Robj>,
    {
        call!("::", self, rhs.into())
    }

    /// Do the equivalent of x(a, b, c)
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     let function = R!(function(a, b) a + b).unwrap();
    ///     assert_eq!(function.is_function(), true);
    ///     assert_eq!(function.call(pairlist!(a=1, b=2)).unwrap(), r!(3));
    /// }
    /// ```
    pub fn call(&self, args: Robj) -> Result<Robj> {
        if self.rtype() != RType::Function {
            return Err(Error::ExpectedFunction(self.clone()));
        }

        if args.rtype() != RType::Pairlist {
            return Err(Error::ExpectedPairlist(args.clone()));
        }

        unsafe {
            let call = new_owned(Rf_lcons(self.get(), args.get()));
            call.eval()
        }
    }
}

impl<Rhs> Add<Rhs> for Robj
where
    Rhs: Into<Robj>,
{
    type Output = Robj;

    /// Add two R objects, consuming the left hand side.
    /// panics on error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// // lhs and rhs get dropped here
    /// let lhs = r!([1, 2]);
    /// let rhs = r!([10, 20]);
    /// assert_eq!(lhs + rhs, r!([11, 22]));
    ///
    /// // lhs gets dropped and rhs is a temporary object.
    /// let lhs = r!([1, 2]);
    /// assert_eq!(lhs + 1000, r!([1001, 1002]));
    ///
    /// // Only lhs gets dropped.
    /// let lhs = r!([1, 2]);
    /// let rhs = r!([10, 20]);
    /// assert_eq!(lhs + &rhs, r!([11, 22]));
    /// }
    /// ```
    fn add(self, rhs: Rhs) -> Self::Output {
        call!("+", self, rhs.into()).expect("Robj add failed")
    }
}

impl<Rhs> Sub<Rhs> for Robj
where
    Rhs: Into<Robj>,
{
    type Output = Robj;

    /// Subtract two R objects, consuming the left hand side.
    /// panics on error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// // lhs and rhs get dropped here
    /// let lhs = r!([10, 20]);
    /// let rhs = r!([1, 2]);
    /// assert_eq!(lhs - rhs, r!([9, 18]));
    ///
    /// // lhs gets dropped and rhs is a temporary object.
    /// let lhs = r!([1000, 2000]);
    /// assert_eq!(lhs - 1, r!([999, 1999]));
    ///
    /// // Only lhs gets dropped.
    /// let lhs = r!([10, 20]);
    /// let rhs = r!([1, 2]);
    /// assert_eq!(lhs - &rhs, r!([9, 18]));
    /// }
    /// ```
    fn sub(self, rhs: Rhs) -> Self::Output {
        call!("-", self, rhs.into()).expect("Robj subtract failed")
    }
}

impl<Rhs> Mul<Rhs> for Robj
where
    Rhs: Into<Robj>,
{
    type Output = Robj;

    /// Multiply two R objects, consuming the left hand side.
    /// panics on error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// // lhs and rhs get dropped here
    /// let lhs = r!([10.0, 20.0]);
    /// let rhs = r!([1.0, 2.0]);
    /// assert_eq!(lhs * rhs, r!([10.0, 40.0]));
    ///
    /// // lhs gets dropped and rhs is a temporary object.
    /// let lhs = r!([1.0, 2.0]);
    /// assert_eq!(lhs * 10.0, r!([10.0, 20.0]));
    ///
    /// // Only lhs gets dropped.
    /// let lhs = r!([10.0, 20.0]);
    /// let rhs = r!([1.0, 2.0]);
    /// assert_eq!(lhs * &rhs, r!([10.0, 40.0]));
    /// }
    /// ```
    fn mul(self, rhs: Rhs) -> Self::Output {
        call!("*", self, rhs.into()).expect("Robj multiply failed")
    }
}

impl<Rhs> Div<Rhs> for Robj
where
    Rhs: Into<Robj>,
{
    type Output = Robj;

    /// Divide two R objects, consuming the left hand side.
    /// panics on error.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///
    /// // lhs and rhs get dropped here
    /// let lhs = r!([10.0, 20.0]);
    /// let rhs = r!([1.0, 2.0]);
    /// assert_eq!(lhs / rhs, r!([10.0, 10.0]));
    ///
    /// // lhs gets dropped and rhs is a temporary object.
    /// let lhs = r!([10.0, 30.0]);
    /// assert_eq!(lhs / 10.0, r!([1.0, 3.0]));
    ///
    /// // Only lhs gets dropped.
    /// let lhs = r!([10.0, 20.0]);
    /// let rhs = r!([1.0, 2.0]);
    /// assert_eq!(lhs / &rhs, r!([10.0, 10.0]));
    /// }
    /// ```
    fn div(self, rhs: Rhs) -> Self::Output {
        call!("/", self, rhs.into()).expect("Robj divide failed")
    }
}

// Calls are still experimental.
//
// impl<Args> Fn(Args) for Robj
// {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output {

//     }
// }

/* list of primitives in base.
> b[sapply(b, function(b) is.primitive(get(b, baseenv())))]
  [1] "-"               ":"               "!"               "!="
  [5] "("               "["               "[["              "[[<-"
  [9] "[<-"             "{"               "@"               "@<-"
 [13] "*"               "/"               "&"               "&&"
 [17] "%*%"             "%/%"             "%%"              "^"
 [21] "+"               "<"               "<-"              "<<-"
 [25] "<="              "="               "=="              ">"
 [29] ">="              "|"               "||"              "~"
 [33] "$"               "$<-"             "abs"             "acos"
 [37] "acosh"           "all"             "any"             "anyNA"
 [41] "Arg"             "as.call"         "as.character"    "as.complex"
 [45] "as.double"       "as.environment"  "as.integer"      "as.logical"
 [49] "as.numeric"      "as.raw"          "asin"            "asinh"
 [53] "atan"            "atanh"           "attr"            "attr<-"
 [57] "attributes"      "attributes<-"    "baseenv"         "break"
 [61] "browser"         "c"               "call"            "ceiling"
 [65] "class"           "class<-"         "Conj"            "cos"
 [69] "cosh"            "cospi"           "cummax"          "cummin"
 [73] "cumprod"         "cumsum"          "digamma"         "dim"
 [77] "dim<-"           "dimnames"        "dimnames<-"      "emptyenv"
 [81] "enc2native"      "enc2utf8"        "environment<-"   "exp"
 [85] "expm1"           "expression"      "floor"           "for"
 [89] "forceAndCall"    "function"        "gamma"           "gc.time"
 [93] "globalenv"       "if"              "Im"              "interactive"
 [97] "invisible"       "is.array"        "is.atomic"       "is.call"
[101] "is.character"    "is.complex"      "is.double"       "is.environment"
[105] "is.expression"   "is.finite"       "is.function"     "is.infinite"
[109] "is.integer"      "is.language"     "is.list"         "is.logical"
[113] "is.matrix"       "is.na"           "is.name"         "is.nan"
[117] "is.null"         "is.numeric"      "is.object"       "is.pairlist"
[121] "is.raw"          "is.recursive"    "is.single"       "is.symbol"
[125] "isS4"            "lazyLoadDBfetch" "length"          "length<-"
[129] "levels<-"        "lgamma"          "list"            "log"
[133] "log10"           "log1p"           "log2"            "max"
[137] "min"             "missing"         "Mod"             "names"
[141] "names<-"         "nargs"           "next"            "nzchar"
[145] "oldClass"        "oldClass<-"      "on.exit"         "pos.to.env"
[149] "proc.time"       "prod"            "quote"           "range"
[153] "Re"              "rep"             "repeat"          "retracemem"
[157] "return"          "round"           "seq_along"       "seq_len"
[161] "seq.int"         "sign"            "signif"          "sin"
[165] "sinh"            "sinpi"           "sqrt"            "standardGeneric"
[169] "storage.mode<-"  "substitute"      "sum"             "switch"
[173] "tan"             "tanh"            "tanpi"           "tracemem"
[177] "trigamma"        "trunc"           "unclass"         "untracemem"
[181] "UseMethod"       "while"           "xtfrm"
*/
