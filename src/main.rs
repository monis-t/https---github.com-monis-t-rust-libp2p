use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;


//Creating a network identity for our local node in async.

#[async_std::main]  

async fn main() -> Result< () , Box<dyn Error> > {

    let local_key = identity::Keypair::generate_ed25519();
    // ^ Setting local key for peer.
    let local_peer_id = PeerId::from(local_key.public());
    // ^ Setting local peer id.
    println!(" Local peer id: {:?}", local_peer_id);
    // ^ Printing peer id. run and test the program at this point.
    let transport = libp2p::development_transport(local_key).await?;
    // ^ Constructing a transport.

    // v Creating a ping network beahviour.
    // Ping protocol is comfigured for keeping connection
    // alive.
    // Remember to import ping, pingconfig from libp2p.
    
    let ping_behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    // v adding functionality to allow transport and 
    // behaviour to communicate with each other.

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // swarm setsups communication between behaviour and transport.
    // mutliaddr helps in establishing connection to peers.

    // telling swarm to listen to all interfaces and OS assigned ports.

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi address given as the
    // second command line argument, if any:

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!( " Dialed{:?} ", addr);
    }

    // with everything in place now. We put the swarm in a loop.
    // this is done so that swarm can be looking for incoming communications
    // and establish an outgoing connection in case we specify address in cli.


    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {{}}
        }
    }

}



