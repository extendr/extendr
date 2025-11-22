//! Generation of the C shims and metadata builders for exported Rust functions.
//! Focuses on turning a `syn::Signature` plus attributes into the wrapper and
//! associated metadata in an idiomatic, parsed form.

use crate::{
    extendr_options::{ExtendrOptions, ResultMode},
    utils::{doc_string, sanitize_identifier, take_string_literal_attr, type_name},
};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::{collections::HashMap, sync::Mutex};
use syn::{
    parse_quote, spanned::Spanned, Expr, FnArg, ItemFn, Pat, PatIdent, ReturnType, Signature, Stmt,
    Token, Type,
};

pub const META_PREFIX: &str = "meta__";
pub const WRAP_PREFIX: &str = "wrap__";

lazy_static::lazy_static! {
    static ref STRUCT_DOCS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Called by the struct-level `#[extendr]` macro to register docstrings.
pub fn register_struct_doc(name: &str, doc: &str) {
    STRUCT_DOCS
        .lock()
        .unwrap()
        .insert(name.to_string(), doc.to_string());
}

/// Retrieve the struct-level docs (or empty if none).
pub fn get_struct_doc(name: &str) -> String {
    STRUCT_DOCS
        .lock()
        .unwrap()
        .get(name)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn make_function_wrappers(
    opts: &ExtendrOptions,
    wrappers: &mut Vec<ItemFn>,
    prefix: &str,
    attrs: &[syn::Attribute],
    sig: &mut Signature,
    self_ty: Option<&Type>,
) -> syn::Result<()> {
    let builder = WrapperBuilder::new(opts, prefix, attrs, sig, self_ty)?;
    let tokens = builder.build();
    wrappers.push(tokens.wrapper);
    wrappers.push(tokens.metadata);
    Ok(())
}

struct WrapperTokens {
    wrapper: ItemFn,
    metadata: ItemFn,
}

#[derive(Debug)]
struct FnParam<'a> {
    ident: Ident,
    ty: &'a Type,
    default: Option<String>,
}

#[derive(Debug)]
enum Receiver<'a> {
    Ref { mutable: bool, ty: &'a Type },
}

#[derive(Debug, Default)]
struct FunctionArgs<'a> {
    receiver: Option<Receiver<'a>>,
    params: Vec<FnParam<'a>>,
}

impl<'a> FunctionArgs<'a> {
    fn parse(sig: &'a mut Signature, self_ty: Option<&'a Type>) -> syn::Result<Self> {
        let mut res = Self::default();

        for input in sig.inputs.iter_mut() {
            match input {
                FnArg::Receiver(receiver) => {
                    if receiver.attrs.is_empty() && receiver.reference.is_some() {
                        let ty = self_ty.ok_or_else(|| {
                            syn::Error::new(
                                receiver.span(),
                                "found &self in non-impl function - have you missed the #[extendr] before the impl?",
                            )
                        })?;
                        res.receiver = Some(Receiver::Ref {
                            mutable: receiver.mutability.is_some(),
                            ty,
                        });
                    } else {
                        return Err(syn::Error::new_spanned(
                            receiver,
                            "expected &self or &mut self",
                        ));
                    }
                }
                FnArg::Typed(pat_type) => {
                    let ident = param_ident(pat_type.pat.as_ref())?;
                    let default = extendr_default(pat_type)
                        .or_else(|| take_string_literal_attr(&mut pat_type.attrs, "default"));
                    res.params.push(FnParam {
                        ident,
                        ty: pat_type.ty.as_ref(),
                        default,
                    });
                }
            }
        }

        Ok(res)
    }
}

struct WrapperBuilder<'a> {
    opts: &'a ExtendrOptions,
    prefix: &'a str,
    attrs: &'a [syn::Attribute],
    rust_name: Ident,
    return_type: ReturnType,
    args: FunctionArgs<'a>,
    self_ty: Option<&'a Type>,
}

impl<'a> WrapperBuilder<'a> {
    fn new(
        opts: &'a ExtendrOptions,
        prefix: &'a str,
        attrs: &'a [syn::Attribute],
        sig: &'a mut Signature,
        self_ty: Option<&'a Type>,
    ) -> syn::Result<Self> {
        let rust_name = sig.ident.clone();
        let return_type = sig.output.clone();
        let args = FunctionArgs::parse(sig, self_ty)?;
        Ok(Self {
            opts,
            prefix,
            attrs,
            rust_name,
            return_type,
            args,
            self_ty,
        })
    }

    fn build(&self) -> WrapperTokens {
        let rust_name = self.rust_name.clone();
        let r_name = self
            .opts
            .r_name
            .clone()
            .unwrap_or_else(|| rust_name.to_string());
        let mod_ident = sanitize_identifier(
            self.opts
                .mod_name
                .as_ref()
                .map(|name| format_ident!("{}", name))
                .unwrap_or_else(|| rust_name.clone()),
        );
        let wrap_name = format_ident!("{}{}{}", WRAP_PREFIX, self.prefix, mod_ident);
        let meta_name = format_ident!("{}{}{}", META_PREFIX, self.prefix, mod_ident);
        let rust_name_str = rust_name.to_string();
        let wrap_name_str = wrap_name.to_string();
        let mod_name_str = mod_ident.to_string();
        let doc_string = doc_string(self.attrs);
        let return_type_string = return_type(&self.return_type);
        let opts_invisible = match self.opts.invisible {
            Some(true) => quote!(Some(true)),
            Some(false) => quote!(Some(false)),
            None => quote!(None),
        };

        let formal_args = self.formal_args();
        let sexp_args = formal_args
            .iter()
            .map(|arg| match arg {
                FnArg::Typed(typed) => param_ident(typed.pat.as_ref()).unwrap(),
                FnArg::Receiver(_) => unreachable!(),
            })
            .collect::<Vec<_>>();
        let convert_args = self.robj_conversions();
        let actual_args = self.actual_args();
        let call_expr = self.call_expression(&actual_args);
        let return_conversion = self.return_conversion(call_expr, &sexp_args);
        let rng_start = self.rng_start();
        let rng_end = self.rng_end();
        let len_meta_args = self.args.params.len() + self.args.receiver.is_some() as usize;

        let wrapper = parse_quote!(
            #[no_mangle]
            #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
            pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
                use extendr_api::robj::*;

                #rng_start

                let wrap_result_state: std::result::Result<
                    std::result::Result<extendr_api::Robj, Box<dyn std::error::Error>>,
                    Box<dyn std::any::Any + Send>
                > = unsafe {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || -> std::result::Result<extendr_api::Robj, Box<dyn std::error::Error>> {
                        #(#convert_args;)*
                        #return_conversion
                    }))
                };

                #rng_end

                match wrap_result_state {
                    Ok(Ok(zz)) => unsafe { zz.get() },
                    Ok(Err(conversion_err)) => {
                        let err_string = conversion_err.to_string();
                        drop(conversion_err);
                        extendr_api::throw_r_error(&err_string);
                    }
                    Err(unwind_err) => {
                        let mut msg = if let Some(s) = unwind_err.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = unwind_err.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            format!("panic in {}", #r_name)
                        };

                        let env_bt = std::env::var("EXTENDR_BACKTRACE")
                            .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
                            .unwrap_or(false);
                        if !env_bt {
                            // leave msg as-is to avoid Rust backtrace noise
                        }
                        extendr_api::throw_r_error(msg.as_str());
                    }
                }
            }
        );

        let meta_args = self.meta_args();
        let metadata = parse_quote!(
            #[allow(non_snake_case)]
            fn #meta_name(metadata: &mut Vec<extendr_api::metadata::Func>) {
                let mut args = Vec::with_capacity(#len_meta_args);
                #(
                    args.push(#meta_args);
                )*
                let args = args;

                metadata.push(extendr_api::metadata::Func {
                    doc: #doc_string,
                    rust_name: #rust_name_str,
                    r_name: #r_name,
                    c_name: #wrap_name_str,
                    mod_name: #mod_name_str,
                    args: args,
                    return_type: #return_type_string,
                    func_ptr: #wrap_name as * const u8,
                    hidden: false,
                    invisible: #opts_invisible,
                })
            }
        );

        WrapperTokens { wrapper, metadata }
    }

    fn formal_args(&self) -> syn::punctuated::Punctuated<FnArg, Token![,]> {
        let mut formal_args = syn::punctuated::Punctuated::<FnArg, Token![,]>::new();
        if self.args.receiver.is_some() {
            formal_args.push(parse_quote! { _self: extendr_api::SEXP });
        }
        for param in &self.args.params {
            let ident = &param.ident;
            formal_args.push(parse_quote! { #ident: extendr_api::SEXP });
        }
        formal_args
    }

    fn robj_conversions(&self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        if self.args.receiver.is_some() {
            stmts.push(
                parse_quote! { let mut _self_robj = extendr_api::robj::Robj::from_sexp(_self); },
            );
        }
        for param in &self.args.params {
            let robj_ident = format_ident!("_{}_robj", param.ident);
            let ident = &param.ident;
            stmts.push(
                parse_quote! { let #robj_ident = extendr_api::robj::Robj::from_sexp(#ident); },
            );
        }
        stmts
    }

    fn actual_args(&self) -> Vec<Expr> {
        self.args
            .params
            .iter()
            .map(|param| {
                let robj_ident = format_ident!("_{}_robj", param.ident);
                let arg_name = param.ident.to_string();
                parse_quote! {
                    #robj_ident
                        .try_into()
                        .map_err(|e| extendr_api::error::Error::Other(format!(
                            "failed to convert argument '{}' from R: {}",
                            #arg_name,
                            e
                        )))?
                }
            })
            .collect()
    }

    fn call_expression(&self, actual_args: &[Expr]) -> proc_macro2::TokenStream {
        let rust_name = &self.rust_name;
        match &self.args.receiver {
            Some(Receiver::Ref { mutable, ty }) => {
                if *mutable {
                    quote! { extendr_api::unwrap_or_throw_error(<&mut #ty>::try_from(&mut _self_robj)).#rust_name(#(#actual_args),*) }
                } else {
                    quote! { extendr_api::unwrap_or_throw_error(<&#ty>::try_from(&_self_robj)).#rust_name(#(#actual_args),*) }
                }
            }
            None => self
                .self_ty
                .map(|ty| quote!(<#ty>::#rust_name(#(#actual_args),*)))
                .unwrap_or_else(|| quote!(#rust_name(#(#actual_args),*))),
        }
    }

    fn return_conversion(
        &self,
        call_expression: proc_macro2::TokenStream,
        sexp_args: &[Ident],
    ) -> proc_macro2::TokenStream {
        if let Some(result_mode) = self.opts.result_mode {
            if is_result_type(&self.return_type) {
                return match result_mode {
                    ResultMode::Default => {
                        quote!(Ok(extendr_api::Robj::from(#call_expression)))
                    }
                    ResultMode::List => quote!({
                        match #call_expression {
                            Ok(val) => Ok(extendr_api::Robj::from(extendr_api::list!(ok = extendr_api::Robj::from(val), err = extendr_api::NULL))),
                            Err(err) => {
                                let err_robj = extendr_api::robj::IntoRobj::into_robj(err);
                                if err_robj.is_null() {
                                    return Err("result_list not allowed to return NULL as err-value".into());
                                }
                                Ok(extendr_api::Robj::from(extendr_api::list!(ok = extendr_api::NULL, err = err_robj)))
                            }
                        }
                    }),
                    ResultMode::Condition => quote!({
                        match #call_expression {
                            Ok(val) => Ok(extendr_api::robj::IntoRobj::into_robj(val)),
                            Err(err) => {
                                let mut cond = extendr_api::list!(
                                    message = "extendr_err",
                                    value = extendr_api::robj::IntoRobj::into_robj(err)
                                );
                                cond.set_class(["extendr_error", "error", "condition"])
                                    .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;
                                Ok(extendr_api::Robj::from(cond))
                            }
                        }
                    }),
                };
            }
        }

        if !returns_self_reference(&self.return_type, self.self_ty) {
            return quote!(Ok(extendr_api::Robj::from(#call_expression)));
        }

        let self_ty = self.self_ty.expect("self_ty available for self reference");
        let pointer_matches = sexp_args.iter().map(|sexp_arg| {
            quote! {
                let arg_ref = extendr_api::R_ExternalPtrAddr(#sexp_arg)
                    .cast::<Box<dyn std::any::Any>>()
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<#self_ty>()
                    .unwrap();
                if std::ptr::addr_eq(arg_ref, std::ptr::from_ref(return_ref_to_self)) {
                    return Ok(extendr_api::Robj::from_sexp(#sexp_arg));
                }
            }
        });

        quote!({
            let return_ref_to_self = #call_expression;
            #(#pointer_matches)*
            Err(extendr_api::Error::ExpectedExternalPtrReference.into())
        })
    }

    fn meta_args(&self) -> Vec<Expr> {
        let mut meta_args = Vec::new();
        if let Some(self_ty) = self.self_ty {
            if self.args.receiver.is_some() {
                let type_string = type_name(self_ty);
                meta_args.push(parse_quote! {
                    extendr_api::metadata::Arg {
                        name: "self",
                        arg_type: #type_string,
                        default: None
                    }
                });
            }
        }

        for param in &self.args.params {
            let default = if let Some(default) = &param.default {
                quote!(Some(#default))
            } else {
                quote!(None)
            };
            let param_name = param.ident.to_string();
            let param_type = type_name(param.ty);
            meta_args.push(parse_quote! {
                extendr_api::metadata::Arg {
                    name: #param_name,
                    arg_type: #param_type,
                    default: #default
                }
            });
        }
        meta_args
    }

    fn rng_start(&self) -> proc_macro2::TokenStream {
        if self.opts.use_rng {
            quote!(extendr_api::single_threaded(|| unsafe { extendr_api::GetRNGstate(); });)
        } else {
            quote!()
        }
    }

    fn rng_end(&self) -> proc_macro2::TokenStream {
        if self.opts.use_rng {
            quote!(extendr_api::single_threaded(|| unsafe { extendr_api::PutRNGstate(); });)
        } else {
            quote!()
        }
    }
}

fn param_ident(pat: &Pat) -> syn::Result<Ident> {
    match pat {
        Pat::Ident(PatIdent { ident, .. }) => Ok(ident.clone()),
        _ => Err(syn::Error::new_spanned(
            pat,
            "failed to translate name of argument",
        )),
    }
}

fn return_type(output: &ReturnType) -> String {
    match output {
        syn::ReturnType::Default => "()".into(),
        syn::ReturnType::Type(_, return_type) => type_name(return_type),
    }
}

fn returns_self_reference(output: &ReturnType, self_ty: Option<&Type>) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, return_type) => match return_type.as_ref() {
            Type::Reference(reference_type) => {
                if let Type::Path(path) = reference_type.elem.as_ref() {
                    let matches_self_ty = self_ty
                        .map(|ty| ty == reference_type.elem.as_ref())
                        .unwrap_or(false);
                    path.path.is_ident("Self") || matches_self_ty
                } else {
                    false
                }
            }
            _ => false,
        },
    }
}

fn is_result_type(output: &ReturnType) -> bool {
    let ty = match output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => return false,
    };
    if let Type::Path(path) = ty.as_ref() {
        let ident_matches = |ident: &syn::Ident| ident == "Result";
        if let Some(last) = path.path.segments.last() {
            if ident_matches(&last.ident) {
                return true;
            }
        }
        if let Some(first) = path.path.segments.first() {
            if first.ident == "std" || first.ident == "core" {
                if let Some(second) = path.path.segments.iter().nth(1) {
                    if second.ident == "result" {
                        if let Some(third) = path.path.segments.iter().nth(2) {
                            if ident_matches(&third.ident) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn extendr_default(attr: &mut syn::PatType) -> Option<String> {
    use syn::Lit;

    let mut new_attrs = Vec::new();
    let mut res = None;

    for i in attr.attrs.drain(..) {
        if let syn::Meta::List(ref meta_list) = i.meta {
            if meta_list.path.is_ident("extendr") {
                let mut default_value = None;
                let mut has_default = false;

                let parse_result = meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        has_default = true;
                        let value = meta.value()?;
                        if let Ok(Lit::Str(litstr)) = value.parse() {
                            default_value = Some(litstr.value());
                        }
                    }
                    Ok(())
                });

                if parse_result.is_ok() && has_default {
                    res = default_value;
                    continue;
                }
            }
        }
        new_attrs.push(i);
    }

    attr.attrs = new_attrs;
    res
}
