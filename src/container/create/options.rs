use crate::container::create::host_config::HostConfig;
use crate::container::create::networking_config::NetworkingConfig;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug)]
pub struct Options {
    fields: HashMap<&'static str, Value>,
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
        self.fields.insert("Hostname", json!(hostname));
    }

    pub fn domain_name(&mut self, domain_name: &str) {
        self.fields.insert("Domainname", json!(domain_name));
    }

    pub fn user(&mut self, user: &str) {
        self.fields.insert("User", json!(user));
    }

    pub fn attach_stdin(&mut self, attach: bool) {
        self.fields.insert("AttachStdin", json!(attach));
    }

    pub fn attach_stderr(&mut self, attach: bool) {
        self.fields.insert("AttachStderr", json!(attach));
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
            self.fields.insert("ExposedPorts", json!(map));
        }
    }

    pub fn tty(&mut self, tty: bool) {
        self.fields.insert("Tty", json!(tty));
    }

    pub fn open_stdin(&mut self, open_stdin: bool) {
        self.fields.insert("OpenStdin", json!(open_stdin));
    }

    pub fn stdin_once(&mut self, stdin_once: bool) {
        self.fields.insert("StdinOnce", json!(stdin_once));
    }

    pub fn add_env(&mut self, key: &str, value: &str) {
        let formatted = format!("{}={}", key, value);

        if let Some(env) = self.fields.get_mut("Env") {
            if let Some(env) = env.as_array_mut() {
                env.push(json!(formatted));
            }
        } else {
            self.fields.insert("Env", json!(vec![formatted]));
        }
    }

    pub fn cmd(&mut self, cmd: &str) {
        self.fields.insert("Cmd", json!(cmd));
    }

    pub fn health_check(&mut self) {
        // todo
    }

    pub fn args_escaped(&mut self) {
        // todo
    }

    pub fn image(&mut self, image: &str) {
        self.fields.insert("Image", json!(image));
    }

    pub fn volumes(&mut self) {
        // todo
    }

    pub fn working_dir(&mut self, working_dir: &str) {
        self.fields
            .insert("WorkingDir", Value::String(working_dir.to_string()));
    }

    pub fn entry_point(&mut self) {
        // todo
    }

    pub fn network_disabled(&mut self, network_disabled: bool) {
        self.fields
            .insert("NetworkDisabled", json!(network_disabled));
    }

    pub fn mac_address(&mut self, mac_address: &str) {
        self.fields.insert("MacAddress", json!(mac_address));
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
            .insert("HostConfig", json!(host_config.consume()));
    }

    pub fn networking_config(&mut self, networking_config: NetworkingConfig) {
        self.fields
            .insert("NetworkingConfig", json!(networking_config.consume()));
    }
}

#[derive(EnumString, Debug, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
    Sctp,
}

#[cfg(test)]
mod tests {
    use crate::container::create::host_config::HostConfig;
    use crate::container::create::networking_config::{
        EndpointSettings, EndpointsConfig, NetworkingConfig,
    };
    use crate::container::create::options::Options;
    use crate::network::NetworkMode;

    #[test]
    fn test() {
        let mut options = Options::new();
        options.image("9cdb6dec40d33ee2329ae6fc70158c9712cb8d66ebfe77da5a3069519850c17a");
        options.hostname("tfb-server");
        options.domain_name("tfb-server");

        let mut host_config = HostConfig::new();
        host_config.network_mode(NetworkMode::Bridge);
        host_config.publish_all_ports(true);

        options.host_config(host_config);

        let mut endpoint_settings = EndpointSettings::new();
        endpoint_settings.alias("tfb-server");
        endpoint_settings.network_id("aa126a3a0c13");

        options.networking_config(NetworkingConfig {
            endpoints_config: EndpointsConfig { endpoint_settings },
        });

        options.tty(true);

        eprintln!("{}", options.to_json());
    }
}
