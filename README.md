# `Etheryal WebAssembly Extensions` 💖

A framework for creating WebAssembly (WASI) modules for Etheryal extensions in the Rust programming language, enabling the creation of custom functionality like advanced universe generation, custom creature AI, registering commands, and more! All module extensions are loaded at runtime, and can be enabled or disabled at any time.

If you don't need advanced functionality, you can also create a simple extension with an `etheryal.toml` file and the [data-oriented extension system](https://docs.etheryal.net/extensions).

# Getting Started

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo WASI](https://bytecodealliance.github.io/cargo-wasi/install.html)

## Installation

1. Add the following to your `Cargo.toml` file:

   ```toml
   [dependencies]
   etheryal-extension = { git = "https://github.com/etheryal/extension-framework.git" }
   bevy_app = "0.11"
   bevy_ecs = "0.11"
   ```

2. Add the following to your `main.rs` file:

   ```rust, ignore
   use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
   use bevy_ecs::event::EventReader;
   use bevy_ecs::system::Res;
   use etheryal_extension::{
       EtheryalExtensionPlugin, ExtensionEvent, ExtensionGuest, ExtensionModuleInfo,
       Version, VersionReq
   };
   use etheryal_extension_common::message::debug::{Ping, Pong};
   use etheryal_extension_common::message::events::ShutdownHost;
   use etheryal_extension_common::ExtensionModuleDependency;
   use etheryal_identifier::{Identifier, NamespacedIdentifier};

   pub fn main() {
       let extension_info = ExtensionModuleInfo::builder()
           .name("Example Extension Module".into())
           .identifier(NamespacedIdentifier::new(
               "example".try_into().unwrap(),
               "extension_module".try_into().unwrap(),
           ))
           .version(Version::new(0, 1, 0))
           .dependencies(vec![ExtensionModuleDependency::builder()
               // Set the required Etheryal engine version
               .identifier(NamespacedIdentifier::new(
                   Identifier::default(),
                   Identifier::default(),
               ))
               .version(VersionReq::parse(">=0.1.0-nightly").unwrap())
               .build()])
           .build();
       App::new()
           .add_plugin(ScheduleRunnerPlugin::default())
           .add_plugin(EtheryalExtensionPlugin::new(extension_info))
           .add_systems(Startup, setup)
           .add_systems(Update, events)
           .run();
   }

   /// Send a ping message to the host
   /// when the extension is started
   fn setup(guest: Res<ExtensionGuest>) {
       println!("Extension guest started");
       guest.send_message(Ping).ok();
   }

   /// When the host sends a pong message,
   /// send a shutdown message back
   fn events(mut events: EventReader<ExtensionEvent<Pong>>, guest: Res<ExtensionGuest>) {
       for _ in events.iter() {
           println!("Received pong message");
           // This message will cause the Etheryal server to shutdown
           guest.send_message(ShutdownHost).ok();
       }
   }
   ```

3. Build your extension module:

   ```bash
   cargo wasi build --release
   ```

4. Go to the `extensions` directory of your Etheryal server and create a new directory for your extension module. Inside this directory, create a `etheryal.toml` file with the following contents:

   ```toml
   name = "Example Extension"
   id = "example_extension"
   description = "An example Etheryal extension with a WebAssembly module"
   author = "Your Name"
   version = "0.1.0"
   license = "MIT"
   ```

5. Copy your extension module to the directory you created in step 4. You can find your compiled `.wasm` module in the `target/wasm32-wasi/release` directory.

6. Start your Etheryal server and enable your extension module in the `extensions.toml` file.

# License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

# Contributing

When interacting with the open-source Etheryal projects, please follow our [Code of Conduct](CODE_OF_CONDUCT.md). If you'd like to contribute to this project, please read the [Contributing Guide](CONTRIBUTING.md).