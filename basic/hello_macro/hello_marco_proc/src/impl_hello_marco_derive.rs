use proc_macro::TokenStream;
use quote;
use syn;

pub fn impl_hello_marco_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote::quote! {
        impl HelloMacroTrait for #name {
            fn signature() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    gen.into()
}
