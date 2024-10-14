#[macro_export]
macro_rules! init_fli_from_toml {
    () => {{
        let mut app = $crate::Fli::init(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION")
        );
        app.set_version(env!("CARGO_PKG_VERSION"));
        app
    }};
}

