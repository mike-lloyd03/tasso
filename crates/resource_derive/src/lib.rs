use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DataStruct, Fields};

#[proc_macro_derive(Resource, attributes(primary_key))]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("input should be parsable");

    impl_resource(&ast)
}

fn impl_resource(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let table_name = to_snake_case(&name.to_string());
    let data = match &ast.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Only structs with named fields can derive Resource"),
    };

    let mut primary_key = "".to_string();
    for d in data {
        for a in &d.attrs {
            if a.path.is_ident("primary_key") {
                primary_key = d.ident.clone().unwrap().to_string();
            }
        }
    }
    if primary_key.is_empty() {
        panic!("primary_key must be defined")
    }

    println!("primary_key: '{}'", primary_key);

    let gen = quote! {
        impl Resource for #name {
            fn table_name() -> &'static str {
                // println!("{:?}", #data.to_string());
                // for f in #data {
                //     println!("{:?}", f);
                // };

                // format!("{}s", stringify!(#name)).as_str()
                stringify!(#table_name)
            }

            // fn fields(&self) -> Vec<(&'static str, DataType)> {
            //     vec![
            //         ("first", DataType::Bool(false))
            //     ]
            // }

            fn primary_key() -> &'static str {
                stringify!(#primary_key)
            }

            // fn primary_key_value(&self) -> DataType {
            //     DataType::Int64(0)
            // }
        }
    };

    gen.into()
}

fn to_snake_case(s: &str) -> String {
    let new_s: String = s
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("_{}", c.to_lowercase())
            } else {
                format!("{}", c)
            }
        })
        .collect();
    new_s
}
