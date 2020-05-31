extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, ItemFn, FnArg, Pat, Expr, Type};
use syn::punctuated::Punctuated;
//use syn::parse::Parse;
use syn::Token;
use quote::{quote, format_ident};

//type CompileError = Box<dyn std::error::Error>;

// All arguments are SEXP for .Call in R.
fn translate_formal(input : &FnArg) -> FnArg {
    println!("arg={:?}", &input);
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            match pat {
                Pat::Ident(ref ident) => {
                    return parse_quote!{ #ident : ::libR_sys::SEXP };
                }
                _ => ()
            }
        },
        _ => ()
    }
    panic!("#[export_function] argument must be a primitive or Robj.");
}

// Convert SEXP arguments into native types if we can.
fn translate_actual(input : &FnArg) -> Expr {
    //println!("arg={:?}", &input);
    match input {
        FnArg::Typed(ref pattype) => {
            let pat = &pattype.pat.as_ref();
            let ty = &pattype.ty.as_ref();
            match (pat, ty) {
                (Pat::Ident(ref ident), Type::Path(ref path)) => {
                    //return parse_quote!{ #path :: try_from(extendr_api::new_borrowed(#ident)).unwrap() };
                    return parse_quote!{ unsafe { extendr_api::new_borrowed(#ident).get_best::<#path>() } };
                }
                _ => ()
            }
        },
        _ => ()
    }
    panic!("Exported function argument must be a primitive or Robj.");
}

#[proc_macro_attribute]
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
        pub extern "C" fn #wrap_name(#formal_args) -> ::libR_sys::SEXP {
            let res = #func_name(#actual_args);
            unsafe { extendr_api::Robj::from(res).get() }
        }
    };
    
    //println!("res: {}", expanded);
    TokenStream::from(expanded)
}

