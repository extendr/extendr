use crate::{extendr_options::ExtendrOptions, wrappers};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub(crate) fn extendr_struct(
    mut str: ItemStruct,
    _opts: &ExtendrOptions,
) -> syn::Result<TokenStream> {
    if str.generics.const_params().count() != 0 {
        return Err(syn::Error::new_spanned(
            str,
            "const params not allowed in #[extendr] impl",
        ));
    }
    let self_ty = str.ident.clone();
    let mut self_ty_name = self_ty.to_string();
    for gen in str.generics.type_params() {
        self_ty_name.push('_');
        self_ty_name.push_str(gen.ident.to_string().as_str());
    }
    let mut _prefix = format!("{}__", self_ty_name);

    let _doc_string = wrappers::get_doc_string(&str.attrs);

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
        #str

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
