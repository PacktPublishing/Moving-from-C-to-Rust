use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrettyPrint)]
pub fn pretty_print_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => {
            return syn::Error::new(
                input.ident.span(),
                "PrettyPrint custom derive only supported on structures",
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = match fields {
        syn::Fields::Named(ref fields) => &fields.named,
        syn::Fields::Unnamed(ref fields) => &fields.unnamed,
        syn::Fields::Unit => &Default::default(),
    };

    let field_prints = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            if let Some(field_name) = f.ident.as_ref() {
                quote! {
                    println!(" {}: {:?}", stringify!(#field_name), self.#field_name);
                }
            } else {
                let index = syn::Index::from(i);
                quote! {
                    println!(" .{}: {:?}", #index, self.#index);
                }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        impl #name {
            pub fn pretty_print(&self) {
                #(#field_prints)*
            }
        }
    };
    TokenStream::from(output)
}
