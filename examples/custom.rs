use bevy::prelude::*;
use bevy_osc::{OscEvent, OscSettings, Osc};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Osc)
        .insert_resource(OscSettings {
            max_log_packets: 20,
            recv_addr: Some("127.0.0.1:1234"),
            log: false,
            ..Default::default()
        })
        .add_system(event_listener_system.system())
        .run();
}

//Make events a type param?
fn event_listener_system(mut events: EventReader<OscEvent>) {
    for my_event in events.iter() {
        info!("My custom osc handler: {:?}", my_event.packet);
    }
}
