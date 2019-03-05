fn main() {
    openapi_codegen::client("openapi.yaml", "src/uspto", true).unwrap();
}
