use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::new().with_style("cosmic-dark".to_string());
    slint_build::compile_with_config("ui/app.slint", config).expect("build slint failed");
}
