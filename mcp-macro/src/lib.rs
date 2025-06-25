extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{ parse_macro_input, Expr, ExprLit, ItemImpl, Lit };
use proc_macro2::Span;

fn sanitize_key(key: &str) -> proc_macro2::Ident {
    let valid = key
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    format_ident!("__register_protocol_{}", valid, span = Span::call_site())
}

#[proc_macro_attribute]
pub fn mcp_proto(attr: TokenStream, item: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(attr as Expr);
    let key = match expr {
        Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => s.value(),
        _ => panic!("Expected #[mcp_proto('mcp_method_key')] with a string literal"),
    };
    let impl_block = parse_macro_input!(item as ItemImpl);
    let fn_ident = sanitize_key(&key);
    let key_literal = key.clone();
    let self_ty = &impl_block.self_ty;
    let expanded =
        quote! {
        #impl_block
        #[::ctor::ctor]
        fn #fn_ident() {
            let s = #self_ty::default();
            crate::mcp::protocol::mcp_protocol::register_protocol(#key_literal, s);
        }
    };

    expanded.into()
}
