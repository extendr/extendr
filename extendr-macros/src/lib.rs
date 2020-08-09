extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, ItemFn, FnArg, Expr};
use syn::punctuated::Punctuated;
//use syn::parse::Parse;
use syn::Token;
use quote::{quote, format_ident};

//type CompileError = Box<dyn std::error::Error>;

// All arguments are SEXP for .Call in R.
fn translate_formal(input : &FnArg) -> FnArg {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            return parse_quote!{ #pat : extendr_api::SEXP };
        },
        _ => ()
    }
    panic!("Exported function argument must be a primitive or Robj.");
}

// Convert SEXP arguments into native types if we can.
fn translate_actual(input : &FnArg) -> Expr {
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            let ty = &pattype.ty.as_ref();
            return parse_quote!{ extendr_api::unwrap_or_throw(from_robj::<#ty>(&new_borrowed(#pat))) };
        },
        _ => ()
    }
    panic!("Exported function argument must be a primitive or Robj.");
}

#[proc_macro_attribute]
/// Generate bindings for a single function.
pub fn export_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;
    let wrap_name = format_ident!("__wrap__{}", func_name);

    let formal_args : Punctuated<FnArg, Token![,]> = func.sig.inputs.iter()
        .map(|input| {
            translate_formal(input)
        }).collect();

    let actual_args : Punctuated<Expr, Token![,]> = func.sig.inputs.iter()
        .map(|input| {
            translate_actual(input)
        }).collect();

        //println!("export_function func: {:#?}", func);

    let expanded = quote! {
        // #[allow(dead_code)]
        #func

        #[no_mangle]
        pub extern "C" fn #wrap_name(#formal_args) -> extendr_api::SEXP {
            use extendr_api::{from_robj, new_borrowed};
            unsafe {
                extendr_api::Robj::from(#func_name(#actual_args)).get()
            }
        }
    };
    
    //println!("res: {}", expanded);
    TokenStream::from(expanded)
}

