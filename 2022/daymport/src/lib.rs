#![allow(clippy::enum_glob_use, clippy::needless_pass_by_value)]

extern crate proc_macro;
mod error;

use crate::error::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span, TokenStream as TokenStream2};
use quote::quote;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr};

struct Arg {
    path: LitStr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            path: input.parse()?,
        })
    }
}

#[proc_macro]
// #[proc_macro_derive(PrintAST)]
pub fn dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let rel_path = input.path.value();

    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };

    let match_start = "match day {";
    let content = match source_file_names(dir) {
        Ok(names) => names.into_iter().enumerate().map(|(idx, name)| match_arm(name, idx + 1)).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };
    let last = "_ => None,";
    let match_end = "}";

    let stuff = format!("{}{}{}{}", match_start, content.to_string(), last, match_end);

    TokenStream::from_str(&stuff[..]).unwrap()
}

fn match_arm(name: String, idx: usize) -> TokenStream2 {
    let ident = Ident::new(&name, Span::call_site());
    let lit = Literal::usize_unsuffixed(idx);

    quote! {
      #lit => Some(Box::new(crate::days::#ident::Problem)),
    }
}

fn source_file_names<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        if !entry.file_name().to_str().unwrap().contains("day") {
            continue;
        };
        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        return Err(Error::Utf8(failure));
    }

    if names.is_empty() {
        return Err(Error::Empty);
    }

    names.sort();
    Ok(names)
}
