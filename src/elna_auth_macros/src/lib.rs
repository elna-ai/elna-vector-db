extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn check_authorization(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let params = &input.sig.inputs;
    let name = &input.sig.ident;
    let return_type = &input.sig.output;
    let body = &input.block;
    let output = quote! {
        fn #name(#params) #return_type {
            let caller = ic_cdk::caller();
            let owner_principal = OWNER.with(|owner| owner.borrow().clone());
            let admins = ADMINS.with(|admins| {
                admins.borrow().iter().map(|(k, _)| k.0).collect::<Vec<_>>()
            });

            if (!(caller.to_string() == owner_principal || admins.contains(&caller))) {
                return Err(Error::Unauthorized);
            }
            #body
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn check_is_owner(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let params = &input.sig.inputs;
    let name = &input.sig.ident;
    let return_type = &input.sig.output;
    let body = &input.block;
    let output = quote! {
        fn #name(#params) #return_type {
            let caller = ic_cdk::caller();
            let owner_principal = OWNER.with(|owner| owner.borrow().clone());
            if caller.to_string() != owner_principal {
                return Err(Error::Unauthorized);
            }
            #body
        }
    };

    TokenStream::from(output)
}
