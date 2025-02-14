# OpenStreetMap API Rust Binding

A Rust binding for the [OpenStreetMap API](https://wiki.openstreetmap.org/wiki/API#REST_specifications_for_the_editing_API) that allows you to interact with OpenStreetMap data programmatically. This crate aims to provide a simple and efficient way to access and manipulate OSM data in your Rust projects.
**NOTE: This project is still on early stages of development, and many function are not yet implemented. If you interested in contributing, please feel free to do so.**

## Features

- **Rust-friendly API:** Convenient Rust interface for interacting with OSM data.
- **XML Parsing:** Built-in support for parsing and serializing OSM XML data.
- **Extensible:** Designed to be extended with additional API functions and authentication support.

## API Documentation

- **Official OSM API Documentation:**  
  [OpenStreetMap API v0.6](https://wiki.openstreetmap.org/wiki/API_v0.6)

## Installation

Add this crate to your `Cargo.toml`:

```sh
cargo add osm-api
```

## Usage Example

Below is a simple example demonstrating how to fetch and display OSM data:

```rust
extern crate osm_api;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "some_endpoint" with an actual API endpoint.
    let response = osm_api::get_data("some_endpoint")?;
    println!("{:#?}", response);
    Ok(())
}
```

## Current TODO

- [ ] Implement tests
- [ ] Add functions for direct access to the API
- [ ] Integrate OAuth features for authentication
- [ ] Improve overall documentation

## Version Compatibility

| API version | Rust binding version |
|-------------|----------------------|
| 0.6         | 0.1.*                |


## Contributing

Contributions are welcome! If you encounter issues or have ideas for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE-MIT) OR [APACHE-2.0 License](LICENSE-APACHE).
