mod build_image_handler;

use crate::error::DockerError::{
    DockerImageCreateError, DockerImagePullError, FailedToCreateDockerImageError,
    FailedToPullDockerImageError,
};
use crate::error::DockerResult;
use crate::image::build_image_handler::BuildImageHandler;
use curl::easy::{Easy2, Handler, List};
use std::io::{Error, Write};
use std::path::PathBuf;

///
/// [Reference](https://docs.docker.com/engine/api/v1.40/#operation/ImageList)
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
