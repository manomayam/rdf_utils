pub mod models;
pub mod repr;
pub mod some_syntax_parser;

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    fn setup_simple_tracing() {
        if ! std::env::var("TEST_LOG").is_ok() {
            return;
        }
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    pub static TRACING: Lazy<()> = Lazy::new(|| {
        setup_simple_tracing();
    });
} 
