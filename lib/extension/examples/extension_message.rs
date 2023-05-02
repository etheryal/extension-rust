use bevy_app::{App, ScheduleRunnerPlugin, Startup, Update};
use bevy_ecs::event::EventReader;
use bevy_ecs::system::Res;
use bevy_log::LogPlugin;
use etheryal_extension::{
    EtheryalExtensionPlugin, ExtensionEvent, ExtensionGuest, ExtensionModuleInfo,
};
use etheryal_extension_common::message::debug::{Ping, Pong};
use etheryal_extension_common::message::events::ShutdownHost;
use etheryal_extension_common::ExtensionModuleDependency;
use etheryal_identifier::{Identifier, NamespacedIdentifier};
use semver::{Version, VersionReq};
use tracing::info;

pub fn main() {
    let extension_info = ExtensionModuleInfo::builder()
        .name("Extension Message".into())
        .identifier(NamespacedIdentifier::new(
            Identifier::default(),
            "extension_message".try_into().unwrap(),
        ))
        .version(Version::new(0, 1, 0))
        .dependencies(vec![ExtensionModuleDependency::builder()
            .identifier(NamespacedIdentifier::new(
                Identifier::default(),
                Identifier::default(),
            ))
            .version(VersionReq::parse(">=0.1.0-nightly").unwrap())
            .build()])
        .build();

    App::new()
        .add_plugin(LogPlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(EtheryalExtensionPlugin::new(extension_info))
        .add_systems(Startup, setup)
        .add_systems(Update, events)
        .run();
}

fn setup(guest: Res<ExtensionGuest>) {
    info!("Extension guest started");
    guest.send_message(Ping).ok();
}

fn events(mut events: EventReader<ExtensionEvent<Pong>>, guest: Res<ExtensionGuest>) {
    for _ in events.iter() {
        info!("Received pong message");
        guest.send_message(ShutdownHost).ok();
    }
}
