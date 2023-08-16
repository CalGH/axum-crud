use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, DeriveInput};

/*
#[proc_macro_derive(ShowInfo)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();

        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
    }
    let expanded = quote! {
    impl PrintStruct for #struct_name {
        fn print(&self) {
            #(
                println!(
                    "key={key}, value={value}, type={type_name}",
                    key = #keys,
                    value = self.#idents,
                    type_name = stringify!(#types)
                );
            )*
        }
    }
       };
    expanded.into()
}
*/
#[proc_macro_derive(GetOne)]
pub fn derivegetone(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_str = struct_name.to_string();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let idfield = match fields.named.first() {
        Some(field) => field,
        None => panic!("No field"),
    };

    let type_name = idfield.ty.to_token_stream();

    let expanded = quote! {
        #[async_trait]
        impl crate::GetOneTrait for #struct_name {

            type ExtensionType = Appstate;
            type QueryParamType = QueryParams;
            type PathType = #type_name;

            async fn getroot(
                                         Extension(state): Extension<Self::ExtensionType>,
                                         qval: Option<Query<Self::QueryParamType>>,
                                         Path(id): Path<Self::PathType>,
            ) -> (StatusCode, Json<Value>) {

                let stmt = crate::api::QueryBuilder::<#struct_name, Self::PathType>::Single(String::new(), None, None, axum::http::Method::GET);
                let rows = stmt.structname(#struct_str.to_string()).pkid(id).method(axum::http::Method::GET).build(&state.pool).await;
                dbg!(&rows);

                let qval = match qval {
                    Some(val) => val.0,
                    None => QueryParams{id : None},
                };
                if rows.is_err() {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "query val" : qval.id,
                        "state val" : state.number,
                        "path val" : id,
                    })))
                }
                let result = get_one::<#struct_name>(rows.expect("No error"));
                let (code, resp) = match result {
                      Ok(city) => (StatusCode::OK, json!({
                          "query val" : qval.id,
                          "state val" : state.number,
                          "path val" : id,
                          "city" : city
                      })),
                      Err(e) => (StatusCode::BAD_REQUEST,json!({
                          "query val" : qval.id,
                          "state val" : state.number,
                          "path val" : id,
                          "error" : e.to_string()
                      }))
                };
                  (code, Json(resp))
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(PutOne)]
pub fn deriveputone(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_str = struct_name.to_string();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let idfield = match fields.named.first() {
        Some(field) => field,
        None => panic!("No fields"),
    };

    let type_name = idfield.ty.to_token_stream();

    let output = quote! {
        #[async_trait]
        impl crate::PutOneTrait<#struct_name> for #struct_name {

            type ExtensionType = Appstate;
            type PathType = #type_name;

            async fn putroot(Extension(state): Extension<Self::ExtensionType> ,Path(id): Path<Self::PathType>, jsonbody: Result<Json<#struct_name>, JsonRejection>) -> (StatusCode, Json<Value>) {

                let body = match jsonbody {
                    Ok(body) => body,
                    Err(err) => return (StatusCode::BAD_REQUEST, Json(serde_json::Value::from(format!("err : {}", err.body_text()))))
                };

                let stmt = crate::api::QueryBuilder::<#struct_name, Self::PathType>::Single(String::new(), None, None, axum::http::Method::GET);
                let rows = stmt.structname(#struct_str.to_string()).pkid(id).model(body.0).method(axum::http::Method::PUT).build(&state.pool).await;
                if rows.is_err() {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "state val" : state.number,
                        "path val" : id,
                    })))
                }
                let result = post_put_or_delete_one(rows.expect("No err"));

                let (code, resp) = match result {

                    Ok(_) => (StatusCode::NO_CONTENT, serde_json::json!({
                    })),

                    Err(err) => (StatusCode::BAD_REQUEST, serde_json::json!({
                        "err" : format!("{}", err)
                    }))
                };

                (code, Json(resp))
            }}
    };
    output.into()
}

#[proc_macro_derive(PostOne)]
pub fn derivepostone(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_str = struct_name.to_string();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let idfield = match fields.named.first() {
        Some(field) => field,
        None => panic!("No field"),
    };

    let type_name = idfield.ty.to_token_stream();

    let output = quote! {

        #[async_trait]
        impl crate::PostOneTrait<#struct_name> for #struct_name {

            type ExtensionType = Appstate;

            async fn postroot(Extension(state): Extension<Self::ExtensionType> , jsonbody: Result<Json<#struct_name>, JsonRejection>) -> (StatusCode, Json<Value>) {

                let body = match jsonbody {
                    Ok(body) => body,
                    Err(err) => return (StatusCode::BAD_REQUEST, Json(serde_json::Value::from(format!("err : {}", err.body_text()))))
                };

                let stmt = crate::api::QueryBuilder::<#struct_name, #type_name>::Single(String::new(), None, None, axum::http::Method::GET);

                let rows = stmt.structname(#struct_str.to_string()).model(body.0).method(axum::http::Method::POST).build(&state.pool).await;

                dbg!(&rows);

                if rows.is_err() {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "state val" : state.number,
                        "error" : rows.unwrap_err(),
                    })))
                }

                let result = post_put_or_delete_one(rows.expect("No err"));
                let (code, resp) = match result {

                    Ok(_) => (StatusCode::NO_CONTENT, serde_json::json!({
                    })),

                    Err(err) => (StatusCode::BAD_REQUEST, serde_json::json!({
                        "err" : format!("{}", err)
                    }))
                };

                (code, Json(resp))
            }}
    };
    output.into()
}

#[proc_macro_derive(DeleteOne)]
pub fn derivedeleteone(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_str = struct_name.to_string();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let idfield = match fields.named.first() {
        Some(field) => field,
        None => panic!("No fields"),
    };

    let type_name = idfield.ty.to_token_stream();

    let output = quote! {

        #[async_trait]
        impl crate::DeleteOneTrait for #struct_name {

            type ExtensionType = Appstate;
            type PathType = #type_name;

            async fn deleteroot(Extension(state): Extension<Self::ExtensionType> ,Path(id): Path<Self::PathType>) -> (StatusCode, Json<Value>) {

                let stmt = crate::api::QueryBuilder::<#struct_name, Self::PathType>::Single(String::new(), None, None, axum::http::Method::GET);
                let rows = stmt.structname(#struct_str.to_string()).pkid(id).method(axum::http::Method::DELETE).build(&state.pool).await;

                if rows.is_err() {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "state val" : state.number,
                        "path val" : id,
                    })))
                }
                let result = post_put_or_delete_one(rows.expect("No err"));

                let (code, resp) = match result {

                    Ok(_) => (StatusCode::NO_CONTENT, serde_json::json!({
                    })),

                    Err(err) => (StatusCode::BAD_REQUEST, serde_json::json!({
                        "err" : format!("{}", err)
                    }))
                };

                (code, Json(resp))
            }}
    };
    output.into()
 }

