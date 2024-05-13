extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, ItemImpl};

use crate::extendr_options::ExtendrOptions;
use crate::wrappers;

/// Make inherent implementations available to R
///
/// The extendr_impl function is used to make inherent implementations
/// avaialble to R as an environment. By adding the [`extendr`] attribute
/// macro to an `impl` block (supported with `enum`s and `struct`s), the
/// methods in the impl block are made available as functions in an
/// environment.
///
/// On the R side, an environment with the same name of the inherent
/// implementation is created. The environment has functions within it
/// that correspond to each method in the impl block. Note that in order
/// for an impl block to be compatible with extendr (and thus R), its return
/// type must be able to be returned to R. For example, any struct that might
/// be returned must _also_ have an `#[extendr]` annotated impl block.
///
/// Example:
/// ```ignore
/// use extendr_api::prelude::*;
///
/// // a struct that will be used internal the People struct
/// #[derive(Clone, Debug, IntoDataFrameRow)]
/// struct Person {
///     name: String,
///     age: i32,
/// }
///
/// // This will collect people in the struct
/// #[extendr]
/// #[derive(Clone, Debug)]
/// struct People(Vec<Person>);
///
/// #[extendr]
/// /// @export
/// impl People {
///
///     // instantiate a new struct with an empty vector
///     fn new() -> Self {
///         let vec: Vec<Person> = Vec::new();
///         Self(vec)
///     }
///
///     // add a person to the internal vector
///     fn add_person(&mut self, name: &str, age: i32) -> &mut Self {
///         let person = Person {
///             name: String::from(name),
///             age: age,
///         };
///
///         self.0.push(person);
///
///         // return self
///         self
///     }
///     
///     // Convert the struct into a data.frame
///     fn into_df(&self) -> Robj {
///         let df = self.0.clone().into_dataframe();
///
///         match df {
///             Ok(df) => df.as_robj().clone(),
///             Err(_) => data_frame!(),
///         }
///     }
///
///     // add another `People` struct to self
///     fn add_people(&mut self, others: &People) -> &mut Self {
///         self.0.extend(others.0.clone().into_iter());
///         self
///     }
///
///     // create a function to print the self which can be called
///     // from an R print method
///     fn print_self(&self) -> String {
///         format!("{:?}", self.0)
///     }
///
/// }
///
/// // Macro to generate exports.
/// // This ensures exported functions are registered with R.
/// // See corresponding C code in `entrypoint.c`.
/// extendr_module! {
///     mod testself;
///     impl People;
/// }
/// ```
///     
/// **Known limitation**: if you return `&Self` or `&mut Self` or a reference
/// to the same type, the resultant R object will _always_ be the original
/// self. For example the below method **will not** return `other`.
///
/// ```ignore
///  // This is not possible today
/// #[extendr]
/// impl People {
///   fn return_other(&self, other: &'static T) -> &Self {
///     other
///   }
/// }
/// ```
pub fn extendr_impl(mut item_impl: ItemImpl, opts: &ExtendrOptions) -> syn::Result<TokenStream> {
    // Only `impl name { }` allowed
    if item_impl.defaultness.is_some() {
        return Err(syn::Error::new_spanned(
            item_impl,
            "default not allowed in #[extendr] impl",
        ));
    }

    if item_impl.unsafety.is_some() {
        return Err(syn::Error::new_spanned(
            item_impl,
            "unsafe not allowed in #[extendr] impl",
        ));
    }

    if item_impl.generics.const_params().count() != 0 {
        return Err(syn::Error::new_spanned(
            item_impl,
            "const params not allowed in #[extendr] impl",
        ));
    }

    if item_impl.generics.type_params().count() != 0 {
        return Err(syn::Error::new_spanned(
            item_impl,
            "type params not allowed in #[extendr] impl",
        ));
    }

    // if item_impl.generics.lifetimes().count() != 0 {
    //     return quote! { compile_error!("lifetime params not allowed in #[extendr] impl"); }.into();
    // }

    if item_impl.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            item_impl,
            "where clause not allowed in #[extendr] impl",
        ));
    }

    let self_ty = item_impl.self_ty.as_ref();
    let self_ty_name = wrappers::type_name(self_ty);
    let prefix = format!("{}__", self_ty_name);
    let mut method_meta_names = Vec::new();
    let doc_string = wrappers::get_doc_string(&item_impl.attrs);

    // Generate wrappers for methods.
    // eg.
    // ```
    // #[no_mangle]
    // #[allow(non_snake_case)]
    // pub extern "C" fn wrap__Person__new() -> extendr_api::SEXP {
    //     unsafe {
    //         use extendr_api::FromRobj;
    //         extendr_api::Robj::from(<Person>::new()).get()
    //     }
    // }
    // ```
    let mut wrappers: Vec<ItemFn> = Vec::new();
    for impl_item in &mut item_impl.items {
        if let syn::ImplItem::Fn(ref mut method) = impl_item {
            method_meta_names.push(format_ident!(
                "{}{}__{}",
                wrappers::META_PREFIX,
                self_ty_name,
                method.sig.ident
            ));
            wrappers::make_function_wrappers(
                opts,
                &mut wrappers,
                prefix.as_str(),
                &method.attrs,
                &mut method.sig,
                Some(self_ty),
            )?;
        }
    }

    let meta_name = format_ident!("{}{}", wrappers::META_PREFIX, self_ty_name);
    let finalizer_name = format_ident!("__finalize__{}", self_ty_name);

    let conversion_impls = if opts.use_try_from {
        quote! {
            // Output conversion function for this type.

            impl TryFrom<Robj> for &#self_ty {
                type Error = Error;

                fn try_from(robj: Robj) -> Result<Self> {
                    Self::try_from(&robj)
                }
            }

            impl TryFrom<Robj> for &mut #self_ty {
                type Error = Error;

                fn try_from(mut robj: Robj) -> Result<Self> {
                    Self::try_from(&mut robj)
                }
            }

            // Output conversion function for this type.
            impl TryFrom<&Robj> for &#self_ty {
                type Error = Error;
                fn try_from(robj: &Robj) -> Result<Self> {
                    use libR_sys::R_ExternalPtrAddr;
                    unsafe {
                        let ptr = R_ExternalPtrAddr(robj.get()).cast::<#self_ty>();
                        ptr.as_ref().ok_or_else(|| Error::ExpectedExternalNonNullPtr(robj.clone()))
                    }
                }
            }

            // Input conversion function for a mutable reference to this type.
            impl TryFrom<&mut Robj> for &mut #self_ty {
                type Error = Error;
                fn try_from(robj: &mut Robj) -> Result<Self> {
                    use libR_sys::R_ExternalPtrAddr;
                    unsafe {
                        let ptr = R_ExternalPtrAddr(robj.get_mut()).cast::<#self_ty>();
                        ptr.as_mut().ok_or_else(|| Error::ExpectedExternalNonNullPtr(robj.clone()))
                    }
                }
            }
        }
    } else {
        quote! {
            // Input conversion function for this type.
            impl<'a> extendr_api::FromRobj<'a> for &#self_ty {
                fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                    if robj.check_external_ptr_type::<#self_ty>() {
                        #[allow(clippy::transmute_ptr_to_ref)]
                        Ok(unsafe { std::mem::transmute(robj.external_ptr_addr::<#self_ty>()) })
                    } else {
                        Err(concat!("expected ", #self_ty_name))
                    }
                }
            }

            // Input conversion function for a reference to this type.
            impl<'a> extendr_api::FromRobj<'a> for &mut #self_ty {
                fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
                    if robj.check_external_ptr_type::<#self_ty>() {
                        #[allow(clippy::transmute_ptr_to_ref)]
                        Ok(unsafe { std::mem::transmute(robj.external_ptr_addr::<#self_ty>()) })
                    } else {
                        Err(concat!("expected ", #self_ty_name))
                    }
                }
            }

            // Output conversion function for this type.
                impl<'a> From<&'a #self_ty> for Robj {
                fn from(value: &'a #self_ty) -> Self {
                    unsafe {
                        let ptr = Box::into_raw(Box::new(value));
                        let mut res = Robj::make_external_ptr(ptr, Robj::from(()));
                        res.set_attrib(class_symbol(), #self_ty_name).unwrap();
                        res.register_c_finalizer(Some(#finalizer_name));
                        res
                    }
                }
            }
        }
    };

    let expanded = TokenStream::from(quote! {
        // The impl itself copied from the source.
        #item_impl

        // Function wrappers
        #( #wrappers )*

        #conversion_impls

        // Output conversion function for this type.
        impl From<#self_ty> for Robj {
            fn from(value: #self_ty) -> Self {
                unsafe {
                    let ptr = Box::into_raw(Box::new(value));
                    let mut res = Robj::make_external_ptr(ptr, Robj::from(()));
                    res.set_attrib(class_symbol(), #self_ty_name).unwrap();
                    res.register_c_finalizer(Some(#finalizer_name));
                    res
                }
            }
        }

        // Function to free memory for this type.
        extern "C" fn #finalizer_name (sexp: extendr_api::SEXP) {
            unsafe {
                let robj = extendr_api::robj::Robj::from_sexp(sexp);
                if robj.check_external_ptr_type::<#self_ty>() {
                    //eprintln!("finalize {}", #self_ty_name);
                    let ptr = robj.external_ptr_addr::<#self_ty>();
                    drop(Box::from_raw(ptr));
                }
            }
        }

        #[allow(non_snake_case)]
        fn #meta_name(impls: &mut Vec<extendr_api::metadata::Impl>) {
            let mut methods = Vec::new();
            #( #method_meta_names(&mut methods); )*
            impls.push(extendr_api::metadata::Impl {
                doc: #doc_string,
                name: #self_ty_name,
                methods,
            });
        }
    });

    //eprintln!("{}", expanded);
    Ok(expanded)
}

// This structure contains parameters parsed from the #[extendr_module] definition.
