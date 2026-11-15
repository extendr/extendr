use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub fn derive_rvctr(item: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse::<DeriveInput>(item)?;
    let struct_name = &input.ident;

    let member = match &input.data {
        Data::Struct(data) => match data.fields.members().collect::<Vec<_>>().as_slice() {
            [member] => member.clone(),
            _ => {
                return Err(syn::Error::new_spanned(
                    &data.fields,
                    "Rvctr can only be derived for structs with exactly one field.",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "Rvctr can only be derived for structs.",
            ))
        }
    };

    let mut format_fn = None::<syn::Path>;
    for attr in &input.attrs {
        if attr.path().is_ident("rvctr") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("format") {
                    format_fn = Some(meta.value()?.parse()?);
                    Ok(())
                } else {
                    Err(meta.error("expected `format = <function>`"))
                }
            })?;
        }
    }

    let format_body = match format_fn {
        Some(f) => quote! { #f(self) },
        None => quote! { extendr_api::optional::vctrs::Rvctr::format(&self.#member) },
    };

    let module_name = format_ident!("vctr_{}", struct_name);
    let format_fn_name = format_ident!("format_{}", struct_name);
    let length_fn_name = format_ident!("length_{}", struct_name);
    let subset_fn_name = format_ident!("subset_{}", struct_name);
    let extend_fn_name = format_ident!("extend_{}", struct_name);

    let fmt_method = format!("format.{struct_name}");
    let len_method = format!("length.{struct_name}");
    let c_method = format!("c.{struct_name}");
    let brack_method = format!("`[.{struct_name}`");

    Ok(quote! {
        impl extendr_api::optional::vctrs::Rvctr for #struct_name {
            fn class() -> &'static str {
                stringify!(#struct_name)
            }

            fn format(&self) -> extendr_api::Strings {
                #format_body
            }

            fn length(&self) -> extendr_api::scalar::Rint {
                extendr_api::optional::vctrs::Rvctr::length(&self.#member)
            }

            fn subset(&self, idx: extendr_api::Integers) -> Self {
                Self {
                    #member: extendr_api::optional::vctrs::Rvctr::subset(&self.#member, idx),
                }
            }

            fn extend(self, y: Self) -> Self {
                Self {
                    #member: extendr_api::optional::vctrs::Rvctr::extend(self.#member, y.#member),
                }
            }
        }

        impl<'a> TryFrom<&'a extendr_api::Robj> for &'a #struct_name {
            type Error = extendr_api::Error;

            fn try_from(robj: &'a extendr_api::Robj) -> extendr_api::Result<Self> {
                let ptr = extendr_api::Attributes::get_attrib(robj, "extendr_ptr")
                    .ok_or_else(|| extendr_api::Error::ExpectedExternalPtr(robj.clone()))?;
                let ptr = <&extendr_api::ExternalPtr<#struct_name>>::try_from(&ptr)?;
                let value = std::ptr::from_ref(ptr.try_addr()?);
                // SAFETY: the external pointer is an attribute of `robj`.
                Ok(unsafe { &*value })
            }
        }

        impl TryFrom<&extendr_api::Robj> for #struct_name {
            type Error = extendr_api::Error;

            fn try_from(robj: &extendr_api::Robj) -> extendr_api::Result<Self> {
                <&#struct_name>::try_from(robj).cloned()
            }
        }

        impl From<#struct_name> for extendr_api::Robj {
            fn from(value: #struct_name) -> Self {
                let class = <#struct_name as extendr_api::optional::vctrs::Rvctr>::class();
                let n = extendr_api::optional::vctrs::Rvctr::length(&value).0;
                let mut data = extendr_api::Robj::from(extendr_api::Integers::from_values(
                    (1..n + 1).map(extendr_api::scalar::Rint::from),
                ));
                let ptr = extendr_api::ExternalPtr::new(value);
                extendr_api::Attributes::set_attrib(&mut data, "extendr_ptr", ptr)
                    .and_then(|x| extendr_api::Attributes::set_class(x, [class, "extendr_vctr"]))
                    .map(|x| x.clone())
                    .unwrap_or_else(|e| panic!("failed to build vctr of class `{class}`: {e}"))
            }
        }

        #[allow(non_snake_case)]
        pub(crate) mod #module_name {
            use super::*;

            /// @export
            #[extendr_api::extendr(r_name = #fmt_method)]
            pub fn #format_fn_name(x: &#struct_name) -> extendr_api::Strings {
                extendr_api::optional::vctrs::Rvctr::format(x)
            }

            /// @export
            #[extendr_api::extendr(r_name = #len_method)]
            pub fn #length_fn_name(x: &#struct_name) -> extendr_api::scalar::Rint {
                extendr_api::optional::vctrs::Rvctr::length(x)
            }

            /// @export
            #[extendr_api::extendr(r_name = #brack_method)]
            pub fn #subset_fn_name(
                x: &#struct_name,
                idx: extendr_api::prelude::Either<extendr_api::Doubles, extendr_api::Integers>,
            ) -> #struct_name {
                let idx = match idx {
                    extendr_api::prelude::Either::Left(dbl) => dbl
                        .iter()
                        .map(|di| {
                            if extendr_api::CanBeNA::is_na(&di) {
                                <extendr_api::scalar::Rint as extendr_api::CanBeNA>::na()
                            } else {
                                extendr_api::scalar::Rint::from(di.0 as i32)
                            }
                        })
                        .collect::<extendr_api::Integers>(),
                    extendr_api::prelude::Either::Right(ints) => ints,
                };

                extendr_api::optional::vctrs::Rvctr::subset(x, idx)
            }

            /// @export
            #[extendr_api::extendr(r_name = #c_method)]
            pub fn #extend_fn_name(x: #struct_name, y: #struct_name) -> #struct_name {
                extendr_api::optional::vctrs::Rvctr::extend(x, y)
            }

            extendr_api::extendr_module! {
                mod #module_name;
                fn #format_fn_name;
                fn #length_fn_name;
                fn #subset_fn_name;
                fn #extend_fn_name;
            }
        }
    }
    .into())
}
