use std::{thread, time::Duration};

use env_logger::{Builder, Env};
use log::debug;
use tether_agent_rs::TetherAgent;

fn main() {
    println!("Rust Tether Agent subscribe example");

    let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder.init();

    debug!("Debugging is enabled; could be verbose");

    let mut agent = TetherAgent::new("dummy", "example", None);

    agent.connect();
    agent.add_input_plug("dummy", None, None);

    for _i in 1..10 {
        if let Some(m) = agent.check_messages() {
            println!("Received a message!");
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
