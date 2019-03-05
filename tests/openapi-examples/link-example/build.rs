fn main() {
    openapi_codegen::client("openapi.yaml", "src/link_example", true).unwrap();
}
