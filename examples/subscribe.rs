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
        if let Some(m) = agent.check_messages() {
            let topic = m.topic();
            let payload = m.payload();
            println!(
                "Received a message on topic {} with length {} bytes",
                topic,
                payload.len()
            );
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
