extern crate proc_macro;
use biscuit_auth::builder::BlockBuilder;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::{self, Parse, ParseStream, Parser, Peek, Result},
    parse_macro_input,
    punctuated::{self, Punctuated},
    token::Comma,
    DeriveInput, Expr, ExprAssign, Ident, LitStr, Token,
};

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    // litteral / separator
    // punctuated (comma separated)
    // ident = expression

    let ParsedQuery {
        datalog,
        parameters,
    } = syn::parse(input).unwrap();

    let mut builder = BlockBuilder::new();
    builder.add_code(&datalog).unwrap();

    let gen = quote! {
        {
          #builder
        }
    };

    dbg!(&gen.to_string());
    gen.into()
}

struct ParsedQuery {
    datalog: String,
    parameters: HashMap<String, Expr>,
}

impl Parse for ParsedQuery {
    fn parse(input: ParseStream) -> Result<Self> {
        let datalog = input.parse::<LitStr>()?.value();

        let mut parameters = HashMap::new();

        while input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            if input.is_empty() {
                break;
            }

            let key: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;

            parameters.insert(key.to_string(), value);
        }

        Ok(ParsedQuery {
            datalog,
            parameters,
        })
    }
}

// struct ParsedQuery {
//     datalog: String,
//     parameters: HashMap<String, Expr>,
// }

// impl Parse for ParsedQuery {
//     fn parse(input: ParseStream) -> Result<Self> {
//         // we expect a string literal here, we let syn extract it
//         let query: LitStr = input.parse()?;
//         let string = query.value();

//         let mut parameters = HashMap::new();

//         let _: Token![,] = input.parse()?;
//         let pairs: ExprAssign = input.parse()?;

//         if !input.is_empty() {
//             return Err(syn::parse::Error::new(input.span(), "unpexected token"));
//         }

//         Ok(ParsedQuery {
//             datalog: string,
//             parameters,
//         })
//     }
// }
