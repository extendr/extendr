//!
//! Macros for generating wrappers for rust functions.

//
// We can invoke the #[extendr] macro on functions or struct impls.
// 
// eg.
//
// ```ignore
// #[extendr]
// fn hello() -> &'static str {
//     "hello"
// }
// ```
//
// These macros add additional functions which you can see using the
// `cargo expand` extension.
//
// Invoking the #[extendr_module] macro generates an entrypoint for the
// library that will be called by R. Note that we add a postfix
// `_extendr` to the init function because we need to forward routine
// registration from C to Rust, and the C function will be called
// `R_init_hello()`.
//
// ```ignore
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn R_init_hello_extendr(info: *mut extendr_api::DllInfo) {
//     let mut call_methods = Vec::new();
//     init__hello(info, &mut call_methods);
//     unsafe { extendr_api::register_call_methods(info, call_methods.as_ref()) };
// }
// ```
//
// The module also generates the `init__` functions that provide metadata
// to R to register the wrappers.
//
// ```ignore
// #[allow(non_snake_case)]
// fn init__hello(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
//     call_methods.push(extendr_api::CallMethod {
//         call_symbol: std::ffi::CString::new("wrap__hello").unwrap(),
//         func_ptr: wrap__hello as *const u8,
//         num_args: 0i32,
//     })
// }
// ```
//
// In the case of struct impls we also generate the following:
//
// * Wrappers and init functions for all methods.
// * A single init function that calls the other init functions for the methods.
// * An input conversion from an external pointer to a reference and a move of that type.
// * An output converstion from that type to an owned external pointer object.
// * A finalizer for that type to free memory allocated.

extern crate proc_macro;
use proc_macro::{TokenStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Expr, FnArg, Item, ItemFn, ItemImpl, Ident, parse::ParseStream};
use quote::{format_ident, quote};
use syn::Token;

mod output_r;

const META_PREFIX: &str = "meta__";
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

// Generate a list of arguments for the wrapper. All arguments are SEXP for .Call in R.
fn translate_meta_arg(input: &FnArg, self_ty: Option<&syn::Type>) -> Expr {
    match input {
        // function argument.
        FnArg::Typed(ref pattype) => {
            let pat = pattype.pat.as_ref();
            let ty = pattype.ty.as_ref();
            let name_string = quote!{ #pat }.to_string();
            let type_string = quote!{ #ty }.to_string();
            return parse_quote! {
                extendr_api::metadata::Arg {
                    doc: "",
                    name: #name_string,
                    arg_type: #type_string
                }
            }
        }
        // &self
        FnArg::Receiver(ref reciever) => {
            if !reciever.attrs.is_empty() || reciever.reference.is_none() {
                panic!("expected &self or &mut self");
            }
            if self_ty.is_none() {
                panic!("found &self in non-impl function - have you missed the #[extendr] before the impl?");
            }
            let name_string = quote!{ #reciever }.to_string();
            let type_string = quote!{ #self_ty }.to_string();
            return parse_quote! {
                extendr_api::metadata::Arg {
                    doc: "",
                    name: #name_string,
                    arg_type: #type_string
                }
            }
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
    let meta_name = format_ident!("{}{}{}", META_PREFIX, prefix, func_name);

    // TODO: extract this from attributes.
    let name_str = format!("{}", func_name);
    let doc_string = "";
    let return_type = &sig.output;
    let return_type_string = quote!{ #return_type }.to_string();

    let wrap_name_str = format!("{}", wrap_name);
    let panic_str = format!("{} paniced.\0", func_name);

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

    let meta_args: Vec<Expr> = inputs
        .iter()
        .map(|input| translate_meta_arg(input, self_ty))
        .collect();

    let num_args = inputs.len() as i32;

    output_r::output_r_wrapper(
        func_name,
        &raw_wrap_name,
        sig.inputs.clone().into_iter().collect(),
        &sig.output,
    );

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
    wrappers.push(parse_quote!(
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            unsafe {
                use extendr_api::FromRobj;
                #( #convert_args )*
                extendr_api::handle_panic(#panic_str, ||
                    extendr_api::Robj::from(#call_name(#actual_args)).get()
                )
            }
        }
    ));

    // Generate init functions which gather metadata about functions and methods.
    //
    // Example:
    // #[allow(non_snake_case)]
    // fn init__hello(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
    //     call_methods.push(extendr_api::CallMethod {
    //         call_symbol: std::ffi::CString::new("wrap__hello").unwrap(),
    //         func_ptr: wrap__hello as *const u8,
    //         num_args: 0i32,
    //     })
    // }
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

    wrappers.push(parse_quote!(
        #[allow(non_snake_case)]
        fn #meta_name(metadata: &mut Vec<extendr_api::metadata::Func>) {
            let mut args = vec![
                #( #meta_args, )*
            ];
            metadata.push(extendr_api::metadata::Func {
                doc: #doc_string,
                name: #name_str,
                args: args,
                return_type: #return_type_string,
                func_ptr: #wrap_name as * const u8,
            })
        }
    ));
}

/// Handle trait implementations.
///
/// Example:
/// ```ignore
/// use extendr_api::*;
/// #[derive(Debug)]
/// struct Person {
///     pub name: String,
/// }
/// #[extendr]
/// impl Person {
///     fn new() -> Self {
///         Self { name: "".to_string() }
///     }
///     fn set_name(&mut self, name: &str) {
///         self.name = name.to_string();
///     }
///     fn name(&self) -> &str {
///         self.name.as_str()
///     }
/// }
/// #[extendr]
/// fn aux_func() {
/// }
/// // Macro to generate exports
/// extendr_module! {
///     mod classes;
///     impl Person;
///     fn aux_func;
/// }
/// ```
fn extendr_impl(mut item_impl: ItemImpl) -> TokenStream {
    let opts = ExtendrOptions {};
    let self_ty = item_impl.self_ty.as_ref();
    let self_ty_name = quote! {#self_ty}.to_string();
    let prefix = format!("{}__", self_ty_name);
    let mut method_init_names = Vec::new();
    let mut method_meta_names = Vec::new();
    let mut wrappers = Vec::new();

    // Generate wrappers for methods.
    // eg.
    // ```
    // #[no_mangle]
    // #[allow(non_snake_case)]
    // pub extern "C" fn wrap__Person__new() -> extendr_api::SEXP {
    //     unsafe {
    //         use extendr_api::FromRobj;
    //         extendr_api::Robj::from(<Person>::new()).get()
    //     }
    // }
    // ```
    for impl_item in &mut item_impl.items {
        if let syn::ImplItem::Method(ref mut method) = impl_item {
            method_init_names.push(format_ident!("{}{}__{}", INIT_PREFIX, self_ty_name, method.sig.ident));
            method_meta_names.push(format_ident!("{}{}__{}", META_PREFIX, self_ty_name, method.sig.ident));
            generate_wrappers(&opts, &mut wrappers, prefix.as_str(), &method.sig, Some(self_ty));
        }
    }

    let init_name = format_ident!("{}{}", INIT_PREFIX, self_ty_name);
    let meta_name = format_ident!("{}{}", META_PREFIX, self_ty_name);

    let finalizer_name = format_ident!("__finalize__{}", self_ty_name);

    let expanded = TokenStream::from(quote! {
        // The impl itself copied from the source.
        #item_impl

        // Function wrappers
        #( #wrappers )*

        // Input conversion function for this type.
        impl<'a> extendr_api::FromRobj<'a> for &#self_ty {
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if robj.check_external_ptr(#self_ty_name) {
                    Ok(unsafe { std::mem::transmute(robj.externalPtrAddr::<#self_ty>()) })
                } else {
                    Err(concat!("expected ", #self_ty_name))
                }
            }
        }

        // Input conversion function for a reference to this type.
        impl<'a> extendr_api::FromRobj<'a> for &mut #self_ty {
            fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                if robj.check_external_ptr(#self_ty_name) {
                    Ok(unsafe { std::mem::transmute(robj.externalPtrAddr::<#self_ty>()) })
                } else {
                    Err(concat!("expected ", #self_ty_name))
                }
            }
        }

        // Output conversion function for this type.
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

        // Function to free memory for this type.
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

        // Init function to call the method inits for this type.
        // Example:
        // ```ignore
        // #[allow(non_snake_case)]
        // fn init__Person(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
        //     init__Person__new(info, call_methods);
        //     init__Person__set_name(info, call_methods);
        //     init__Person__name(info, call_methods);
        // }
        // ```
        #[allow(non_snake_case)]
        fn #init_name(info: *mut extendr_api::DllInfo, call_methods: &mut Vec<extendr_api::CallMethod>) {
            #( #method_init_names(info, call_methods); )*
        }

        #[allow(non_snake_case)]
        fn #meta_name(structs: &mut Vec<extendr_api::metadata::Struct>) {
            let mut methods = Vec::new();
            #( #method_meta_names(&mut methods); )*
            structs.push(extendr_api::metadata::Struct {
                doc: "",
                name: #self_ty_name,
                methods,
            });
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

// This structure contains parameters parsed from the #[extendr_module] definition.
#[derive(Debug)]
struct Module {
    modname: Option<Ident>,
    fnnames: Vec<Ident>,
    implnames: Vec<Ident>,
}

// Custom parser for the module.
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
///```ignore
/// extendr_module! {
///     mod name;
///     fn my_func1;
///     fn my_func2;
///     impl MyTrait;
/// }
/// ```
/// Outputs:
///
/// ```ignore
/// #[no_mangle]
/// #[allow(non_snake_case)]
/// pub extern "C" fn R_init_hello_extendr(info: *mut extendr_api::DllInfo) {
///     let mut call_methods = Vec::new();
///     init__hello(info, &mut call_methods);
///     unsafe { extendr_api::register_call_methods(info, call_methods.as_ref()) };
/// }
/// ```
#[proc_macro]
pub fn extendr_module(item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as Module);
    let Module {modname, fnnames, implnames} = module;
    let modname = modname.unwrap();
    let module_init_name = format_ident!("R_init_{}_extendr", modname);
    let module_metadata_name = format_ident!("get_{}_metadata", modname);
    let wrap_module_metadata_name = format_ident!("{}get_{}_metadata", WRAP_PREFIX, modname);

    let fninitnames = fnnames.iter().map(|id| format_ident!("{}{}", INIT_PREFIX, id));
    let implinitnames = implnames.iter().map(|id| format_ident!("{}{}", INIT_PREFIX, id));
    let fnmetanames = fnnames.iter().map(|id| format_ident!("{}{}", META_PREFIX, id));
    let implmetanames = implnames.iter().map(|id| format_ident!("{}{}", META_PREFIX, id));

    TokenStream::from(quote!{
        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn #module_metadata_name() -> extendr_api::metadata::Metadata {
            let mut functions = Vec::new();
            let mut structs = Vec::new();
            #( #fnmetanames(&mut functions); )*
            #( #implmetanames(&mut structs); )*
            extendr_api::metadata::Metadata {
                doc: "",
                functions,
                structs,
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_module_metadata_name() -> SEXP {
            unsafe { Robj::from(#module_metadata_name()).get() }
        }

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
