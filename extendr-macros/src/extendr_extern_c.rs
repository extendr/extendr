use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, spanned::Spanned, Expr, FnArg, ItemFn};

use crate::{
    extendr_options::ExtendrOptions,
    wrappers::{
        get_doc_string, get_named_lit, get_return_type, sanitize_identifier, translate_only_alias,
        type_name, META_PREFIX,
    },
};

/// Returns whether or not a given function is an `extern "C" fn`-item.
/// 
/// Somewhat related to [``]
pub(crate) fn is_extern_c(extern_c_func: &ItemFn) -> bool {
    let abi = &extern_c_func.sig.abi;
    if abi.is_none() {
        return false;
    }
    let abi = abi.as_ref().unwrap();
    //TODO: warn user about missing `"C"` in `extern "C"`
    if let Some(name) = abi.name.as_ref() {
        assert_eq!(&name.value(), "C");
    }
    return true;
}

pub(crate) fn extendr_extern_c(
    mut extern_c_func: ItemFn,
    opts: &ExtendrOptions,
) -> syn::Result<TokenStream> {
    // Generate a function to push the metadata for a function.
    let sig = &mut extern_c_func.sig;
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
    let prefix = "";
    let meta_name = format_ident!("{}{}{}", META_PREFIX, prefix, mod_name);
    let inputs = &mut sig.inputs;
    let meta_args: Vec<Expr> = inputs
        .iter_mut()
        .map(|input| translate_meta_arg(input))
        .collect::<syn::Result<Vec<Expr>>>()?;

    let rust_name = sig.ident.clone();
    let rust_name_str = format!("{}", rust_name);
    let c_name_str = format!("{}", mod_name);
    let doc_string = get_doc_string(&extern_c_func.attrs);
    let return_type_string = get_return_type(&sig);

    //TODO: make a check that the return-type is `SEXP` and all arguments are
    // also `SEXP`. 

    let extern_c_wrapper: ItemFn = parse_quote!(
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
                // extern "C" fn are directly callable.
                func_ptr: #rust_name as *const u8,
                hidden: false,
            })
        }
    );

    // TODO: add #[no_mangle] if not there
    Ok(TokenStream::from(quote! {
        #[no_mangle]
        #extern_c_func

        #extern_c_wrapper
    }))
}

// Generate code to make a metadata::Arg.
pub fn translate_meta_arg(input: &mut FnArg) -> syn::Result<Expr> {
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
            // pop the `default` attribute on parameters
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
        _ => Err(syn::Error::new(input.span(), "unsupported")),
    }
}
