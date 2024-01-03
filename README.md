# Template Wal project

This is a template project designed to help initialize new projects which are using [Wal](https://github.com/walrust/wal) library.

## Project initialization

Code can be copied directly from this repository or it can be geneated using [Cargo Generate](https://github.com/cargo-generate/cargo-generate).

### Initialization with Cargo Generate

To install Cargo Generate call the following command:

```
cargo install cargo-generate
```

Then initialize the project folder using:

```
cargo generate --git https://github.com/walrust/template
```

You will be prompted for the project name: f.e **_tutorial-app_**. After the generation is completed You can navigate int the main project folder:

```
cd ./tutorial-app
```

## Wasm-bindgen-cli and Trunk initialization

Before the app will be ready to run some additional tools are required:

1. **wasm-bindgen-cli**, which is required for the Rust compilation to WebAssembly. 
2. **trunk**, which will host the application on localhost and will provide the hot-reload function during development.

To install both tools run the following commands:

```
cargo install --locked wasm-bindgen-cli
cargo install trunk
```

## Run the application

After installation the app is ready to go. You can launch it by typing:

```
cargo build
trunk serve
