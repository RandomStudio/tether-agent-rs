use log::{debug, error, info};
use mqtt::{Client, Message, MessageBuilder, Receiver};
pub use paho_mqtt as mqtt;
pub use rmp_serde;
use std::{
    net::{IpAddr, Ipv4Addr},
    process,
    time::Duration,
};

#[derive(Debug)]
pub struct Plug {
    name: String,
    topic: String,
    qos: i32,
}

impl Plug {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn topic(&self) -> &str {
        &self.topic
    }
    pub fn qos(&self) -> i32 {
        self.qos
    }
}

pub struct TetherAgent {
    role: String,
    group: String,
    client: Client,
    receiver: Receiver<Option<Message>>,
    input_plugs: Vec<Plug>,
    output_plugs: Vec<Plug>,
}

impl TetherAgent {
    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub fn new(agent_role: &str, agent_group: &str, tether_host: Option<IpAddr>) -> Self {
        let tether_host = tether_host.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        let broker_uri = format!("tcp://{tether_host}:1883");

        info!("Attempt connection broker at {}", &broker_uri);

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(broker_uri)
            .client_id("")
            .finalize();

        // Create the client connection
        let client = mqtt::Client::new(create_opts).unwrap();

        // Initialize the consumer before connecting
        let receiver = client.start_consuming();

        TetherAgent {
            role: String::from(agent_role),
            group: String::from(agent_group),
            client,
            receiver,
            input_plugs: Vec::new(),
            output_plugs: Vec::new(),
        }
    }

    pub fn connect(&self) {
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .user_name("tether")
            .password("sp_ceB0ss!")
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(true)
            .finalize();

        // Make the connection to the broker
        info!("Connecting to the MQTT server...");

        match self.client.connect(conn_opts) {
            Ok(res) => {
                info!("Connected OK: {res:?}");
            }
            Err(e) => {
                error!("Error connecting to the broker: {e:?}");
                process::exit(1);
            }
        }
    }

    // TODO: return a Result with the Plug
    // TODO: use Builder pattern instead of "optional" params?
    pub fn add_input_plug(&mut self, name: &str, qos: Option<i32>, override_topic: Option<&str>) {
        let name = String::from(name);
        let topic = String::from(override_topic.unwrap_or(&default_subscribe_topic(&name)));
        let qos = qos.unwrap_or(1);

        match self.client.subscribe(&topic, qos) {
            Ok(_res) => {
                info!("Subscribed to topic {} OK", &topic);
                let plug = Plug { name, topic, qos };
                debug!("Adding plug: {:?}", &plug);
                self.input_plugs.push(plug);
            }
            Err(e) => {
                error!("Error subscribing to topic {}: {:?}", &topic, e);
            }
        }
    }

    // TODO: return a Result with the Plug?
    // TODO: use Builder pattern instead of "optional" params?
    pub fn add_output_plug(&mut self, name: &str, qos: Option<i32>, override_topic: Option<&str>) {
        let name = String::from(name);
        let topic =
            String::from(override_topic.unwrap_or(&build_topic(&self.role, &self.group, &name)));
        let qos = qos.unwrap_or(1);

        let plug = Plug { name, topic, qos };
        debug!("Adding output plug: {:?}", &plug);
        self.output_plugs.push(plug);
    }

    // type MessageFromInputPlug = (&[u8], &str, &Plug);

    pub fn check_messages(&self) -> Option<(Message, &Plug)> {
        if let Some(message) = self.receiver.try_iter().find_map(|m| m) {
            let topic = message.topic();
            if let Some(plug) = self
                .input_plugs
                .iter()
                .find(|p| p.name.eq(parse_plug_name(topic)))
            {
                Some((message, plug))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn publish_message(&self, plug_name: &str, payload: Option<&[u8]>) -> Result<(), ()> {
        if let Some(plug) = self.output_plugs.iter().find(|p| p.name.eq(plug_name)) {
            let message = MessageBuilder::new()
                .topic(&plug.topic)
                .payload(payload.unwrap_or(&[]))
                .qos(plug.qos)
                .finalize();
            if let Err(e) = self.client.publish(message) {
                error!("Error publishing: {:?}", e);
                Err(())
            } else {
                Ok(())
            }
        } else {
            error!("Could not find matching output plug named {}", plug_name);
            Err(())
        }
    }
}

fn parse_plug_name(topic: &str) -> &str {
    let parts: Vec<&str> = topic.split('/').collect();
    parts[2]
}

pub fn parse_agent_id(topic: &str) -> &str {
    let parts: Vec<&str> = topic.split('/').collect();
    parts[1]
}

pub fn build_topic(agent_role: &str, agent_group: &str, plug_name: &str) -> String {
    format!("{agent_role}/{agent_group}/{plug_name}")
}

pub fn default_subscribe_topic(plug_name: &str) -> String {
    format!("+/+/{plug_name}")
}
