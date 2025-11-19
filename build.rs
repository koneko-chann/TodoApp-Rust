use std::env;

fn main() {
    if let Ok(val) = env::var("BACKEND_URL") {
        println!("cargo:rustc-env=BACKEND_URL={}", val);
        return;
    }

    if let Ok(iter) = dotenvy::from_path_iter(".env") {
        for entry in iter.flatten() {
            if entry.0 == "BACKEND_URL" {
                println!("cargo:rustc-env=BACKEND_URL={}", entry.1);
                break;
            }
        }
    }
}
