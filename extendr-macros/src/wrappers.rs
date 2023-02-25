use quote::{format_ident, quote};
use syn::{parse_quote, punctuated::Punctuated, Expr, FnArg, ItemFn, Token, Type};

pub const META_PREFIX: &str = "meta__";
pub const WRAP_PREFIX: &str = "wrap__";

#[derive(Debug, Default)]
pub struct ExtendrOptions {
    pub use_try_from: bool,
    pub r_name: Option<String>,
    pub mod_name: Option<String>,
}

// Generate wrappers for a specific function.
pub fn make_function_wrappers(
    opts: &ExtendrOptions,
    wrappers: &mut Vec<ItemFn>,
    prefix: &str,
    attrs: &[syn::Attribute],
    sig: &mut syn::Signature,
    self_ty: Option<&syn::Type>,
) {
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

    let wrap_name = format_ident!("{}{}{}", WRAP_PREFIX, prefix, mod_name);
    let meta_name = format_ident!("{}{}{}", META_PREFIX, prefix, mod_name);

    let rust_name_str = format!("{}", rust_name);
    let c_name_str = format!("{}", mod_name);
    let doc_string = get_doc_string(attrs);
    let return_type_string = get_return_type(sig);

    let panic_str = format!("{} panicked.\0", r_name_str);

    let inputs = &mut sig.inputs;
    let has_self = matches!(inputs.iter().next(), Some(FnArg::Receiver(_)));

    let call_name = if has_self {
        let is_mut = match inputs.iter().next() {
            Some(FnArg::Receiver(ref reciever)) => reciever.mutability.is_some(),
            _ => false,
        };
        if is_mut {
            // eg. Person::name(&mut self)
            quote! { extendr_api::unwrap_or_throw(
                <&mut #self_ty>::from_robj(&_self_robj)
            ).#rust_name }
        } else {
            // eg. Person::name(&self)
            quote! { extendr_api::unwrap_or_throw(
                <&#self_ty>::from_robj(&_self_robj)
            ).#rust_name }
        }
    } else if let Some(ref self_ty) = &self_ty {
        // eg. Person::new()
        quote! { <#self_ty>::#rust_name }
    } else {
        // eg. aux_func()
        quote! { #rust_name }
    };

    let formal_args: Punctuated<FnArg, Token![,]> = inputs
        .iter()
        .map(|input| translate_formal(input, self_ty))
        .collect();

    let convert_args: Vec<syn::Stmt> = inputs.iter().map(translate_to_robj).collect();

    let actual_args: Punctuated<Expr, Token![,]> = inputs
        .iter()
        .filter_map(|input| translate_actual(opts, input))
        .collect();

    let meta_args: Vec<Expr> = inputs
        .iter_mut()
        .map(|input| translate_meta_arg(input, self_ty))
        .collect();

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
        #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            unsafe {
                use extendr_api::robj::*;
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
}

// Extract doc strings from attributes.
pub fn get_doc_string(attrs: &[syn::Attribute]) -> String {
    let mut res = String::new();
    for attr in attrs {
        if let Some(id) = attr.path.get_ident() {
            if *id != "doc" {
                continue;
            }

            if let Ok(syn::Meta::NameValue(nv)) = attr.parse_meta() {
                if let syn::Lit::Str(litstr) = nv.lit {
                    if !res.is_empty() {
                        res.push('\n');
                    }
                    res.push_str(&litstr.value());
                }
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

// Return a simplified type name that will be meaningful to R. Defaults to a digest.
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
pub fn translate_formal(input: &FnArg, self_ty: Option<&syn::Type>) -> FnArg {
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
fn translate_meta_arg(input: &mut FnArg, self_ty: Option<&syn::Type>) -> Expr {
    match input {
        // function argument.
        FnArg::Typed(ref mut pattype) => {
            let pat = pattype.pat.as_ref();
            let ty = pattype.ty.as_ref();
            let name_string = quote! { #pat }.to_string();
            let type_string = type_name(ty);
            let default = if let Some(default) = get_named_lit(&mut pattype.attrs, "default") {
                quote!(Some(#default))
            } else {
                quote!(None)
            };
            return parse_quote! {
                extendr_api::metadata::Arg {
                    name: #name_string,
                    arg_type: #type_string,
                    default: #default
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
                    arg_type: #type_string,
                    default: None
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
                parse_quote! { let #varname = extendr_api::new_owned(#pat); }
            } else {
                panic!("expect identifier as arg name")
            }
        }
        FnArg::Receiver(_) => {
            parse_quote! { let mut _self_robj = extendr_api::new_owned(_self); }
        }
    }
}

// Generate actual argument list for the call (ie. a list of conversions).
fn translate_actual(opts: &ExtendrOptions, input: &FnArg) -> Option<Expr> {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            let ty = &pattype.ty.as_ref();
            if let syn::Pat::Ident(ref ident) = pat {
                let varname = format_ident!("_{}_robj", ident.ident);
                if opts.use_try_from {
                    Some(parse_quote! { extendr_api::unwrap_or_throw_error(
                        #varname.try_into()
                        .map_err(|e| extendr_api::Error::from(e)))
                    })
                } else {
                    Some(parse_quote! { extendr_api::unwrap_or_throw(<#ty>::from_robj(&#varname)) })
                }
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
    'f: for a in attrs.drain(0..) {
        if let Ok(syn::Meta::NameValue(nv)) = a.parse_meta() {
            if nv.path.is_ident(name) {
                if let syn::Lit::Str(litstr) = nv.lit {
                    res = Some(litstr.value());
                    continue 'f;
                }
            }
        }
        new_attrs.push(a);
    }
    *attrs = new_attrs;
    res
}
