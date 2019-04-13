//#![feature(proc_macro_diagnostic)]

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

    let Args { path } = parse_macro_input!(attr as Args);

    let fun = match item {
        Item::Fn(fun) => fun,
        _ => {
            return TokenStream::from(syn::Error::new(
                item.span(),
                "only functions are supported right now"
            ).to_compile_error());
        }
    };

    let name: String = fun.ident.to_string();

    let path = match path {
        Some(pathLiteral) => {
            let path_str = pathLiteral.value();
            // TODO: parse path and do path param extraction based on that
            quote! { compile_error!("specifying path doesn't work yet!") }
        }
        None => {
            let params: Vec<_> = fun.decl.inputs.iter()
                .map(|_| quote!(and(path::param())))
                .collect();

            quote! {
                path(#name)
                    #(.#params)*
            }
        }
    };

    let fn_ident = fun.ident.clone();
    let warp_filter = quote! {{
        use alcubierre::warp::*;
        #path
            .map(#fn_ident)
            .map(|r| reply::boxed(r)) // TODO: allocation
            .boxed() // TODO: allocation

    }};

    let to_emit = quote! {
        #fun
        alcubierre::inventory::submit!(#![crate=alcubierre] alcubierre::Route {
            name: #name,
            mod_path: module_path!(),
            filter: #warp_filter
        });
    };

    to_emit.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
