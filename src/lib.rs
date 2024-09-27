use std::path::Path;


pub fn include_gsettings_dev(schema: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let xdg_data_dir = out_dir.join("share");
    let schema_dir = xdg_data_dir.join("glib-2.0/schemas");

    if !schema_dir.exists() {
        std::fs::create_dir_all(&schema_dir).unwrap();
    }

    println!(
        "cargo::rustc-env=INCLUDE_GSETTINGS_DEV_XDG_DATA_DIRS={}",
        xdg_data_dir.display()
    );

    install_gsettings(schema, schema_dir.to_str().unwrap());
}

pub fn install_gsettings(schema: &str, schema_dir: &str) {
    println!("cargo:rerun-if-changed={}", schema);
    let schema_dir = Path::new(schema_dir);

    if !schema_dir.exists() {
        std::fs::create_dir_all(&schema_dir).unwrap();
    }

    let dest_file = schema_dir.join(schema);
    std::fs::copy(schema, dest_file).unwrap();

    std::process::Command::new("glib-compile-schemas")
        .arg(schema_dir)
        .output()
        .unwrap();
}


pub fn load_gsettings_schema() {
    if let Some(path) = option_env!("INCLUDE_GSETTINGS_DEV_XDG_DATA_DIRS") {
        std::env::set_var(
            "XDG_DATA_DIRS",
            format!(
                "{}:{}",
                path,
                std::env::var("XDG_DATA_DIRS").unwrap_or_default()
            ),
        );
    }
}
