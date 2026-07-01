use crate::wrapper::*;
// use crate::robj::GetSexp;
use crate::robj::Attributes;
use crate::robj::RAny;
use crate::robj::RInternals;
use crate::robj::RObj;
use crate::robj::Types;

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_missing_arg() {
            write!(f, "missing_arg()")
        } else {
            #[cfg(not(r_4_5))]
            if self.is_unbound_value() {
                return write!(f, "unbound_value()");
            }
            write!(f, "sym!({})", self.as_symbol().unwrap().as_str())
        }
    }
}

/// Implement {:?} formatting.
impl std::fmt::Debug for RObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_any() {
            RAny::Null(_) => write!(f, "()"),
            RAny::Symbol(value) => value.fmt(f),
            RAny::PairList(value) => value.fmt(f),
            RAny::Function(value) => value.fmt(f),
            RAny::Environment(value) => value.fmt(f),
            RAny::Promise(value) => value.fmt(f),
            RAny::Language(value) => value.fmt(f),
            RAny::Special(value) => value.fmt(f),
            RAny::Builtin(value) => value.fmt(f),
            RAny::RStr(value) => value.fmt(f),
            RAny::Logicals(value) => value.fmt(f),
            RAny::Integers(value) => value.fmt(f),
            RAny::Doubles(value) => value.fmt(f),
            RAny::Complexes(value) => value.fmt(f),
            RAny::Strings(value) => write!(f, "{:?}", value.as_slice()),
            RAny::Dot(_dot) => write!(f, "Dot"),
            RAny::Any(_any) => write!(f, "Any"),
            RAny::List(value) => value.fmt(f),
            RAny::Expressions(value) => value.fmt(f),
            RAny::Bytecode(_bytecode) => write!(f, "Bytecode"),
            RAny::ExternalPtr(_externalptr) => write!(f, "ExternalPtr"),
            RAny::WeakRef(_weakref) => write!(f, "Weakref"),
            RAny::Raw(value) => value.fmt(f),
            RAny::S4(value) => value.fmt(f),
            RAny::Unknown(_unknown) => write!(f, "Unknown"),
        }?;
        /*
               match self.sexptype() {
                   NILSXP => write!(f, "r!(NULL)"),
                   SYMSXP => {
                       if self.is_missing_arg() {
                           write!(f, "missing_arg()")
                       } else if self.is_unbound_value() {
                           write!(f, "unbound_value()")
                       } else {
                           write!(f, "sym!({})", self.as_symbol().unwrap().as_str())
                       }
                   }
                   LISTSXP => {
                       let pairlist = self.as_pairlist().unwrap().iter();
                       write!(f, "r!({:?})", pairlist)
                   }
                   CLOSXP => {
                       let func = self.as_function().unwrap();
                       let formals = func.formals();
                       let body = func.body();
                       let environment = func.environment();
                       write!(
                           f,
                           "r!(Function::from_parts({:?}, {:?}, {:?}))",
                           formals, body, environment
                       )
                   }
                   ENVSXP => unsafe {
                       let sexp = self.get();
                       if sexp == R_GlobalEnv {
                           write!(f, "global_env()")
                       } else if sexp == R_BaseEnv {
                           write!(f, "base_env()")
                       } else if sexp == R_EmptyEnv {
                           write!(f, "empty_env()")
                       } else {
                           write!(f, "r!(Environment::from_pairs(...))")
                       }
                   },
                   PROMSXP => {
                       let p = self.as_promise().unwrap();
                       write!(
                           f,
                           "r!(Promise::from_parts({:?}, {:?}))",
                           p.code(),
                           p.environment()
                       )
                   }
                   LANGSXP => write!(
                       f,
                       "r!(Language::from_values({:?}))",
                       self.as_language().unwrap().values().collect::<Vec<_>>()
                   ),
                   SPECIALSXP => write!(f, "r!(Special())"),
                   BUILTINSXP => write!(f, "r!(Builtin())"),
                   CHARSXP => {
                       let c = RStr::try_from(self.clone()).unwrap();
                       write!(f, "r!(RStr::from_string({:?}))", c.as_str())
                   }
                   LGLSXP => {
                       let slice = self.as_logical_slice().unwrap();
                       if slice.len() == 1 {
                           write!(f, "r!({:?})", slice[0])
                       } else {
                           write!(f, "r!({:?})", slice)
                       }
                   }
                   INTSXP => {
                       let slice = self.as_integer_slice().unwrap();
                       if slice.len() == 1 {
                           write!(f, "r!({:?})", slice[0])
                       } else {
                           write!(f, "r!({:?})", self.as_integer_slice().unwrap())
                       }
                   }
                   REALSXP => {
                       let slice = self.as_real_slice().unwrap();
                       if slice.len() == 1 {
                           write!(f, "r!({:?})", slice[0])
                       } else {
                           write!(f, "r!({:?})", slice)
                       }
                   }
                   VECSXP => {
                       let list = self.as_list().unwrap();
                       if self.names().is_some() {
                           write!(f, "r!(List::from_pairs({:?}))", list.iter())
                       } else {
                           write!(f, "r!(List::from_values({:?}))", list.values())
                       }
                   }
                   EXPRSXP => write!(
                       f,
                       "r!(Expressions::from_values({:?}))",
                       self.as_expressions().unwrap().values()
                   ),
                   WEAKREFSXP => write!(f, "r!(Weakref())"),
                   // CPLXSXP => false,
                   STRSXP => {
                       write!(f, "r!([")?;
                       let mut sep = "";
                       for s in self.as_str_iter().unwrap() {
                           // if s.is_na() {
                           //     write!(f, "{}na_str()", sep)?;
                           // } else {
                           write!(f, "{}{:?}", sep, s)?;
                           // }
                           sep = ", ";
                       }
                       write!(f, "])")
                   }
                   DOTSXP => write!(f, "r!(Dot())"),
                   ANYSXP => write!(f, "r!(Any())"),
                   BCODESXP => write!(f, "r!(Bcode())"),
                   EXTPTRSXP => {
                       write!(f, "r!(ExternalPtr())")
                   }
                   RAWSXP => {
                       write!(
                           f,
                           "r!(Raw::from_bytes({:?}))",
                           self.as_raw().unwrap().as_slice()
                       )
                   }
                   S4SXP => write!(f, "r!(S4())"),
                   NEWSXP => write!(f, "r!(New())"),
                   FREESXP => write!(f, "r!(Free())"),
                   _ => write!(f, "??"),
               }?;
        */
        if let Some(c) = self.class() {
            write!(f, ".set_class({:?}", c)?;
        }
        Ok(())
    }
}
