use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Healthcheck {
    pub test: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExposedPort {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub cmd: Option<Vec<String>>,
    pub domainname: String,
    pub env: Vec<String>,
    pub exposed_ports: Option<HashMap<String, ExposedPort>>,
    pub healthcheck: Option<Healthcheck>,
    pub hostname: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub mac_address: Option<String>,
    pub network_disabled: Option<bool>,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub user: String,
    // todo - volumes: ???,
    pub working_dir: String,
    pub entrypoint: Option<Vec<String>>,
    pub stop_signal: Option<String>,
    pub stop_timeout: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub network_i_d: String,
    pub endpoint_i_d: String,
    pub gateway: String,
    pub i_p_address: String,
    pub i_p_prefix_len: usize,
    pub i_pv6_gateway: String,
    pub global_i_pv6_address: String,
    pub global_i_pv6_prefix_len: usize,
    pub mac_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub bridge: String,
    pub sandbox_i_d: String,
    pub hairpin_mode: bool,
    pub link_local_i_pv6_address: String,
    pub link_local_i_pv6_prefix_len: usize,
    pub sandbox_key: String,
    pub endpoint_i_d: String,
    pub gateway: String,
    pub global_i_pv6_address: String,
    pub global_i_pv6_prefix_len: usize,
    pub i_p_address: String,
    pub i_p_prefix_len: usize,
    pub i_pv6_gateway: String,
    pub mac_address: String,
    pub networks: HashMap<String, Network>,
    pub ports: HashMap<String, Vec<Port>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    pub start: String,
    pub end: String,
    pub exit_code: usize,
    pub output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    pub status: String,
    pub failing_streak: usize,
    pub log: Vec<Log>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub error: String,
    pub exit_code: usize,
    pub finished_at: String,
    pub health: Option<Health>,
    pub o_o_m_killed: bool,
    pub dead: bool,
    pub paused: bool,
    pub pid: usize,
    pub restarting: bool,
    pub running: bool,
    pub started_at: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub name: String,
    pub source: String,
    pub destination: String,
    pub driver: String,
    pub mode: String,
    pub r_w: bool,
    pub propagation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: String,
    pub count: isize,
    pub device_i_ds: Vec<String>,
    pub capabilities: Vec<Vec<String>>,
    pub options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PortBindings {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub maximum_retry_count: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Config {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LogConfig {
    // todo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub maximum_i_ops: Option<usize>,
    pub maximum_i_o_bps: Option<usize>,
    pub blkio_weight: usize,
    // todo - blkio_weight_device: [{}],
    // todo - blkio_device_read_bps: [{}],
    // todo - blkio_device_write_bps: [{}],
    // todo - blkio_device_read_i_ops: [{}],
    // todo - blkio_device_write_i_ops: [{}],
    pub container_i_d_file: String,
    pub cpuset_cpus: String,
    pub cpuset_mems: String,
    pub cpu_percent: usize,
    pub cpu_shares: usize,
    pub cpu_period: usize,
    pub cpu_realtime_period: usize,
    pub cpu_realtime_runtime: usize,
    // todo - devices: [],
    pub device_requests: Option<Vec<DeviceRequest>>,
    pub ipc_mode: String,
    // todo - lxc_conf: [],
    pub log_config: LogConfig,
    pub memory: usize,
    pub memory_swap: usize,
    pub memory_swappiness: Option<usize>,
    pub memory_reservation: usize,
    pub kernel_memory: usize,
    pub oom_kill_disable: Option<bool>,
    pub oom_score_adj: usize,
    pub network_mode: String,
    pub pid_mode: String,
    pub pids_limit: Option<usize>,
    pub port_bindings: Option<PortBindings>,
    pub privileged: bool,
    pub readonly_rootfs: bool,
    pub publish_all_ports: bool,
    pub restart_policy: RestartPolicy,
    pub sysctls: Option<HashMap<String, String>>,
    pub ulimits: Option<Vec<Ulimit>>,
    pub volume_driver: String,
    pub shm_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspection {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    pub config: ContainerConfig,
    pub created: String,
    pub driver: String,
    pub exec_i_ds: Option<Vec<String>>,
    pub host_config: HostConfig,
    pub hostname_path: String,
    pub hosts_path: String,
    pub log_path: String,
    pub id: String,
    pub image: String,
    pub mount_label: String,
    pub name: String,
    pub network_settings: NetworkSettings,
    pub path: String,
    pub process_label: String,
    pub resolv_conf_path: String,
    pub restart_count: usize,
    pub state: State,
    pub mounts: Vec<Mount>,
}
