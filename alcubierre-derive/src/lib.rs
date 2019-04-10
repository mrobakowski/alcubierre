#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::*;
use syn::spanned::Spanned;
use quote::quote;

#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(item as Item);

    dbg!(_attr);

    let fun = match item {
        Item::Fn(fun) => fun,
        _ => {
            item.span().unwrap().error("only functions are supported right now").emit();
            return TokenStream::new()
        }
    };

    let name: String = fun.ident.to_string();
    let path: String = format!("/{}", name);

    let fn_ident = fun.ident.clone();
    let underscores: Vec<_> = fun.decl.inputs.iter().map(|_| quote!(_)).collect();

    let to_emit = quote! {
        #fun
        ::inventory::submit!(::alcubierre::Route::new(
            #name,
            #path,
            // we need to erase the uniqueness, so Any is more friendly
            #fn_ident as fn(#(#underscores),*) -> _)
        );
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
