fn main() {
    openapi_codegen::client("openapi.yaml", "src/petstore", true).unwrap();
}
