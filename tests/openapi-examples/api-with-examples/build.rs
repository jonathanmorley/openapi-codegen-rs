fn main() {
    openapi_codegen::client("openapi.yaml", "src/api_with_examples", true).unwrap();
}
