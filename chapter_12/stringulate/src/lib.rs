use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn stringulate(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    TokenStream::from(quote! {
        #input
    })
}
