extern crate proc_macro;
use biscuit_auth::builder::BlockBuilder;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Parser, Result},
    punctuated::Punctuated,
    Expr, ExprAssign, LitStr, Token,
};

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    let ParsedQuery { datalog, .. } = syn::parse(input).unwrap();

    let mut builder = BlockBuilder::new();
    builder.add_code(&datalog).unwrap();

    let gen = quote! {
      #datalog
    };
    gen.into()
}

struct ParsedQuery {
    datalog: String,
    parameters: HashMap<String, Expr>,
}

impl Parse for ParsedQuery {
    fn parse(input: ParseStream) -> Result<Self> {
        // we expect a string literal here, we let syn extract it
        let query: LitStr = input.parse()?;
        let string = query.value();

        let mut parameters = HashMap::new();

        let _: Token![,] = input.parse()?;
        let pairs: ExprAssign = input.parse()?;

        if !input.is_empty() {
            return Err(syn::parse::Error::new(input.span(), "unpexected token"));
        }

        Ok(ParsedQuery {
            datalog: string,
            parameters,
        })
    }
}
