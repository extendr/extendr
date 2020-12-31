use super::*;
use crate::*;
use std::ops::{Add, Div, Mul, Sub};

///////////////////////////////////////////////////////////////
/// The following impls add operators to Robj.
///
impl Robj {
    /// Do the equivalent of x$y
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let env = r!(Env{parent: global_env(), names: &["a", "b"], values: &[1, 2]});
    /// assert_eq!(env.dollar("a").unwrap(), r!(1));
    /// assert_eq!(env.dollar("b").unwrap(), r!(2));
    /// ```
    pub fn dollar<'a, T>(&self, symbol: T) -> Result<Robj, AnyError>
    where
        Symbol<'a>: From<T>,
    {
        let symbol: Symbol = Symbol::from(symbol);
        call!("$", self, symbol)
    }

    /// Do the equivalent of `x[y]`
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.slice(2).unwrap(), r!(20));
    /// assert_eq!(vec.slice(2..=3).unwrap(), r!([20, 30]));
    /// ```
    pub fn slice<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T: Into<Robj>,
    {
        call!("[", self, rhs.into())
    }

    /// Do the equivalent of `x[[y]]`
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.index(2).unwrap(), r!(20));
    /// assert_eq!(vec.index(2..=3).is_err(), true);
    /// ```
    pub fn index<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T: Into<Robj>,
    {
        call!("[[", self, rhs.into())
    }

    /// Do the equivalent of x ~ y
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let x = r!(Symbol("x"));
    /// let y = r!(Symbol("y"));
    /// let tilda = x.tilda(y).unwrap();
    /// assert_eq!(tilda, r!(Lang(&[r!(Symbol("~")), r!(Symbol("x")), r!(Symbol("y"))])));
    /// assert_eq!(tilda.inherits("formula"), true);
    /// ```
    pub fn tilda<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T: Into<Robj>,
    {
        call!("~", self, rhs.into())
    }

    /// Do the equivalent of x :: y
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let base = r!(Symbol("base"));
    /// let env = r!(Symbol(".getNamespace"));
    /// let base_env = base.double_colon(env).unwrap();
    /// assert_eq!(base_env.is_function(), true);
    /// ```
    pub fn double_colon<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T: Into<Robj>,
    {
        call!("::", self, rhs.into())
    }

    // /// Do the equivalent of x(y)
    // /// ```
    // /// use extendr_api::*;
    // /// extendr_engine::start_r();
    // /// let function = R!(function(a, b) a + b).unwrap();
    // /// assert_eq!(function.is_function(), true);
    // /// assert_eq!(function.call((1, 2)), r!(3));
    // /// ```
    // pub fn call<Args>(&self, _args: Args) -> Robj {
    //     r!(NULL)
    // }
}

impl<Rhs> Add<Rhs> for Robj
where
    Rhs: Into<Robj>,
{
    type Output = Robj;

    /// Add two R objects, consuming the left hand side.
    /// panics on error.
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
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
    /// use extendr_api::*;
    /// extendr_engine::start_r();
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
