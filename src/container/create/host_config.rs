use crate::network::NetworkMode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit<'a> {
    pub name: &'a str,
    pub soft: u32,
    pub hard: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    fields: HashMap<String, Value>,
}
impl HostConfig {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn consume(self) -> HashMap<String, Value> {
        self.fields
    }

    pub fn cpu_shares(&mut self, weight: u32) {
        self.fields.insert("CpuShares".to_string(), json!(weight));
    }

    pub fn memory(&mut self, memory: u64) {
        self.fields.insert("Memory".to_string(), json!(memory));
    }

    pub fn cgroup_parent(&mut self) {
        // todo
    }

    pub fn blkio_weight(&mut self) {
        // todo
    }

    pub fn blkio_device(&mut self) {
        // todo
    }

    pub fn blkio_device_read_bps(&mut self) {
        // todo
    }

    pub fn blkio_device_write_bps(&mut self) {
        // todo
    }

    pub fn blkio_device_read_i_ops(&mut self) {
        // todo
    }

    pub fn blkio_device_write_i_ops(&mut self) {
        // todo
    }

    pub fn cpu_period(&mut self, _length_of_period: i64) {
        // todo
    }

    pub fn cpu_quoate(&mut self, _micro: i64) {
        // todo
    }

    pub fn cpu_realtime_period(&mut self, _micro: i64) {
        // todo
    }

    pub fn cpu_realtime_runtime(&mut self, _micro: i64) {
        // todo
    }

    pub fn cpuset_cpus(&mut self, _cpus: &str) {
        // todo
    }

    pub fn cpuset_memset(&mut self, _memory_nodes: &str) {
        // todo
    }

    pub fn devices(&mut self) {
        // todo
    }

    pub fn device_cgroup_rules(&mut self) {
        // todo
    }

    pub fn device_requests(&mut self) {
        // todo
    }

    pub fn kernel_memory(&mut self, _memory: i64) {
        // todo
    }

    pub fn kernel_memory_tcp(&mut self, _memory: i64) {
        // todo
    }

    pub fn memory_reservation(&mut self, _memory: i64) {
        // todo
    }

    pub fn memory_swap(&mut self, _memory_and_swap: i64) {
        // todo
    }

    pub fn memory_swappiness(&mut self, _swappiness: i64) {
        // todo
    }

    pub fn nano_cpus(&mut self, _nano_cpus: i64) {
        // todo
    }

    pub fn oom_kill_disable(&mut self, _disable_oom_killer: bool) {
        // todo
    }

    pub fn init(&mut self, _init: bool) {
        // todo
    }

    pub fn pids_limit(&mut self, _limit: i64) {
        // todo
    }

    pub fn ulimits(&mut self, ulimits: Vec<Ulimit>) {
        self.fields.insert("Ulimits".to_string(), json!(ulimits));
    }

    pub fn cpu_count(&mut self, _count: i64) {
        // todo
    }

    pub fn cpu_percent(&mut self, _percent: i64) {
        // todo
    }

    pub fn io_maximum_iops(&mut self, _iops: i64) {
        // todo
    }

    pub fn io_maximum_bandwidth(&mut self, _bytes_per_second: i64) {
        // todo
    }

    pub fn binds(&mut self) {
        // todo
    }

    pub fn container_id_file(&mut self) {
        // todo
    }

    pub fn log_config(&mut self) {
        // todo
    }

    pub fn network_mode(&mut self, network_mode: NetworkMode) {
        self.fields
            .insert("NetworkMode".to_string(), json!(network_mode.to_string()));
    }

    pub fn port_binding(&mut self, host_ip: &str, host_port: &str) {
        let mut port_binding = HashMap::new();
        port_binding.insert(host_ip, host_port);

        if let Some(port_bindings) = self.fields.get_mut("PortBindings") {
            if let Some(port_bindings) = port_bindings.as_array_mut() {
                port_bindings.push(json!(port_binding));
            }
        } else {
            self.fields
                .insert("PortBindings".to_string(), json!(vec![port_binding]));
        }
    }

    pub fn restart_policy(&mut self) {
        // todo
    }

    pub fn auto_remove(&mut self, auto_remove: bool) {
        self.fields
            .insert("AutoRemove".to_string(), json!(auto_remove));
    }

    pub fn volume_driver(&mut self) {
        // todo
    }

    pub fn volumes_from(&mut self) {
        // todo
    }

    pub fn mounts(&mut self) {
        // todo
    }

    pub fn capabilites(&mut self) {
        // todo
    }

    pub fn cap_add(&mut self) {
        // todo
    }

    pub fn cap_drop(&mut self) {
        // todo
    }

    pub fn dns(&mut self) {
        // todo
    }

    pub fn dns_options(&mut self) {
        // todo
    }

    pub fn dns_search(&mut self) {
        // todo
    }

    pub fn extra_host(&mut self, hostname: &str, ip: &str) {
        let extra_host = format!("{}:{}", hostname, ip);

        if let Some(extra_hosts) = self.fields.get_mut("ExtraHosts") {
            if let Some(extra_hosts) = extra_hosts.as_array_mut() {
                extra_hosts.push(json!(extra_host));
            }
        } else {
            self.fields
                .insert("ExtraHosts".to_string(), json!(vec![extra_host]));
        }
    }

    pub fn group_add(&mut self) {
        // todo
    }

    pub fn ipc_mode(&mut self) {
        // todo
    }

    pub fn cgroup(&mut self) {
        // todo
    }

    pub fn links(&mut self) {
        // todo
    }

    pub fn oom_score_adj(&mut self) {
        // todo
    }

    pub fn pid_mode(&mut self) {
        // todo
    }

    pub fn privileged(&mut self, privileged: bool) {
        self.fields
            .insert("Privileged".to_string(), json!(privileged));
    }

    pub fn publish_all_ports(&mut self, publish_all_ports: bool) {
        self.fields
            .insert("PublishAllPorts".to_string(), json!(publish_all_ports));
    }

    pub fn readonly_root_fs(&mut self) {
        // todo
    }

    pub fn security_opt(&mut self) {
        // todo
    }

    pub fn storage_opts(&mut self) {
        // todo
    }

    pub fn tmpfb(&mut self) {
        // todo
    }

    pub fn uts_mode(&mut self) {
        // todo
    }

    pub fn userns_mode(&mut self) {
        // todo
    }

    pub fn shm_size(&mut self) {
        // todo
    }

    pub fn sysctls(&mut self, sysctls: HashMap<&str, &str>) {
        self.fields.insert("Sysctls".to_string(), json!(sysctls));
    }

    pub fn runtime(&mut self) {
        // todo
    }

    pub fn console_size(&mut self) {
        // todo
    }

    pub fn isolation(&mut self) {
        // todo
    }

    pub fn masked_paths(&mut self) {
        // todo
    }
}
