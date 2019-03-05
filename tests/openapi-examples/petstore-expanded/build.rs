fn main() {
    openapi_codegen::client("openapi.yaml", "src/petstore_expanded", true).unwrap();
}
