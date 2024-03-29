# Tuna 🍣

Tuna is an open-source music API designed to allow client-side automation and contributions, built with Rust, Rocket, and SQLite. It aims to provide a simple, fast, and reliable way to manage your music libraries.


> ### Work in Progress 🚧
>
> Tuna is currently under active development. All features and documentation are subject to change.


## Features 📦

- **User Management:** Tuna offers a comprehensive API for managing other users, invites, and permissions. So you can safely share your music library with friends and family (or your army of bots).

- **Music Management:** Tuna allows you to easily manage your music library, including adding, removing, and updating songs, albums, and artists.


## Quick Start 🚀

To get started, you'll need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

Then follow these steps to run Tuna on your local machine:

```bash
# Clone the repository
git clone https://github.com/5-pebbles/tuna.git tuna
# Change into the project directory
cd tuna
# Launch the server
cargo run
# And you're ready to go!
```


## Documentation 📚

> Not all endpoints are documented yet; well, actually, most aren't...

For detailed API documentation, including endpoints, parameters, and responses, please refer to the [OpenAPI documentation](./docs/openapi.json) or [YAML version](./docs/openapi.yaml).


## Testing 🧪

To run tests, you'll need to have [hurl](https://github.com/Orange-OpenSource/hurl) installed. Then simply run `cargo test` in your terminal.


## Contact 📫

For any questions, feedback, or support, please open an issue on [GitHub](https://github.com/5-pebbles/tuna/issues) or contact me at [5-pebble@protonmail.com](mailto:5-pebble@protonmail.com)
