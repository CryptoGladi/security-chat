use std::env;
use std::path::Path;

fn main() {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");

    built::write_built_file_with_opts(
        &built::Options::default()
            .set_dependencies(true)
            .set_cfg(false)
            .set_ci(false)
            .set_compiler(false)
            .set_env(false)
            .set_features(false),
        src.as_ref(),
        &dst,
    )
    .expect("Failed to acquire build-time information");
}
