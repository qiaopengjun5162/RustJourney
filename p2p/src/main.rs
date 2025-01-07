use libp2p::{
    futures::StreamExt, // 异步流有关
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("New Peer ID is: {:?}", new_peer_id);

    let transport = libp2p::development_transport(new_key).await?; // 使用密钥对创建传输

    let behaviour = Mdns::new(MdnsConfig::default()).await?; // 创建网络行为

    let mut swarm = Swarm::new(transport, behaviour, new_peer_id); // 创建Swarm
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local Address {:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }

            _ => {}
        }
    }
}
