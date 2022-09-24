use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DataStruct, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(Resource, attributes(primary_key))]
pub fn resource_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("input should be parsable");

    impl_resource(&ast)
}

fn impl_resource(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let table_name = to_snake_case(&ast.ident.to_string());

    let ((primary_key, primary_key_dt), fields) = get_fields(&ast);

    let gen = quote! {
        #[async_trait::async_trait]
        impl Resource for #name {
            async fn create(&self, pool: &sqlx::postgres::PgPool) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
                let mut query: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO ");
                query.push(#table_name)
                    .push(" (")
                    .push(stringify!(#(#fields),*))
                    .push(") VALUES (");

                let mut sep = query.separated(", ");
                #(sep.push_bind(self.#fields.clone());)*

                query.push(")");

                query.build().execute(pool).await
            }

            async fn get_all(pool: &sqlx::postgres::PgPool) -> Result<Vec<Self>, sqlx::Error> {
                let mut query: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT * FROM ");
                query
                    .push(#table_name)
                    .push(" ORDER BY ")
                    .push(stringify!(#primary_key));

                query.build_query_as().fetch_all(pool).await
            }

            async fn update(&self, pool: &sqlx::postgres::PgPool) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
                let mut query: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE ");
                query.push(#table_name)
                    .push(" SET ");

                let mut sep = query.separated(", ");
                #(sep.push(stringify!(#fields)).push_unseparated(" = ").push_bind_unseparated(self.#fields.clone());)*

                query.push(" WHERE ")
                    .push(stringify!(#primary_key))
                    .push(" = ")
                    .push_bind(self.#primary_key.clone());

                query.build().execute(pool).await
            }

            async fn delete(&self, pool: &sqlx::postgres::PgPool) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
                let mut query: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM ");
                query.push(#table_name)
                    .push(" WHERE ")
                    .push(stringify!(#primary_key))
                    .push(" = ")
                    .push_bind(self.#primary_key);

                query.build().execute(pool)
                .await
            }
        }

        impl #name {
            async fn get(pool: &sqlx::postgres::PgPool, identifier: #primary_key_dt) -> Result<Self, sqlx::Error> {
                let mut query: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT * FROM ");
                query
                    .push(#table_name)
                    .push(" WHERE ")
                    .push(stringify!(#primary_key))
                    .push(" = ")
                    .push(identifier.clone());

                query.build_query_as().fetch_one(pool).await
            }
        }
    };

    gen.into()
}

fn to_snake_case(s: &str) -> String {
    let new_s: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() {
                if i == 0 {
                    format!("{}", c.to_lowercase())
                } else {
                    format!("_{}", c.to_lowercase())
                }
            } else {
                if i == s.len() - 1 {
                    format!("{}s", c)
                } else {
                    format!("{}", c)
                }
            }
        })
        .collect();
    new_s
}

fn get_fields(ast: &DeriveInput) -> ((Ident, Type), Vec<Ident>) {
    let mut primary_key: Option<Ident> = None;
    let mut primary_key_dt: Option<Type> = None;
    let mut fields = Vec::new();

    let data = match &ast.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Only structs with named fields can derive Resource"),
    };

    for d in data {
        if d.attrs.iter().any(|a| a.path.is_ident("primary_key")) {
            primary_key = d.ident.clone();
            primary_key_dt = Some(d.ty.clone());
        } else {
            let mut field = vec![d.ident.clone().expect("field should be named")];
            fields.append(&mut field)
        }
    }

    if primary_key.is_none() || primary_key_dt.is_none() {
        panic!("primary_key must be defined");
    };

    (
        (
            primary_key.expect("primary_key should be some"),
            primary_key_dt.expect("primary_key_dt should be some"),
        ),
        fields,
    )
}
