use crate::utils::type_name;
use crate::wrappers;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, Type,
};

pub fn extendr_module(item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as Module);
    let Module {
        modname,
        fnnames,
        implnames,
        usenames,
    } = module;
    let modname_string = modname.to_string();
    let module_init_name = format_ident!("R_init_{}_extendr", modname);

    let module_metadata_name = format_ident!("get_{}_metadata", modname);
    let module_metadata_name_string = module_metadata_name.to_string();
    let wrap_module_metadata_name =
        format_ident!("{}get_{}_metadata", wrappers::WRAP_PREFIX, modname);
    let wrap_module_metadata_name_string = wrap_module_metadata_name.to_string();

    let make_module_wrappers_name = format_ident!("make_{}_wrappers", modname);
    let make_module_wrappers_name_string = make_module_wrappers_name.to_string();
    let wrap_make_module_wrappers =
        format_ident!("{}make_{}_wrappers", wrappers::WRAP_PREFIX, modname);
    let wrap_make_module_wrappers_string = wrap_make_module_wrappers.to_string();

    let fnmetanames = fnnames
        .iter()
        .map(|id| format_ident!("{}{}", wrappers::META_PREFIX, id));
    let implmetanames = implnames
        .iter()
        .map(|id| format_ident!("{}{}", wrappers::META_PREFIX, type_name(id)));
    let usemetanames = usenames
        .iter()
        .map(|id| format_ident!("get_{}_metadata", id))
        .collect::<Vec<Ident>>();

    TokenStream::from(quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn #module_metadata_name() -> extendr_api::metadata::Metadata {
            let mut functions = Vec::new();
            let mut impls = Vec::new();

            // Pushes metadata (eg. extendr_api::metadata::Func) to functions and impl vectors.
            #( #fnmetanames(&mut functions); )*
            #( #implmetanames(&mut impls); )*

            // Extends functions and impls with the submodules metadata
            #( functions.extend(#usenames::#usemetanames().functions); )*
            #( impls.extend(#usenames::#usemetanames().impls); )*

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Metadata access function.",
                rust_name: #module_metadata_name_string,
                mod_name: #module_metadata_name_string,
                c_name: #wrap_module_metadata_name_string,
                r_name: #module_metadata_name_string,
                args: Vec::new(),
                return_type: "Metadata",
                func_ptr: #wrap_module_metadata_name as * const u8,
                hidden: true,
                invisible: None,
            });
            let mut args = Vec::with_capacity(2usize);
            args.push(extendr_api::metadata::Arg { name: "use_symbols", arg_type: "bool", default: None });
            args.push(extendr_api::metadata::Arg { name: "package_name", arg_type: "&str", default: None });
            let args = args;

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Wrapper generator.",
                rust_name: #make_module_wrappers_name_string,
                mod_name: #make_module_wrappers_name_string,
                c_name: #wrap_make_module_wrappers_string,
                r_name: #make_module_wrappers_name_string,
                args,
                return_type: "String",
                func_ptr: #wrap_make_module_wrappers as * const u8,
                hidden: true,
                invisible: None,
            });

            extendr_api::metadata::Metadata {
                name: #modname_string,
                functions,
                impls,
            }
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #wrap_module_metadata_name() -> extendr_api::SEXP {
            use extendr_api::GetSexp;
            unsafe { extendr_api::Robj::from(#module_metadata_name()).get() }
        }

        #[no_mangle]
        #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn #wrap_make_module_wrappers(
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
                    #module_metadata_name()
                        .make_r_wrappers(
                            use_symbols,
                            package_name,
                        ).unwrap()
                ).get()
            }
        }

        #[no_mangle]
        #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn #module_init_name(info: * mut extendr_api::DllInfo) {
            unsafe { extendr_api::register_call_methods(info, #module_metadata_name()) };
        }
    })
}

#[derive(Debug)]
struct Module {
    modname: Ident,
    fnnames: Vec<Ident>,
    implnames: Vec<Type>,
    usenames: Vec<Ident>,
}

#[derive(Debug)]
enum ModuleItem {
    Mod(Ident),
    Fn(Ident),
    Impl(Type),
    Use(Ident),
}

// Custom parser for the module.
impl Parse for Module {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items: Punctuated<ModuleItem, Token![;]> = Punctuated::parse_terminated(input)?;

        let mut modname: Option<Ident> = None;
        let mut fnnames = Vec::new();
        let mut implnames = Vec::new();
        let mut usenames = Vec::new();

        for item in items {
            match item {
                ModuleItem::Mod(name) => {
                    if modname.replace(name).is_some() {
                        return Err(syn::Error::new(input.span(), "only one mod allowed"));
                    }
                }
                ModuleItem::Fn(name) => fnnames.push(name),
                ModuleItem::Impl(ty) => implnames.push(ty),
                ModuleItem::Use(name) => usenames.push(name),
            }
        }

        let modname =
            modname.ok_or_else(|| syn::Error::new(input.span(), "expected one 'mod name'"))?;

        Ok(Self {
            modname,
            fnnames,
            implnames,
            usenames,
        })
    }
}

impl Parse for ModuleItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mod]) {
            input.parse::<Token![mod]>()?;
            Ok(Self::Mod(input.parse()?))
        } else if input.peek(Token![fn]) {
            input.parse::<Token![fn]>()?;
            Ok(Self::Fn(input.parse()?))
        } else if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            Ok(Self::Impl(input.parse()?))
        } else if input.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            Ok(Self::Use(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected mod, fn or impl"))
        }
    }
}
