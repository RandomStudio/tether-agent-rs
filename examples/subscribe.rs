use std::{thread, time::Duration};

use env_logger::{Builder, Env};
use log::{debug, info};
use tether_agent_rs::TetherAgent;

fn main() {
    println!("Rust Tether Agent subscribe example");

    let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder.init();

    debug!("Debugging is enabled; could be verbose");

    let mut agent = TetherAgent::new("dummy", "example", None);

    agent.connect();
    agent.add_input_plug("dummy", None, None);

    info!("Checking messages every 1s, 10x...");

    for i in 1..10 {
        info!("#{i}: Checking for messages...");
        if let Some((message, plug)) = agent.check_messages() {
            let topic = message.topic();
            let payload = message.payload();
            println!(
                "Received a message from plug named {} on topic {} with length {} bytes",
                &plug.name(),
                topic,
                payload.len()
            );
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
