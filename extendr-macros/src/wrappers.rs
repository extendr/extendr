//! This is responsible for generating the C functions that act as wrappers of
//! the exported Rust functions.
//!
//! extendr relies on the [`.Call`-interface](https://cran.r-project.org/doc/manuals/R-exts.html#Calling-_002eCall)
//! In short, it is necessary the the signature of the C-function have [`SEXP`]
//! as the type for return type, and argument types.
//!
//! For instance, if your function returns nothing, the return type is not
//! allowed to be `void`, instead `SEXP` must be used, and one should return
//! [`R_NilValue`].
//!
//! ## R wrappers
//!
//! Within R, you may call `rextendr::document()` to generate R functions,
//! that use the `.Call`-interface, to call the wrapped Rust functions.
//!
//! You may also manually implement these wrappers, in order to do special
//! type-checking, or other annotation, that could be more convenient to do
//! on the R-side. The C-functions are named according to `"{WRAP_PREFIX}{prefix}{mod_name}"`.
//! See [`WRAP_PREFIX`], and note that `prefix` is set specifically for methods in
//! `extendr`-impl blocks, while for functions have no prefix.
//!
//! [`R_NilValue`]: https://extendr.github.io/libR-sys/libR_sys/static.R_NilValue.html
//! [`SEXP`]: https://extendr.github.io/libR-sys/libR_sys/type.SEXP.html

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Expr, ExprLit, FnArg, ItemFn, Token, Type};

use crate::extendr_options::ExtendrOptions;

pub const META_PREFIX: &str = "meta__";
pub const WRAP_PREFIX: &str = "wrap__";

// Generate wrappers for a specific function.
pub(crate) fn make_function_wrappers(
    opts: &ExtendrOptions,
    wrappers: &mut Vec<ItemFn>,
    prefix: &str,
    attrs: &[syn::Attribute],
    sig: &mut syn::Signature,
    self_ty: Option<&syn::Type>,
) -> syn::Result<()> {
    let rust_name = sig.ident.clone();

    let r_name_str = if let Some(r_name) = opts.r_name.as_ref() {
        r_name.clone()
    } else {
        sig.ident.to_string()
    };

    let mod_name = if let Some(mod_name) = opts.mod_name.as_ref() {
        format_ident!("{}", mod_name)
    } else {
        sig.ident.clone()
    };

    let mod_name = sanitize_identifier(mod_name);
    let wrap_name = format_ident!("{}{}{}", WRAP_PREFIX, prefix, mod_name);
    let meta_name = format_ident!("{}{}{}", META_PREFIX, prefix, mod_name);

    let rust_name_str = format!("{}", rust_name);
    let c_name_str = format!("{}", mod_name);
    let doc_string = get_doc_string(attrs);
    let return_type_string = get_return_type(sig);

    let inputs = &mut sig.inputs;
    let has_self = matches!(inputs.iter().next(), Some(FnArg::Receiver(_)));

    let call_name = if has_self {
        let is_mut = match inputs.iter().next() {
            Some(FnArg::Receiver(ref receiver)) => receiver.mutability.is_some(),
            _ => false,
        };
        if is_mut {
            // eg. Person::name(&mut self)
            quote! { extendr_api::unwrap_or_throw_error(
                <&mut #self_ty>::try_from(&mut _self_robj)
            ).#rust_name }
        } else {
            // eg. Person::name(&self)
            quote! { extendr_api::unwrap_or_throw_error(
                <&#self_ty>::try_from(&_self_robj)
            ).#rust_name }
        }
    } else if let Some(ref self_ty) = &self_ty {
        // eg. Person::new()
        quote! { <#self_ty>::#rust_name }
    } else {
        // eg. aux_func()
        quote! { #rust_name }
    };

    // arguments for the wrapper with type being `SEXP`
    let formal_args = inputs
        .iter()
        .map(|input| translate_formal(input, self_ty))
        .collect::<syn::Result<Punctuated<FnArg, Token![,]>>>()?;

    // extract the names of the arguments only (`mut` are ignored in `formal_args` already)
    let sexp_args = formal_args
        .clone()
        .into_iter()
        .map(|x| match x {
            // the wrapper doesn't use `self` arguments
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(ref typed) => match typed.pat.as_ref() {
                syn::Pat::Ident(ref pat_ident) => pat_ident.ident.clone(),
                _ => unreachable!(),
            },
        })
        .collect::<Vec<Ident>>();

    // arguments from R (`SEXP`s) are converted to `Robj`
    let convert_args: Vec<syn::Stmt> = inputs
        .iter()
        .map(translate_to_robj)
        .collect::<syn::Result<Vec<syn::Stmt>>>()?;

    let actual_args: Punctuated<Expr, Token![,]> =
        inputs.iter().filter_map(translate_actual).collect();

    let meta_args: Vec<Expr> = inputs
        .iter_mut()
        .map(|input| translate_meta_arg(input, self_ty))
        .collect::<syn::Result<Vec<Expr>>>()?;

    // Generate wrappers for rust functions to be called from R.
    // Example:
    // ```
    // #[no_mangle]
    // #[allow(non_snake_case)]
    // pub extern "C" fn wrap__hello() -> extendr_api::SEXP {
    //     unsafe {
    //         use extendr_api::FromRobj;
    //         extendr_api::Robj::from(hello()).get()
    //     }
    // }
    // ```
    let rng_start = opts
        .use_rng
        .then(|| {
            quote!(single_threaded(|| unsafe {
                extendr_api::GetRNGstate();
            });)
        })
        .unwrap_or_default();
    let rng_end = opts
        .use_rng
        .then(|| {
            quote!(single_threaded(|| unsafe {
                extendr_api::PutRNGstate();
            });)
        })
        .unwrap_or_default();

    // figure out if
    // -> &Self
    // -> &mut Self
    // Or if instead of `Self` the type name is used directly
    // -> &ImplType / &mut ImplType
    let return_is_ref_self = {
        match sig.output {
            // matches -> () or no-return type
            syn::ReturnType::Default => false,
            // ignoring the `-> Self` or `-> ImplType`, as that is not a Reference-type
            // matches -> &T or &mut T
            syn::ReturnType::Type(_, ref return_type) => match return_type.as_ref() {
                Type::Reference(ref reference_type) => {
                    // checks if T is Self or explicit impl type name
                    if let Type::Path(path) = reference_type.elem.as_ref() {
                        let is_typename_impl_type = self_ty
                            .map(|x| x == reference_type.elem.as_ref())
                            .unwrap_or(false);
                        path.path.is_ident("Self") || is_typename_impl_type
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    };

    let return_type_conversion = if return_is_ref_self {
        // instead of converting &Self / &mut Self, pass on the passed
        // ExternalPtr<Self>
        quote!(
            let return_ref_to_self = #call_name(#actual_args);

            #(
            if std::ptr::addr_eq(
                extendr_api::R_ExternalPtrAddr(#sexp_args),
                std::ptr::from_ref(return_ref_to_self)) {
                    return Ok(extendr_api::Robj::from_sexp(#sexp_args))
                }
            )*
            Err(Error::ExpectedExternalPtrReference)
        )
    } else {
        quote!(Ok(extendr_api::Robj::from(#call_name(#actual_args))))
    };

    // TODO: the unsafe in here is unnecessary
    wrappers.push(parse_quote!(
        #[no_mangle]
        #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            use extendr_api::robj::*;

            // pull RNG state before evaluation
            #rng_start

            let wrap_result_state: std::result::Result<
                std::result::Result<extendr_api::Robj, extendr_api::Error>,
                Box<dyn std::any::Any + Send>
            > = unsafe {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || -> std::result::Result<extendr_api::Robj, extendr_api::Error> {
                    #(#convert_args)*
                    #return_type_conversion
                }))
            };

            // return RNG state back to r after evaluation
            #rng_end

            // any obj created in above unsafe scope, which are not moved into wrap_result_state are now dropped
            match wrap_result_state {
                Ok(Ok(zz)) => {
                    return unsafe { zz.get() };
                }
                // any conversion error bubbled from #actual_args conversions of incoming args from R.
                Ok(Err(conversion_err)) => {
                    let err_string = conversion_err.to_string();
                    drop(conversion_err); // try_from=true errors contain Robj, this must be dropped to not leak
                    extendr_api::throw_r_error(&err_string);
                }
                // any panic (induced by user func code or if user func yields a Result-Err as return value)
                Err(unwind_err) => {
                    drop(unwind_err); //did not notice any difference if dropped or not.
                    // It should be possible to downcast the unwind_err Any type to the error
                    // included in panic. The advantage would be the panic cause could be included
                    // in the R terminal error message and not only via std-err.
                    // but it should be handled in a separate function and not in-lined here.
                    let err_string = format!("User function panicked: {}", #r_name_str);
                    // cannot use throw_r_error here for some reason.
                    // handle_panic() exports err string differently than throw_r_error.
                    extendr_api::handle_panic(err_string.as_str(), || panic!());
                }
            }
            unreachable!("internal extendr error, this should never happen.")
        }
    ));

    // Generate a function to push the metadata for a function.
    wrappers.push(parse_quote!(
        #[allow(non_snake_case)]
        fn #meta_name(metadata: &mut Vec<extendr_api::metadata::Func>) {
            let args = vec![
                #( #meta_args, )*
            ];

            metadata.push(extendr_api::metadata::Func {
                doc: #doc_string,
                rust_name: #rust_name_str,
                r_name: #r_name_str,
                mod_name: #c_name_str,
                args: args,
                return_type: #return_type_string,
                func_ptr: #wrap_name as * const u8,
                hidden: false,
            })
        }
    ));

    Ok(())
}

// Extract doc strings from attributes.
pub fn get_doc_string(attrs: &[syn::Attribute]) -> String {
    let mut res = String::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }

        if let syn::Meta::NameValue(ref nv) = attr.meta {
            if let Expr::Lit(ExprLit {
                lit: syn::Lit::Str(ref litstr),
                ..
            }) = nv.value
            {
                if !res.is_empty() {
                    res.push('\n');
                }
                res.push_str(&litstr.value());
            }
        }
    }
    res
}

pub fn get_return_type(sig: &syn::Signature) -> String {
    match &sig.output {
        syn::ReturnType::Default => "()".into(),
        syn::ReturnType::Type(_, ref rettype) => type_name(rettype),
    }
}

pub fn mangled_type_name(type_: &Type) -> String {
    let src = quote!( #type_ ).to_string();
    let mut res = String::new();
    for c in src.chars() {
        if c != ' ' {
            if c.is_alphanumeric() {
                res.push(c)
            } else {
                let f = format!("_{:02x}", c as u32);
                res.push_str(&f);
            }
        }
    }
    res
}

/// Return a simplified type name that will be meaningful to R. Defaults to a digest.
// For example:
// & Fred -> Fred
// * Fred -> Fred
// && Fred -> Fred
// Fred<'a> -> Fred
// &[i32] -> _hex_hex_hex_hex
//
pub fn type_name(type_: &Type) -> String {
    match type_ {
        Type::Path(syn::TypePath { path, .. }) => {
            if let Some(ident) = path.get_ident() {
                ident.to_string()
            } else if path.segments.len() == 1 {
                let seg = path.segments.clone().into_iter().next().unwrap();
                seg.ident.to_string()
            } else {
                mangled_type_name(type_)
            }
        }
        Type::Group(syn::TypeGroup { elem, .. }) => type_name(elem),
        Type::Reference(syn::TypeReference { elem, .. }) => type_name(elem),
        Type::Paren(syn::TypeParen { elem, .. }) => type_name(elem),
        Type::Ptr(syn::TypePtr { elem, .. }) => type_name(elem),
        _ => mangled_type_name(type_),
    }
}

// Generate a list of arguments for the wrapper. All arguments are SEXP for .Call in R.
pub fn translate_formal(input: &FnArg, self_ty: Option<&syn::Type>) -> syn::Result<FnArg> {
    match input {
        // function argument.
        FnArg::Typed(ref pattype) => {
            let pat = pattype.pat.as_ref();
            // ensure that `mut` in args are ignored in the wrapper
            let pat_ident = translate_only_alias(pat)?;
            Ok(parse_quote! { #pat_ident: extendr_api::SEXP })
        }
        // &self / &mut self
        FnArg::Receiver(ref receiver) => {
            if !receiver.attrs.is_empty() || receiver.reference.is_none() {
                return Err(syn::Error::new_spanned(
                    input,
                    "expected &self or &mut self",
                ));
            }
            if self_ty.is_none() {
                return Err(syn::Error::new_spanned(
                    input,"found &self in non-impl function - have you missed the #[extendr] before the impl?"
                ));
            }
            Ok(parse_quote! { _self : extendr_api::SEXP })
        }
    }
}

/// Returns only the alias from a function argument.
///
/// For example `mut x: Vec<i32>`, the alias is `x`, but the `mut` would still
/// be present if only the `Ident` of `PatType` was used.
fn translate_only_alias(pat: &syn::Pat) -> Result<&Ident, syn::Error> {
    Ok(match pat {
        syn::Pat::Ident(ref pat_ident) => &pat_ident.ident,
        _ => {
            return Err(syn::Error::new_spanned(
                pat,
                "failed to translate name of argument",
            ));
        }
    })
}

// Generate code to make a metadata::Arg.
fn translate_meta_arg(input: &mut FnArg, self_ty: Option<&syn::Type>) -> syn::Result<Expr> {
    match input {
        // function argument.
        FnArg::Typed(ref mut pattype) => {
            let pat = pattype.pat.as_ref();
            let ty = pattype.ty.as_ref();
            // here the argument name is extracted, without the `mut` keyword,
            // ensuring the generated r-wrappers, can use these argument names
            let pat_ident = translate_only_alias(pat)?;
            let name_string = quote! { #pat_ident }.to_string();
            let type_string = type_name(ty);
            let default = if let Some(default) = get_named_lit(&mut pattype.attrs, "default") {
                quote!(Some(#default))
            } else {
                quote!(None)
            };
            Ok(parse_quote! {
                extendr_api::metadata::Arg {
                    name: #name_string,
                    arg_type: #type_string,
                    default: #default
                }
            })
        }
        // &self
        FnArg::Receiver(ref receiver) => {
            if !receiver.attrs.is_empty() || receiver.reference.is_none() {
                return Err(syn::Error::new_spanned(
                    input,
                    "expected &self or &mut self",
                ));
            }
            if self_ty.is_none() {
                return Err(syn::Error::new_spanned(
                    input,
            "found &self in non-impl function - have you missed the #[extendr] before the impl?"
        )
    );
            }
            let type_string = type_name(self_ty.unwrap());
            Ok(parse_quote! {
                extendr_api::metadata::Arg {
                    name: "self",
                    arg_type: #type_string,
                    default: None
                }
            })
        }
    }
}

/// Convert `SEXP` arguments into `Robj`.
/// This maintains the lifetime of references.
///
/// These conversions are from R into Rust
fn translate_to_robj(input: &FnArg) -> syn::Result<syn::Stmt> {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                let ident = &ident.ident;
                // TODO: these do not need protection, as they come from R
                Ok(parse_quote! { let #varname = extendr_api::robj::Robj::from_sexp(#ident); })
            } else {
                Err(syn::Error::new_spanned(
                    input,
                    "expect identifier as arg name",
                ))
            }
        }
        FnArg::Receiver(_) => {
            // this is `mut`, in case of a mutable reference
            Ok(parse_quote! { let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self); })
        }
    }
}

// Generate actual argument list for the call (ie. a list of conversions).
fn translate_actual(input: &FnArg) -> Option<Expr> {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                Some(parse_quote! {
                    #varname.try_into()?
                })
            } else {
                None
            }
        }
        FnArg::Receiver(_) => {
            // Do not use self explicitly as an actual arg.
            None
        }
    }
}

// Get a single named literal from a list of attributes.
// eg. #[default="xyz"]
// Remove the attribute from the list.
fn get_named_lit(attrs: &mut Vec<syn::Attribute>, name: &str) -> Option<String> {
    let mut new_attrs = Vec::new();
    let mut res = None;
    for a in attrs.drain(0..) {
        if let syn::Meta::NameValue(ref nv) = a.meta {
            if nv.path.is_ident(name) {
                if let Expr::Lit(ExprLit {
                    lit: syn::Lit::Str(ref litstr),
                    ..
                }) = nv.value
                {
                    res = Some(litstr.value());
                    continue;
                }
            }
        }

        new_attrs.push(a);
    }
    *attrs = new_attrs;
    res
}

// Remove the raw identifier prefix (`r#`) from an [`Ident`]
// If the `Ident` does not start with the prefix, it is returned as is.
fn sanitize_identifier(ident: Ident) -> Ident {
    static PREFIX: &str = "r#";
    let (ident, span) = (ident.to_string(), ident.span());
    let ident = match ident.strip_prefix(PREFIX) {
        Some(ident) => ident.into(),
        None => ident,
    };

    Ident::new(&ident, span)
}
