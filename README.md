# Minimal config.toml Rust Macro
Creates a structure and instantiate it with values from a chosen toml file or chosen default values.

## Features
The minimal but powerfull [rust marcro](https://doc.rust-lang.org/book/ch20-05-macros.html) to use a [toml file](https://toml.io/) as a config.  
You can chose a **config file** path, a **struct** and **default values** that will be uses if the key is not found in the config.

## Usage
```rust
let config = config_toml::Config!(
    "config.toml",                       // File Name
    {                                    // Struct and default values
      value1: String = String::from("Hello Rust"),
      value2: u32 = 7,
      value3: bool = false,
    }
);

println!("Value 1: {}", config.value1);  // Use a config value :)
```
Example config.toml file:
```toml
value1 = "192.168.1.1"
value2 = 4
value3 = true
```

## Notes

{Array} like values and [table] are not supportet at the moment.