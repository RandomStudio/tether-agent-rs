use std::{thread, time::Duration};

use env_logger::{Builder, Env};
use log::{debug, info};
use serde::Serialize;
use tether_agent::TetherAgent;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomStruct {
    foo: String,
    bar: f32,
}
fn main() {
    println!("Rust Tether Agent publish example");

    let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder.init();

    debug!("Debugging is enabled; could be verbose");

    let mut agent = TetherAgent::new("RustDemoAgent", None, None);

    agent.connect();

    let empty_message_output = agent
        .create_output_plug("emptyMessage", None, None)
        .unwrap();
    let boolean_message_output = agent
        .create_output_plug("booleanMessage", None, None)
        .unwrap();
    let custom_output = agent.create_output_plug("custom", None, None).unwrap();

    for i in 1..10 {
        info!("#{i}: Sending empty message...");
        agent.publish(&empty_message_output, None).unwrap();

        let bool = i % 2 == 0;
        info!("#{i}: Sending boolean message...");
        agent
            .publish(&boolean_message_output, Some(&[bool.into()]))
            .unwrap();

        let custom_message = CustomStruct {
            foo: "hello".into(),
            bar: 0.42,
        };
        agent
            .encode_and_publish(&custom_output, custom_message)
            .unwrap();

        thread::sleep(Duration::from_millis(1000))
    }
}
