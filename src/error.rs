use std::io;

use thiserror::Error;

pub type DockerResult<T> = Result<T, DockerError>;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Curl error occurred: {0}")]
    CurlError(#[from] curl::Error),

    #[error("IO error occurred: {0}")]
    IoError(#[from] io::Error),

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("Error creating Docker Image: {0}")]
    FailedToCreateDockerImageError(String),

    #[error("Error creating Docker Image")]
    DockerImageCreateError,

    #[error("Error pulling Docker Image: {0}")]
    FailedToPullDockerImageError(String),

    #[error("Error pulling Docker Image")]
    DockerImagePullError,

    #[error("Unknown error pruning Docker Image(s)")]
    DockerImagePruneError,

    #[error("Error pruning Docker Image(s): {0}")]
    FailedToPruneDockerImageError(String),

    #[error("Unknown error deleting Docker Image(s)")]
    DockerImageDeleteError,

    #[error("Error deleting Docker Image(s): {0}")]
    FailedToDeleteDockerImageError(String),

    #[error("Error creating Docker Network: {0}")]
    FailedToCreateDockerNetworkError(String),

    #[error("Error creating Docker Network")]
    DockerNetworkCreateError,

    #[error("Error creating Docker Network; Network already exists: {0}")]
    DockerNetworkAlreadyExistsCreateError(String),

    #[error("Error deleting Docker Network: {0}")]
    FailedToDeleteNetworkError(String),

    #[error("Error deleting Docker Network")]
    DockerNetworkDeleteError,

    #[error("Network not found error: {0}")]
    NetworkNotFoundError(String),

    #[error("Error inspecting network")]
    InspectNetworkError,

    #[error("Error attaching Docker Container to Network: {0}")]
    FailedToAttachDockerContainerToNetworkError(String),

    #[error("Operation not supported for swarm scoped networks")]
    OperationNotSupportedError,

    #[error("Network or container not found. Network: {0}; Container: {1}")]
    NetworkOrContainerNotFoundError(String, String),

    #[error("Docker server error")]
    DockerServerError,

    #[error("Unknown docker server error: {0}")]
    UnknownDockerError(String),

    #[error("No such container: {0}")]
    NoSuchContainerError(String),

    #[error("Error stopping Docker Container: {0}")]
    StopContainerError(String),

    #[error("Error killing Docker Container: {0}")]
    KillContainerError(String),

    #[error("Error starting Docker Container: {0}; response code {1}")]
    FailedToStartDockerContainerError(String, u32),

    #[error("Error starting Docker Container; response code {0}")]
    DockerContainerStartError(u32),

    #[error("Error deleting Docker Container; bad parameter: container id - {0}, message - {1}")]
    DockerContainerDeleteBadParameterError(String, String),

    #[error(
        "Error deleting Docker Container; no such container: container id - {0}, message - {1}"
    )]
    DockerContainerDeleteNoSuchContainer(String, String),

    #[error("Error deleting Docker Container; conflict: container id - {0}, message - {1}")]
    DockerContainerDeleteConflictError(String, String),

    #[error(
        "Error deleting Docker Container; internal server error: container id - {0}, message - {1}"
    )]
    DockerContainerDeleteInternalServerError(String, String),

    #[error("Error deleting Docker Container; unknown error: response code - {0}, message - {1}")]
    DockerContainerDeleteUnknownError(u32, String),

    #[error("Error creating Docker Container: {0}")]
    FailedToCreateDockerContainerError(String),

    #[error("Error creating Docker Container")]
    DockerContainerCreateError,

    #[error("Error inspecting Docker Container")]
    ContainerInspectionError,

    #[error("Error inspecting Docker Container")]
    ContainerInspectionRequestError(String, u32),

    #[error("Docker Daemon Error")]
    DockerDaemonError,
}
