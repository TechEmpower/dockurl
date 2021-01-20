mod build_image_handler;
mod prune_images_handler;

use crate::error::DockerError::{
    DockerImageCreateError, DockerImageDeleteError, DockerImagePruneError, DockerImagePullError,
    FailedToCreateDockerImageError, FailedToDeleteDockerImageError, FailedToPruneDockerImageError,
    FailedToPullDockerImageError,
};
use crate::error::DockerResult;
use crate::image::build_image_handler::BuildImageHandler;
use crate::image::prune_images_handler::PruneImagesHandler;
use curl::easy::{Easy2, Handler, List};
use std::io::{Error, Write};
use std::path::PathBuf;

// pub struct BuildImageOptions<'a> {
//     dockerfile: &'a PathBuf,
//     t: &'a str,
//     extrahosts: Option<&'a str>,
//     remote: Option<&'a str>,
//     q: bool,
//     nocache: bool,
//     pull: Option<&'a str>,
//     rm: Option<&'a str>,
//     forcerm: Option<&'a str>,
//     memory: Option<i32>,
//     memswap: Option<i32>,
//     cpushares: Option<i32>,
//     cpusetcpus: Option<&'a str>,
//     cpuperiod: Option<i32>,
//     cpuquota: Option<i32>,
//     buildargs: Option<&'a str>,
//     shmsize: Option<i32>,
//     squash: Option<bool>,
//     labels: Option<&'a str>,
//     networkmode: Option<&'a str>,
//     platform: Option<&'a str>,
//     target: Option<&'a str>,
//     outputs: Option<&'a str>,
// }

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ImageBuild)
pub fn build_image<H: Handler>(
    name_and_tag: &str,
    dockerfile: &PathBuf,
    context_dir: &PathBuf,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<String> {
    let mut tarchive = Tarchive(Vec::new());
    let mut tar = tar::Builder::new(&mut tarchive);
    tar.append_dir_all("", context_dir.to_str().unwrap())?;
    tar.finish()?;

    let dockerfile = dockerfile.to_str().unwrap();

    let query_string = format!("?dockerfile={}&t={}", dockerfile, name_and_tag);
    let mut headers = List::new();
    headers.append("Content-Type: application/x-tar")?;
    let bytes = tar.get_mut().buffer();
    let len = bytes.len();

    let mut easy = Easy2::new(BuildImageHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.http_headers(headers)?;
    easy.in_filesize(len as u64)?;
    easy.post_field_size(len as u64)?;
    easy.url(&format!("http://{}/build{}", docker_host, query_string))?;
    easy.post_fields_copy(bytes)?;
    easy.perform()?;

    match easy.response_code() {
        Ok(code) => match code {
            200 => {
                if let Some(image_id) = &easy.get_ref().image_id {
                    return Ok(image_id.to_owned());
                } else if let Some(error_message) = &easy.get_ref().error_message {
                    return Err(FailedToCreateDockerImageError(error_message.to_owned()));
                }
                Err(DockerImageCreateError)
            }
            _ => {
                if let Some(error_message) = &easy.get_ref().error_message {
                    return Err(FailedToCreateDockerImageError(error_message.to_owned()));
                }
                Err(DockerImageCreateError)
            }
        },
        Err(e) => Err(FailedToCreateDockerImageError(e.to_string())),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ImageCreate)
pub fn create_image<H: Handler>(
    from_image: &str,
    tag: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let query_string = format!("?fromImage={}&tag={}", from_image, tag);

    let mut easy = Easy2::new(BuildImageHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/images/create{}",
        docker_host, query_string
    ))?;
    easy.perform()?;

    match easy.response_code() {
        Ok(code) => match code {
            200 => Ok(()), // todo - do we want to return the image_id?
            _ => {
                let error_message = &easy.get_ref().error_message;
                if error_message.is_some() {
                    return Err(FailedToPullDockerImageError(error_message.clone().unwrap()));
                }
                Err(DockerImagePullError)
            }
        },
        Err(e) => Err(FailedToPullDockerImageError(e.to_string())),
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageDelete)
pub fn delete_image<H: Handler>(
    image_name_or_id: &str,
    force: bool,
    no_prune: bool,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<Option<String>> {
    let query_string = format!("?force={}&noprune={}", force, no_prune);

    let mut easy = Easy2::new(PruneImagesHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.custom_request("DELETE")?;
    easy.url(&format!(
        "http://{}/images/{}{}",
        docker_host, image_name_or_id, query_string
    ))?;
    easy.perform()?;

    let message = easy.get_ref().message.clone();
    let error_message = &easy.get_ref().error_message;
    if error_message.is_some() {
        Err(FailedToDeleteDockerImageError(
            error_message.clone().unwrap(),
        ))
    } else {
        match easy.response_code() {
            Ok(code) => match code {
                200 => Ok(message),
                _ => Err(DockerImageDeleteError),
            },
            Err(e) => Err(FailedToDeleteDockerImageError(e.to_string())),
        }
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.41/#operation/BuildPrune)
pub fn delete_builder_cache<H: Handler>(
    keep_storage: i64,
    remove_all: bool,
    filters: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let query_string = format!(
        "?keep-storage={}&all={}&filters={}",
        keep_storage, remove_all, filters
    );

    let mut easy = Easy2::new(PruneImagesHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/build/prune{}",
        docker_host, query_string
    ))?;
    easy.perform()?;

    let error_message = &easy.get_ref().error_message;
    if error_message.is_some() {
        Err(FailedToPruneDockerImageError(
            error_message.clone().unwrap(),
        ))
    } else {
        match easy.response_code() {
            Ok(code) => match code {
                200 => Ok(()),
                _ => Err(DockerImagePruneError),
            },
            Err(e) => Err(FailedToPruneDockerImageError(e.to_string())),
        }
    }
}

///
/// [Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImagePrune)
pub fn delete_unused_images<H: Handler>(
    filters: &str,
    docker_host: &str,
    use_unix_socket: bool,
    log_handler: H,
) -> DockerResult<()> {
    let query_string = format!("?filters={}", filters);

    let mut easy = Easy2::new(PruneImagesHandler::new(log_handler));
    if use_unix_socket {
        easy.unix_socket("/var/run/docker.sock")?;
    }

    easy.post(true)?;
    easy.url(&format!(
        "http://{}/images/prune{}",
        docker_host, query_string
    ))?;
    easy.perform()?;

    let error_message = &easy.get_ref().error_message;
    if error_message.is_some() {
        Err(FailedToPruneDockerImageError(
            error_message.clone().unwrap(),
        ))
    } else {
        match easy.response_code() {
            Ok(code) => match code {
                200 => Ok(()),
                _ => Err(DockerImagePruneError),
            },
            Err(e) => Err(FailedToPruneDockerImageError(e.to_string())),
        }
    }
}

// PRIVATES

/// Simple helper for housing a tarball in a buffer. We just want the bytes
/// and this keeps us from writing to disk.
struct Tarchive(Vec<u8>);
impl Tarchive {
    fn buffer(&mut self) -> &[u8] {
        self.0.as_slice()
    }
}
impl Write for Tarchive {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        // ¯\_(ツ)_/¯
        Ok(())
    }
}
