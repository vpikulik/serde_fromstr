use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DeserializeFromStr)]
pub fn deserialize_from_str_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_deserialize_from_str(&ast)
}

fn impl_deserialize_from_str(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::Error;
                let raw = String::deserialize(deserializer)?;
                raw.parse()
                    .map_err(|err: anyhow::Error| D::Error::custom(err.to_string()))
            }
        }
    };
    gen.into()
}
