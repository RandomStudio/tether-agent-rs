use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tether_agent::TetherAgent;

fn main() {
    println!("Rust Tether Agent subscribe example");

    let agent = Arc::new(Mutex::new(TetherAgent::new(
        "RustDemoAgent",
        Some("example"),
        None,
    )));

    match agent.try_lock() {
        Ok(a) => {
            a.connect().expect("failed to connect");
            a.create_input_plug("one", None, None)
                .expect("failed to create Input Plug");
        }
        Err(e) => {
            println!("Failed to acquire lock: {}", e);
        }
    };

    let receiver_agent = Arc::clone(&agent);
    let handle = thread::spawn(move || {
        println!("Checkig messages every 1s, 10x...");

        for i in 1..=10 {
            println!("#{i}: Checking messages...");
            match receiver_agent.try_lock() {
                Ok(a) => {
                    if let Some((topic, _message)) = a.check_messages() {
                        println!("<<<<<<<< CHECKING LOOP: Received a message on topic {topic}",);
                    }
                }
                Err(e) => {
                    println!("Failed to acquire lock: {}", e);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Doing some other random thing, every 2s, 10x");
    for i in 1..=10 {
        println!("#{i}: Main thread sleep...");
        thread::sleep(Duration::from_secs(2));
        println!("...Main thread #{i} loop done");
    }

    handle.join().expect("failed to join thread(s)");
}
