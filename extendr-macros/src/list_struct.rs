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
    let tokens: Vec<TokenStream2> = inside
        .fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_str = field_name.to_string();
            quote!(
                (#field_str, value.#field_name.into())
            )
        })
        .collect();

    // The only thing we emit from this macro is the conversion trait impl
    TokenStream::from(quote!(
        impl std::convert::From<#struct_name> for Robj {
            fn from(value: Foo) -> Self {
                extendr_api::List::from_pairs([#(#tokens),*]).into()
            }
        }
    ))
}
