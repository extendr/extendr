use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, ItemImpl};

use crate::extendr_options::ExtendrOptions;
use crate::utils::{doc_string, type_name};
use crate::wrappers;

/// Transform the method documentation to Roxygen format.
fn transform_method_doc_roxygen(method_name: &str, doc: &str) -> (String, Vec<String>) {
    let mut description = Vec::new();
    let mut other_groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut current_other: Option<(String, Vec<String>)> = None;
    let mut params: Vec<(String, Vec<String>)> = Vec::new();
    let mut current_param: Option<(String, Vec<String>)> = None;
    let mut state = "description";

    // for each line of the docstring
    for line in doc.lines() {
        let trimmed = line.trim();
        // params
        if trimmed.starts_with("@param") {
            if let Some((name, lines)) = current_param.take() {
                params.push((name, lines));
            }
            if let Some((tag, content)) = current_other.take() {
                other_groups.push((tag, content));
            }
            // split tag name from the rest
            let mut parts = trimmed.splitn(3, ' ');
            parts.next();
            // Extract the parameter name and description (unwrap_or handles empty cases like multiline tags)
            // it appends multiline param docstrings later
            let param_name = parts.next().unwrap_or("").to_string();
            let param_desc = parts.next().unwrap_or("").to_string();
            current_param = Some((param_name, vec![param_desc]));
            state = "param";
            continue;
        // same for every other tags
        } else if trimmed.starts_with('@') {
            if let Some((name, lines)) = current_param.take() {
                params.push((name, lines));
            }
            let mut parts = trimmed.splitn(2, ' ');
            let tag_with_at = parts.next().unwrap_or("");
            let tag = tag_with_at.trim_start_matches('@').to_string();
            // flush current group if changing tag
            if let Some((curr_tag, _)) = &current_other {
                if *curr_tag != tag {
                    let (t, c) = current_other.take().unwrap();
                    other_groups.push((t, c));
                    current_other = Some((tag.clone(), Vec::new()));
                }
            } else {
                current_other = Some((tag.clone(), Vec::new()));
            }
            // add inline content if present
            if let Some(inline) = parts.next() {
                let inline = inline.trim();
                if !inline.is_empty() {
                    if let Some((_, ref mut vec)) = current_other {
                        vec.push(inline.to_string());
                    }
                }
            }
            state = "other";
            continue;
        }

        match state {
            "description" => description.push(trimmed.to_string()),
            // handle multiline `@...` docstrings
            "other" => {
                if let Some((_, ref mut vec)) = current_other {
                    vec.push(trimmed.to_string());
                }
            }
            // handle multiline `@param` docstrings
            "param" => {
                if let Some((_, ref mut lines)) = current_param {
                    lines.push(trimmed.to_string());
                }
            }
            _ => description.push(trimmed.to_string()),
        }
    }
    if let Some((name, lines)) = current_param.take() {
        params.push((name, lines));
    }
    if let Some((tag, content)) = current_other.take() {
        other_groups.push((tag, content));
    }

    // creates `method` subsection (obs.: for each impl block)
    let mut output = String::new();
    output.push_str(&format!("\\subsection{{Method `{}`}}{{\n", method_name));
    if !description.is_empty() {
        output.push_str(&description.join("\n"));
        output.push('\n');
    }
    if !params.is_empty() {
        // params docstrings goes here
        output.push_str(" \\subsection{Arguments}{\n\\describe{\n");
        for (pname, plines) in params {
            let param_text = plines.join(" ");
            output.push_str(&format!("\\item{{`{}`}}{{{}}}\n", pname, param_text));
        }
        output.push_str("}}\n");
    }
    // for other docsstring, it creates a subsection for each tag
    // usage is special because if we don't enclose it in
    // a preformatted block, it will be be sent as one single line, e.g.:
    // @usage
    // #' foo(
    // #'   bar,
    // #'   baz
    // #' )
    // becomes
    // @usage
    // #' foo(bar, baz)
    //
    // examples are also special: they should be appended and put under @examples (not in a custom subsection)
    // if there's another special treatment needed, it should be added here
    let mut examples: Vec<String> = Vec::new();
    for (tag, contents) in other_groups {
        match tag.as_str() {
            "examples" => {
                examples.push(format!(
                    "## ---- Method `{}` ---- ##\n{}",
                    method_name,
                    contents.join("\n")
                ));
            }
            "usage" => {
                output.push_str(&format!(
                    " \\subsection{{{}}}{{\n \\preformatted{{\n{}\n}}\n}}\n",
                    tag,
                    contents.join("\n")
                ));
            }
            other => {
                output.push_str(&format!(
                    " \\subsection{{{}}}{{\n{}\n}}\n",
                    other,
                    contents.join("\n")
                ));
            }
        }
    }
    output.push_str("}\n");
    (output, examples)
}

/// Make inherent implementations available to R
///
/// The `extendr_impl` function is used to make inherent implementations
/// available to R as an environment. By adding the `extendr` attribute
/// macro to an `impl` block (supported with `enum`s and `struct`s), the
/// methods in the impl block are made available as functions in an
/// environment.
///
///
/// On the R side, an environment with the same name of the inherent
/// implementation is created. The environment has functions within it
/// that correspond to each method in the impl block. Note that in order
/// for an impl block to be compatible with extendr (and thus R), its return
/// type must be able to be returned to R. For example, any struct that might
/// be returned must _also_ have an `#[extendr]` annotated impl block.
///
/// Example:
/// ```dont_run
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
pub(crate) fn extendr_impl(
    mut item_impl: ItemImpl,
    opts: &ExtendrOptions,
) -> syn::Result<TokenStream> {
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
    let self_ty_name = type_name(self_ty);
    let prefix = format!("{}__", self_ty_name);
    let mut method_meta_names = Vec::new();

    // Now we get struct level docs but I think it's nice to let impl level docs too
    // that way a user can add a impl-related docstring locally, without having to bloat the struct docs
    let impl_doc = doc_string(&item_impl.attrs);
    let struct_doc = wrappers::get_struct_doc(&self_ty_name);
    // Since struct docs are generated by extendr_impl, just as the methods section, it'll get duplicated for each
    // impl block. A hack to solve it without having to split both logics is to erase struct docs so subsequent impl
    // blocks don't repeat it
    wrappers::register_struct_doc(&self_ty_name, "");

    let combined_doc = if struct_doc.is_empty() {
        impl_doc
    } else {
        format!("{}\n{}", struct_doc.trim_end(), impl_doc)
    };

    let mut method_docs: Vec<(String, String)> = Vec::new();
    let mut all_examples: Vec<String> = Vec::new();

    for impl_item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = impl_item {
            let mdoc = doc_string(&method.attrs);
            if !mdoc.is_empty() {
                let (sect, examples) =
                    transform_method_doc_roxygen(&method.sig.ident.to_string(), &mdoc);
                method_docs.push((method.sig.ident.to_string(), sect));
                for ex in examples {
                    all_examples.push(ex);
                    all_examples.push(String::new());
                }
            }
        }
    }

    // Build a Methods section
    // It actually creates a method section for each impl block, but Roxygen won't complain about that.
    let methods_section = if !method_docs.is_empty() {
        let mut sec = String::from("\n @section Methods:");
        for (_name, doc) in &method_docs {
            sec.push('\n');
            sec.push_str(doc);
        }
        sec
    } else {
        String::new()
    };

    // Append single examples block if any
    let examples_section = if !all_examples.is_empty() {
        let mut ex = String::from("\n @examples\n");
        for line in &all_examples {
            ex.push_str(line);
            ex.push('\n');
        }
        ex
    } else {
        String::new()
    };

    let full_doc = format!("{}{}{}", combined_doc, methods_section, examples_section);

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

    let expanded = TokenStream::from(quote! {
        // The impl itself copied from the source.
        #item_impl

        // Function wrappers
        #( #wrappers )*

        #[allow(non_snake_case)]
        fn #meta_name(impls: &mut Vec<extendr_api::metadata::Impl>) {
            let mut methods = Vec::new();
            #( #method_meta_names(&mut methods); )*
            impls.push(extendr_api::metadata::Impl {
                doc: #full_doc,
                name: #self_ty_name,
                methods,
            });
        }
    });

    //eprintln!("{}", expanded);
    Ok(expanded)
}

// This structure contains parameters parsed from the #[extendr_module] definition.
