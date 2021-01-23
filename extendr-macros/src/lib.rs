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
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{
    parse::ParseStream, parse_macro_input, parse_quote, Expr, FnArg, Ident, Item, ItemFn, ItemImpl,
    Type,
};

mod output_r;

const META_PREFIX: &str = "meta__";
const WRAP_PREFIX: &str = "wrap__";

#[derive(Debug)]
struct ExtendrOptions {}

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

// Generate code to make a metadata::Arg.
fn translate_meta_arg(input: &FnArg, self_ty: Option<&syn::Type>) -> Expr {
    match input {
        // function argument.
        FnArg::Typed(ref pattype) => {
            let pat = pattype.pat.as_ref();
            let ty = pattype.ty.as_ref();
            let name_string = quote! { #pat }.to_string();
            let type_string = type_name(ty);
            return parse_quote! {
                extendr_api::metadata::Arg {
                    name: #name_string,
                    arg_type: #type_string
                }
            };
        }
        // &self
        FnArg::Receiver(ref reciever) => {
            if !reciever.attrs.is_empty() || reciever.reference.is_none() {
                panic!("expected &self or &mut self");
            }
            if self_ty.is_none() {
                panic!("found &self in non-impl function - have you missed the #[extendr] before the impl?");
            }
            let type_string = type_name(self_ty.unwrap());
            return parse_quote! {
                extendr_api::metadata::Arg {
                    name: "self",
                    arg_type: #type_string
                }
            };
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
                Some(parse_quote! { extendr_api::unwrap_or_throw(<#ty>::from_robj(&#varname)) })
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
    make_function_wrappers(&opts, &mut wrappers, "", &func.attrs, &func.sig, None);

    TokenStream::from(quote! {
        #func

        # ( #wrappers )*
    })
}

// Extract doc strings from attributes.
fn get_doc_string(attrs: &Vec<syn::Attribute>) -> String {
    let mut res = String::new();
    for attr in attrs {
        if let Some(id) = attr.path.get_ident() {
            if id.to_string() == "doc" {
                if let Ok(meta) = attr.parse_meta() {
                    if let syn::Meta::NameValue(nv) = meta {
                        if let syn::Lit::Str(litstr) = nv.lit {
                            if !res.is_empty() {
                                res.extend("\n".chars());
                            }
                            res.extend(litstr.value().chars());
                        }
                    }
                }
            }
        }
    }
    res
}

fn get_return_type(sig: &syn::Signature) -> String {
    match &sig.output {
        syn::ReturnType::Default => "()".into(),
        syn::ReturnType::Type(_, ref rettype) => type_name(rettype),
    }
}

fn mangled_type_name(type_: &Type) -> String {
    let src = quote!( #type_ ).to_string();
    let mut res = String::new();
    for c in src.chars() {
        if c != ' ' {
            if c.is_alphanumeric() {
                res.push(c)
            } else {
                let f = format!("_{:02x}", c as u32);
                res.extend(f.chars());
            }
        }
    }
    res
}

// Return a simplified type name that will be meaningful to R. Defaults to a digest.
// For example:
// & Fred -> Fred
// * Fred -> Fred
// && Fred -> Fred
// Fred<'a> -> Fred
// &[i32] -> _hex_hex_hex_hex
//
fn type_name(type_: &Type) -> String {
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

// Generate wrappers for a specific function.
fn make_function_wrappers(
    _opts: &ExtendrOptions,
    wrappers: &mut Vec<ItemFn>,
    prefix: &str,
    attrs: &Vec<syn::Attribute>,
    sig: &syn::Signature,
    self_ty: Option<&syn::Type>,
) {
    let func_name = &sig.ident;

    let raw_wrap_name = format!("{}{}{}", WRAP_PREFIX, prefix, func_name);
    let wrap_name = format_ident!("{}{}{}", WRAP_PREFIX, prefix, func_name);
    let meta_name = format_ident!("{}{}{}", META_PREFIX, prefix, func_name);

    let name_str = format!("{}", func_name);
    let doc_string = get_doc_string(attrs);
    let return_type_string = get_return_type(&sig);

    let panic_str = format!("{} paniced.\0", func_name);

    let inputs = &sig.inputs;
    let has_self = match inputs.iter().next() {
        Some(FnArg::Receiver(_)) => true,
        _ => false,
    };

    let call_name = if has_self {
        let is_mut = match inputs.iter().next() {
            Some(FnArg::Receiver(ref reciever)) => reciever.mutability.is_some(),
            _ => false,
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

    // Generate a function to push the metadata for a function.
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
                hidden: false,
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
    // Only `impl name { }` allowed
    if item_impl.defaultness.is_some() {
        return quote! { compile_error!("default not allowed in #[extendr] impl"); }.into();
    }

    if item_impl.unsafety.is_some() {
        return quote! { compile_error!("unsafe not allowed in #[extendr] impl"); }.into();
    }

    if item_impl.generics.const_params().count() != 0 {
        return quote! { compile_error!("const params not allowed in #[extendr] impl"); }.into();
    }

    if item_impl.generics.type_params().count() != 0 {
        return quote! { compile_error!("type params not allowed in #[extendr] impl"); }.into();
    }

    // if item_impl.generics.lifetimes().count() != 0 {
    //     return quote! { compile_error!("lifetime params not allowed in #[extendr] impl"); }.into();
    // }

    if item_impl.generics.where_clause.is_some() {
        return quote! { compile_error!("where clause not allowed in #[extendr] impl"); }.into();
    }

    let opts = ExtendrOptions {};
    let self_ty = item_impl.self_ty.as_ref();
    let self_ty_name = type_name(&self_ty);
    let prefix = format!("{}__", self_ty_name);
    let mut method_meta_names = Vec::new();
    let doc_string = get_doc_string(&item_impl.attrs);

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
    let mut wrappers: Vec<ItemFn> = Vec::new();
    for impl_item in &mut item_impl.items {
        if let syn::ImplItem::Method(ref mut method) = impl_item {
            method_meta_names.push(format_ident!(
                "{}{}__{}",
                META_PREFIX,
                self_ty_name,
                method.sig.ident
            ));
            make_function_wrappers(
                &opts,
                &mut wrappers,
                prefix.as_str(),
                &method.attrs,
                &method.sig,
                Some(self_ty),
            );
        }
    }

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
                    res.set_attrib(class_symbol(), #self_ty_name);
                    res.registerCFinalizer(Some(#finalizer_name));
                    res
                }
            }
        }

        // Output conversion function for this type.
        impl<'a> From<&'a #self_ty> for Robj {
            fn from(value: &'a #self_ty) -> Self {
                unsafe {
                    let ptr = Box::into_raw(Box::new(value));
                    let res = Robj::makeExternalPtr(ptr, Robj::from(#self_ty_name), Robj::from(()));
                    res.set_attrib(class_symbol(), #self_ty_name);
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

        #[allow(non_snake_case)]
        fn #meta_name(impls: &mut Vec<extendr_api::metadata::Impl>) {
            let mut methods = Vec::new();
            #( #method_meta_names(&mut methods); )*
            impls.push(extendr_api::metadata::Impl {
                doc: #doc_string,
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
        other_item => TokenStream::from(quote! {#other_item}),
    }
}

// This structure contains parameters parsed from the #[extendr_module] definition.
#[derive(Debug)]
struct Module {
    modname: Option<Ident>,
    fnnames: Vec<Ident>,
    implnames: Vec<Type>,
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
                let name: Ident = input.parse()?;
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
    let Module {
        modname,
        fnnames,
        implnames,
    } = module;
    let modname = modname.unwrap();
    let modname_string = modname.to_string();
    let module_init_name = format_ident!("R_init_{}_extendr", modname);

    let module_metadata_name = format_ident!("get_{}_metadata", modname);
    let module_metadata_name_string = module_metadata_name.to_string();
    let wrap_module_metadata_name = format_ident!("{}get_{}_metadata", WRAP_PREFIX, modname);

    let make_module_wrappers_name = format_ident!("make_{}_wrappers", modname);
    let make_module_wrappers_name_string = make_module_wrappers_name.to_string();
    let wrap_make_module_wrappers = format_ident!("{}make_{}_wrappers", WRAP_PREFIX, modname);

    let fnmetanames = fnnames
        .iter()
        .map(|id| format_ident!("{}{}", META_PREFIX, id));
    let implmetanames = implnames
        .iter()
        .map(|id| format_ident!("{}{}", META_PREFIX, type_name(id)));

    TokenStream::from(quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn #module_metadata_name() -> extendr_api::metadata::Metadata {
            let mut functions = Vec::new();
            let mut impls = Vec::new();
            #( #fnmetanames(&mut functions); )*
            #( #implmetanames(&mut impls); )*

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Metadata access function.",
                name: #module_metadata_name_string,
                args: Vec::new(),
                return_type: "Metadata",
                func_ptr: #wrap_module_metadata_name as * const u8,
                hidden: true,
            });

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Wrapper generator.",
                name: #make_module_wrappers_name_string,
                args: vec![extendr_api::metadata::Arg { name: "use_symbols", arg_type: "bool" }],
                return_type: "String",
                func_ptr: #wrap_make_module_wrappers as * const u8,
                hidden: true,
            });

            extendr_api::metadata::Metadata {
                name: #modname_string,
                functions,
                impls,
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_module_metadata_name() -> SEXP {
            unsafe { extendr_api::Robj::from(#module_metadata_name()).get() }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_make_module_wrappers(use_symbols_sexp: SEXP) -> SEXP {
            unsafe {
                let robj = new_borrowed(use_symbols_sexp);
                let use_symbols : bool = <bool>::from_robj(&robj).unwrap();

                extendr_api::Robj::from(#module_metadata_name().make_r_wrappers(use_symbols).unwrap()).get()
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #module_init_name(info: * mut extendr_api::DllInfo) {
            unsafe { extendr_api::register_call_methods(info, #module_metadata_name()) };
        }
    })
}
