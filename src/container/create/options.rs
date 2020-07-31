use crate::container::create::host_config::HostConfig;
use crate::container::create::network_config::NetworkConfig;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use strum_macros::EnumString;

pub struct Options {
    fields: HashMap<String, Value>,
}
impl Options {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.fields).unwrap()
    }

    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn hostname(&mut self, hostname: &str) {
        self.fields.insert("Hostname".to_string(), json!(hostname));
    }

    pub fn domain_name(&mut self, domain_name: &str) {
        self.fields
            .insert("Domainname".to_string(), json!(domain_name));
    }

    pub fn user(&mut self, user: &str) {
        self.fields.insert("User".to_string(), json!(user));
    }

    pub fn attach_stdin(&mut self, attach: bool) {
        self.fields.insert("AttachStdin".to_string(), json!(attach));
    }

    pub fn attach_stderr(&mut self, attach: bool) {
        self.fields
            .insert("AttachStderr".to_string(), json!(attach));
    }

    pub fn expose_port(&mut self, port: u16, protocol: Protocol) {
        let formatted = format!("{}/{:?}", port, protocol);

        if let Some(exposed_ports) = self.fields.get_mut("ExposedPorts") {
            if let Some(exposed_ports) = exposed_ports.as_object_mut() {
                exposed_ports.insert(formatted, json!({}));
            }
        } else {
            let mut map = Map::new();
            map.insert(formatted, json!({}));
            self.fields.insert("ExposedPorts".to_string(), json!(map));
        }
    }

    pub fn tty(&mut self, tty: bool) {
        self.fields.insert("Tty".to_string(), json!(tty));
    }

    pub fn open_stdin(&mut self, open_stdin: bool) {
        self.fields
            .insert("OpenStdin".to_string(), json!(open_stdin));
    }

    pub fn stdin_once(&mut self, stdin_once: bool) {
        self.fields
            .insert("StdinOnce".to_string(), json!(stdin_once));
    }

    pub fn add_env(&mut self, key: &str, value: &str) {
        let formatted = format!("{}={}", key, value);

        if let Some(env) = self.fields.get_mut("Env") {
            if let Some(env) = env.as_array_mut() {
                env.push(json!(formatted));
            }
        } else {
            self.fields
                .insert("Env".to_string(), json!(vec![formatted]));
        }
    }

    pub fn cmd(&mut self, cmd: &str) {
        self.fields.insert("Cmd".to_string(), json!(cmd));
    }

    pub fn health_check(&mut self) {
        // todo
    }

    pub fn args_escaped(&mut self) {
        // todo
    }

    pub fn image(&mut self, image: &str) {
        self.fields.insert("Image".to_string(), json!(image));
    }

    pub fn volumes(&mut self) {
        // todo
    }

    pub fn working_dir(&mut self, working_dir: &str) {
        self.fields.insert(
            "WorkingDir".to_string(),
            Value::String(working_dir.to_string()),
        );
    }

    pub fn entry_point(&mut self) {
        // todo
    }

    pub fn network_disabled(&mut self, network_disabled: bool) {
        self.fields
            .insert("NetworkDisabled".to_string(), json!(network_disabled));
    }

    pub fn mac_address(&mut self, mac_address: &str) {
        self.fields
            .insert("MacAddress".to_string(), json!(mac_address));
    }

    pub fn on_build(&mut self) {
        // todo
    }

    pub fn labels(&mut self) {
        // todo
    }

    pub fn stop_signal(&mut self) {
        // todo
    }

    pub fn stop_timeout(&mut self) {
        // todo
    }

    pub fn shell(&mut self) {
        // todo
    }

    pub fn host_config(&mut self, host_config: HostConfig) {
        self.fields
            .insert("HostConfig".to_string(), json!(host_config.consume()));
    }

    pub fn network_config(&mut self, network_config: NetworkConfig) {
        self.fields
            .insert("NetworkConfig".to_string(), json!(network_config.consume()));
    }
}

#[derive(EnumString, Debug, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
    Sctp,
}
