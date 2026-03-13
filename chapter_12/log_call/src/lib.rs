use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemFn;

fn log_call_internal(_attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let item: ItemFn = syn::parse2(item).expect("Failed to parse fn");
    let sig = &item.sig;
    let name = &item.sig.ident;
    let body = &item.block;
    let vis = &item.vis;

    let output = quote! {
        #vis #sig {
            println!("start {}", stringify!(#name));
            let result = (|| #body)();
            println!("end {}, result {}", stringify!(#name), result);
            return result;
        }
    };
    output
}

#[proc_macro_attribute]
pub fn log_call(attr: TokenStream, item: TokenStream) -> TokenStream {
    log_call_internal(attr.into(), item.into()).into()
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;

    #[test]
    fn test_expansion() {
        let input = quote! {
            fn square(x: u32, y: u32) -> u32 { x * x }
        };

        let expected = quote! {
            fn square(x: u32, y: u32) -> u32 {
                println!("start {}", stringify!(square));
                let result = (|| { x * x })();
                println!("end {}, result {}", stringify!(square), result);
                return result;
            }
        };

        let attr = TokenStream2::new();
        let item = input;
        let output = log_call_internal(attr, item);

        assert_eq!(output.to_string(), expected.to_string());
    }
}
