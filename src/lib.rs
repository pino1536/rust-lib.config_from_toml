/// Creates a structure and instantiate it with values from a chosen toml file or chosen default values.
/// 
/// # Example
/// ```
/// let config = config_from_toml::config_from_toml!(
///     "config.toml",                       // File Name
///     {                                    // Struct and default values
///         value1: String = String::from("Standardpfad"),
///         value2: u32 = 7,
///         value3: bool = false,
///     }
/// );
/// ```

#[macro_export]
macro_rules! config_from_toml {
    ($file:expr, { $($key:ident: $typ:ty = $default:expr),* $(,)? }) => {{
        
        struct Config {
            $( pub $key: $typ, )*
        }

        let mut map = std::collections::HashMap::new();
        if let Ok(file) = std::fs::File::open($file) {
            for line in std::io::BufRead::lines(std::io::BufReader::new(file)).flatten() {
                let line = line.trim();
                if line.starts_with('#') || !line.contains('=') { continue; }
                
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let k = parts[0].trim();
                    let v = parts[1].trim().trim_matches('"');
                    map.insert(String::from(k), String::from(v));
                }
            }
        }

        Config {
            $( $key: map.get(stringify!($key)).and_then(|v| v.parse::<$typ>().ok()).unwrap_or($default), )*
        }
    }};
}
