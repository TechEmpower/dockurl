mod connect_container_to_network_handler;
mod create_network_handler;
mod inspect_network_handler;

use crate::error::DockerError::{
    DockerNetworkAlreadyExistsCreateError, DockerNetworkCreateError, DockerNetworkDeleteError,
    DockerServerError, FailedToAttachDockerContainerToNetworkError,
    FailedToCreateDockerNetworkError, FailedToDeleteNetworkError, InspectNetworkError,
    NetworkNotFoundError, NetworkOrContainerNotFoundError, OperationNotSupportedError,
    UnknownDockerError,
};
use crate::error::DockerResult;
use crate::network::connect_container_to_network_handler::ConnectContainerToNetworkHandler;
use crate::network::create_network_handler::CreateNetworkHandler;
use crate::network::inspect_network_handler::InspectNetworkHandler;
use curl::easy::{Easy2, Handler, List};
use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Serialize, Deserialize, Debug, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum NetworkMode {
    Bridge,
    Host,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct NetworkCreationOptions {
    pub name: String,
    pub driver: String,
    pub internal: bool,
    pub check_duplicate: bool,
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/NetworkCreate)
pub fn create_network<H: Handler>(
    network_name: &str,
    network_mode: NetworkMode,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<String> {
    let mut easy = Easy2::new(CreateNetworkHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    let options = NetworkCreationOptions {
        name: network_name.to_string(),
        driver: network_mode.to_string(),
        internal: false,
        check_duplicate: true,
    };
    let json = serde_json::to_string(&options)?;
    let len = json.len();

    easy.post(true)?;
    easy.url(&format!("http://{}/networks/create", docker_host))?;
    easy.http_headers(headers)?;
    easy.in_filesize(len as u64)?;
    easy.post_field_size(len as u64)?;
    easy.post_fields_copy(json.as_bytes())?;
    easy.perform()?;

    match easy.response_code() {
        Ok(201) => {
            if let Some(network_id) = &easy.get_ref().network_id {
                return Ok(network_id.clone());
            } else {
                let error_message = &easy.get_ref().error_message;
                if error_message.is_some() {
                    return Err(FailedToCreateDockerNetworkError(
                        error_message.clone().unwrap(),
                    ));
                }
            }
            Err(DockerNetworkCreateError)
        }
        Ok(409) => Err(DockerNetworkAlreadyExistsCreateError(
            network_name.to_string(),
        )),
        Ok(_code) => Err(DockerNetworkCreateError),
        Err(e) => Err(FailedToCreateDockerNetworkError(e.to_string())),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct NetworkConnectOptions {
    pub container: String,
    pub endpoint_config: EndpointConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EndpointConfig {
    pub i_p_a_m_config: IPAMConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct IPAMConfig {
    pub aliases: Vec<String>,
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/NetworkConnect)
pub fn connect_container_to_network<H: Handler>(
    container_id: &str,
    network_id: &str,
    aliases: Vec<String>,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(ConnectContainerToNetworkHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    let options = NetworkConnectOptions {
        container: container_id.to_string(),
        endpoint_config: EndpointConfig {
            i_p_a_m_config: IPAMConfig { aliases },
        },
    };
    let json = serde_json::to_string(&options)?;
    let len = json.len();

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/networks/{}/connect",
        docker_host, network_id
    ))?;
    easy.http_headers(headers)?;
    easy.in_filesize(len as u64)?;
    easy.post_field_size(len as u64)?;
    easy.post_fields_copy(json.as_bytes())?;
    easy.perform()?;

    match easy.response_code() {
        Ok(200) => Ok(()),
        Ok(403) => Err(OperationNotSupportedError),
        Ok(404) => Err(NetworkOrContainerNotFoundError(
            network_id.to_string(),
            container_id.to_string(),
        )),
        Ok(500) => Err(DockerServerError),
        Ok(code) => Err(UnknownDockerError(format!(
            "Response code: {}; Response: {}",
            code,
            easy.get_ref().body()
        ))),
        Err(e) => Err(FailedToAttachDockerContainerToNetworkError(e.to_string())),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/NetworkDelete)
pub fn delete_network<H: Handler>(
    network_name: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.custom_request("DELETE")?;
    easy.url(&format!("http://{}/networks/{}", docker_host, network_name))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(204) => Ok(()),
        Ok(_) => Err(DockerNetworkDeleteError),
        Err(e) => Err(FailedToDeleteNetworkError(e.to_string())),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/NetworkInspect)
pub fn inspect_network<H: Handler>(
    network_id_or_name: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<Network> {
    let mut easy = Easy2::new(InspectNetworkHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.url(&format!(
        "http://{}/networks/{}",
        docker_host, network_id_or_name
    ))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(200) => {
            let network: Network = serde_json::from_str(&easy.get_ref().body()).unwrap();
            Ok(network)
        }
        Ok(404) => Err(NetworkNotFoundError(network_id_or_name.to_string())),
        _ => Err(InspectNetworkError),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub name: String,
    pub id: String,
    // todo
}
