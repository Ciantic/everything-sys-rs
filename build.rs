use std::{env, path::Path};
use std::{fs::OpenOptions, io::Write};

fn main() {
    if cfg!(not(running_inside_docs_rs)) {
        let cargo_dir_str = &env::var("CARGO_MANIFEST_DIR").unwrap();
        let cargo_dir = Path::new(cargo_dir_str);
        let bindings_file = cargo_dir.join("src/lib.rs");
        let bindings_header = cargo_dir.join("src/bindings.h").display().to_string();

        // Build everything
        cc::Build::new()
            .file(cargo_dir.join("Everything/src/Everything.c"))
            .compile("Everything64");

        println!("cargo:rerun-if-changed={}", bindings_header);

        // Generate bindings.rs file
        let output = bindgen::Builder::default()
            .header(bindings_header)
            .blacklist_type("*")
            .whitelist_function("Everything.*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings")
            .to_string();

        let mut output_contents = r#"
        use winapi::shared::windef::*;
        use winapi::shared::minwindef::*;
        use winapi::um::winnt::*;
        "#
        .replace("        ", "");

        output_contents.push_str(&output);

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(bindings_file)
            .unwrap()
            .write_all(output_contents.as_bytes())
            .unwrap();
    }
}
