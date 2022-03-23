use proc_macro::TokenStream;
use syn::{self};  // Spanned：A trait that can provide the Span of the complete contents of a syntax tree nod
use quote::{quote};


#[proc_macro_attribute]
pub fn my_test_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("{:#?}", attr);
    eprintln!("{:#?}", item);
    item
}

#[proc_macro_derive(Httper)]
pub fn deriver(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(st: &syn::DeriveInput)  -> syn::Result<proc_macro2::TokenStream> {
    eprintln!("{}",st.ident.to_string());

    let struct_name = &st.ident;
    let ret = quote! {
        impl #struct_name {
            //  pub fn get_content(self) {
            //         // use hyper::{body::HttpBody as _, Client,Uri};
            //         //
            //         // let client = Client::new();
            //         // // Make a GET /ip to 'http://httpbin.org'
            //         // let res = client.get(Uri::from_static(self.url)).await.unwrap();
            //         // // And then, if the request gets a response...
            //         // println!("status: {}", res.status());
            //         println!("xxx")
            // };
            async fn get(self) {
                println!("xxx");
                use hyper::{body::HttpBody as _, Client,Uri};
                let client = Client::new();
                let res = client.get(Uri::from_static("http://httpbin.org/ip")).await.unwrap();
                println!("status: {}", res.status());
            }
        }
    };

    Ok(ret)
}


