//! The [`cleanup`] macro iterates over all functions marked as tests
//! in the input token stream and adds a call to the provided function
//! passed as an attribute to the macro.
//!
//! This macro can be added to the top of your test module and will ensure
//! that the passed cleanup function is called at each test function's end.
//!
//! # Example
//!
//! ```rust
//! use cleanup::cleanup;
//!
//! #[cleanup(my_clean_up)]
//! #[cfg(test)]
//! mod tests {
//!     fn my_clean_up() {
//!        println!("cleaning up resources");
//!     }   
//!
//!     #[test]
//!     fn a_test() {}
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::quote;
use quote::spanned::Spanned;
use syn::parse_macro_input;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn cleanup(attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = parse_macro_input!(item as syn::ItemMod);

    // Assert the module is marked with the `#[cfg(test)]` attribute
    assert_test_mod(&module);

    // Parse the cleanup function identifier
    let attr = proc_macro2::TokenStream::from(attr);
    let clean_fn = syn::parse2(attr.clone()).inspect_err(
        |_| emit_error!(&attr, "expected clean up function identifier"; help = "provide a identifier to your clean up function"),
    ).unwrap_or_else(|_| syn::Ident::new("default", attr.__span()));

    // Add the cleanup function call where needed
    let module = clean(module, clean_fn);

    quote!(
        #module
    )
    .into()
}

/// Asserts the module is marked with the `#[cfg(test)]` attribute
fn assert_test_mod(module: &syn::ItemMod) {
    let is_test_mod = module.attrs.iter().any(|attr| {
        attr.meta.path().is_ident("cfg")
            && attr
                .meta
                .require_list()
                .map(|l| l.tokens.to_string() == "test")
                .unwrap_or_default()
    });

    if !is_test_mod {
        emit_error!(
            module,
            "module should be marked with the `#[cfg(test)]` attribute";
            help = "add `#[cfg(test)]` to the module"
        );
    }
}

/// Adds the cleanup function call on each function marked
/// with `#[test]` attribute
fn clean(mut module: syn::ItemMod, clean_fn: syn::Ident) -> syn::ItemMod {
    let is_test = |f: &syn::ItemFn| f.attrs.iter().any(|attr| attr.path().is_ident("test"));

    module.content = module.content.map(|(brace, items)| {
        let n_items = items
            .into_iter()
            .filter_map(|i| {
                let f = match &i {
                    syn::Item::Fn(f) if is_test(f) => {
                        let attr = &f.attrs;
                        let block = &f.block;
                        let sig = &f.sig;
                        let vis = &f.vis;
                        syn::parse2(quote!(
                            #(#attr)*
                            #vis #sig {
                                #block
                                #clean_fn();
                            }
                        ))
                    }
                    i => syn::parse2(quote!(#i)),
                };
                f.inspect_err(|err| emit_error!(i, err.to_string())).ok()
            })
            .collect::<Vec<_>>();

        (brace, n_items)
    });

    module
}
