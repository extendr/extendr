use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Implementation of the IntoRList macro. Refer to the documentation there
pub fn derive_r_struct(item: TokenStream) -> TokenStream {
    // Parse the tokens into a Struct
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name = ast.ident;
    let inside;
    if let Data::Struct(inner) = ast.data {
        inside = inner;
    } else {
        panic!("This is a derive macro, only use it on a struct")
    };

    // Here we iterate each struct field and make a TokenStream that creates a KeyValue pair for
    // each field
    let num_fields = inside.fields.len();
    let mut into_robj_tokens = Vec::<TokenStream2>::with_capacity(num_fields);
    let mut from_robj_tokens = Vec::<TokenStream2>::with_capacity(num_fields);

    for field in inside.fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        into_robj_tokens.push(quote!(
            (#field_str, (&value.#field_name).into())
        ));
        // e.g. `foo: value.dollar("foo")?.into()`, which is like `value$foo` in R
        from_robj_tokens.push(quote!(
            #field_name: value.dollar(#field_str)?.try_into()?
        ));
    }

    // The only thing we emit from this macro is the conversion trait impl
    TokenStream::from(quote!(
        impl std::convert::From<&#struct_name> for Robj {
            fn from(value: &#struct_name) -> Self {
                extendr_api::List::from_pairs([#(#into_robj_tokens),*]).into()
            }
        }

        impl std::convert::TryFrom<&Robj> for #struct_name {
            type Error = extendr_api::Error;

            fn try_from(value: &Robj) -> extendr_api::Result<Self> {
                Ok(#struct_name {
                    #(#from_robj_tokens),*
                })
            }
        }
    ))
}
