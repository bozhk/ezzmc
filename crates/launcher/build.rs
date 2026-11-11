fn main() {
    let cfg = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    if let Err(e) = slint_build::compile_with_config("ui/app.slint", cfg) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
