fn main() {
    let config = config_toml::Config!(
        "tests/config.toml",
        {
            example_value: String = String::from("No!"),
        }
    );
    println!("Value from config.toml?: {}", config.example_value);
}