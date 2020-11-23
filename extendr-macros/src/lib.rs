extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Expr, FnArg, Item, ItemFn, ItemImpl, Ident, parse::ParseStream};
use quote::{format_ident, quote};
use syn::Token;

mod output_r;

const INIT_PREFIX: &str = "init__";
const WRAP_PREFIX: &str = "wrap__";

#[derive(Debug)]
struct ExtendrOptions {
}

// Generate a list of arguments for the wrapper. All arguments are SEXP for .Call in R.
fn translate_formal(input: &FnArg, self_ty: Option<&syn::Type>) -> FnArg {
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
            if self_ty.is_none() {
                panic!("found &self in non-impl function - have you missed the #[extendr] before the impl?");
            }
            return parse_quote! { _self : extendr_api::SEXP };
        }
    }
}

// Convert SEXP arguments into Robj. This maintains the lifetime of references.
fn translate_to_robj(input: &FnArg) -> syn::Stmt {
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
        FnArg::Receiver(_) => {
            parse_quote! { let mut _self_robj = extendr_api::new_borrowed(_self); }
        }
    }
}

// Generate actual argument list for the call (ie. a list of conversions).
fn translate_actual(input: &FnArg) -> Option<Expr> {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            let ty = &pattype.ty.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                Some(parse_quote!{ extendr_api::unwrap_or_throw(<#ty>::from_robj(&#varname)) })
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

/// Parse a set of attribute arguments for #[extendr(opts...)]
fn parse_options(_opts: &mut ExtendrOptions, _arg: &syn::NestedMeta) {
    /*use syn::{Lit, Meta, MetaNameValue, NestedMeta};

    match arg {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ref path,
            eq_token: _,
            lit: Lit::Str(ref _lit_str),
        })) => {
        }
        _ => panic!("expected #[extendr(opt = \"string\", ...)]"),
    }*/
}

/// Generate bindings for a single function.
fn extendr_function(args: Vec<syn::NestedMeta>, func: ItemFn) -> TokenStream {
    let mut opts = ExtendrOptions {};

    for arg in &args {
        parse_options(&mut opts, arg);
    }

    let mut wrappers = Vec::new();
    generate_wrappers(&opts, &mut wrappers, "", &func.sig, None);

    TokenStream::from(quote!{
        #func

        # ( #wrappers )*
    })
}

// Generate wrappers for a specific function.
fn generate_wrappers(_opts: &ExtendrOptions, wrappers: &mut Vec<ItemFn>, prefix: &str, sig: &syn::Signature, self_ty: Option<&syn::Type>) {
    let func_name = &sig.ident;

    let raw_wrap_name = format!("{}{}{}", WRAP_PREFIX, prefix, func_name);
    let wrap_name = format_ident!("{}{}{}", WRAP_PREFIX, prefix, func_name);
    let init_name = format_ident!("{}{}{}", INIT_PREFIX, prefix, func_name);

    let wrap_name_str = format!("{}", wrap_name);

    let inputs = &sig.inputs;
    let has_self = match inputs.iter().next() {
        Some(FnArg::Receiver(_)) => true,
        _ => false
    };

    let call_name = if has_self {
        let is_mut = match inputs.iter().next() {
            Some(FnArg::Receiver(ref reciever)) => reciever.mutability.is_some(),
            _ => false
        };
        if is_mut {
            // eg. Person::name(&mut self)
            quote! { extendr_api::unwrap_or_throw(
                <&mut #self_ty>::from_robj(&_self_robj)
            ).#func_name }
        } else {
            // eg. Person::name(&self)
            quote! { extendr_api::unwrap_or_throw(
                <&#self_ty>::from_robj(&_self_robj)
            ).#func_name }
        }
    } else if let Some(ref self_ty) = &self_ty {
        // eg. Person::new()
        quote! { <#self_ty>::#func_name }
    } else {
        // eg. aux_func()
        quote! { #func_name }
    };

    let formal_args: Punctuated<FnArg, Token![,]> = inputs
        .iter()
        .map(|input| translate_formal(input, self_ty))
        .collect();

    let convert_args: Vec<syn::Stmt> = inputs
        .iter()
        .map(|input| translate_to_robj(input))
        .collect();

    let actual_args: Punctuated<Expr, Token![,]> = inputs
        .iter()
        .filter_map(|input| translate_actual(input))
        .collect();

    // let R_formal_args: Vec<String> = inputs
    //     .iter()
    //     .filter_map(|input| translate_R_formal(input))
    //     .collect();

    let num_args = inputs.len() as i32;

    output_r::output_r_wrapper(
        func_name,
        &raw_wrap_name,
        sig.inputs.clone().into_iter().collect(),
        &sig.output,
    );
    wrappers.push(parse_quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            unsafe {
                use extendr_api::FromRobj;
                #( #convert_args )*
                extendr_api::Robj::from(#call_name(#actual_args)).get()
            }
        }
    ));

    wrappers.push(parse_quote!(
        #[allow(non_snake_case)]
        fn #init_name(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
            call_methods.push(
                extendr_api::CallMethod {
                    call_symbol: std::ffi::CString::new(#wrap_name_str).unwrap(),
                    func_ptr: #wrap_name as * const u8,
                    num_args: #num_args,
                }
            )
        }
    ));
}

/// Handle trait implementations.
fn extendr_impl(mut item_impl: ItemImpl) -> TokenStream {
    let opts = ExtendrOptions {};
    let self_ty = item_impl.self_ty.as_ref();
    let self_ty_name = quote! {#self_ty}.to_string();
    let prefix = format!("{}__", self_ty_name);
    let mut method_init_names = Vec::new();
    let mut wrappers = Vec::new();
    for impl_item in &mut item_impl.items {
        if let syn::ImplItem::Method(ref mut method) = impl_item {
            method_init_names.push(format_ident!("{}{}__{}", INIT_PREFIX, self_ty_name, method.sig.ident));
            generate_wrappers(&opts, &mut wrappers, prefix.as_str(), &method.sig, Some(self_ty));
        }
    }

    let init_name = format_ident!("{}{}", INIT_PREFIX, self_ty_name);

    let finalizer_name = format_ident!("__finalize__{}", self_ty_name);

    let expanded = TokenStream::from(quote! {
        #item_impl

        #( #wrappers )*

        impl<'a> extendr_api::FromRobj<'a> for &#self_ty {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                if robj.check_external_ptr(#self_ty_name) {
                    Ok(unsafe { std::mem::transmute(robj.externalPtrAddr::<#self_ty>()) })
                } else {
                    Err(concat!("expected ", #self_ty_name))
                }
            }
        }

        impl<'a> extendr_api::FromRobj<'a> for &mut #self_ty {
            fn from_robj(robj: &'a Robj) -> Result<Self, &'static str> {
                if robj.check_external_ptr(#self_ty_name) {
                    Ok(unsafe { std::mem::transmute(robj.externalPtrAddr::<#self_ty>()) })
                } else {
                    Err(concat!("expected ", #self_ty_name))
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

        extern "C" fn #finalizer_name (sexp: extendr_api::SEXP) {
            unsafe {
                let robj = extendr_api::new_borrowed(sexp);
                if robj.check_external_ptr(#self_ty_name) {
                    //eprintln!("finalize {}", #self_ty_name);
                    let ptr = robj.externalPtrAddr::<#self_ty>();
                    Box::from_raw(ptr);
                }
            }
        }

        #[allow(non_snake_case)]
        fn #init_name(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
            #( #method_init_names(info, call_methods); )*
        }
    });

    //eprintln!("{}", expanded);
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

#[derive(Debug)]
struct Module {
    modname: Option<Ident>,
    fnnames: Vec<Ident>,
    implnames: Vec<Ident>,
}

impl syn::parse::Parse for Module {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::spanned::Spanned;
        let mut res = Self {
            modname: None,
            fnnames: Vec::new(),
            implnames: Vec::new(),
        };
        while !input.is_empty() {
            if let Ok(kmod) = input.parse::<Token![mod]>() {
                let name : Ident = input.parse()?;
                if !res.modname.is_none() {
                    return Err(syn::Error::new(kmod.span(), "only one mod allowed"));
                }
                res.modname = Some(name);
            } else if let Ok(_) = input.parse::<Token![fn]>() {
                res.fnnames.push(input.parse()?);
            } else if let Ok(_) = input.parse::<Token![impl]>() {
                res.implnames.push(input.parse()?);
            } else {
                return Err(syn::Error::new(input.span(), "expected mod, fn or impl"));
            }

            input.parse::<Token![;]>()?;
        }
        if res.modname.is_none() {
            return Err(syn::Error::new(input.span(), "expected one 'mod name'"));
        }
        Ok(res)
    }
}


/// Define a module and export symbols to R
/// Example:
///
/// extendr_module! {
///     mod name;
///     fn my_func1;
///     fn my_func2;
///     impl MyTrait;
/// }
/// 
#[proc_macro]
pub fn extendr_module(item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as Module);
    let Module {modname, fnnames, implnames} = module;
    let modname = modname.unwrap();
    let module_init_name = format_ident!("R_init_lib{}", modname);

    let fninitnames = fnnames.iter().map(|id| format_ident!("{}{}", INIT_PREFIX, id));
    let implinitnames = implnames.iter().map(|id| format_ident!("{}{}", INIT_PREFIX, id));

    TokenStream::from(quote!{
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #module_init_name(info: * mut extendr_api::DllInfo) {
            let mut call_methods = Vec::new();
            #( #fninitnames(info, &mut call_methods); )*
            #( #implinitnames(info, &mut call_methods); )*
            unsafe { extendr_api::register_call_methods(info, call_methods.as_ref()) };
        }
    })
}



