use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if env::var("DOCS_RS").is_err() {
        println!("cargo:rerun-if-changed=static/app.ts");
        println!("cargo:rerun-if-changed=static/tsconfig.json");

        let static_dir = Path::new("static");
        let dist_dir = static_dir.join("dist");

        if !dist_dir.exists() {
            std::fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");
        }

        let status = Command::new("npx")
            .args(["tsc", "--project", "static/tsconfig.json"])
            .status();

        match status {
            Ok(exit_status) => {
                if !exit_status.success() {
                    panic!("TypeScript compilation failed");
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run TypeScript compiler: {}", e);
                println!("cargo:warning=Make sure Node.js and TypeScript are installed");
            }
        }
    }
}
