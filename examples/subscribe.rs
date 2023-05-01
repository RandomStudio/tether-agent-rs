use std::{thread, time::Duration};

use env_logger::{Builder, Env};
use log::{debug, info};
use tether_agent::TetherAgent;

fn main() {
    println!("Rust Tether Agent subscribe example");

    let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder.init();

    debug!("Debugging is enabled; could be verbose");

    let mut agent = TetherAgent::new("dummy", "example", None);

    agent.connect();

    let dummy_plug = agent.create_input_plug("dummy", None, None).unwrap();

    let input_plugs = vec![&dummy_plug];

    info!("Checking messages every 1s, 10x...");

    for i in 1..10 {
        info!("#{i}: Checking for messages...");
        if let Some((plug_name, message)) = agent.check_messages(&input_plugs) {
            if &dummy_plug.name == plug_name.as_str() {
                println!(
                    "Received a message from plug named {} on topic {} with length {} bytes",
                    dummy_plug.name,
                    message.topic(),
                    message.payload().len()
                );
            } else {
                println!(
                    "This message with topic {} does not match the plug name {}",
                    message.topic(),
                    plug_name
                );
            }
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
