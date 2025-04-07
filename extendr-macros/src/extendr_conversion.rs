use crate::{extendr_options::ExtendrOptions, wrappers};
use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::Generics;
use syn::Ident;
use syn::Item;

struct TypeFields {
    ident: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
}

fn pull_fields(item: &Item) -> syn::Result<TypeFields> {
    let fields = match item {
        Item::Struct(str) => TypeFields {
            ident: str.ident.clone(),
            generics: str.generics.clone(),
            attrs: str.attrs.clone(),
        },
        Item::Enum(str) => TypeFields {
            ident: str.ident.clone(),
            generics: str.generics.clone(),
            attrs: str.attrs.clone(),
        },
        _ => {
            return Err(syn::Error::new_spanned(
                item,
                "#[extendr] conversions can only be built for `struct` and `enum`",
            ))
        }
    };
    Ok(fields)
}

pub(crate) fn extendr_type_conversion(item: Item, opts: &ExtendrOptions) -> TokenStream {
    match do_extendr_type_conversion(item, opts) {
        Ok(result) => result,
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_extendr_type_conversion(item: Item, _opts: &ExtendrOptions) -> syn::Result<TokenStream> {
    let TypeFields {
        ident: self_ty,
        generics,
        attrs,
    } = pull_fields(&item)?;
    if generics.const_params().count() != 0 {
        return Err(syn::Error::new_spanned(
            item,
            "const params not allowed in #[extendr] impl",
        ));
    }
    let mut self_ty_name = self_ty.to_string();
    for gen in generics.type_params() {
        self_ty_name.push('_');
        self_ty_name.push_str(gen.ident.to_string().as_str());
    }

    // TODO: Should documenting the struct be moved to R?
    // At the moment, only documentattion above the impl
    // block makes it to R.
    let _doc_string = wrappers::get_doc_string(&attrs);

    let conversion_impls = quote! {
        // Output conversion function for this type.

        impl TryFrom<Robj> for &#self_ty {
            type Error = extendr_api::Error;

            fn try_from(robj: Robj) -> extendr_api::Result<Self> {
                Self::try_from(&robj)
            }
        }

        impl TryFrom<Robj> for &mut #self_ty {
            type Error = extendr_api::Error;

            fn try_from(mut robj: Robj) -> extendr_api::Result<Self> {
                Self::try_from(&mut robj)
            }
        }

        // Output conversion function for this type.
        impl TryFrom<&Robj> for &#self_ty {
            type Error = extendr_api::Error;
            fn try_from(robj: &Robj) -> extendr_api::Result<Self> {
                use extendr_api::ExternalPtr;
                unsafe {
                    let external_ptr: &ExternalPtr<#self_ty> = robj.try_into()?;
                    external_ptr.try_addr()
                }
            }
        }

        // Input conversion function for a mutable reference to this type.
        impl TryFrom<&mut Robj> for &mut #self_ty {
            type Error = extendr_api::Error;
            fn try_from(robj: &mut Robj) -> extendr_api::Result<Self> {
                use extendr_api::ExternalPtr;
                unsafe {
                    let external_ptr: &mut ExternalPtr<#self_ty> = robj.try_into()?;
                    external_ptr.try_addr_mut()
                }
            }
        }
    };

    let output = TokenStream::from(quote! {
        #item

        #conversion_impls

        // Output conversion function for this type.
        impl From<#self_ty> for Robj {
            fn from(value: #self_ty) -> Self {
                use extendr_api::ExternalPtr;
                unsafe {
                    let mut res: ExternalPtr<#self_ty> = ExternalPtr::new(value);
                    res.set_attrib(class_symbol(), #self_ty_name).unwrap();
                    res.into()
                }
            }
        }

    });
    Ok(output)
}
