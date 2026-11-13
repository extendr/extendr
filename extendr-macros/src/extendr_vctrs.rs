use crate::{extendr_options::ExtendrOptions, wrappers::META_PREFIX};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Generics, Ident, Item};

/// Generate the conversions and S3 method wrappers for `#[extendr(vctrs)]`.
pub(crate) fn extendr_vctrs(
    item: &Item,
    ident: &Ident,
    generics: &Generics,
    opts: &ExtendrOptions,
) -> syn::Result<TokenStream> {
    if !generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            generics,
            "#[extendr(vctrs)] does not support generic types",
        ));
    }

    let name = ident.to_string();
    let class = opts.r_name.clone().unwrap_or_else(|| name.clone());

    let format_fn = format_ident!("__extendr_vctr_format__{}", ident);
    let restore_fn = format_ident!("__extendr_vctr_restore__{}", ident);
    let c_fn = format_ident!("__extendr_vctr_c__{}", ident);

    let format_mod = format!("vctr_format__{name}");
    let restore_mod = format!("vctr_restore__{name}");
    let c_mod = format!("vctr_c__{name}");

    let format_meta = format_ident!("{META_PREFIX}{format_mod}");
    let restore_meta = format_ident!("{META_PREFIX}{restore_mod}");
    let c_meta = format_ident!("{META_PREFIX}{c_mod}");
    let vctr_meta = format_ident!("{META_PREFIX}vctr__{}", ident);

    let format_r = format!("format.{class}");
    let restore_r = format!("vec_restore.{class}");
    let c_r = format!("c.{class}");

    Ok(quote! {
        #item

        impl<'a> TryFrom<&'a extendr_api::Robj> for &'a #ident {
            type Error = extendr_api::Error;
            fn try_from(robj: &'a extendr_api::Robj) -> extendr_api::Result<Self> {
                extendr_api::optional::vctrs::try_vctr_ref(robj)
            }
        }

        impl TryFrom<&extendr_api::Robj> for #ident {
            type Error = extendr_api::Error;
            fn try_from(robj: &extendr_api::Robj) -> extendr_api::Result<Self> {
                extendr_api::optional::vctrs::try_vctr_ref::<#ident>(robj).cloned()
            }
        }

        impl From<#ident> for extendr_api::Robj {
            fn from(value: #ident) -> Self {
                extendr_api::optional::vctrs::into_vctr(value, #class)
            }
        }

        #[allow(non_snake_case, dead_code)]
        #[extendr_api::extendr(r_name = #format_r, mod_name = #format_mod)]
        fn #format_fn(x: &#ident) -> extendr_api::Strings {
            <#ident as extendr_api::optional::vctrs::Rvctr>::format(x)
        }

        #[allow(non_snake_case, dead_code)]
        #[extendr_api::extendr(r_name = #restore_r, mod_name = #restore_mod)]
        fn #restore_fn(x: extendr_api::Integers, to: &#ident) -> #ident {
            extendr_api::optional::vctrs::restore(&x, to)
        }

        #[allow(non_snake_case, dead_code)]
        #[extendr_api::extendr(r_name = #c_r, mod_name = #c_mod)]
        fn #c_fn(xs: extendr_api::List) -> extendr_api::Robj {
            match extendr_api::optional::vctrs::concat::<#ident>(&xs) {
                Ok(Some(x)) => x.into(),
                Ok(None) => extendr_api::Robj::from(()),
                Err(e) => panic!("{}", e),
            }
        }

        #[allow(non_snake_case, dead_code)]
        fn #vctr_meta(vctrs: &mut Vec<extendr_api::metadata::Vctr>) {
            let mut methods = Vec::new();
            #format_meta(&mut methods);
            #restore_meta(&mut methods);
            #c_meta(&mut methods);
            vctrs.push(extendr_api::metadata::Vctr {
                name: #name,
                class: #class,
                methods,
            });
        }
    })
}
