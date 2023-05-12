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
        let interval = 0.3;
        let count = 30;
        println!("Checking messages every {interval}s, {count}x...");

        let mut count_messages = 0;

        for i in 1..=count {
            println!("#{i}: Checking messages...");
            match receiver_agent.try_lock() {
                Ok(a) => {
                    if let Some((topic, _message)) = a.check_messages() {
                        println!("<<<<<<<< CHECKING LOOP: Received a message on topic {topic}",);
                        count_messages += 1;
                        println!("<<<<<<<< CHECKING LOOP: Now has {count_messages} messages");
                    }
                }
                Err(e) => {
                    println!("CHECKING LOOP: Failed to acquire lock: {}", e);
                }
            }
            thread::sleep(Duration::from_secs_f32(interval));
        }
    });

    let sending_agent = Arc::clone(&agent);
    println!("Sending a message, every 2s, 10x");
    for i in 1..=10 {
        println!("#{i}: Main thread publish...");
        match sending_agent.try_lock() {
            Ok(a) => {
                let plug = a
                    .create_output_plug("one", None, None)
                    .expect("failed to create Output Plug");
                a.publish(&plug, Some(&[0])).expect("Failed to publish");
            }
            Err(e) => {
                println!("MAIN THREAD LOOP: Failed to acquire lock: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(2));
        println!("...Main thread #{i} loop done");
    }

    handle.join().expect("failed to join thread(s)");
}
