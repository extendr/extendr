//! Error handling in Rust called from R.

use std::convert::Infallible;

use crate::conversions::try_into_int::ConversionError;
use crate::robj::Types;
use crate::{throw_r_error, Robj};

/// Throw an R error if a result is an error.
#[doc(hidden)]
pub fn unwrap_or_throw<T>(r: std::result::Result<T, &'static str>) -> T {
    match r {
        Err(e) => {
            throw_r_error(e);
        }
        Ok(v) => v,
    }
}

#[doc(hidden)]
pub fn unwrap_or_throw_error<T>(r: std::result::Result<T, Error>) -> T {
    match r {
        Err(e) => {
            throw_r_error(e.to_string());
        }
        Ok(v) => v,
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Panic(Robj),
    NotFound(Robj),
    EvalError(Robj),
    ParseError(Robj),
    NamesLengthMismatch(Robj),

    ExpectedNull(Robj),
    ExpectedSymbol(Robj),
    ExpectedPairlist(Robj),
    ExpectedFunction(Robj),
    ExpectedEnvironment(Robj),
    ExpectedPromise(Robj),
    ExpectedLanguage(Robj),
    ExpectedSpecial(Robj),
    ExpectedBuiltin(Robj),
    ExpectedRstr(Robj),
    ExpectedLogical(Robj),
    ExpectedInteger(Robj),
    ExpectedReal(Robj),
    ExpectedComplex(Robj),
    ExpectedString(Robj),
    ExpectedDot(Robj),
    ExpectedAny(Robj),
    ExpectedList(Robj),
    ExpectedExpression(Robj),
    ExpectedBytecode(Robj),
    ExpectedExternalPtr(Robj),
    ExpectedWeakRef(Robj),
    ExpectedRaw(Robj),
    ExpectedS4(Robj),
    ExpectedPrimitive(Robj),

    ExpectedScalar(Robj),
    ExpectedVector(Robj),
    ExpectedMatrix(Robj),
    ExpectedMatrix3D(Robj),
    ExpectedNumeric(Robj),
    ExpectedAltrep(Robj),
    ExpectedDataframe(Robj),

    OutOfRange(Robj),
    MustNotBeNA(Robj),
    ExpectedWholeNumber(Robj, ConversionError),
    ExpectedNonZeroLength(Robj),
    OutOfLimits(Robj),
    TypeMismatch(Robj),
    NamespaceNotFound(Robj),
    NoGraphicsDevices(Robj),

    ExpectedExternalPtrType(Robj, String),
    ExpectedExternalNonNullPtr(Robj),
    ExpectedExternalPtrReference,
    Other(String),

    #[cfg(feature = "ndarray")]
    NDArrayShapeError(ndarray::ShapeError),

    #[cfg(feature = "either")]
    EitherError(Box<Error>, Box<Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Panic(robj) => write!(f, "Panic detected {:?}.", robj),
            Error::NotFound(robj) => write!(f, "Not found. {:?}", robj),
            Error::EvalError(robj) => write!(f, "Evaluation error in {:?}.", robj),
            Error::ParseError(code) => write!(f, "Parse error in {:?}.", code),
            Error::NamesLengthMismatch(robj) => {
                write!(f, "Length of names does not match vector. {:?}", robj)
            }

            Error::ExpectedNull(robj) => write!(f, "Expected Null got {:?}", robj.rtype()),
            Error::ExpectedSymbol(robj) => write!(f, "Expected Symbol got {:?}", robj.rtype()),
            Error::ExpectedPairlist(robj) => write!(f, "Expected Pairlist got {:?}", robj.rtype()),
            Error::ExpectedFunction(robj) => write!(f, "Expected Function got {:?}", robj.rtype()),
            Error::ExpectedEnvironment(robj) => {
                write!(f, "Expected Environment got {:?}", robj.rtype())
            }
            Error::ExpectedPromise(robj) => write!(f, "Expected Promise got {:?}", robj.rtype()),
            Error::ExpectedLanguage(robj) => write!(f, "Expected Language got {:?}", robj.rtype()),
            Error::ExpectedSpecial(robj) => write!(f, "Expected Special got {:?}", robj.rtype()),
            Error::ExpectedBuiltin(robj) => write!(f, "Expected Builtin got {:?}", robj.rtype()),
            Error::ExpectedRstr(robj) => {
                write!(f, "Expected Rstr got {:?}", robj.rtype())
            }
            Error::ExpectedLogical(robj) => write!(f, "Expected Logicals got {:?}", robj.rtype()),
            Error::ExpectedInteger(robj) => write!(f, "Expected Integers got {:?}", robj.rtype()),
            Error::ExpectedReal(robj) => write!(f, "Expected Doubles got {:?}", robj.rtype()),
            Error::ExpectedComplex(robj) => write!(f, "Expected Complexes got {:?}", robj.rtype()),
            Error::ExpectedString(robj) => write!(f, "Expected Strings got {:?}", robj.rtype()),
            Error::ExpectedDot(robj) => write!(f, "Expected Dot got {:?}", robj.rtype()),
            Error::ExpectedAny(robj) => write!(f, "Expected Any got {:?}", robj.rtype()),
            Error::ExpectedList(robj) => write!(f, "Expected List got {:?}", robj.rtype()),
            Error::ExpectedExpression(robj) => {
                write!(f, "Expected Expression got {:?}", robj.rtype())
            }
            Error::ExpectedBytecode(robj) => write!(f, "Expected Bytecode got {:?}", robj.rtype()),
            Error::ExpectedExternalPtr(robj) => {
                write!(f, "Expected ExternalPtr got {:?}", robj.rtype())
            }
            Error::ExpectedWeakRef(robj) => write!(f, "Expected WeakRef got {:?}", robj.rtype()),
            Error::ExpectedRaw(robj) => write!(f, "Expected Raw got {:?}", robj.rtype()),
            Error::ExpectedS4(robj) => write!(f, "Expected S4 got {:?}", robj.rtype()),
            Error::ExpectedPrimitive(robj) => {
                write!(f, "Expected Primitive got {:?}", robj.rtype())
            }

            Error::ExpectedScalar(robj) => write!(f, "Expected Scalar, got {:?}", robj.rtype()),
            Error::ExpectedVector(robj) => write!(f, "Expected Vector, got {:?}", robj.rtype()),
            Error::ExpectedMatrix(robj) => write!(f, "Expected Matrix, got {:?}", robj.rtype()),
            Error::ExpectedMatrix3D(robj) => write!(f, "Expected Matrix3D, got {:?}", robj.rtype()),
            Error::ExpectedNumeric(robj) => write!(f, "Expected Numeric, got {:?}", robj.rtype()),
            Error::ExpectedAltrep(robj) => write!(f, "Expected Altrep, got {:?}", robj.rtype()),
            Error::ExpectedDataframe(robj) => {
                write!(f, "Expected Dataframe, got {:?}", robj.rtype())
            }

            Error::OutOfRange(_robj) => write!(f, "Out of range."),
            Error::MustNotBeNA(_robj) => write!(f, "Must not be NA."),
            Error::ExpectedNonZeroLength(_robj) => write!(f, "Expected non zero length"),
            Error::OutOfLimits(robj) => write!(f, "The value is too big: {:?}", robj),
            Error::TypeMismatch(_robj) => write!(f, "Type mismatch"),

            Error::NamespaceNotFound(robj) => write!(f, "Namespace {:?} not found", robj),
            Error::ExpectedExternalPtrType(_robj, type_name) => {
                write!(f, "Incorrect external pointer type {}", type_name)
            }
            Error::ExpectedExternalNonNullPtr(robj) => {
                write!(
                    f,
                    "expected non-null pointer in externalptr, instead {:?}",
                    robj
                )
            }
            Error::ExpectedExternalPtrReference => {
                write!(f, "It is only possible to return a reference to self.")
            }
            Error::NoGraphicsDevices(_robj) => write!(f, "No graphics devices active."),
            Error::Other(str) => write!(f, "{}", str),

            Error::ExpectedWholeNumber(robj, conversion_error) => {
                write!(
                    f,
                    "Failed to convert a float to a whole number: {}. Actual value received: {:?}",
                    conversion_error, robj
                )
            }

            #[cfg(feature = "ndarray")]
            Error::NDArrayShapeError(shape_error) => {
                write!(f, "NDArray failed with error: {}.", shape_error)
            }

            #[cfg(feature = "either")]
            Error::EitherError(left_err, right_err) => {
                write!(
                    f,
                    "Both cases of Either errored. Left: '{}'; Right: '{}'.",
                    left_err, right_err
                )
            }
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Error {
        Error::Other(format!("{}", err))
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}

// NoneError is unstable.
//
// impl From<std::option::NoneError> for Error {
//     fn from(err: std::option::NoneError) -> Error {
//         Error::None
//     }
// }

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Other("".to_string())
    }
}
