extern crate proc_macro;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Expr, FnArg, Item, ItemFn, ItemImpl};
use quote::{format_ident, quote};
use syn::Token;

#[derive(Debug)]
struct ExtendrOptions {
    self_ty: Option<syn::Type>,
}

// Generate a list of arguments for the wrapper. All arguments are SEXP for .Call in R.
fn translate_formal(input: &FnArg, opts: &ExtendrOptions) -> FnArg {
    match input {
        // function argument.
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            return parse_quote! { #pat : extendr_api::SEXP };
        }
        // &self
        FnArg::Receiver(ref reciever) => {
            if !reciever.attrs.is_empty() || reciever.reference.is_none() {
                panic!("expected &self or &mut self");
            }
            if opts.self_ty.is_none() {
                panic!("found &self in non-impl function - have you missed the #[extendr] before the impl?");
            }
            return parse_quote! { _self : extendr_api::SEXP };
        }
    }
}

// Convert SEXP arguments into Robj. This maintains the lifetime of references.
fn translate_convert(input: &FnArg, opts: &ExtendrOptions) -> syn::Stmt {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                parse_quote! { let #varname = extendr_api::new_borrowed(#pat); }
            } else {
                panic!("expect identifier as arg name")
            }
        }
        FnArg::Receiver(ref _reciever) => {
            let ty = opts.self_ty.clone().unwrap();
            //return parse_quote! { extendr_api::unwrap_or_throw(from_robj::<#ty>(&new_borrowed(&_self))) };
            parse_quote! { let mut _self = extendr_api::unwrap_or_throw(extendr_api::from_robj::<#ty>(&extendr_api::new_borrowed(_self))); }
        }
    }
}

// Generate actual argument list for the call (ie. a list of conversions).
fn translate_actual(input: &FnArg, _opts: &ExtendrOptions) -> Option<Expr> {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            let ty = &pattype.ty.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                Some(parse_quote!{ extendr_api::unwrap_or_throw(extendr_api::from_robj::<#ty>(&#varname)) })
            } else {
                None
            }
        }
        FnArg::Receiver(ref _reciever) => {
            // Do not use self explicitly as an actual arg.
            None
        }
    }
}

/// Parse a set of attribute arguments for #[extendr(opts...)]
fn parse_options(opts: &mut ExtendrOptions, arg: &syn::NestedMeta) {
    use syn::{Lit, Meta, MetaNameValue, NestedMeta, Type};

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit: Lit::Str(ref lit_str),
        })) => {
            if path.is_ident("self_ty") {
                use std::str::FromStr;
                let tokens = TokenStream::from_str(lit_str.value().as_str()).unwrap();
                opts.self_ty = Some(syn::parse::<Type>(tokens).unwrap());
            } else {
                panic!("expected self_ty = <Type>");
            }
        }
        _ => panic!("expected #[extendr(opt = \"string\", ...)]"),
    }
}

/// Generate bindings for a single function.
fn extendr_function(args: Vec<syn::NestedMeta>, func: ItemFn) -> TokenStream {
    let mut opts = ExtendrOptions { self_ty: None };

    for arg in &args {
        parse_options(&mut opts, arg);
    }

    let func_name = &func.sig.ident;

    let wrap_name = if let Some(ref self_ty) = &opts.self_ty {
        // Methods get __wrap__<type name>__<function name>
        let ty_name = quote!(#self_ty).to_string();
        format_ident!("__wrap__{}__{}", ty_name, func_name)
    } else {
        // Regular functions get __wrap__<function name>
        format_ident!("__wrap__{}", func_name)
    };

    let inputs = &func.sig.inputs;
    let has_self = match inputs.iter().next() {
        Some(FnArg::Receiver(_)) => true,
        _ => false
    };

    let call_name = if has_self {
        quote! { _self.#func_name }
    } else if let Some(ref self_ty) = &opts.self_ty {
        quote! { <#self_ty>::#func_name }
    } else {
        quote! { #func_name }
    };

    let formal_args: Punctuated<FnArg, Token![,]> = inputs
        .iter()
        .map(|input| translate_formal(input, &opts))
        .collect();

    let convert_args: Punctuated<syn::Stmt, Token![;]> = inputs
        .iter()
        .map(|input| translate_convert(input, &opts))
        .collect();

    let actual_args: Punctuated<Expr, Token![,]> = inputs
        .iter()
        .filter_map(|input| translate_actual(input, &opts))
        .collect();

    // Output the original function plus a wrapper function.
    let expanded = quote! {
        #func

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            unsafe {
                #convert_args
                extendr_api::Robj::from(#call_name(#actual_args)).get()
            }
        }
    };

    //println!("res: {}", expanded);
    TokenStream::from(expanded)
}

/// Handle trait implementations.
/// convert #[extendr] to #[extendr(self_ty = <type name>)]
fn extendr_impl(mut item_impl: ItemImpl) -> TokenStream {
    let self_ty = &item_impl.self_ty;
    let self_ty_name = quote! {#self_ty}.to_string();
    for impl_item in &mut item_impl.items {
        // Add extendr attribute to every method with the trait name.
        // TODO: Append to existing extendr attributes.
        if let syn::ImplItem::Method(ref mut method) = impl_item {
            let new_attr: syn::Attribute = parse_quote! { #[extendr(self_ty = #self_ty_name)] };
            method.attrs.push(new_attr);
        }
    }

    let finalizer_name = format_ident!("__finalize__{}", self_ty_name);


    let expanded = TokenStream::from(quote! {
        #item_impl

        impl<'a> extendr_api::FromRobj<'a> for #self_ty {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                Err("not done yet")
            }
        }

        extern "C" fn #finalizer_name (sexp: extendr_api::SEXP) {
            unsafe {
                let robj = extendr_api::new_borrowed(sexp);
                let tag = robj.externalPtrTag();
                if tag.as_str() == Some(#self_ty_name) {
                    let ptr = robj.externalPtrAddr::<#self_ty>();
                    Box::from_raw(ptr);
                }
            }
        }

        impl From<#self_ty> for Robj {
            fn from(value: #self_ty) -> Self {
                unsafe {
                    let ptr = Box::into_raw(Box::new(value));
                    let res = Robj::makeExternalPtr(ptr, Robj::from(#self_ty_name), Robj::from(()));
                    res.registerCFinalizer(Some(#finalizer_name));
                    res
                }
            }
        }
    });

    eprintln!("{}", expanded);
    expanded
}

/// Generate bindings for a single function.
#[proc_macro_attribute]
pub fn extendr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    match parse_macro_input!(item as Item) {
        Item::Fn(func) => return extendr_function(args, func),
        Item::Impl(item_impl) => return extendr_impl(item_impl),
        other_item => {
            TokenStream::from(quote! {#other_item})
        }
    }
}
