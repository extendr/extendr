use crate::*;
use super::*;

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
        Symbol<'a> : From<T>
    {
        let symbol : Symbol = Symbol::from(symbol);
        call!("$", self, symbol)
    }

    /// Do the equivalent of x[y]
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.slice(2).unwrap(), r!(20));
    /// assert_eq!(vec.slice(2..=3).unwrap(), r!([20, 30]));
    /// ```
    pub fn slice<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T : Into<Robj>,
    {
        call!("[", self, rhs.into())
    }

    /// Do the equivalent of x[y]
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let vec = r!([10, 20, 30]);
    /// assert_eq!(vec.index(2).unwrap(), r!(20));
    /// assert_eq!(vec.index(2..=3).is_err(), true);
    /// ```
    pub fn index<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T : Into<Robj>,
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
        T : Into<Robj>,
    {
        call!("~", self, rhs.into())
    }

    /// Do the equivalent of x :: y
    /// ```
    /// use extendr_api::*;
    /// extendr_engine::start_r();
    /// let rlang = r!(Symbol("rlang"));
    /// let env = r!(Symbol("env"));
    /// let rlang_env = rlang.double_colon(env).unwrap();
    /// assert_eq!(rlang_env.is_function(), true);
    /// ```
    pub fn double_colon<T>(&self, rhs: T) -> Result<Robj, AnyError>
    where
        T : Into<Robj>,
    {
        call!("::", self, rhs.into())
    }
}
