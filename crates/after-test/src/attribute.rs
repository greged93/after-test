use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::ExprClosure;

/// The cleanup function that will be called at the end of each test.
#[derive(Clone)]
pub(crate) enum CleanupFunction {
    None,
    Call(Ident),
    Closure(ExprClosure),
}

impl CleanupFunction {
    /// Returns the [`Call`] variant of the cleanup function.
    pub(crate) fn with_ident(ident: Ident) -> Self {
        Self::Call(ident)
    }

    /// Returns the [`Closure`] variant of the cleanup function.
    pub(crate) fn with_closure(closure: ExprClosure) -> Self {
        Self::Closure(closure)
    }
}

impl Parse for CleanupFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Check if the first token is an identifier
        let ident = input.parse::<Ident>();
        if ident.is_ok() {
            return Ok(CleanupFunction::with_ident(ident?));
        }

        // Check if the first token is a closure
        let closure = input.parse::<ExprClosure>().inspect_err(
            |_| emit_error!(&input.span(), "expected closure"; help = "provide a closure to your clean up function"),
        );

        if closure.is_ok() {
            return Ok(CleanupFunction::with_closure(closure?));
        }

        emit_error!(input.span(), "expected identifier or closure"; help = "provide a closure to your clean up function");

        Ok(CleanupFunction::None)
    }
}

impl ToTokens for CleanupFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CleanupFunction::None => {}
            CleanupFunction::Call(ident) => ident.to_tokens(tokens),
            CleanupFunction::Closure(closure) => tokens.extend(quote!((#closure))),
        }
    }
}
