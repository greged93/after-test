use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{ExprCall, ExprClosure};

/// The cleanup function that will be called at the end of each test.
#[derive(Clone)]
pub(crate) enum CleanupFunction {
    None,
    Call(Ident),
    CallWithArgs(ExprCall),
    Closure(ExprClosure),
}

impl CleanupFunction {
    /// Returns the [`Call`] variant of the cleanup function.
    pub(crate) fn with_call(ident: Ident) -> Self {
        Self::Call(ident)
    }

    /// Returns the [`CallWithArgs`] variant of the cleanup function.
    pub(crate) fn with_call_args(call: ExprCall) -> Self {
        Self::CallWithArgs(call)
    }

    /// Returns the [`Closure`] variant of the cleanup function.
    pub(crate) fn with_closure(closure: ExprClosure) -> Self {
        Self::Closure(closure)
    }
}

impl Parse for CleanupFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Check if the input stream is an expr call
        if input.fork().parse::<ExprCall>().is_ok() {
            return Ok(CleanupFunction::with_call_args(input.parse::<ExprCall>()?));
        }

        // Check if the input stream is an identifier
        let ident = input.parse::<Ident>();
        if ident.is_ok() && input.is_empty() {
            return Ok(CleanupFunction::with_call(ident?));
        }

        // Check if the token is a closure
        let closure = input.parse::<ExprClosure>();
        if closure.is_ok() {
            return Ok(CleanupFunction::with_closure(closure?));
        }

        emit_error!(
            input.span(),
            "expected identifier, function call or closure";
            help = "provide a identifier (e.g. clean_up), a function call (e.g. clean_up(10)) or a closure (e.g. || {println!(\"cleaned up!\")}"
        );

        Ok(CleanupFunction::None)
    }
}

impl ToTokens for CleanupFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CleanupFunction::None => {}
            CleanupFunction::Call(ident) => tokens.extend(quote!(#ident())),
            CleanupFunction::CallWithArgs(call) => call.to_tokens(tokens),
            CleanupFunction::Closure(closure) => tokens.extend(quote!((#closure)())),
        }
    }
}
