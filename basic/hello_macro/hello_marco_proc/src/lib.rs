use proc_macro::TokenStream;
use syn;

use impl_hello_marco_derive::impl_hello_marco_derive;

mod impl_hello_marco_derive;

#[proc_macro_derive(HelloMarco)]
pub fn hello_marco_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).ok().unwrap_or_else(|| {
        panic!("Error");
    });

    impl_hello_marco_derive(&ast)
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn sql(_item: TokenStream) -> TokenStream {
    let sql = "SELECT * FROM users";
    sql.parse().unwrap()
}
