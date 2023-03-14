use std::{thread, time::Duration};

use env_logger::{Builder, Env};
use log::{debug, info};
use tether_agent::TetherAgent;

fn main() {
    println!("Rust Tether Agent publish example");

    let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder.init();

    debug!("Debugging is enabled; could be verbose");

    let mut agent = TetherAgent::new("dummy", "example", None);

    agent.connect();
    agent.add_output_plug("emptyMessages", None, None);
    agent.add_output_plug("booleanMessages", None, None);

    for i in 1..10 {
        info!("#{i}: Sending empty message...");
        agent.publish_message("emptyMessages", None).unwrap();

        let bool = i % 2 == 0;
        info!("#{i}: Sending boolean message...");
        agent
            .publish_message("booleanMessages", Some(&[bool.into()]))
            .unwrap();

        thread::sleep(Duration::from_millis(1000))
    }
}
