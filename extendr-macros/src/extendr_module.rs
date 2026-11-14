use crate::wrappers;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::ParseStream, parse_macro_input, Attribute, Ident, Token, Type};

pub fn extendr_module(item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as Module);
    let Module {
        modname,
        fnnames,
        implnames,
        usenames,
    } = module;
    let modname = modname.expect("cannot include unnamed modules");
    let modname_string = modname.to_string();
    let module_init_name = format_ident!("R_init_{}_extendr", modname);

    let module_metadata_name = format_ident!("get_{}_metadata", modname);
    let module_metadata_name_string = module_metadata_name.to_string();
    let wrap_module_metadata_name =
        format_ident!("{}get_{}_metadata", wrappers::WRAP_PREFIX, modname);
    let wrap_module_metadata_name_str = wrap_module_metadata_name.to_string();

    let make_module_wrappers_name = format_ident!("make_{}_wrappers", modname);
    let make_module_wrappers_name_string = make_module_wrappers_name.to_string();
    let wrap_make_module_wrappers =
        format_ident!("{}make_{}_wrappers", wrappers::WRAP_PREFIX, modname);
    let wrap_make_module_wrappers_string = wrap_make_module_wrappers.to_string();
    let write_make_module_wrappers = format_ident!("write__make_{}_wrappers", modname);

    // Each statement is prefixed with its item's `#[cfg(...)]` (and other)
    // attributes, so that `cfg`-gating an item in `extendr_module!` compiles out
    // both the item and the metadata call that references it.
    let fnmeta_stmts = fnnames.iter().map(|(attrs, id)| {
        let meta = format_ident!("{}{}", wrappers::META_PREFIX, id);
        quote! { #(#attrs)* #meta(&mut functions); }
    });
    let implmeta_stmts = implnames.iter().map(|(attrs, ty)| {
        let meta = format_ident!("{}{}", wrappers::META_PREFIX, wrappers::type_name(ty));
        quote! { #(#attrs)* #meta(&mut impls); }
    });
    let use_fn_stmts = usenames.iter().map(|(attrs, id)| {
        let meta = format_ident!("get_{}_metadata", id);
        quote! { #(#attrs)* functions.extend(#id::#meta().functions); }
    });
    let use_impl_stmts = usenames.iter().map(|(attrs, id)| {
        let meta = format_ident!("get_{}_metadata", id);
        quote! { #(#attrs)* impls.extend(#id::#meta().impls); }
    });

    TokenStream::from(quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn #module_metadata_name() -> extendr_api::metadata::Metadata {
            let mut functions = Vec::new();
            let mut impls = Vec::new();

            // Pushes metadata (eg. extendr_api::metadata::Func) to functions and impl vectors.
            #( #fnmeta_stmts )*
            #( #implmeta_stmts )*

            // Extends functions and impls with the submodules metadata
            #( #use_fn_stmts )*
            #( #use_impl_stmts )*

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Metadata access function.",
                rust_name: #module_metadata_name_string,
                mod_name: #module_metadata_name_string,
                r_name: #module_metadata_name_string,
                c_name: #wrap_module_metadata_name_str,
                args: Vec::new(),
                return_type: "Metadata",
                func_ptr: #wrap_module_metadata_name as * const u8,
                hidden: true,
                invisible: None,
            });
            let mut args = vec![
                extendr_api::metadata::Arg { name: "use_symbols", arg_type: "bool", default: None },
                extendr_api::metadata::Arg { name: "package_name", arg_type: "&str", default: None }
            ];
            let args = args;

            // Add this function to the list, but set hidden: true.
            functions.push(extendr_api::metadata::Func {
                doc: "Wrapper generator.",
                rust_name: #make_module_wrappers_name_string,
                mod_name: #make_module_wrappers_name_string,
                r_name: #make_module_wrappers_name_string,
                c_name: #wrap_make_module_wrappers_string,
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
        pub extern "C" fn #write_make_module_wrappers(
            package_name: *const std::os::raw::c_char,
            out_path: *const std::os::raw::c_char,
        ) -> i32 {
            let pkg = match unsafe { std::ffi::CStr::from_ptr(package_name) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("extendr: package_name is not valid UTF-8: {}", e);
                    return 2;
                }
            };
            let path = match unsafe { std::ffi::CStr::from_ptr(out_path) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("extendr: out_path is not valid UTF-8: {}", e);
                    return 2;
                }
            };
            match #module_metadata_name().write_r_wrappers(pkg, path) {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("extendr: writing wrappers for '{pkg}' failed: {e}");
                    1
                }
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
    modname: Option<Ident>,
    fnnames: Vec<(Vec<Attribute>, Ident)>,
    implnames: Vec<(Vec<Attribute>, Type)>,
    usenames: Vec<(Vec<Attribute>, Ident)>,
}

// Custom parser for the module.
impl syn::parse::Parse for Module {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::spanned::Spanned;
        let mut res = Self {
            modname: None,
            fnnames: Vec::new(),
            implnames: Vec::new(),
            usenames: Vec::new(),
        };
        while !input.is_empty() {
            // Outer attributes (e.g. `#[cfg(...)]`) preceding an item are
            // captured and forwarded to the generated metadata call, so that
            // `cfg`-gating an item here matches the item's own compilation.
            let attrs = input.call(Attribute::parse_outer)?;
            if let Ok(kmod) = input.parse::<Token![mod]>() {
                if !attrs.is_empty() {
                    return Err(syn::Error::new(kmod.span(), "attributes are not supported on `mod`"));
                }
                let name: Ident = input.parse()?;
                if res.modname.is_some() {
                    return Err(syn::Error::new(kmod.span(), "only one mod allowed"));
                }
                res.modname = Some(name);
            } else if input.parse::<Token![fn]>().is_ok() {
                res.fnnames.push((attrs, input.parse()?));
            } else if input.parse::<Token![impl]>().is_ok() {
                res.implnames.push((attrs, input.parse()?));
            } else if input.parse::<Token![use]>().is_ok() {
                res.usenames.push((attrs, input.parse()?));
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
