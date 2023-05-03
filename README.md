# `etheryal WebAssembly Extensions` 💖

A framework for creating WebAssembly (WASI) modules for etheryal extensions in the Rust programming language, enabling the creation of custom functionality like advanced universe generation, custom creature AI, registering commands, and more! All module extensions are loaded at runtime, and can be enabled or disabled at any time.

If you don't need advanced functionality, you can also create a simple extension with an `etheryal.toml` file and the [data-oriented extension system](https://docs.etheryal.net/extensions).

# Getting Started

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo WASI](https://bytecodealliance.github.io/cargo-wasi/install.html)

## Installation

1. Create a new Rust project:

   ```bash
   cargo new my-extension
   ```

2. Add the following to your `Cargo.toml` file:

   ```toml
   [dependencies]
   etheryal-extension = { git = "https://github.com/etheryal/extension-framework.git" }
   bevy_app = "0.11"
   bevy_ecs = "0.11"
   ```

3. Add the following to your `main.rs` file (see the full example [here](examples/example_extension.rs)):

   ```rust
    pub fn main() {
        // Create the extension module info, which will be used to register the
        // extension module with the etheryal Server
        let extension_info = ExtensionModuleInfo::builder()
            .name("Example Extension Module".into())
            .identifier(NamespacedIdentifier::try_from("example:extension_module").unwrap())
            .version(Version::new(0, 1, 0))
            .dependencies(vec![
                // Require a specific version of the etheryal Server
                ExtensionModuleDependency::builder()
                    .identifier(NamespacedIdentifier::new(
                        // The namespace of the etheryal Server
                        Identifier::default(),
                        // The identifier of the etheryal Server
                        Identifier::default(),
                    ))
                    .version(VersionReq::parse(">=0.1.0-nightly").unwrap())
                    .build(),
            ])
            .build();
        App::new()
            .add_plugin(LogPlugin::default())
            .add_plugin(ScheduleRunnerPlugin::default())
            .add_plugin(etheryalExtensionPlugin::new(extension_info))
            .add_systems(Startup, setup)
            .add_systems(Update, events)
            .run();
    }

    /// This system will be called when the extension starts
    fn setup(guest: Res<ExtensionGuest>) {
        info!("Extension guest started");

        // Send a ping message to the etheryal server
        guest.send_message(Ping).ok();
    }

    /// This system will be called when the extension receives a pong message from
    /// the etheryal server
    fn events(mut events: EventReader<ExtensionEvent<Pong>>, guest: Res<ExtensionGuest>) {
        for _ in events.iter() {
            info!("Received pong message");

            // Request the etheryal server to shutdown
            guest.send_message(ShutdownHost).ok();
        }
    }
   ```

4. Build your extension module:

   ```bash
   cargo wasi build --release
   ```

5. Go to the `extensions` directory of your etheryal server and create a new directory for your extension module. Inside this directory, create a `etheryal.toml` file with the following contents:

   ```toml
   name = "Example Extension"
   id = "example_extension"
   description = "An example etheryal extension with a WebAssembly module"
   author = "Your Name"
   version = "0.1.0"
   license = "MIT"
   ```

6. Copy your extension module to the directory you created in step 4. You can find your compiled `.wasm` module in the `target/wasm32-wasi/release` directory.

7. Start your etheryal server and enable your extension module in the `extensions.toml` file.

# License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

# Contributing

When interacting with the open-source etheryal projects, please follow our [Code of Conduct](CODE_OF_CONDUCT.md). If you'd like to contribute to this project, please read the [Contributing Guide](CONTRIBUTING.md).
