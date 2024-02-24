//! Procedural macros for floccus.
//! 
//! This crate contains procedural attribute macros (currently only one) used by the
//! [floccus](https://crates.io/crates/floccus). But potentially can be used in some
//! other crates, although will never be adapted to work with any other crate.
//! 
//! Source code of this crate is a heavily modified copy of [log-derive](https://crates.io/crates/log-derive).
//! Check that crate for more versatile logging procedural macros.

use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, spanned::Spanned, token, Expr, ExprAsync, ExprAwait, ExprBlock, ExprCall,
    ExprClosure, ExprParen, FnArg, Ident, ItemFn, Pat, Result, ReturnType,
};

/// Not so simple proc macro with no attributes that logs an error
/// when function it is applied to returns `Err()`. Log message contains
/// details of function inputs and returned error.
/// 
/// Internally, this macro converts the function into a closure and appends
/// `.map_err()` which passes the error untouched logging it along the way.
#[proc_macro_attribute]
pub fn logerr(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let original_fn: ItemFn = parse_macro_input!(item as ItemFn);

    let input_specs = log_fn_inputs(&original_fn);

    let closure = make_closure(&original_fn);
    let new_fn =
        generate_function(&closure, &original_fn, input_specs).expect("Failed generating function");
    let new_fn = replace_function_headers(original_fn, new_fn);

    new_fn.into_token_stream().into()
}

fn log_fn_inputs(func: &ItemFn) -> (String, Punctuated<Ident, token::Comma>) {
    let inputs: Vec<Ident> = func
        .sig
        .inputs
        .iter()
        .cloned()
        .map(|arg| match arg {
            FnArg::Receiver(arg) => arg.self_token.into(),
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(ident) = *pat_type.pat {
                    ident.ident
                } else {
                    unimplemented!()
                }
            }
        })
        .collect();

    let items: Punctuated<_, token::Comma> = inputs.iter().cloned().collect();
    let fmt = {
        let mut fmt = String::with_capacity(inputs.len() * 9);

        for input in inputs {
            fmt.push_str(&input.to_string());
            fmt.push_str(": {:?}, ");
        }
        fmt.pop(); // Remove the extra space.
        fmt.pop(); // Remove the extra comma.
        fmt
    };

    (fmt, items)
}

fn make_closure(original: &ItemFn) -> Expr {
    match original.sig.asyncness {
        Some(asyncness) => Expr::Await(ExprAwait {
            attrs: Vec::default(),
            await_token: token::Await::default(),
            dot_token: token::Dot::default(),
            base: Box::new(syn::Expr::Async(ExprAsync {
                attrs: Vec::default(),
                capture: Some(token::Move {
                    span: original.span(),
                }),
                block: *original.block.clone(),
                async_token: asyncness,
            })),
        }),
        None => Expr::Call(ExprCall {
            attrs: Vec::default(),
            args: Punctuated::default(),
            paren_token: token::Paren::default(),
            func: Box::new(syn::Expr::Paren(ExprParen {
                attrs: Vec::default(),
                paren_token: token::Paren::default(),
                expr: Box::new(syn::Expr::Closure(ExprClosure {
                    attrs: Vec::default(),
                    asyncness: Option::default(),
                    movability: Option::default(),
                    capture: Some(token::Move {
                        span: original.span(),
                    }),
                    or1_token: token::Or::default(),
                    inputs: Punctuated::default(),
                    or2_token: token::Or::default(),
                    output: ReturnType::Default,
                    body: Box::new(Expr::Block(ExprBlock {
                        attrs: Vec::default(),
                        label: Option::default(),
                        block: *original.block.clone(),
                    })),
                })),
            })),
        }),
    }
}

fn generate_function(
    closure: &Expr,
    original_fn: &ItemFn,
    input_specs: (String, Punctuated<Ident, token::Comma>),
) -> Result<ItemFn> {
    let (input_fmt, input_items) = input_specs;

    let fmt = original_fn.sig.ident.to_string() + "(" + &input_fmt + ") => {:?}";
    let err_expr: proc_macro2::TokenStream = quote! {log::error!(#fmt, #input_items, err)};

    let code = {
        quote! {
            fn temp() {
                let result = #closure;
                result.map_err(|err| { #err_expr; err })
            }
        }
    };

    syn::parse2(code)
}

fn replace_function_headers(original: ItemFn, new: ItemFn) -> ItemFn {
    let block = new.block;
    let mut new = original;
    new.block = block;

    new
}
