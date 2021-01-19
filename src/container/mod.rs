pub mod create;
pub mod inspect;
mod log_handlers;

use crate::container::create::options::Options;
use crate::container::inspect::ContainerInspection;
use crate::container::log_handlers::create_container_handler::CreateContainerHandler;
use crate::container::log_handlers::delete_container_handler::DeleteContainerHandler;
use crate::container::log_handlers::inspect_container_handler::InspectContainerHandler;
use crate::container::log_handlers::start_container_handler::StartContainerHandler;
use crate::error::DockerError::{
    ContainerInspectionError, ContainerInspectionRequestError, CurlError,
    DockerContainerCreateError, DockerContainerDeleteBadParameterError,
    DockerContainerDeleteConflictError, DockerContainerDeleteInternalServerError,
    DockerContainerDeleteNoSuchContainer, DockerContainerDeleteUnknownError,
    DockerContainerStartError, DockerDaemonError, FailedToCreateDockerContainerError,
    FailedToStartDockerContainerError, KillContainerError, NoSuchContainerError,
    StopContainerError,
};
use crate::error::{DockerError, DockerResult};
use curl::easy::{Easy2, Handler, List};

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerAttach)
pub fn attach_to_container<H: Handler + Clone>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<H> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    let query_string = "?logs=1&stream=1&stdout=1&stderr=1";
    easy.post(true)?;
    easy.url(&format!(
        "http://{}/containers/{}/attach{}",
        docker_host, container_id, query_string
    ))?;
    easy.perform()?;

    Ok(easy.get_ref().clone())
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerLogs)
pub fn get_container_logs<H: Handler + Clone>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<H> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    let query_params = "?stdout=1&stderr=1";

    easy.url(&format!(
        "http://{}/containers/{}/logs{}",
        docker_host, container_id, query_params,
    ))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(code) => match code {
            200 => Ok(easy.get_ref().clone()),
            404 => Err(NoSuchContainerError(container_id.to_string())),
            _ => Err(DockerDaemonError),
        },
        Err(e) => Err(CurlError(e)),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerCreate)
pub fn create_container<H: Handler>(
    options: Options,
    use_unix_socket: bool,
    docker_host: &str,
    log_handler: H,
) -> DockerResult<String> {
    let mut easy = Easy2::new(CreateContainerHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    let mut headers = List::new();
    headers.append("Content-Type: application/json")?;

    let json = options.to_json();
    let len = json.as_bytes().len();

    easy.post(true)?;
    easy.url(&format!("http://{}/containers/create", docker_host))?;
    easy.http_headers(headers)?;
    easy.in_filesize(len as u64)?;
    easy.post_field_size(len as u64)?;
    easy.post_fields_copy(json.as_bytes())?;
    easy.perform()?;

    match easy.response_code() {
        Ok(code) => match code {
            201 => {
                if let Some(container_id) = &easy.get_mut().container_id {
                    return Ok(container_id.clone());
                } else if let Some(error) = &easy.get_ref().error_message {
                    return Err(FailedToCreateDockerContainerError(error.clone()));
                }
                Err(DockerContainerCreateError)
            }
            code => {
                if let Some(error) = &easy.get_ref().error_message {
                    return Err(FailedToCreateDockerContainerError(error.clone()));
                }
                Err(FailedToCreateDockerContainerError(format!("{}", code)))
            }
        },
        Err(e) => Err(FailedToCreateDockerContainerError(e.to_string())),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerInspect)
pub fn inspect_container<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<ContainerInspection> {
    let mut easy = Easy2::new(InspectContainerHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.url(&format!(
        "http://{}/containers/{}/json",
        docker_host, container_id
    ))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(200) => {
            if let Ok(json_str) = std::str::from_utf8(&easy.get_ref().accumulator) {
                match serde_json::from_str(json_str) {
                    Ok(json) => Ok(json),
                    Err(e) => Err(DockerError::SerdeJsonError(e)),
                }
            } else {
                Err(ContainerInspectionError)
            }
        }
        Ok(404) => Err(ContainerInspectionError),
        Ok(code) => {
            if let Some(error) = &easy.get_ref().error_message {
                return Err(ContainerInspectionRequestError(error.clone(), code));
            }
            Err(DockerContainerStartError(code))
        }
        Err(e) => Err(DockerError::CurlError(e)),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerStart)
pub fn start_container<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(StartContainerHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/containers/{}/start",
        docker_host, container_id
    ))?;
    easy.post_fields_copy(&[])?;
    easy.perform()?;

    match easy.response_code() {
        Ok(204) => Ok(()),
        Ok(code) => {
            if let Some(error) = &easy.get_ref().error_message {
                return Err(FailedToStartDockerContainerError(error.clone(), code));
            }
            Err(DockerContainerStartError(code))
        }
        Err(e) => Err(DockerError::CurlError(e)),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerStop)
pub fn stop_container<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/containers/{}/stop",
        docker_host, container_id
    ))?;
    easy.perform()?;

    match easy.response_code()? {
        204 => Ok(()),
        304 => Ok(()), // container already stopped
        404 => Err(NoSuchContainerError(container_id.to_string())),
        _ => Err(StopContainerError(format!(
            "An error occurred while trying to stop container: {}",
            container_id
        ))),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerKill)
pub fn kill_container<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/containers/{}/kill",
        docker_host, container_id
    ))?;
    easy.perform()?;

    match easy.response_code()? {
        204 => Ok(()),
        _ => Err(KillContainerError(format!(
            "An error occurred while trying to kill container: {}",
            container_id
        ))),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerDelete)
pub fn delete_container<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
    delete_anonymous_volumes: bool,
    force: bool,
    remove_associated_link: bool,
) -> DockerResult<()> {
    let mut easy = Easy2::new(DeleteContainerHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.custom_request("DELETE")?;
    easy.url(&format!(
        "http://{}/containers/{}?v={}&force={}&link={}",
        docker_host, container_id, delete_anonymous_volumes, force, remove_associated_link,
    ))?;
    easy.perform()?;

    let accumulator = easy.get_ref().accumulator.clone();
    let response = std::str::from_utf8(&accumulator).unwrap();

    match easy.response_code()? {
        204 => Ok(()),
        400 => Err(DockerContainerDeleteBadParameterError(
            container_id.to_string(),
            response.to_string(),
        )),
        404 => Err(DockerContainerDeleteNoSuchContainer(
            container_id.to_string(),
            response.to_string(),
        )),
        409 => Err(DockerContainerDeleteConflictError(
            container_id.to_string(),
            response.to_string(),
        )),
        500 => Err(DockerContainerDeleteInternalServerError(
            container_id.to_string(),
            response.to_string(),
        )),
        code => Err(DockerContainerDeleteUnknownError(
            code,
            container_id.to_string(),
        )),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ContainerWait)
pub fn wait_for_container_to_exit<H: Handler>(
    container_id: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let mut easy = Easy2::new(log_handler);
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/containers/{}/wait",
        docker_host, container_id,
    ))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(code) => match code {
            200 => Ok(()),
            404 => Err(NoSuchContainerError(container_id.to_string())),
            _ => Err(DockerDaemonError),
        },
        Err(e) => Err(CurlError(e)),
    }
}
