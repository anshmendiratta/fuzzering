#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(proc_macro_quote)]

use std::str::FromStr;

use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};
use primitives::Type;
use proc_macro::{quote, TokenStream};
use syn::{
    parse::{self, discouraged::AnyDelimiter, Parse},
    parse_macro_input, parse_quote, Data, DataStruct, DeriveInput, Field, Fields, Ident,
    Type as SynType,
};

extern crate proc_macro;

mod primitives {
    #[derive(Debug)]
    struct Identifier<'a>(&'a str);
    #[derive(Debug)]
    pub struct Type<'a>(pub &'a str);

    #[derive(Debug)]
    pub struct Field<'a> {
        identifier: Identifier<'a>,
        rust_type: Type<'a>,
    }
}

#[proc_macro_attribute]
pub fn fuzzthis(attributes: TokenStream, items: TokenStream) -> TokenStream {
    let quote = quote!(items);
    // let parsed_quote = parse_quote!(quote);

    let derive_input = parse_macro_input!(items as DeriveInput);
    let fields = get_struct_fields(derive_input.data);
    for field in fields {
        dbg!(field.ident.unwrap());
        match field.ty {
            SynType::Array(ta) => {
                // dbg!(ta.elem);
            }
            _ => return TokenStream::from_str("").unwrap(),
        }
    }

    TokenStream::from_str("").unwrap()
}

fn get_struct_fields(data: Data) -> Vec<Field> {
    let mut fields = vec![];
    if let Data::Struct(ds) = data {
        if let Fields::Named(f) = ds.fields {
            for field in f.named.iter() {
                fields.push(field.clone());
            }
        }
    }

    fields
}
