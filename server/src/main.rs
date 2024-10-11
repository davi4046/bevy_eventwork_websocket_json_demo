use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{
    log::LogPlugin,
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::{
    managers::{network::Network, NetworkInstance},
    EventworkPlugin, EventworkRuntime, NetworkData, NetworkEvent,
};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use protocol::{ChatMessage, DespawnMessage, SpawnMessage};
use serde_json::EventworkSerdeJsonAppExt;

mod protocol;
mod serde_json;

fn main() {
    let mut app = App::new();

    app.add_plugins((MinimalPlugins, LogPlugin::default()));

    app.add_plugins(EventworkPlugin::<WebSocketProvider, TaskPool>::default());

    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    app.insert_resource(NetworkSettings::default());

    app.register_json_message::<SpawnMessage, WebSocketProvider>();
    app.register_json_message::<DespawnMessage, WebSocketProvider>();
    app.register_json_message::<ChatMessage, WebSocketProvider>();

    app.add_systems(Startup, setup_networking);

    app.add_systems(
        Update,
        (
            handle_connection_events,
            handle_disconnect_events,
            handle_player_spawning,
            handle_player_despawning,
            handle_chat_messages,
        ),
    );

    app.run();
}

fn setup_networking(
    mut net: ResMut<NetworkInstance<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    let ip_address = "127.0.0.1".parse().expect("Failed to pass ip address");

    let _socket_address = SocketAddr::new(ip_address, 8080);

    match net.listen(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
        &task_pool.0,
        &settings,
    ) {
        Ok(_) => {}
        Err(error) => {
            error!("Failed to start listening: {}", error);
            panic!();
        }
    }

    info!("Started listening for connections");
}

fn handle_connection_events(
    mut network_events: EventReader<NetworkEvent>,
    mut net: Network<WebSocketProvider>,
) {
    for event in network_events.read() {
        if let NetworkEvent::Connected(conn_id) = event {
            info!("User {} joined the server", conn_id.id);
            let _ = net.send_message(
                *conn_id,
                ChatMessage {
                    message: "hello".into(),
                },
            );
        }
    }
}

fn handle_disconnect_events(mut network_events: EventReader<NetworkEvent>) {
    for event in network_events.read() {
        if let NetworkEvent::Disconnected(conn_id) = event {
            info!("User {} left the server", conn_id.id)
        }
    }
}

fn handle_player_spawning(mut spawn_messages: EventReader<NetworkData<SpawnMessage>>) {
    for message in spawn_messages.read() {
        let conn_id = message.source();
        info!(
            "User {} wants to be spawned with name: {}",
            conn_id.id, message.player_name
        );
    }
}

fn handle_player_despawning(mut despawn_messages: EventReader<NetworkData<DespawnMessage>>) {
    for message in despawn_messages.read() {
        let conn_id = message.source();
        info!("User {} wants to be despawned", conn_id.id);
    }
}

fn handle_chat_messages(mut chat_messages: EventReader<NetworkData<ChatMessage>>) {
    for message in chat_messages.read() {
        let conn_id = message.source();
        info!("User {} says: {}", conn_id.id, message.message);
    }
}
