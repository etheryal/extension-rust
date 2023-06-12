use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_ecs::event::EventReader;
use bevy_ecs::system::Res;
use etheryal_extension::common::message::debug::{Ping, Pong};
use etheryal_extension::common::message::events::ShutdownHost;
use etheryal_extension::prelude::*;
use etheryal_extension::semver::{Version, VersionReq};

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
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(EtheryalExtensionPlugin::new(extension_info))
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

/// This system will be called when the extension starts
fn setup(guest: Res<ExtensionGuest>) {
    println!("Extension guest started");

    // Send a ping message to the etheryal server
    guest.send_message(Ping).ok();
}

/// This system will be called when the extension receives a pong message from
/// the etheryal server
fn events(mut events: EventReader<ExtensionEvent<Pong>>, guest: Res<ExtensionGuest>) {
    for _ in events.iter() {
        println!("Received pong message");

        // Request the etheryal server to shutdown
        guest.send_message(ShutdownHost).ok();
    }
}
