fn main() {
    openapi_codegen::client("api-with-examples.yaml", "src/api_with_examples").unwrap();
    openapi_codegen::client("callback-example.yaml", "src/callback_example").unwrap();
    openapi_codegen::client("link-example.yaml", "src/link_example").unwrap();
    //openapi_codegen::client("petstore-expanded.yaml", "src/petstore_expanded").unwrap();
    openapi_codegen::client("petstore.yaml", "src/petstore").unwrap();
    //openapi_codegen::client("uspto.yaml", "src/uspto").unwrap();
}
