#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::*;
use syn::spanned::Spanned;
use quote::quote;

#[derive(Debug)]
struct Args {
    path: Option<LitStr>,
}

impl parse::Parse for Args {
    fn parse(input: &parse::ParseBuffer) -> syn::Result<Self> {
        if input.is_empty() { return Ok(Args { path: None }); }

        let path = input.parse::<LitStr>().or_else(|_| {
            let path_ident: Ident = input.parse::<Ident>()?;

            if path_ident.to_string() != "path" {
                return Err(syn::Error::new(
                    path_ident.span(),
                    "only `path` attribute is supported currently",
                ));
            }

            input.parse::<Token![=]>()?;
            input.parse::<LitStr>()
        })?;

        Ok(Args { path: Some(path) })
    }
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(item as Item);

    let attr: Args = dbg!(parse_macro_input!(attr as Args));

    let fun = match item {
        Item::Fn(fun) => fun,
        _ => {
            item.span().unwrap().error("only functions are supported right now").emit();
            return TokenStream::new();
        }
    };

    let name: String = fun.ident.to_string();
    let path: String = format!("/{}", name);

    let fn_ident = fun.ident.clone();
    let params: Vec<_> = fun.decl.inputs.iter()
        .map(|_| quote!(.and(path::param())))
        .collect();
    let warp_filter = quote! {{
        use ::alcubierre::warp::*;
        path(#path)
            #(#params)*
            .map(#fn_ident)
            .map(|r| reply::boxed(r)) // TODO: allocation
            .boxed() // TODO: allocation

    }};

    let to_emit = quote! {
        #fun
        ::inventory::submit!(::alcubierre::Route {
            name: #name,
            path: #path,
            filter: #warp_filter
        });
    };

    println!("{}", to_emit);

    to_emit.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
