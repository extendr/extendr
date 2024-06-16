# Macro expansion of lib.rs

    Code
      cat(result$stdout)
    Output
      #![feature(prelude_import)]
      #[prelude_import]
      use std::prelude::rust_2021::*;
      #[macro_use]
      extern crate std;
      use extendr_api::{graphics::*, prelude::*};
      mod altrep {
          use extendr_api::prelude::*;
          pub struct VecUsize(pub Vec<Option<usize>>);
          #[automatically_derived]
          impl ::core::fmt::Debug for VecUsize {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_tuple_field1_finish(f, "VecUsize", &&self.0)
              }
          }
          #[automatically_derived]
          impl ::core::clone::Clone for VecUsize {
              #[inline]
              fn clone(&self) -> VecUsize {
                  VecUsize(::core::clone::Clone::clone(&self.0))
              }
          }
          impl AltrepImpl for VecUsize {
              fn length(&self) -> usize {
                  self.0.len()
              }
          }
          #[cfg(use_r_altlist)]
          impl VecUsize {}
          impl TryFrom<Robj> for &VecUsize {
              type Error = Error;
              fn try_from(robj: Robj) -> Result<Self> {
                  Self::try_from(&robj)
              }
          }
          impl TryFrom<Robj> for &mut VecUsize {
              type Error = Error;
              fn try_from(mut robj: Robj) -> Result<Self> {
                  Self::try_from(&mut robj)
              }
          }
          impl TryFrom<&Robj> for &VecUsize {
              type Error = Error;
              fn try_from(robj: &Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &ExternalPtr<VecUsize> = robj.try_into()?;
                      external_ptr.try_addr()
                  }
              }
          }
          impl TryFrom<&mut Robj> for &mut VecUsize {
              type Error = Error;
              fn try_from(robj: &mut Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &mut ExternalPtr<VecUsize> = robj.try_into()?;
                      external_ptr.try_addr_mut()
                  }
              }
          }
          impl From<VecUsize> for Robj {
              fn from(value: VecUsize) -> Self {
                  unsafe {
                      let mut res: ExternalPtr<VecUsize> = ExternalPtr::new(value);
                      res.set_attrib(class_symbol(), "VecUsize").unwrap();
                      res.into()
                  }
              }
          }
          #[allow(non_snake_case)]
          fn meta__VecUsize(impls: &mut Vec<extendr_api::metadata::Impl>) {
              let mut methods = Vec::new();
              impls
                  .push(extendr_api::metadata::Impl {
                      doc: "",
                      name: "VecUsize",
                      methods,
                  });
          }
          #[cfg(use_r_altlist)]
          impl AltListImpl for VecUsize {
              fn elt(&self, index: usize) -> Robj {
                  Self(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([self.0[index]])))
                      .into()
              }
          }
          #[cfg(use_r_altlist)]
          fn new_usize(robj: Integers) -> Altrep {
              let x = robj
                  .iter()
                  .map(|x| match &x {
                      _ if x.is_na() => None,
                      _ if x.inner() < 0 => None,
                      _ => Some(x.inner() as usize),
                  })
                  .collect();
              let obj = VecUsize(x);
              let class = Altrep::make_altlist_class::<VecUsize>("li", "mypkg");
              Altrep::from_state_and_class(obj, class, false)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__new_usize(robj: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _robj_robj = extendr_api::robj::Robj::from_sexp(robj);
                          Ok(extendr_api::Robj::from(new_usize(_robj_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "new_usize"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__new_usize(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "robj",
                          arg_type: "Integers",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "new_usize",
                      r_name: "new_usize",
                      mod_name: "new_usize",
                      args: args,
                      return_type: "Altrep",
                      func_ptr: wrap__new_usize as *const u8,
                      hidden: false,
                  })
          }
          struct StringInts {
              len: usize,
          }
          #[automatically_derived]
          impl ::core::fmt::Debug for StringInts {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_struct_field1_finish(
                      f,
                      "StringInts",
                      "len",
                      &&self.len,
                  )
              }
          }
          #[automatically_derived]
          impl ::core::clone::Clone for StringInts {
              #[inline]
              fn clone(&self) -> StringInts {
                  StringInts {
                      len: ::core::clone::Clone::clone(&self.len),
                  }
              }
          }
          impl AltrepImpl for StringInts {
              fn length(&self) -> usize {
                  self.len as usize
              }
          }
          impl AltStringImpl for StringInts {
              fn elt(&self, index: usize) -> Rstr {
                  {
                      let res = ::alloc::fmt::format(format_args!("{0}", index));
                      res
                  }
                      .into()
              }
          }
          fn tst_altstring() -> Altrep {
              let mystate = StringInts { len: 10 };
              let class = Altrep::make_altstring_class::<StringInts>("si", "mypkg");
              Altrep::from_state_and_class(mystate, class, false)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__tst_altstring() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(tst_altstring())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "tst_altstring"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__tst_altstring(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "tst_altstring",
                      r_name: "tst_altstring",
                      mod_name: "tst_altstring",
                      args: args,
                      return_type: "Altrep",
                      func_ptr: wrap__tst_altstring as *const u8,
                      hidden: false,
                  })
          }
          struct MyCompactIntRange {
              start: i32,
              len: i32,
              step: i32,
              missing_index: usize,
          }
          #[automatically_derived]
          impl ::core::fmt::Debug for MyCompactIntRange {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_struct_field4_finish(
                      f,
                      "MyCompactIntRange",
                      "start",
                      &self.start,
                      "len",
                      &self.len,
                      "step",
                      &self.step,
                      "missing_index",
                      &&self.missing_index,
                  )
              }
          }
          #[automatically_derived]
          impl ::core::clone::Clone for MyCompactIntRange {
              #[inline]
              fn clone(&self) -> MyCompactIntRange {
                  MyCompactIntRange {
                      start: ::core::clone::Clone::clone(&self.start),
                      len: ::core::clone::Clone::clone(&self.len),
                      step: ::core::clone::Clone::clone(&self.step),
                      missing_index: ::core::clone::Clone::clone(&self.missing_index),
                  }
              }
          }
          impl AltrepImpl for MyCompactIntRange {
              fn length(&self) -> usize {
                  self.len as usize
              }
          }
          impl AltIntegerImpl for MyCompactIntRange {
              fn elt(&self, index: usize) -> Rint {
                  if index == self.missing_index {
                      Rint::na()
                  } else {
                      Rint::new(self.start + self.step * index as i32)
                  }
              }
          }
          fn tst_altinteger() -> Altrep {
              let mystate = MyCompactIntRange {
                  start: 0,
                  len: 10,
                  step: 1,
                  missing_index: usize::MAX,
              };
              let class = Altrep::make_altinteger_class::<MyCompactIntRange>("cir", "mypkg");
              Altrep::from_state_and_class(mystate, class.clone(), false)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__tst_altinteger() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(tst_altinteger())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "tst_altinteger"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__tst_altinteger(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "tst_altinteger",
                      r_name: "tst_altinteger",
                      mod_name: "tst_altinteger",
                      args: args,
                      return_type: "Altrep",
                      func_ptr: wrap__tst_altinteger as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_altrep_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__new_usize(&mut functions);
              meta__tst_altstring(&mut functions);
              meta__tst_altinteger(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_altrep_metadata",
                      mod_name: "get_altrep_metadata",
                      r_name: "get_altrep_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_altrep_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_altrep_wrappers",
                      mod_name: "make_altrep_wrappers",
                      r_name: "make_altrep_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_altrep_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "altrep",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_altrep_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_altrep_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_altrep_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_altrep_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_altrep_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_altrep_metadata()) };
          }
      }
      mod attributes {
          use extendr_api::prelude::*;
          fn dbls_named(mut x: Doubles) -> Doubles {
              x.set_attrib(
                      "names",
                      x.iter().map(|xi| xi.inner().to_string()).collect::<Vec<_>>(),
                  )
                  .unwrap();
              x
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__dbls_named(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(dbls_named(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "dbls_named"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__dbls_named(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Doubles",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "dbls_named",
                      r_name: "dbls_named",
                      mod_name: "dbls_named",
                      args: args,
                      return_type: "Doubles",
                      func_ptr: wrap__dbls_named as *const u8,
                      hidden: false,
                  })
          }
          fn strings_named(mut x: Strings) -> Strings {
              x.set_attrib(
                      "names",
                      x.iter().map(|xi| xi.as_str().to_string()).collect::<Vec<_>>(),
                  )
                  .unwrap();
              x
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__strings_named(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(strings_named(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "strings_named"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__strings_named(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Strings",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "strings_named",
                      r_name: "strings_named",
                      mod_name: "strings_named",
                      args: args,
                      return_type: "Strings",
                      func_ptr: wrap__strings_named as *const u8,
                      hidden: false,
                  })
          }
          fn list_named(mut x: List, nms: Strings) -> List {
              let _ = x.set_attrib("names", nms);
              x
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__list_named(
              x: extendr_api::SEXP,
              nms: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          let _nms_robj = extendr_api::robj::Robj::from_sexp(nms);
                          Ok(
                              extendr_api::Robj::from(
                                  list_named(_x_robj.try_into()?, _nms_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "list_named"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__list_named(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "List",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "nms",
                          arg_type: "Strings",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "list_named",
                      r_name: "list_named",
                      mod_name: "list_named",
                      args: args,
                      return_type: "List",
                      func_ptr: wrap__list_named as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_attributes_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__dbls_named(&mut functions);
              meta__strings_named(&mut functions);
              meta__list_named(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_attributes_metadata",
                      mod_name: "get_attributes_metadata",
                      r_name: "get_attributes_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_attributes_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_attributes_wrappers",
                      mod_name: "make_attributes_wrappers",
                      r_name: "make_attributes_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_attributes_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "attributes",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_attributes_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_attributes_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_attributes_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_attributes_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_attributes_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_attributes_metadata()) };
          }
      }
      mod dataframe {
          use extendr_api::prelude::*;
          struct MyStruct {
              x: Rint,
              y: Rstr,
          }
          #[automatically_derived]
          impl ::core::fmt::Debug for MyStruct {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_struct_field2_finish(
                      f,
                      "MyStruct",
                      "x",
                      &self.x,
                      "y",
                      &&self.y,
                  )
              }
          }
          impl IntoDataFrameRow<MyStruct> for Vec<MyStruct> {
              fn into_dataframe(self) -> Result<Dataframe<MyStruct>> {
                  let mut x = Vec::with_capacity(self.len());
                  let mut y = Vec::with_capacity(self.len());
                  for val in self {
                      x.push(val.x);
                      y.push(val.y);
                  }
                  let caller = eval_string("data.frame")?;
                  let res = caller
                      .call(
                          Pairlist::from_pairs(
                              &[
                                  ("x", extendr_api::robj::Robj::from(x)),
                                  ("y", extendr_api::robj::Robj::from(y)),
                              ],
                          ),
                      )?;
                  res.try_into()
              }
          }
          impl<I> IntoDataFrameRow<MyStruct> for (I,)
          where
              I: ExactSizeIterator<Item = MyStruct>,
          {
              /// Thanks to RFC 2451, we need to wrap a generic iterator in a tuple!
              fn into_dataframe(self) -> Result<Dataframe<MyStruct>> {
                  let mut x = Vec::with_capacity(self.0.len());
                  let mut y = Vec::with_capacity(self.0.len());
                  for val in self.0 {
                      x.push(val.x);
                      y.push(val.y);
                  }
                  let caller = eval_string("data.frame")?;
                  let res = caller
                      .call(
                          Pairlist::from_pairs(
                              &[
                                  ("x", extendr_api::robj::Robj::from(x)),
                                  ("y", extendr_api::robj::Robj::from(y)),
                              ],
                          ),
                      )?;
                  res.try_into()
              }
          }
          fn test_derive_into_dataframe() -> Dataframe<MyStruct> {
              let v = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      MyStruct {
                          x: 0.into(),
                          y: "abc".into(),
                      },
                      MyStruct {
                          x: 1.into(),
                          y: "xyz".into(),
                      },
                  ]),
              );
              v.into_dataframe().unwrap()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__test_derive_into_dataframe() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(test_derive_into_dataframe())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "test_derive_into_dataframe",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__test_derive_into_dataframe(
              metadata: &mut Vec<extendr_api::metadata::Func>,
          ) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "test_derive_into_dataframe",
                      r_name: "test_derive_into_dataframe",
                      mod_name: "test_derive_into_dataframe",
                      args: args,
                      return_type: "Dataframe",
                      func_ptr: wrap__test_derive_into_dataframe as *const u8,
                      hidden: false,
                  })
          }
          fn test_into_robj_dataframe() -> Robj {
              let v = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      MyStruct {
                          x: 0.into(),
                          y: "abc".into(),
                      },
                      MyStruct {
                          x: 1.into(),
                          y: "xyz".into(),
                      },
                  ]),
              );
              v.into_dataframe().unwrap().into_robj()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__test_into_robj_dataframe() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(test_into_robj_dataframe())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "test_into_robj_dataframe",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__test_into_robj_dataframe(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "test_into_robj_dataframe",
                      r_name: "test_into_robj_dataframe",
                      mod_name: "test_into_robj_dataframe",
                      args: args,
                      return_type: "Robj",
                      func_ptr: wrap__test_into_robj_dataframe as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_dataframe_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__test_derive_into_dataframe(&mut functions);
              meta__test_into_robj_dataframe(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_dataframe_metadata",
                      mod_name: "get_dataframe_metadata",
                      r_name: "get_dataframe_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_dataframe_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_dataframe_wrappers",
                      mod_name: "make_dataframe_wrappers",
                      r_name: "make_dataframe_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_dataframe_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "dataframe",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_dataframe_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_dataframe_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_dataframe_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_dataframe_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_dataframe_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_dataframe_metadata()) };
          }
      }
      mod externalptr {
          use extendr_api::prelude::*;
          struct Wrapper {
              a: i32,
          }
          #[automatically_derived]
          impl ::core::default::Default for Wrapper {
              #[inline]
              fn default() -> Wrapper {
                  Wrapper {
                      a: ::core::default::Default::default(),
                  }
              }
          }
          #[automatically_derived]
          impl ::core::fmt::Debug for Wrapper {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_struct_field1_finish(
                      f,
                      "Wrapper",
                      "a",
                      &&self.a,
                  )
              }
          }
          #[automatically_derived]
          impl ::core::clone::Clone for Wrapper {
              #[inline]
              fn clone(&self) -> Wrapper {
                  let _: ::core::clone::AssertParamIsClone<i32>;
                  *self
              }
          }
          #[automatically_derived]
          impl ::core::marker::Copy for Wrapper {}
          /// Class for testing (exported)
          /// @examples
          /// x <- Wrapper$new()
          /// x$a()
          /// x$set_a(10)
          /// x$a()
          /// @export
          impl Wrapper {
              /// Method for making a new object.
              fn new() -> Self {
                  Self { a: 0 }
              }
              /// Method for setting stuff.
              /// @param x a number
              fn set_a(&mut self, x: i32) {
                  self.a = x;
              }
              /// Method for getting stuff.
              fn a(&self) -> i32 {
                  self.a
              }
              /// Method for getting one's (by way of a copy) self.
              fn me_owned(&self) -> Self {
                  *self
              }
              /// Method for getting one's (ref) self.
              fn me_ref(&self) -> &Self {
                  self
              }
              /// Method for getting one's (ref mut) self.
              fn me_mut(&mut self) -> &mut Self {
                  self
              }
              /// Method for getting one's ref (explicit) self.
              fn me_explicit_ref(&self) -> &Wrapper {
                  self
              }
              /// Method for getting one's ref mut (explicit) self.
              fn me_explicit_mut(&mut self) -> &mut Wrapper {
                  self
              }
              fn max_ref(&self, other: &'static Wrapper) -> &Self {
                  if self.a > other.a { self } else { other }
              }
              /// `offset` does nothing.
              fn max_ref_offset(&self, other: &'static Wrapper, _offset: i32) -> &Self {
                  if self.a > other.a { self } else { other }
              }
              fn max_ref2(&self, other: &'static Self) -> &Self {
                  if self.a > other.a { self } else { other }
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__new() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(<Wrapper>::new())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "new"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__new(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for making a new object.",
                      rust_name: "new",
                      r_name: "new",
                      mod_name: "new",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__new as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__set_a(
              _self: extendr_api::SEXP,
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  extendr_api::unwrap_or_throw_error(
                                          <&mut Wrapper>::try_from(&mut _self_robj),
                                      )
                                      .set_a(_x_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "set_a"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__set_a(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "i32",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for setting stuff.\n @param x a number",
                      rust_name: "set_a",
                      r_name: "set_a",
                      mod_name: "set_a",
                      args: args,
                      return_type: "()",
                      func_ptr: wrap__Wrapper__set_a as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__a(_self: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          Ok(
                              extendr_api::Robj::from(
                                  extendr_api::unwrap_or_throw_error(
                                          <&Wrapper>::try_from(&_self_robj),
                                      )
                                      .a(),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "a"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__a(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting stuff.",
                      rust_name: "a",
                      r_name: "a",
                      mod_name: "a",
                      args: args,
                      return_type: "i32",
                      func_ptr: wrap__Wrapper__a as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__me_owned(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          Ok(
                              extendr_api::Robj::from(
                                  extendr_api::unwrap_or_throw_error(
                                          <&Wrapper>::try_from(&_self_robj),
                                      )
                                      .me_owned(),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "me_owned"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__me_owned(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting one's (by way of a copy) self.",
                      rust_name: "me_owned",
                      r_name: "me_owned",
                      mod_name: "me_owned",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__me_owned as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__me_ref(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&Wrapper>::try_from(&_self_robj),
                              )
                              .me_ref();
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "me_ref"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__me_ref(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting one's (ref) self.",
                      rust_name: "me_ref",
                      r_name: "me_ref",
                      mod_name: "me_ref",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__me_ref as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__me_mut(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&mut Wrapper>::try_from(&mut _self_robj),
                              )
                              .me_mut();
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "me_mut"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__me_mut(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting one's (ref mut) self.",
                      rust_name: "me_mut",
                      r_name: "me_mut",
                      mod_name: "me_mut",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__me_mut as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__me_explicit_ref(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&Wrapper>::try_from(&_self_robj),
                              )
                              .me_explicit_ref();
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "me_explicit_ref"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__me_explicit_ref(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting one's ref (explicit) self.",
                      rust_name: "me_explicit_ref",
                      r_name: "me_explicit_ref",
                      mod_name: "me_explicit_ref",
                      args: args,
                      return_type: "Wrapper",
                      func_ptr: wrap__Wrapper__me_explicit_ref as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__me_explicit_mut(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&mut Wrapper>::try_from(&mut _self_robj),
                              )
                              .me_explicit_mut();
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "me_explicit_mut"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__me_explicit_mut(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting one's ref mut (explicit) self.",
                      rust_name: "me_explicit_mut",
                      r_name: "me_explicit_mut",
                      mod_name: "me_explicit_mut",
                      args: args,
                      return_type: "Wrapper",
                      func_ptr: wrap__Wrapper__me_explicit_mut as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__max_ref(
              _self: extendr_api::SEXP,
              other: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let _other_robj = extendr_api::robj::Robj::from_sexp(other);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&Wrapper>::try_from(&_self_robj),
                              )
                              .max_ref(_other_robj.try_into()?);
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(other),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(other));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "max_ref"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__max_ref(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "other",
                          arg_type: "Wrapper",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "max_ref",
                      r_name: "max_ref",
                      mod_name: "max_ref",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__max_ref as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__max_ref_offset(
              _self: extendr_api::SEXP,
              other: extendr_api::SEXP,
              _offset: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let _other_robj = extendr_api::robj::Robj::from_sexp(other);
                          let __offset_robj = extendr_api::robj::Robj::from_sexp(_offset);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&Wrapper>::try_from(&_self_robj),
                              )
                              .max_ref_offset(
                                  _other_robj.try_into()?,
                                  __offset_robj.try_into()?,
                              );
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(other),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(other));
                          }
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_offset),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_offset));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "max_ref_offset"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__max_ref_offset(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "other",
                          arg_type: "Wrapper",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "_offset",
                          arg_type: "i32",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " `offset` does nothing.",
                      rust_name: "max_ref_offset",
                      r_name: "max_ref_offset",
                      mod_name: "max_ref_offset",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__max_ref_offset as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__Wrapper__max_ref2(
              _self: extendr_api::SEXP,
              other: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let _other_robj = extendr_api::robj::Robj::from_sexp(other);
                          let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                                  <&Wrapper>::try_from(&_self_robj),
                              )
                              .max_ref2(_other_robj.try_into()?);
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(_self),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(_self));
                          }
                          if std::ptr::addr_eq(
                              extendr_api::R_ExternalPtrAddr(other),
                              std::ptr::from_ref(return_ref_to_self),
                          ) {
                              return Ok(extendr_api::Robj::from_sexp(other));
                          }
                          Err(Error::ExpectedExternalPtrReference)
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "max_ref2"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper__max_ref2(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "Wrapper",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "other",
                          arg_type: "Self",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "max_ref2",
                      r_name: "max_ref2",
                      mod_name: "max_ref2",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__Wrapper__max_ref2 as *const u8,
                      hidden: false,
                  })
          }
          impl TryFrom<Robj> for &Wrapper {
              type Error = Error;
              fn try_from(robj: Robj) -> Result<Self> {
                  Self::try_from(&robj)
              }
          }
          impl TryFrom<Robj> for &mut Wrapper {
              type Error = Error;
              fn try_from(mut robj: Robj) -> Result<Self> {
                  Self::try_from(&mut robj)
              }
          }
          impl TryFrom<&Robj> for &Wrapper {
              type Error = Error;
              fn try_from(robj: &Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &ExternalPtr<Wrapper> = robj.try_into()?;
                      external_ptr.try_addr()
                  }
              }
          }
          impl TryFrom<&mut Robj> for &mut Wrapper {
              type Error = Error;
              fn try_from(robj: &mut Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &mut ExternalPtr<Wrapper> = robj.try_into()?;
                      external_ptr.try_addr_mut()
                  }
              }
          }
          impl From<Wrapper> for Robj {
              fn from(value: Wrapper) -> Self {
                  unsafe {
                      let mut res: ExternalPtr<Wrapper> = ExternalPtr::new(value);
                      res.set_attrib(class_symbol(), "Wrapper").unwrap();
                      res.into()
                  }
              }
          }
          #[allow(non_snake_case)]
          fn meta__Wrapper(impls: &mut Vec<extendr_api::metadata::Impl>) {
              let mut methods = Vec::new();
              meta__Wrapper__new(&mut methods);
              meta__Wrapper__set_a(&mut methods);
              meta__Wrapper__a(&mut methods);
              meta__Wrapper__me_owned(&mut methods);
              meta__Wrapper__me_ref(&mut methods);
              meta__Wrapper__me_mut(&mut methods);
              meta__Wrapper__me_explicit_ref(&mut methods);
              meta__Wrapper__me_explicit_mut(&mut methods);
              meta__Wrapper__max_ref(&mut methods);
              meta__Wrapper__max_ref_offset(&mut methods);
              meta__Wrapper__max_ref2(&mut methods);
              impls
                  .push(extendr_api::metadata::Impl {
                      doc: " Class for testing (exported)\n @examples\n x <- Wrapper$new()\n x$a()\n x$set_a(10)\n x$a()\n @export",
                      name: "Wrapper",
                      methods,
                  });
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_externalptr_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__Wrapper(&mut impls);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_externalptr_metadata",
                      mod_name: "get_externalptr_metadata",
                      r_name: "get_externalptr_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_externalptr_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_externalptr_wrappers",
                      mod_name: "make_externalptr_wrappers",
                      r_name: "make_externalptr_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_externalptr_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "externalptr",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_externalptr_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_externalptr_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_externalptr_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_externalptr_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_externalptr_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_externalptr_metadata()) };
          }
      }
      mod graphic_device {
          use extendr_api::{graphics::*, prelude::*};
          pub(crate) struct MyDevice<'a> {
              pub(crate) welcome_message: &'a str,
          }
          impl<'a> DeviceDriver for MyDevice<'a> {
              fn activate(&mut self, _dd: DevDesc) {
                  let welcome_message = self.welcome_message;
                  print_r_output({
                      let res = ::alloc::fmt::format(
                          format_args!("message from device: {0}", welcome_message),
                      );
                      res
                  });
                  print_r_output("\n");
              }
              fn close(&mut self, _dd: DevDesc) {
                  print_r_output({
                      let res = ::alloc::fmt::format(format_args!("good bye..."));
                      res
                  });
                  print_r_output("\n");
              }
          }
      }
      mod memory_leaks {
          use extendr_api::prelude::*;
          fn leak_arg2_try_implicit_strings(_y: Doubles, x: Strings) -> String {
              x.len().to_string()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_arg2_try_implicit_strings(
              _y: extendr_api::SEXP,
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let __y_robj = extendr_api::robj::Robj::from_sexp(_y);
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  leak_arg2_try_implicit_strings(
                                      __y_robj.try_into()?,
                                      _x_robj.try_into()?,
                                  ),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_arg2_try_implicit_strings",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_arg2_try_implicit_strings(
              metadata: &mut Vec<extendr_api::metadata::Func>,
          ) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "_y",
                          arg_type: "Doubles",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Strings",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_arg2_try_implicit_strings",
                      r_name: "leak_arg2_try_implicit_strings",
                      mod_name: "leak_arg2_try_implicit_strings",
                      args: args,
                      return_type: "String",
                      func_ptr: wrap__leak_arg2_try_implicit_strings as *const u8,
                      hidden: false,
                  })
          }
          fn leak_arg2_try_implicit_doubles(_y: Doubles, x: Doubles) -> String {
              x.len().to_string()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_arg2_try_implicit_doubles(
              _y: extendr_api::SEXP,
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let __y_robj = extendr_api::robj::Robj::from_sexp(_y);
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  leak_arg2_try_implicit_doubles(
                                      __y_robj.try_into()?,
                                      _x_robj.try_into()?,
                                  ),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_arg2_try_implicit_doubles",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_arg2_try_implicit_doubles(
              metadata: &mut Vec<extendr_api::metadata::Func>,
          ) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "_y",
                          arg_type: "Doubles",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Doubles",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_arg2_try_implicit_doubles",
                      r_name: "leak_arg2_try_implicit_doubles",
                      mod_name: "leak_arg2_try_implicit_doubles",
                      args: args,
                      return_type: "String",
                      func_ptr: wrap__leak_arg2_try_implicit_doubles as *const u8,
                      hidden: false,
                  })
          }
          fn leak_unwrap_strings(x: Robj) -> String {
              let x = x.as_string_vector().ok_or("ERROR").unwrap();
              x.len().to_string()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_unwrap_strings(
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(leak_unwrap_strings(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_unwrap_strings",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_unwrap_strings(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Robj",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_unwrap_strings",
                      r_name: "leak_unwrap_strings",
                      mod_name: "leak_unwrap_strings",
                      args: args,
                      return_type: "String",
                      func_ptr: wrap__leak_unwrap_strings as *const u8,
                      hidden: false,
                  })
          }
          fn leak_unwrap_doubles(x: Robj) -> String {
              x.as_real_vector().ok_or("ERROR").unwrap().len().to_string()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_unwrap_doubles(
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(leak_unwrap_doubles(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_unwrap_doubles",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_unwrap_doubles(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Robj",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_unwrap_doubles",
                      r_name: "leak_unwrap_doubles",
                      mod_name: "leak_unwrap_doubles",
                      args: args,
                      return_type: "String",
                      func_ptr: wrap__leak_unwrap_doubles as *const u8,
                      hidden: false,
                  })
          }
          fn leak_positive_control(x: Robj) {
              std::mem::forget(x);
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_positive_control(
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  leak_positive_control(_x_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_positive_control",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_positive_control(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Robj",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_positive_control",
                      r_name: "leak_positive_control",
                      mod_name: "leak_positive_control",
                      args: args,
                      return_type: "()",
                      func_ptr: wrap__leak_positive_control as *const u8,
                      hidden: false,
                  })
          }
          fn leak_negative_control(x: Robj) {
              drop(x)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__leak_negative_control(
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  leak_negative_control(_x_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "leak_negative_control",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__leak_negative_control(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Robj",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "leak_negative_control",
                      r_name: "leak_negative_control",
                      mod_name: "leak_negative_control",
                      args: args,
                      return_type: "()",
                      func_ptr: wrap__leak_negative_control as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_memory_leaks_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__leak_arg2_try_implicit_strings(&mut functions);
              meta__leak_arg2_try_implicit_doubles(&mut functions);
              meta__leak_unwrap_strings(&mut functions);
              meta__leak_unwrap_doubles(&mut functions);
              meta__leak_positive_control(&mut functions);
              meta__leak_negative_control(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_memory_leaks_metadata",
                      mod_name: "get_memory_leaks_metadata",
                      r_name: "get_memory_leaks_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_memory_leaks_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_memory_leaks_wrappers",
                      mod_name: "make_memory_leaks_wrappers",
                      r_name: "make_memory_leaks_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_memory_leaks_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "memory_leaks",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_memory_leaks_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_memory_leaks_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_memory_leaks_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_memory_leaks_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_memory_leaks_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_memory_leaks_metadata()) };
          }
      }
      mod optional_either {
          use extendr_api::prelude::*;
          fn type_aware_sum(input: Either<Integers, Doubles>) -> Either<Rint, Rfloat> {
              match input {
                  Left(left) => Left(left.iter().sum()),
                  Right(right) => Right(right.iter().sum()),
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__type_aware_sum(
              input: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _input_robj = extendr_api::robj::Robj::from_sexp(input);
                          Ok(extendr_api::Robj::from(type_aware_sum(_input_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "type_aware_sum"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__type_aware_sum(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "input",
                          arg_type: "Either",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "type_aware_sum",
                      r_name: "type_aware_sum",
                      mod_name: "type_aware_sum",
                      args: args,
                      return_type: "Either",
                      func_ptr: wrap__type_aware_sum as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_optional_either_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__type_aware_sum(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_optional_either_metadata",
                      mod_name: "get_optional_either_metadata",
                      r_name: "get_optional_either_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_optional_either_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_optional_either_wrappers",
                      mod_name: "make_optional_either_wrappers",
                      r_name: "make_optional_either_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_optional_either_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "optional_either",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_optional_either_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_optional_either_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_optional_either_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_optional_either_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_optional_either_extendr(info: *mut extendr_api::DllInfo) {
              unsafe {
                  extendr_api::register_call_methods(info, get_optional_either_metadata())
              };
          }
      }
      mod optional_faer {
          use extendr_api::prelude::*;
          use faer::{Mat, MatRef};
          fn mat_to_mat(x: Mat<f64>) -> Mat<f64> {
              x
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__mat_to_mat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(mat_to_mat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "mat_to_mat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__mat_to_mat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Mat",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "mat_to_mat",
                      r_name: "mat_to_mat",
                      mod_name: "mat_to_mat",
                      args: args,
                      return_type: "Mat",
                      func_ptr: wrap__mat_to_mat as *const u8,
                      hidden: false,
                  })
          }
          fn mat_to_rmat(x: Mat<f64>) -> RMatrix<f64> {
              RMatrix::<f64>::from(x)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__mat_to_rmat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(mat_to_rmat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "mat_to_rmat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__mat_to_rmat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Mat",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "mat_to_rmat",
                      r_name: "mat_to_rmat",
                      mod_name: "mat_to_rmat",
                      args: args,
                      return_type: "RMatrix",
                      func_ptr: wrap__mat_to_rmat as *const u8,
                      hidden: false,
                  })
          }
          fn mat_to_robj(x: Mat<f64>) -> Robj {
              x.into_robj()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__mat_to_robj(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(mat_to_robj(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "mat_to_robj"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__mat_to_robj(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Mat",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "mat_to_robj",
                      r_name: "mat_to_robj",
                      mod_name: "mat_to_robj",
                      args: args,
                      return_type: "Robj",
                      func_ptr: wrap__mat_to_robj as *const u8,
                      hidden: false,
                  })
          }
          fn mat_to_rmatfloat(x: Mat<f64>) -> RMatrix<Rfloat> {
              RMatrix::<Rfloat>::from(x)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__mat_to_rmatfloat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(mat_to_rmatfloat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "mat_to_rmatfloat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__mat_to_rmatfloat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Mat",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "mat_to_rmatfloat",
                      r_name: "mat_to_rmatfloat",
                      mod_name: "mat_to_rmatfloat",
                      args: args,
                      return_type: "RMatrix",
                      func_ptr: wrap__mat_to_rmatfloat as *const u8,
                      hidden: false,
                  })
          }
          fn rmat_to_mat(x: RMatrix<f64>) -> Mat<f64> {
              Mat::<f64>::from(x)
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__rmat_to_mat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(rmat_to_mat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "rmat_to_mat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__rmat_to_mat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "RMatrix",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "rmat_to_mat",
                      r_name: "rmat_to_mat",
                      mod_name: "rmat_to_mat",
                      args: args,
                      return_type: "Mat",
                      func_ptr: wrap__rmat_to_mat as *const u8,
                      hidden: false,
                  })
          }
          fn robj_to_mat(x: Robj) -> Mat<f64> {
              Mat::<f64>::try_from(x).unwrap()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__robj_to_mat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(robj_to_mat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "robj_to_mat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__robj_to_mat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "Robj",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "robj_to_mat",
                      r_name: "robj_to_mat",
                      mod_name: "robj_to_mat",
                      args: args,
                      return_type: "Mat",
                      func_ptr: wrap__robj_to_mat as *const u8,
                      hidden: false,
                  })
          }
          fn matref_to_mat(x: MatRef<'_, f64>) -> Robj {
              RMatrix::<f64>::from(x).into()
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__matref_to_mat(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(matref_to_mat(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "matref_to_mat"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__matref_to_mat(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "MatRef",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "matref_to_mat",
                      r_name: "matref_to_mat",
                      mod_name: "matref_to_mat",
                      args: args,
                      return_type: "Robj",
                      func_ptr: wrap__matref_to_mat as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_optional_faer_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__mat_to_mat(&mut functions);
              meta__mat_to_rmat(&mut functions);
              meta__mat_to_robj(&mut functions);
              meta__mat_to_rmatfloat(&mut functions);
              meta__rmat_to_mat(&mut functions);
              meta__robj_to_mat(&mut functions);
              meta__matref_to_mat(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_optional_faer_metadata",
                      mod_name: "get_optional_faer_metadata",
                      r_name: "get_optional_faer_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_optional_faer_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_optional_faer_wrappers",
                      mod_name: "make_optional_faer_wrappers",
                      r_name: "make_optional_faer_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_optional_faer_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "optional_faer",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_optional_faer_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_optional_faer_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_optional_faer_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_optional_faer_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_optional_faer_extendr(info: *mut extendr_api::DllInfo) {
              unsafe {
                  extendr_api::register_call_methods(info, get_optional_faer_metadata())
              };
          }
      }
      mod optional_ndarray {
          use extendr_api::prelude::*;
          /// Calculate Euclidean distance matrix
          /// Test case adopted from https://github.com/mikemahoney218/examplerust/blob/23d21b1ced4e24b7a7c00dd36290114dc1bbd113/src/rust/src/lib.rs#L5
          /// @param a : Matrix of real values or `NULL`
          /// @export
          fn euclidean_dist(a: Nullable<ArrayView2<Rfloat>>) -> Nullable<Doubles> {
              if let NotNull(a) = a {
                  let nrow = a.nrows();
                  let result = (0..(nrow - 1))
                      .map(|x| {
                          ((x + 1)..nrow)
                              .map(move |y| {
                                  let z = &a
                                      .slice(
                                          match x {
                                              r => {
                                                  match .. {
                                                      r => {
                                                          let in_dim = ::ndarray::SliceNextDim::next_in_dim(
                                                              &r,
                                                              ::ndarray::SliceNextDim::next_in_dim(
                                                                  &r,
                                                                  ::core::marker::PhantomData::<::ndarray::Ix0>,
                                                              ),
                                                          );
                                                          let out_dim = ::ndarray::SliceNextDim::next_out_dim(
                                                              &r,
                                                              ::ndarray::SliceNextDim::next_out_dim(
                                                                  &r,
                                                                  ::core::marker::PhantomData::<::ndarray::Ix0>,
                                                              ),
                                                          );
                                                          #[allow(unsafe_code)]
                                                          unsafe {
                                                              ::ndarray::SliceInfo::new_unchecked(
                                                                  [
                                                                      <::ndarray::SliceInfoElem as ::core::convert::From<
                                                                          _,
                                                                      >>::from(r),
                                                                      <::ndarray::SliceInfoElem as ::core::convert::From<
                                                                          _,
                                                                      >>::from(r),
                                                                  ],
                                                                  in_dim,
                                                                  out_dim,
                                                              )
                                                          }
                                                      }
                                                  }
                                              }
                                          },
                                      )
                                      - &a
                                          .slice(
                                              match y {
                                                  r => {
                                                      match .. {
                                                          r => {
                                                              let in_dim = ::ndarray::SliceNextDim::next_in_dim(
                                                                  &r,
                                                                  ::ndarray::SliceNextDim::next_in_dim(
                                                                      &r,
                                                                      ::core::marker::PhantomData::<::ndarray::Ix0>,
                                                                  ),
                                                              );
                                                              let out_dim = ::ndarray::SliceNextDim::next_out_dim(
                                                                  &r,
                                                                  ::ndarray::SliceNextDim::next_out_dim(
                                                                      &r,
                                                                      ::core::marker::PhantomData::<::ndarray::Ix0>,
                                                                  ),
                                                              );
                                                              #[allow(unsafe_code)]
                                                              unsafe {
                                                                  ::ndarray::SliceInfo::new_unchecked(
                                                                      [
                                                                          <::ndarray::SliceInfoElem as ::core::convert::From<
                                                                              _,
                                                                          >>::from(r),
                                                                          <::ndarray::SliceInfoElem as ::core::convert::From<
                                                                              _,
                                                                          >>::from(r),
                                                                      ],
                                                                      in_dim,
                                                                      out_dim,
                                                                  )
                                                              }
                                                          }
                                                      }
                                                  }
                                              },
                                          );
                                  (&z * &z).iter().sum::<Rfloat>().sqrt()
                              })
                      })
                      .flatten()
                      .collect();
                  Nullable::NotNull(result)
              } else {
                  Nullable::Null
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__euclidean_dist(a: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _a_robj = extendr_api::robj::Robj::from_sexp(a);
                          Ok(extendr_api::Robj::from(euclidean_dist(_a_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "euclidean_dist"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__euclidean_dist(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "a",
                          arg_type: "Nullable",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Calculate Euclidean distance matrix\n Test case adopted from https://github.com/mikemahoney218/examplerust/blob/23d21b1ced4e24b7a7c00dd36290114dc1bbd113/src/rust/src/lib.rs#L5\n @param a : Matrix of real values or `NULL`\n @export",
                      rust_name: "euclidean_dist",
                      r_name: "euclidean_dist",
                      mod_name: "euclidean_dist",
                      args: args,
                      return_type: "Nullable",
                      func_ptr: wrap__euclidean_dist as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_optional_ndarray_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__euclidean_dist(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_optional_ndarray_metadata",
                      mod_name: "get_optional_ndarray_metadata",
                      r_name: "get_optional_ndarray_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_optional_ndarray_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_optional_ndarray_wrappers",
                      mod_name: "make_optional_ndarray_wrappers",
                      r_name: "make_optional_ndarray_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_optional_ndarray_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "optional_ndarray",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_optional_ndarray_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_optional_ndarray_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_optional_ndarray_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_optional_ndarray_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_optional_ndarray_extendr(info: *mut extendr_api::DllInfo) {
              unsafe {
                  extendr_api::register_call_methods(info, get_optional_ndarray_metadata())
              };
          }
      }
      mod raw_identifiers {
          use extendr_api::prelude::*;
          /// Test raw identifiers (`r#`) in function arguments are parsed correctly.
          /// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
          /// @param type : i32 or `NULL`
          /// @export
          fn raw_identifier_in_fn_args(r#type: Nullable<i32>) -> Nullable<i32> {
              r#type
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__raw_identifier_in_fn_args(
              r#type: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _type_robj = extendr_api::robj::Robj::from_sexp(r#type);
                          Ok(
                              extendr_api::Robj::from(
                                  raw_identifier_in_fn_args(_type_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!(
                                  "User function panicked: {0}",
                                  "raw_identifier_in_fn_args",
                              ),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__raw_identifier_in_fn_args(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "r#type",
                          arg_type: "Nullable",
                          default: Some("NULL"),
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Test raw identifiers (`r#`) in function arguments are parsed correctly.\n See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.\n @param type : i32 or `NULL`\n @export",
                      rust_name: "raw_identifier_in_fn_args",
                      r_name: "raw_identifier_in_fn_args",
                      mod_name: "raw_identifier_in_fn_args",
                      args: args,
                      return_type: "Nullable",
                      func_ptr: wrap__raw_identifier_in_fn_args as *const u8,
                      hidden: false,
                  })
          }
          /// Test raw identifiers (`r#`) as function names are parsed correctly.
          /// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
          /// @export
          fn r#true() -> bool {
              true
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__true() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(r#true())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "r#true"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__true(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Test raw identifiers (`r#`) as function names are parsed correctly.\n See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.\n @export",
                      rust_name: "r#true",
                      r_name: "r#true",
                      mod_name: "true",
                      args: args,
                      return_type: "bool",
                      func_ptr: wrap__true as *const u8,
                      hidden: false,
                  })
          }
          /// Combine raw identifiers (`r#`) as a function name and in arguments are parsed correctly.
          /// See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.
          /// @param type : i32 or `NULL`
          /// @export
          fn r#false(r#type: bool) -> bool {
              !r#type
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__false(r#type: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _type_robj = extendr_api::robj::Robj::from_sexp(r#type);
                          Ok(extendr_api::Robj::from(r#false(_type_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "r#false"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__false(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "r#type",
                          arg_type: "bool",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Combine raw identifiers (`r#`) as a function name and in arguments are parsed correctly.\n See [Issue #582](https://github.com/extendr/extendr/issues/528) for details.\n @param type : i32 or `NULL`\n @export",
                      rust_name: "r#false",
                      r_name: "r#false",
                      mod_name: "false",
                      args: args,
                      return_type: "bool",
                      func_ptr: wrap__false as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_raw_identifiers_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__raw_identifier_in_fn_args(&mut functions);
              meta__true(&mut functions);
              meta__false(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_raw_identifiers_metadata",
                      mod_name: "get_raw_identifiers_metadata",
                      r_name: "get_raw_identifiers_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_raw_identifiers_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_raw_identifiers_wrappers",
                      mod_name: "make_raw_identifiers_wrappers",
                      r_name: "make_raw_identifiers_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_raw_identifiers_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "raw_identifiers",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_raw_identifiers_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_raw_identifiers_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_raw_identifiers_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_raw_identifiers_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_raw_identifiers_extendr(info: *mut extendr_api::DllInfo) {
              unsafe {
                  extendr_api::register_call_methods(info, get_raw_identifiers_metadata())
              };
          }
      }
      mod submodule {
          use extendr_api::prelude::*;
          /// Return string `"Hello world!"` to R.
          /// @export
          fn hello_submodule() -> &'static str {
              "Hello World!"
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__hello_submodule() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(hello_submodule())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "hello_submodule"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__hello_submodule(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Return string `\"Hello world!\"` to R.\n @export",
                      rust_name: "hello_submodule",
                      r_name: "hello_submodule",
                      mod_name: "hello_submodule",
                      args: args,
                      return_type: "str",
                      func_ptr: wrap__hello_submodule as *const u8,
                      hidden: false,
                  })
          }
          struct MySubmoduleClass {
              a: i32,
          }
          #[automatically_derived]
          impl ::core::default::Default for MySubmoduleClass {
              #[inline]
              fn default() -> MySubmoduleClass {
                  MySubmoduleClass {
                      a: ::core::default::Default::default(),
                  }
              }
          }
          #[automatically_derived]
          impl ::core::fmt::Debug for MySubmoduleClass {
              #[inline]
              fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                  ::core::fmt::Formatter::debug_struct_field1_finish(
                      f,
                      "MySubmoduleClass",
                      "a",
                      &&self.a,
                  )
              }
          }
          #[automatically_derived]
          impl ::core::clone::Clone for MySubmoduleClass {
              #[inline]
              fn clone(&self) -> MySubmoduleClass {
                  let _: ::core::clone::AssertParamIsClone<i32>;
                  *self
              }
          }
          #[automatically_derived]
          impl ::core::marker::Copy for MySubmoduleClass {}
          /// Class for testing (exported)
          /// @examples
          /// x <- MySubmoduleClass$new()
          /// x$a()
          /// x$set_a(10)
          /// x$a()
          /// @export
          impl MySubmoduleClass {
              /// Method for making a new object.
              fn new() -> Self {
                  Self { a: 0 }
              }
              /// Method for setting stuff.
              /// @param x a number
              fn set_a(&mut self, x: i32) {
                  self.a = x;
              }
              /// Method for getting stuff.
              fn a(&self) -> i32 {
                  self.a
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__MySubmoduleClass__new() -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > { Ok(extendr_api::Robj::from(<MySubmoduleClass>::new())) }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "new"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__MySubmoduleClass__new(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = ::alloc::vec::Vec::new();
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for making a new object.",
                      rust_name: "new",
                      r_name: "new",
                      mod_name: "new",
                      args: args,
                      return_type: "Self",
                      func_ptr: wrap__MySubmoduleClass__new as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__MySubmoduleClass__set_a(
              _self: extendr_api::SEXP,
              x: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(
                              extendr_api::Robj::from(
                                  extendr_api::unwrap_or_throw_error(
                                          <&mut MySubmoduleClass>::try_from(&mut _self_robj),
                                      )
                                      .set_a(_x_robj.try_into()?),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "set_a"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__MySubmoduleClass__set_a(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "MySubmoduleClass",
                          default: None,
                      },
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "i32",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for setting stuff.\n @param x a number",
                      rust_name: "set_a",
                      r_name: "set_a",
                      mod_name: "set_a",
                      args: args,
                      return_type: "()",
                      func_ptr: wrap__MySubmoduleClass__set_a as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__MySubmoduleClass__a(
              _self: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                          Ok(
                              extendr_api::Robj::from(
                                  extendr_api::unwrap_or_throw_error(
                                          <&MySubmoduleClass>::try_from(&_self_robj),
                                      )
                                      .a(),
                              ),
                          )
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "a"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__MySubmoduleClass__a(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "self",
                          arg_type: "MySubmoduleClass",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: " Method for getting stuff.",
                      rust_name: "a",
                      r_name: "a",
                      mod_name: "a",
                      args: args,
                      return_type: "i32",
                      func_ptr: wrap__MySubmoduleClass__a as *const u8,
                      hidden: false,
                  })
          }
          impl TryFrom<Robj> for &MySubmoduleClass {
              type Error = Error;
              fn try_from(robj: Robj) -> Result<Self> {
                  Self::try_from(&robj)
              }
          }
          impl TryFrom<Robj> for &mut MySubmoduleClass {
              type Error = Error;
              fn try_from(mut robj: Robj) -> Result<Self> {
                  Self::try_from(&mut robj)
              }
          }
          impl TryFrom<&Robj> for &MySubmoduleClass {
              type Error = Error;
              fn try_from(robj: &Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &ExternalPtr<MySubmoduleClass> = robj.try_into()?;
                      external_ptr.try_addr()
                  }
              }
          }
          impl TryFrom<&mut Robj> for &mut MySubmoduleClass {
              type Error = Error;
              fn try_from(robj: &mut Robj) -> Result<Self> {
                  unsafe {
                      let external_ptr: &mut ExternalPtr<MySubmoduleClass> = robj.try_into()?;
                      external_ptr.try_addr_mut()
                  }
              }
          }
          impl From<MySubmoduleClass> for Robj {
              fn from(value: MySubmoduleClass) -> Self {
                  unsafe {
                      let mut res: ExternalPtr<MySubmoduleClass> = ExternalPtr::new(value);
                      res.set_attrib(class_symbol(), "MySubmoduleClass").unwrap();
                      res.into()
                  }
              }
          }
          #[allow(non_snake_case)]
          fn meta__MySubmoduleClass(impls: &mut Vec<extendr_api::metadata::Impl>) {
              let mut methods = Vec::new();
              meta__MySubmoduleClass__new(&mut methods);
              meta__MySubmoduleClass__set_a(&mut methods);
              meta__MySubmoduleClass__a(&mut methods);
              impls
                  .push(extendr_api::metadata::Impl {
                      doc: " Class for testing (exported)\n @examples\n x <- MySubmoduleClass$new()\n x$a()\n x$set_a(10)\n x$a()\n @export",
                      name: "MySubmoduleClass",
                      methods,
                  });
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_submodule_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__hello_submodule(&mut functions);
              meta__MySubmoduleClass(&mut impls);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_submodule_metadata",
                      mod_name: "get_submodule_metadata",
                      r_name: "get_submodule_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_submodule_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_submodule_wrappers",
                      mod_name: "make_submodule_wrappers",
                      r_name: "make_submodule_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_submodule_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "submodule",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_submodule_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_submodule_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_submodule_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_submodule_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_submodule_extendr(info: *mut extendr_api::DllInfo) {
              unsafe { extendr_api::register_call_methods(info, get_submodule_metadata()) };
          }
      }
      mod typedsliceargs {
          use extendr_api::prelude::*;
          fn middle_zero(integers: &mut [Rint]) {
              let middle = integers.len() / 2;
              integers[middle] = 42_i32.into();
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__middle_zero(
              integers: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _integers_robj = extendr_api::robj::Robj::from_sexp(integers);
                          Ok(extendr_api::Robj::from(middle_zero(_integers_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "middle_zero"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__middle_zero(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "integers",
                          arg_type: "_5bRint_5d",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "middle_zero",
                      r_name: "middle_zero",
                      mod_name: "middle_zero",
                      args: args,
                      return_type: "()",
                      func_ptr: wrap__middle_zero as *const u8,
                      hidden: false,
                  })
          }
          fn logicals_sum(logicals: &[Rbool]) -> i32 {
              logicals
                  .iter()
                  .fold(
                      0,
                      |acc, next| {
                          let bool = next.is_true();
                          acc + bool as i32
                      },
                  )
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__logicals_sum(
              logicals: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _logicals_robj = extendr_api::robj::Robj::from_sexp(logicals);
                          Ok(extendr_api::Robj::from(logicals_sum(_logicals_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "logicals_sum"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__logicals_sum(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "logicals",
                          arg_type: "_5bRbool_5d",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "logicals_sum",
                      r_name: "logicals_sum",
                      mod_name: "logicals_sum",
                      args: args,
                      return_type: "i32",
                      func_ptr: wrap__logicals_sum as *const u8,
                      hidden: false,
                  })
          }
          fn floats_mean(x: &[Rfloat]) -> f64 {
              let n = x.len();
              let x_sum = x
                  .iter()
                  .fold(
                      0.0,
                      |acc, next| {
                          if next.is_na() {
                              acc
                          } else {
                              let v = next.inner();
                              acc + v
                          }
                      },
                  );
              x_sum / n as f64
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__floats_mean(x: extendr_api::SEXP) -> extendr_api::SEXP {
              use extendr_api::robj::*;
              let wrap_result_state: std::result::Result<
                  std::result::Result<Robj, extendr_api::Error>,
                  Box<dyn std::any::Any + Send>,
              > = unsafe {
                  std::panic::catch_unwind(
                      std::panic::AssertUnwindSafe(move || -> std::result::Result<
                          Robj,
                          extendr_api::Error,
                      > {
                          let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                          Ok(extendr_api::Robj::from(floats_mean(_x_robj.try_into()?)))
                      }),
                  )
              };
              match wrap_result_state {
                  Ok(Ok(zz)) => {
                      return unsafe { zz.get() };
                  }
                  Ok(Err(conversion_err)) => {
                      let err_string = conversion_err.to_string();
                      drop(conversion_err);
                      extendr_api::throw_r_error(&err_string);
                  }
                  Err(unwind_err) => {
                      drop(unwind_err);
                      let err_string = {
                          let res = ::alloc::fmt::format(
                              format_args!("User function panicked: {0}", "floats_mean"),
                          );
                          res
                      };
                      extendr_api::handle_panic(
                          err_string.as_str(),
                          || {
                              #[cold]
                              #[track_caller]
                              #[inline(never)]
                              const fn panic_cold_explicit() -> ! {
                                  ::core::panicking::panic_explicit()
                              }
                              panic_cold_explicit();
                          },
                      );
                  }
              }
              {
                  ::core::panicking::panic_fmt(
                      format_args!(
                          "internal error: entered unreachable code: {0}",
                          format_args!("internal extendr error, this should never happen."),
                      ),
                  );
              }
          }
          #[allow(non_snake_case)]
          fn meta__floats_mean(metadata: &mut Vec<extendr_api::metadata::Func>) {
              let mut args = <[_]>::into_vec(
                  #[rustc_box]
                  ::alloc::boxed::Box::new([
                      extendr_api::metadata::Arg {
                          name: "x",
                          arg_type: "_5bRfloat_5d",
                          default: None,
                      },
                  ]),
              );
              metadata
                  .push(extendr_api::metadata::Func {
                      doc: "",
                      rust_name: "floats_mean",
                      r_name: "floats_mean",
                      mod_name: "floats_mean",
                      args: args,
                      return_type: "f64",
                      func_ptr: wrap__floats_mean as *const u8,
                      hidden: false,
                  })
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub fn get_typedsliceargs_metadata() -> extendr_api::metadata::Metadata {
              let mut functions = Vec::new();
              let mut impls = Vec::new();
              meta__middle_zero(&mut functions);
              meta__logicals_sum(&mut functions);
              meta__floats_mean(&mut functions);
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Metadata access function.",
                      rust_name: "get_typedsliceargs_metadata",
                      mod_name: "get_typedsliceargs_metadata",
                      r_name: "get_typedsliceargs_metadata",
                      args: Vec::new(),
                      return_type: "Metadata",
                      func_ptr: wrap__get_typedsliceargs_metadata as *const u8,
                      hidden: true,
                  });
              functions
                  .push(extendr_api::metadata::Func {
                      doc: "Wrapper generator.",
                      rust_name: "make_typedsliceargs_wrappers",
                      mod_name: "make_typedsliceargs_wrappers",
                      r_name: "make_typedsliceargs_wrappers",
                      args: <[_]>::into_vec(
                          #[rustc_box]
                          ::alloc::boxed::Box::new([
                              extendr_api::metadata::Arg {
                                  name: "use_symbols",
                                  arg_type: "bool",
                                  default: None,
                              },
                              extendr_api::metadata::Arg {
                                  name: "package_name",
                                  arg_type: "&str",
                                  default: None,
                              },
                          ]),
                      ),
                      return_type: "String",
                      func_ptr: wrap__make_typedsliceargs_wrappers as *const u8,
                      hidden: true,
                  });
              extendr_api::metadata::Metadata {
                  name: "typedsliceargs",
                  functions,
                  impls,
              }
          }
          #[no_mangle]
          #[allow(non_snake_case)]
          pub extern "C" fn wrap__get_typedsliceargs_metadata() -> extendr_api::SEXP {
              use extendr_api::GetSexp;
              unsafe { extendr_api::Robj::from(get_typedsliceargs_metadata()).get() }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn wrap__make_typedsliceargs_wrappers(
              use_symbols_sexp: extendr_api::SEXP,
              package_name_sexp: extendr_api::SEXP,
          ) -> extendr_api::SEXP {
              unsafe {
                  use extendr_api::robj::*;
                  use extendr_api::GetSexp;
                  let robj = Robj::from_sexp(use_symbols_sexp);
                  let use_symbols: bool = <bool>::try_from(&robj).unwrap();
                  let robj = Robj::from_sexp(package_name_sexp);
                  let package_name: &str = <&str>::try_from(&robj).unwrap();
                  extendr_api::Robj::from(
                          get_typedsliceargs_metadata()
                              .make_r_wrappers(use_symbols, package_name)
                              .unwrap(),
                      )
                      .get()
              }
          }
          #[no_mangle]
          #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
          pub extern "C" fn R_init_typedsliceargs_extendr(info: *mut extendr_api::DllInfo) {
              unsafe {
                  extendr_api::register_call_methods(info, get_typedsliceargs_metadata())
              };
          }
      }
      fn hello_world() -> &'static str {
          "Hello world!"
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__hello_world() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(hello_world())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "hello_world"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__hello_world(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "hello_world",
                  r_name: "hello_world",
                  mod_name: "hello_world",
                  args: args,
                  return_type: "str",
                  func_ptr: wrap__hello_world as *const u8,
                  hidden: false,
              })
      }
      fn do_nothing() {}
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__do_nothing() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(do_nothing())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "do_nothing"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__do_nothing(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "do_nothing",
                  r_name: "do_nothing",
                  mod_name: "do_nothing",
                  args: args,
                  return_type: "()",
                  func_ptr: wrap__do_nothing as *const u8,
                  hidden: false,
              })
      }
      fn double_scalar(x: f64) -> f64 {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__double_scalar(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(double_scalar(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "double_scalar"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__double_scalar(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "f64",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "double_scalar",
                  r_name: "double_scalar",
                  mod_name: "double_scalar",
                  args: args,
                  return_type: "f64",
                  func_ptr: wrap__double_scalar as *const u8,
                  hidden: false,
              })
      }
      fn int_scalar(x: i32) -> i32 {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__int_scalar(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(int_scalar(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "int_scalar"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__int_scalar(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "int_scalar",
                  r_name: "int_scalar",
                  mod_name: "int_scalar",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__int_scalar as *const u8,
                  hidden: false,
              })
      }
      fn bool_scalar(x: bool) -> bool {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__bool_scalar(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(bool_scalar(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "bool_scalar"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__bool_scalar(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "bool",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "bool_scalar",
                  r_name: "bool_scalar",
                  mod_name: "bool_scalar",
                  args: args,
                  return_type: "bool",
                  func_ptr: wrap__bool_scalar as *const u8,
                  hidden: false,
              })
      }
      fn char_scalar(x: String) -> String {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__char_scalar(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(char_scalar(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "char_scalar"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__char_scalar(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "String",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "char_scalar",
                  r_name: "char_scalar",
                  mod_name: "char_scalar",
                  args: args,
                  return_type: "String",
                  func_ptr: wrap__char_scalar as *const u8,
                  hidden: false,
              })
      }
      fn char_vec(x: Vec<String>) -> Vec<String> {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__char_vec(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(char_vec(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "char_vec"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__char_vec(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Vec",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "char_vec",
                  r_name: "char_vec",
                  mod_name: "char_vec",
                  args: args,
                  return_type: "Vec",
                  func_ptr: wrap__char_vec as *const u8,
                  hidden: false,
              })
      }
      fn double_vec(x: Vec<f64>) -> Vec<f64> {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__double_vec(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(double_vec(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "double_vec"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__double_vec(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Vec",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "double_vec",
                  r_name: "double_vec",
                  mod_name: "double_vec",
                  args: args,
                  return_type: "Vec",
                  func_ptr: wrap__double_vec as *const u8,
                  hidden: false,
              })
      }
      fn try_rfloat_na() -> Rfloat {
          Rfloat::na()
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__try_rfloat_na() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(try_rfloat_na())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "try_rfloat_na"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__try_rfloat_na(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "try_rfloat_na",
                  r_name: "try_rfloat_na",
                  mod_name: "try_rfloat_na",
                  args: args,
                  return_type: "Rfloat",
                  func_ptr: wrap__try_rfloat_na as *const u8,
                  hidden: false,
              })
      }
      fn try_rint_na() -> Rint {
          Rint::na()
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__try_rint_na() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(try_rint_na())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "try_rint_na"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__try_rint_na(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "try_rint_na",
                  r_name: "try_rint_na",
                  mod_name: "try_rint_na",
                  args: args,
                  return_type: "Rint",
                  func_ptr: wrap__try_rint_na as *const u8,
                  hidden: false,
              })
      }
      fn check_rfloat_na(x: Rfloat) -> bool {
          x.is_na()
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__check_rfloat_na(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(check_rfloat_na(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "check_rfloat_na"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__check_rfloat_na(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Rfloat",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "check_rfloat_na",
                  r_name: "check_rfloat_na",
                  mod_name: "check_rfloat_na",
                  args: args,
                  return_type: "bool",
                  func_ptr: wrap__check_rfloat_na as *const u8,
                  hidden: false,
              })
      }
      fn check_rint_na(x: Rint) -> bool {
          x.is_na()
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__check_rint_na(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(check_rint_na(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "check_rint_na"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__check_rint_na(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Rint",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "check_rint_na",
                  r_name: "check_rint_na",
                  mod_name: "check_rint_na",
                  args: args,
                  return_type: "bool",
                  func_ptr: wrap__check_rint_na as *const u8,
                  hidden: false,
              })
      }
      fn get_doubles_element(x: Doubles, i: i32) -> Rfloat {
          x.elt(i as usize)
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__get_doubles_element(
          x: extendr_api::SEXP,
          i: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      let _i_robj = extendr_api::robj::Robj::from_sexp(i);
                      Ok(
                          extendr_api::Robj::from(
                              get_doubles_element(_x_robj.try_into()?, _i_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "get_doubles_element"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__get_doubles_element(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Doubles",
                      default: None,
                  },
                  extendr_api::metadata::Arg {
                      name: "i",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "get_doubles_element",
                  r_name: "get_doubles_element",
                  mod_name: "get_doubles_element",
                  args: args,
                  return_type: "Rfloat",
                  func_ptr: wrap__get_doubles_element as *const u8,
                  hidden: false,
              })
      }
      fn get_integers_element(x: Integers, i: i32) -> Rint {
          x.elt(i as usize)
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__get_integers_element(
          x: extendr_api::SEXP,
          i: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      let _i_robj = extendr_api::robj::Robj::from_sexp(i);
                      Ok(
                          extendr_api::Robj::from(
                              get_integers_element(_x_robj.try_into()?, _i_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "get_integers_element"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__get_integers_element(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Integers",
                      default: None,
                  },
                  extendr_api::metadata::Arg {
                      name: "i",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "get_integers_element",
                  r_name: "get_integers_element",
                  mod_name: "get_integers_element",
                  args: args,
                  return_type: "Rint",
                  func_ptr: wrap__get_integers_element as *const u8,
                  hidden: false,
              })
      }
      fn get_logicals_element(x: Logicals, i: i32) -> Rbool {
          x.elt(i as usize)
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__get_logicals_element(
          x: extendr_api::SEXP,
          i: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      let _i_robj = extendr_api::robj::Robj::from_sexp(i);
                      Ok(
                          extendr_api::Robj::from(
                              get_logicals_element(_x_robj.try_into()?, _i_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "get_logicals_element"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__get_logicals_element(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Logicals",
                      default: None,
                  },
                  extendr_api::metadata::Arg {
                      name: "i",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "get_logicals_element",
                  r_name: "get_logicals_element",
                  mod_name: "get_logicals_element",
                  args: args,
                  return_type: "Rbool",
                  func_ptr: wrap__get_logicals_element as *const u8,
                  hidden: false,
              })
      }
      fn doubles_square(input: Doubles) -> Doubles {
          let mut result = Doubles::new(input.len());
          for (x, y) in result.iter_mut().zip(input.iter()) {
              *x = y * y;
          }
          result
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__doubles_square(input: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _input_robj = extendr_api::robj::Robj::from_sexp(input);
                      Ok(extendr_api::Robj::from(doubles_square(_input_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "doubles_square"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__doubles_square(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "input",
                      arg_type: "Doubles",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "doubles_square",
                  r_name: "doubles_square",
                  mod_name: "doubles_square",
                  args: args,
                  return_type: "Doubles",
                  func_ptr: wrap__doubles_square as *const u8,
                  hidden: false,
              })
      }
      fn complexes_square(input: Complexes) -> Complexes {
          let mut result = Complexes::new(input.len());
          for (x, y) in result.iter_mut().zip(input.iter()) {
              *x = Rcplx::from((y.re() * y.re(), 0.0.into()));
          }
          result
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__complexes_square(input: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _input_robj = extendr_api::robj::Robj::from_sexp(input);
                      Ok(extendr_api::Robj::from(complexes_square(_input_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "complexes_square"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__complexes_square(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "input",
                      arg_type: "Complexes",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "complexes_square",
                  r_name: "complexes_square",
                  mod_name: "complexes_square",
                  args: args,
                  return_type: "Complexes",
                  func_ptr: wrap__complexes_square as *const u8,
                  hidden: false,
              })
      }
      fn integers_square(input: Integers) -> Integers {
          let mut result = Integers::new(input.len());
          for (x, y) in result.iter_mut().zip(input.iter()) {
              *x = y * y;
          }
          result
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__integers_square(input: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _input_robj = extendr_api::robj::Robj::from_sexp(input);
                      Ok(extendr_api::Robj::from(integers_square(_input_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "integers_square"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__integers_square(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "input",
                      arg_type: "Integers",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "integers_square",
                  r_name: "integers_square",
                  mod_name: "integers_square",
                  args: args,
                  return_type: "Integers",
                  func_ptr: wrap__integers_square as *const u8,
                  hidden: false,
              })
      }
      fn logicals_not(input: Logicals) -> Logicals {
          let mut result = Logicals::new(input.len());
          for (x, y) in result.iter_mut().zip(input.iter()) {
              *x = !y;
          }
          result
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__logicals_not(input: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _input_robj = extendr_api::robj::Robj::from_sexp(input);
                      Ok(extendr_api::Robj::from(logicals_not(_input_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "logicals_not"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__logicals_not(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "input",
                      arg_type: "Logicals",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "logicals_not",
                  r_name: "logicals_not",
                  mod_name: "logicals_not",
                  args: args,
                  return_type: "Logicals",
                  func_ptr: wrap__logicals_not as *const u8,
                  hidden: false,
              })
      }
      fn check_default(x: Robj) -> bool {
          x.is_null()
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__check_default(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(check_default(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "check_default"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__check_default(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Robj",
                      default: Some("NULL"),
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "check_default",
                  r_name: "check_default",
                  mod_name: "check_default",
                  args: args,
                  return_type: "bool",
                  func_ptr: wrap__check_default as *const u8,
                  hidden: false,
              })
      }
      /// Test whether `_arg` parameters are treated correctly in R
      /// Executes \code{`_x` - `_y`}
      /// @param _x an integer scalar, ignored
      /// @param `_y` an integer scalar, ignored
      /// @export
      fn special_param_names(_x: i32, _y: i32) -> i32 {
          _x - _y
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__special_param_names(
          _x: extendr_api::SEXP,
          _y: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let __x_robj = extendr_api::robj::Robj::from_sexp(_x);
                      let __y_robj = extendr_api::robj::Robj::from_sexp(_y);
                      Ok(
                          extendr_api::Robj::from(
                              special_param_names(__x_robj.try_into()?, __y_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "special_param_names"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__special_param_names(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "_x",
                      arg_type: "i32",
                      default: None,
                  },
                  extendr_api::metadata::Arg {
                      name: "_y",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Test whether `_arg` parameters are treated correctly in R\n Executes \\code{`_x` - `_y`}\n @param _x an integer scalar, ignored\n @param `_y` an integer scalar, ignored\n @export",
                  rust_name: "special_param_names",
                  r_name: "special_param_names",
                  mod_name: "special_param_names",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__special_param_names as *const u8,
                  hidden: false,
              })
      }
      /// Test wrapping of special function name
      /// @name f__00__special_function_name
      /// @export
      #[allow(non_snake_case)]
      fn __00__special_function_name() {}
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap____00__special_function_name() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(__00__special_function_name())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!(
                              "User function panicked: {0}",
                              "__00__special_function_name",
                          ),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta____00__special_function_name(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Test wrapping of special function name\n @name f__00__special_function_name\n @export",
                  rust_name: "__00__special_function_name",
                  r_name: "__00__special_function_name",
                  mod_name: "__00__special_function_name",
                  args: args,
                  return_type: "()",
                  func_ptr: wrap____00__special_function_name as *const u8,
                  hidden: false,
              })
      }
      fn test_rename() -> i32 {
          1
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__test_rename_mymod() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(test_rename())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "test.rename.rlike"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__test_rename_mymod(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "test_rename",
                  r_name: "test.rename.rlike",
                  mod_name: "test_rename_mymod",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__test_rename_mymod as *const u8,
                  hidden: false,
              })
      }
      fn get_default_value(x: i32) -> i32 {
          x
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__get_default_value(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(get_default_value(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "get_default_value"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__get_default_value(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "i32",
                      default: Some("42"),
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "get_default_value",
                  r_name: "get_default_value",
                  mod_name: "get_default_value",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__get_default_value as *const u8,
                  hidden: false,
              })
      }
      fn add_5_if_not_null(x: Nullable<Rint>) -> Nullable<Rint> {
          x.map(|y| y + 5)
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__add_5_if_not_null(x: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(extendr_api::Robj::from(add_5_if_not_null(_x_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "add_5_if_not_null"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__add_5_if_not_null(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "Nullable",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "add_5_if_not_null",
                  r_name: "add_5_if_not_null",
                  mod_name: "add_5_if_not_null",
                  args: args,
                  return_type: "Nullable",
                  func_ptr: wrap__add_5_if_not_null as *const u8,
                  hidden: false,
              })
      }
      struct MyClass {
          a: i32,
      }
      #[automatically_derived]
      impl ::core::default::Default for MyClass {
          #[inline]
          fn default() -> MyClass {
              MyClass {
                  a: ::core::default::Default::default(),
              }
          }
      }
      #[automatically_derived]
      impl ::core::fmt::Debug for MyClass {
          #[inline]
          fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
              ::core::fmt::Formatter::debug_struct_field1_finish(f, "MyClass", "a", &&self.a)
          }
      }
      /// Class for testing (exported)
      /// @examples
      /// x <- MyClass$new()
      /// x$a()
      /// x$set_a(10)
      /// x$a()
      /// @export
      impl MyClass {
          /// Method for making a new object.
          fn new() -> Self {
              Self { a: 0 }
          }
          /// Method for setting stuff.
          /// @param x a number
          fn set_a(&mut self, x: i32) {
              self.a = x;
          }
          /// Method for getting stuff.
          fn a(&self) -> i32 {
              self.a
          }
          /// Method for getting one's self.
          fn me(&self) -> &Self {
              self
          }
          fn restore_from_robj(robj: Robj) -> Self {
              let res: ExternalPtr<MyClass> = robj.try_into().unwrap();
              Self { a: res.a }
          }
          fn get_default_value(x: i32) -> i32 {
              x
          }
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__new() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(<MyClass>::new())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "new"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__new(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for making a new object.",
                  rust_name: "new",
                  r_name: "new",
                  mod_name: "new",
                  args: args,
                  return_type: "Self",
                  func_ptr: wrap__MyClass__new as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__set_a(
          _self: extendr_api::SEXP,
          x: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(
                          extendr_api::Robj::from(
                              extendr_api::unwrap_or_throw_error(
                                      <&mut MyClass>::try_from(&mut _self_robj),
                                  )
                                  .set_a(_x_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "set_a"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__set_a(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "self",
                      arg_type: "MyClass",
                      default: None,
                  },
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "i32",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for setting stuff.\n @param x a number",
                  rust_name: "set_a",
                  r_name: "set_a",
                  mod_name: "set_a",
                  args: args,
                  return_type: "()",
                  func_ptr: wrap__MyClass__set_a as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__a(_self: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                      Ok(
                          extendr_api::Robj::from(
                              extendr_api::unwrap_or_throw_error(
                                      <&MyClass>::try_from(&_self_robj),
                                  )
                                  .a(),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "a"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__a(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "self",
                      arg_type: "MyClass",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for getting stuff.",
                  rust_name: "a",
                  r_name: "a",
                  mod_name: "a",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__MyClass__a as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__me(_self: extendr_api::SEXP) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                      let return_ref_to_self = extendr_api::unwrap_or_throw_error(
                              <&MyClass>::try_from(&_self_robj),
                          )
                          .me();
                      if std::ptr::addr_eq(
                          extendr_api::R_ExternalPtrAddr(_self),
                          std::ptr::from_ref(return_ref_to_self),
                      ) {
                          return Ok(extendr_api::Robj::from_sexp(_self));
                      }
                      Err(Error::ExpectedExternalPtrReference)
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "me"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__me(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "self",
                      arg_type: "MyClass",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for getting one's self.",
                  rust_name: "me",
                  r_name: "me",
                  mod_name: "me",
                  args: args,
                  return_type: "Self",
                  func_ptr: wrap__MyClass__me as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__restore_from_robj(
          robj: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _robj_robj = extendr_api::robj::Robj::from_sexp(robj);
                      Ok(
                          extendr_api::Robj::from(
                              <MyClass>::restore_from_robj(_robj_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "restore_from_robj"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__restore_from_robj(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "robj",
                      arg_type: "Robj",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "restore_from_robj",
                  r_name: "restore_from_robj",
                  mod_name: "restore_from_robj",
                  args: args,
                  return_type: "Self",
                  func_ptr: wrap__MyClass__restore_from_robj as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClass__get_default_value(
          x: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _x_robj = extendr_api::robj::Robj::from_sexp(x);
                      Ok(
                          extendr_api::Robj::from(
                              <MyClass>::get_default_value(_x_robj.try_into()?),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "get_default_value"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass__get_default_value(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "x",
                      arg_type: "i32",
                      default: Some("42"),
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: "",
                  rust_name: "get_default_value",
                  r_name: "get_default_value",
                  mod_name: "get_default_value",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__MyClass__get_default_value as *const u8,
                  hidden: false,
              })
      }
      impl TryFrom<Robj> for &MyClass {
          type Error = Error;
          fn try_from(robj: Robj) -> Result<Self> {
              Self::try_from(&robj)
          }
      }
      impl TryFrom<Robj> for &mut MyClass {
          type Error = Error;
          fn try_from(mut robj: Robj) -> Result<Self> {
              Self::try_from(&mut robj)
          }
      }
      impl TryFrom<&Robj> for &MyClass {
          type Error = Error;
          fn try_from(robj: &Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &ExternalPtr<MyClass> = robj.try_into()?;
                  external_ptr.try_addr()
              }
          }
      }
      impl TryFrom<&mut Robj> for &mut MyClass {
          type Error = Error;
          fn try_from(robj: &mut Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &mut ExternalPtr<MyClass> = robj.try_into()?;
                  external_ptr.try_addr_mut()
              }
          }
      }
      impl From<MyClass> for Robj {
          fn from(value: MyClass) -> Self {
              unsafe {
                  let mut res: ExternalPtr<MyClass> = ExternalPtr::new(value);
                  res.set_attrib(class_symbol(), "MyClass").unwrap();
                  res.into()
              }
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClass(impls: &mut Vec<extendr_api::metadata::Impl>) {
          let mut methods = Vec::new();
          meta__MyClass__new(&mut methods);
          meta__MyClass__set_a(&mut methods);
          meta__MyClass__a(&mut methods);
          meta__MyClass__me(&mut methods);
          meta__MyClass__restore_from_robj(&mut methods);
          meta__MyClass__get_default_value(&mut methods);
          impls
              .push(extendr_api::metadata::Impl {
                  doc: " Class for testing (exported)\n @examples\n x <- MyClass$new()\n x$a()\n x$set_a(10)\n x$a()\n @export",
                  name: "MyClass",
                  methods,
              });
      }
      struct __MyClass {}
      #[automatically_derived]
      impl ::core::default::Default for __MyClass {
          #[inline]
          fn default() -> __MyClass {
              __MyClass {}
          }
      }
      #[automatically_derived]
      impl ::core::fmt::Debug for __MyClass {
          #[inline]
          fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
              ::core::fmt::Formatter::write_str(f, "__MyClass")
          }
      }
      impl __MyClass {
          /// Method for making a new object.
          fn new() -> Self {
              Self {}
          }
          /// Method with special name unsupported by R
          fn __name_test(&self) {}
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap____MyClass__new() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(<__MyClass>::new())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "new"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta____MyClass__new(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for making a new object.",
                  rust_name: "new",
                  r_name: "new",
                  mod_name: "new",
                  args: args,
                  return_type: "Self",
                  func_ptr: wrap____MyClass__new as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap____MyClass____name_test(
          _self: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                      Ok(
                          extendr_api::Robj::from(
                              extendr_api::unwrap_or_throw_error(
                                      <&__MyClass>::try_from(&_self_robj),
                                  )
                                  .__name_test(),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "__name_test"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta____MyClass____name_test(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "self",
                      arg_type: "__MyClass",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method with special name unsupported by R",
                  rust_name: "__name_test",
                  r_name: "__name_test",
                  mod_name: "__name_test",
                  args: args,
                  return_type: "()",
                  func_ptr: wrap____MyClass____name_test as *const u8,
                  hidden: false,
              })
      }
      impl TryFrom<Robj> for &__MyClass {
          type Error = Error;
          fn try_from(robj: Robj) -> Result<Self> {
              Self::try_from(&robj)
          }
      }
      impl TryFrom<Robj> for &mut __MyClass {
          type Error = Error;
          fn try_from(mut robj: Robj) -> Result<Self> {
              Self::try_from(&mut robj)
          }
      }
      impl TryFrom<&Robj> for &__MyClass {
          type Error = Error;
          fn try_from(robj: &Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &ExternalPtr<__MyClass> = robj.try_into()?;
                  external_ptr.try_addr()
              }
          }
      }
      impl TryFrom<&mut Robj> for &mut __MyClass {
          type Error = Error;
          fn try_from(robj: &mut Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &mut ExternalPtr<__MyClass> = robj.try_into()?;
                  external_ptr.try_addr_mut()
              }
          }
      }
      impl From<__MyClass> for Robj {
          fn from(value: __MyClass) -> Self {
              unsafe {
                  let mut res: ExternalPtr<__MyClass> = ExternalPtr::new(value);
                  res.set_attrib(class_symbol(), "__MyClass").unwrap();
                  res.into()
              }
          }
      }
      #[allow(non_snake_case)]
      fn meta____MyClass(impls: &mut Vec<extendr_api::metadata::Impl>) {
          let mut methods = Vec::new();
          meta____MyClass__new(&mut methods);
          meta____MyClass____name_test(&mut methods);
          impls
              .push(extendr_api::metadata::Impl {
                  doc: "",
                  name: "__MyClass",
                  methods,
              });
      }
      struct MyClassUnexported {
          a: i32,
      }
      #[automatically_derived]
      impl ::core::default::Default for MyClassUnexported {
          #[inline]
          fn default() -> MyClassUnexported {
              MyClassUnexported {
                  a: ::core::default::Default::default(),
              }
          }
      }
      #[automatically_derived]
      impl ::core::fmt::Debug for MyClassUnexported {
          #[inline]
          fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
              ::core::fmt::Formatter::debug_struct_field1_finish(
                  f,
                  "MyClassUnexported",
                  "a",
                  &&self.a,
              )
          }
      }
      /// Class for testing (unexported)
      impl MyClassUnexported {
          /// Method for making a new object.
          fn new() -> Self {
              Self { a: 22 }
          }
          /// Method for getting stuff.
          fn a(&self) -> i32 {
              self.a
          }
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClassUnexported__new() -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > { Ok(extendr_api::Robj::from(<MyClassUnexported>::new())) }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "new"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClassUnexported__new(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = ::alloc::vec::Vec::new();
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for making a new object.",
                  rust_name: "new",
                  r_name: "new",
                  mod_name: "new",
                  args: args,
                  return_type: "Self",
                  func_ptr: wrap__MyClassUnexported__new as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__MyClassUnexported__a(
          _self: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self);
                      Ok(
                          extendr_api::Robj::from(
                              extendr_api::unwrap_or_throw_error(
                                      <&MyClassUnexported>::try_from(&_self_robj),
                                  )
                                  .a(),
                          ),
                      )
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "a"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClassUnexported__a(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "self",
                      arg_type: "MyClassUnexported",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Method for getting stuff.",
                  rust_name: "a",
                  r_name: "a",
                  mod_name: "a",
                  args: args,
                  return_type: "i32",
                  func_ptr: wrap__MyClassUnexported__a as *const u8,
                  hidden: false,
              })
      }
      impl TryFrom<Robj> for &MyClassUnexported {
          type Error = Error;
          fn try_from(robj: Robj) -> Result<Self> {
              Self::try_from(&robj)
          }
      }
      impl TryFrom<Robj> for &mut MyClassUnexported {
          type Error = Error;
          fn try_from(mut robj: Robj) -> Result<Self> {
              Self::try_from(&mut robj)
          }
      }
      impl TryFrom<&Robj> for &MyClassUnexported {
          type Error = Error;
          fn try_from(robj: &Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &ExternalPtr<MyClassUnexported> = robj.try_into()?;
                  external_ptr.try_addr()
              }
          }
      }
      impl TryFrom<&mut Robj> for &mut MyClassUnexported {
          type Error = Error;
          fn try_from(robj: &mut Robj) -> Result<Self> {
              unsafe {
                  let external_ptr: &mut ExternalPtr<MyClassUnexported> = robj.try_into()?;
                  external_ptr.try_addr_mut()
              }
          }
      }
      impl From<MyClassUnexported> for Robj {
          fn from(value: MyClassUnexported) -> Self {
              unsafe {
                  let mut res: ExternalPtr<MyClassUnexported> = ExternalPtr::new(value);
                  res.set_attrib(class_symbol(), "MyClassUnexported").unwrap();
                  res.into()
              }
          }
      }
      #[allow(non_snake_case)]
      fn meta__MyClassUnexported(impls: &mut Vec<extendr_api::metadata::Impl>) {
          let mut methods = Vec::new();
          meta__MyClassUnexported__new(&mut methods);
          meta__MyClassUnexported__a(&mut methods);
          impls
              .push(extendr_api::metadata::Impl {
                  doc: " Class for testing (unexported)",
                  name: "MyClassUnexported",
                  methods,
              });
      }
      /// Create a new device.
      ///
      /// @param welcome_message A warm message to welcome you.
      /// @export
      fn my_device(welcome_message: String) {
          let device_driver = graphic_device::MyDevice {
              welcome_message: welcome_message.as_str(),
          };
          let device_descriptor = DeviceDescriptor::new();
          device_driver
              .create_device::<graphic_device::MyDevice>(device_descriptor, "my device");
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__my_device(
          welcome_message: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          use extendr_api::robj::*;
          let wrap_result_state: std::result::Result<
              std::result::Result<Robj, extendr_api::Error>,
              Box<dyn std::any::Any + Send>,
          > = unsafe {
              std::panic::catch_unwind(
                  std::panic::AssertUnwindSafe(move || -> std::result::Result<
                      Robj,
                      extendr_api::Error,
                  > {
                      let _welcome_message_robj = extendr_api::robj::Robj::from_sexp(
                          welcome_message,
                      );
                      Ok(extendr_api::Robj::from(my_device(_welcome_message_robj.try_into()?)))
                  }),
              )
          };
          match wrap_result_state {
              Ok(Ok(zz)) => {
                  return unsafe { zz.get() };
              }
              Ok(Err(conversion_err)) => {
                  let err_string = conversion_err.to_string();
                  drop(conversion_err);
                  extendr_api::throw_r_error(&err_string);
              }
              Err(unwind_err) => {
                  drop(unwind_err);
                  let err_string = {
                      let res = ::alloc::fmt::format(
                          format_args!("User function panicked: {0}", "my_device"),
                      );
                      res
                  };
                  extendr_api::handle_panic(
                      err_string.as_str(),
                      || {
                          #[cold]
                          #[track_caller]
                          #[inline(never)]
                          const fn panic_cold_explicit() -> ! {
                              ::core::panicking::panic_explicit()
                          }
                          panic_cold_explicit();
                      },
                  );
              }
          }
          {
              ::core::panicking::panic_fmt(
                  format_args!(
                      "internal error: entered unreachable code: {0}",
                      format_args!("internal extendr error, this should never happen."),
                  ),
              );
          }
      }
      #[allow(non_snake_case)]
      fn meta__my_device(metadata: &mut Vec<extendr_api::metadata::Func>) {
          let mut args = <[_]>::into_vec(
              #[rustc_box]
              ::alloc::boxed::Box::new([
                  extendr_api::metadata::Arg {
                      name: "welcome_message",
                      arg_type: "String",
                      default: None,
                  },
              ]),
          );
          metadata
              .push(extendr_api::metadata::Func {
                  doc: " Create a new device.\n\n @param welcome_message A warm message to welcome you.\n @export",
                  rust_name: "my_device",
                  r_name: "my_device",
                  mod_name: "my_device",
                  args: args,
                  return_type: "()",
                  func_ptr: wrap__my_device as *const u8,
                  hidden: false,
              })
      }
      #[no_mangle]
      #[allow(non_snake_case)]
      pub fn get_extendrtests_metadata() -> extendr_api::metadata::Metadata {
          let mut functions = Vec::new();
          let mut impls = Vec::new();
          meta__hello_world(&mut functions);
          meta__do_nothing(&mut functions);
          meta__double_scalar(&mut functions);
          meta__int_scalar(&mut functions);
          meta__bool_scalar(&mut functions);
          meta__char_scalar(&mut functions);
          meta__char_vec(&mut functions);
          meta__double_vec(&mut functions);
          meta__get_doubles_element(&mut functions);
          meta__get_integers_element(&mut functions);
          meta__get_logicals_element(&mut functions);
          meta__doubles_square(&mut functions);
          meta__complexes_square(&mut functions);
          meta__integers_square(&mut functions);
          meta__logicals_not(&mut functions);
          meta__check_default(&mut functions);
          meta__try_rfloat_na(&mut functions);
          meta__try_rint_na(&mut functions);
          meta__check_rfloat_na(&mut functions);
          meta__check_rint_na(&mut functions);
          meta__special_param_names(&mut functions);
          meta____00__special_function_name(&mut functions);
          meta__test_rename_mymod(&mut functions);
          meta__get_default_value(&mut functions);
          meta__add_5_if_not_null(&mut functions);
          meta__my_device(&mut functions);
          meta__MyClass(&mut impls);
          meta____MyClass(&mut impls);
          meta__MyClassUnexported(&mut impls);
          functions.extend(altrep::get_altrep_metadata().functions);
          functions.extend(attributes::get_attributes_metadata().functions);
          functions.extend(dataframe::get_dataframe_metadata().functions);
          functions.extend(memory_leaks::get_memory_leaks_metadata().functions);
          functions.extend(optional_either::get_optional_either_metadata().functions);
          functions.extend(optional_ndarray::get_optional_ndarray_metadata().functions);
          functions.extend(optional_faer::get_optional_faer_metadata().functions);
          functions.extend(raw_identifiers::get_raw_identifiers_metadata().functions);
          functions.extend(submodule::get_submodule_metadata().functions);
          functions.extend(externalptr::get_externalptr_metadata().functions);
          functions.extend(typedsliceargs::get_typedsliceargs_metadata().functions);
          impls.extend(altrep::get_altrep_metadata().impls);
          impls.extend(attributes::get_attributes_metadata().impls);
          impls.extend(dataframe::get_dataframe_metadata().impls);
          impls.extend(memory_leaks::get_memory_leaks_metadata().impls);
          impls.extend(optional_either::get_optional_either_metadata().impls);
          impls.extend(optional_ndarray::get_optional_ndarray_metadata().impls);
          impls.extend(optional_faer::get_optional_faer_metadata().impls);
          impls.extend(raw_identifiers::get_raw_identifiers_metadata().impls);
          impls.extend(submodule::get_submodule_metadata().impls);
          impls.extend(externalptr::get_externalptr_metadata().impls);
          impls.extend(typedsliceargs::get_typedsliceargs_metadata().impls);
          functions
              .push(extendr_api::metadata::Func {
                  doc: "Metadata access function.",
                  rust_name: "get_extendrtests_metadata",
                  mod_name: "get_extendrtests_metadata",
                  r_name: "get_extendrtests_metadata",
                  args: Vec::new(),
                  return_type: "Metadata",
                  func_ptr: wrap__get_extendrtests_metadata as *const u8,
                  hidden: true,
              });
          functions
              .push(extendr_api::metadata::Func {
                  doc: "Wrapper generator.",
                  rust_name: "make_extendrtests_wrappers",
                  mod_name: "make_extendrtests_wrappers",
                  r_name: "make_extendrtests_wrappers",
                  args: <[_]>::into_vec(
                      #[rustc_box]
                      ::alloc::boxed::Box::new([
                          extendr_api::metadata::Arg {
                              name: "use_symbols",
                              arg_type: "bool",
                              default: None,
                          },
                          extendr_api::metadata::Arg {
                              name: "package_name",
                              arg_type: "&str",
                              default: None,
                          },
                      ]),
                  ),
                  return_type: "String",
                  func_ptr: wrap__make_extendrtests_wrappers as *const u8,
                  hidden: true,
              });
          extendr_api::metadata::Metadata {
              name: "extendrtests",
              functions,
              impls,
          }
      }
      #[no_mangle]
      #[allow(non_snake_case)]
      pub extern "C" fn wrap__get_extendrtests_metadata() -> extendr_api::SEXP {
          use extendr_api::GetSexp;
          unsafe { extendr_api::Robj::from(get_extendrtests_metadata()).get() }
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn wrap__make_extendrtests_wrappers(
          use_symbols_sexp: extendr_api::SEXP,
          package_name_sexp: extendr_api::SEXP,
      ) -> extendr_api::SEXP {
          unsafe {
              use extendr_api::robj::*;
              use extendr_api::GetSexp;
              let robj = Robj::from_sexp(use_symbols_sexp);
              let use_symbols: bool = <bool>::try_from(&robj).unwrap();
              let robj = Robj::from_sexp(package_name_sexp);
              let package_name: &str = <&str>::try_from(&robj).unwrap();
              extendr_api::Robj::from(
                      get_extendrtests_metadata()
                          .make_r_wrappers(use_symbols, package_name)
                          .unwrap(),
                  )
                  .get()
          }
      }
      #[no_mangle]
      #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
      pub extern "C" fn R_init_extendrtests_extendr(info: *mut extendr_api::DllInfo) {
          unsafe { extendr_api::register_call_methods(info, get_extendrtests_metadata()) };
      }

