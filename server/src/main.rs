use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{
    log::LogPlugin,
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::{EventworkRuntime, Network, NetworkData, NetworkEvent};
use bevy_eventwork_mod_websockets::{NetworkSettings, WebSocketProvider};
use protocol::client_to_server::{ChatMessage, DespawnMessage, SpawnMessage};
use serializer::JsonSerializer;

mod protocol;
mod serializer;


fn main() {
    use bevy_eventwork::AppNetworkMessage;

    let mut app = App::new();

    app.add_plugins((MinimalPlugins, LogPlugin::default()));

    app.add_plugins(bevy_eventwork::EventworkPlugin::<
        JsonSerializer,
        WebSocketProvider,
        bevy::tasks::TaskPool,
    >::default());

    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    app.insert_resource(NetworkSettings::default());

    app.listen_for_message::<SpawnMessage, WebSocketProvider, JsonSerializer>();
    app.listen_for_message::<DespawnMessage, WebSocketProvider, JsonSerializer>();
    app.listen_for_message::<ChatMessage, WebSocketProvider, JsonSerializer>();

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

    app.run()
}

fn setup_networking(
    mut net: ResMut<Network<WebSocketProvider, JsonSerializer>>,
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
    net: ResMut<Network<WebSocketProvider, JsonSerializer>>,
) {
    for event in network_events.read() {
        if let NetworkEvent::Connected(conn_id) = event {
            info!("User {} joined the server", conn_id.id);
            net.broadcast(ChatMessage {
                message: "hello".into(),
            })
        }
    }
}

fn handle_disconnect_events(
    mut network_events: EventReader<NetworkEvent>,
) {
    for event in network_events.read() {
        if let NetworkEvent::Disconnected(conn_id) = event {
            info!("User {} left the server", conn_id.id)
        }
    }
}

fn handle_player_spawning(
    mut spawn_messages: EventReader<NetworkData<SpawnMessage>>,
) {
    for message in spawn_messages.read() {
        let conn_id = message.source();
        info!(
            "User {} wants to be spawned with name: {}",
            conn_id.id, message.player_name
        );
    }
}

fn handle_player_despawning(
    mut despawn_messages: EventReader<NetworkData<DespawnMessage>>,
) {
    for message in despawn_messages.read() {
        let conn_id = message.source();
        info!("User {} wants to be despawned", conn_id.id);
    }
}

fn handle_chat_messages(
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
) {
    for message in chat_messages.read() {
        let conn_id = message.source();
        info!("User {} says: {}", conn_id.id, message.message);
    }
}
