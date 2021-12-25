use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Item};
use quote::quote;

#[proc_macro_attribute]
pub fn my_test_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("{:#?}", parse_macro_input!(attr as AttributeArgs));
    let body_ast = parse_macro_input!(item as Item);
    eprintln!("{:#?}", body_ast);
    quote!(#body_ast).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
