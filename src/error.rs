use std::io;

use thiserror::Error;

pub type DockerResult<T> = Result<T, DockerError>;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Curl error occurred")]
    CurlError(#[from] curl::Error),

    #[error("IO error occurred")]
    IoError(#[from] io::Error),

    #[error("Serde json error")]
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("Error creating Docker Image: {0}")]
    FailedToCreateDockerImageError(String),

    #[error("Error creating Docker Image")]
    DockerImageCreateError,

    #[error("Error pulling Docker Image: {0}")]
    FailedToPullDockerImageError(String),

    #[error("Error pulling Docker Image")]
    DockerImagePullError,

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

    #[error("Error attaching Docker Container to Network")]
    DockerAttachContainerToNetworkError,

    #[error("No such container to stop: {0}")]
    NoSuchContainerToStopError(String),

    #[error("Error stopping Docker Container: {0}")]
    StopContainerError(String),

    #[error("Error killing Docker Container: {0}")]
    KillContainerError(String),

    #[error("Error starting Docker Container: {0}; response code {1}")]
    FailedToStartDockerContainerError(String, u32),

    #[error("Error starting Docker Container; response code {0}")]
    DockerContainerStartError(u32),

    #[error("Error creating Docker Container: {0}")]
    FailedToCreateDockerContainerError(String),

    #[error("Error creating Docker Container")]
    DockerContainerCreateError,

    #[error("Error inspecting Docker Container")]
    ContainerInspectionError,

    #[error("Error inspecting Docker Container")]
    ContainerInspectionRequestError(String, u32),
}