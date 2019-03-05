use std::process::Command;

fn main() {
    openapi_codegen::client("openapi.yaml", "src/callback_example", true).unwrap();

    Command::new("docker")
            .args(&["build", "-t=test-apisprout", "."])
            .output()
            .expect("failed to execute process");
}
