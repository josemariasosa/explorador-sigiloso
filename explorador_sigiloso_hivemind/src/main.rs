use futures::StreamExt;
use libp2p::{
    identity, PeerId,
    gossipsub::{
        Gossipsub, GossipsubEvent, GossipsubMessage, MessageAuthenticity,
        MessageId, ValidationMode, IdentTopic as Topic,
    },
    swarm::Swarm,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    noise,
    yamux, tcp::TokioTcpConfig, Transport,
    core::upgrade,
    swarm::NetworkBehaviour,
    dns::TokioDnsConfig,
    Multiaddr, NetworkBehaviour,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[tokio::main]
async fn main() {
    env_logger::init();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🐿️ Hivemind starting as {}", local_peer_id);

    // Create gossip topic
    let topic = Topic::new("forest-status");

    // Build gossipsub
    let mut gossipsub = Gossipsub::new(
        MessageAuthenticity::Signed(local_key.clone()),
        libp2p::gossipsub::GossipsubConfigBuilder::default()
            .validation_mode(ValidationMode::Permissive)
            .build()
            .expect("valid config"),
    ).unwrap();
    gossipsub.subscribe(&topic).unwrap();

    // mDNS for local peer discovery
    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();

    #[derive(NetworkBehaviour)]
    struct ForestBehaviour {
        gossipsub: Gossipsub,
        mdns: Mdns,
    }

    let behaviour = ForestBehaviour { gossipsub, mdns };

    // Create transport
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&local_key).unwrap())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let mut swarm = Swarm::with_tokio_executor(transport, behaviour, local_peer_id);

    // Listen on random TCP port
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Periodic message
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let msg = serde_json::json!({
                    "node_id": local_peer_id.to_string(),
                    "eth_block": 999,
                    "btc_block": 847394,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }).to_string();

                swarm.behaviour_mut().gossipsub.publish(topic.clone(), msg.as_bytes()).unwrap();
            }
            event = swarm.select_next_some() => match event {
                libp2p::swarm::SwarmEvent::Behaviour(ForestBehaviour {
                    gossipsub: GossipsubEvent::Message { message, .. },
                    ..
                }) => {
                    if let Ok(payload) = String::from_utf8(message.data.clone()) {
                        println!("🛰️ Gossip received: {}", payload);
                    }
                }
                libp2p::swarm::SwarmEvent::Behaviour(ForestBehaviour {
                    mdns: MdnsEvent::Discovered(peers),
                    ..
                }) => {
                    for (peer, _) in peers {
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer);
                    }
                }
                _ => {}
            }
        }
    }
}
