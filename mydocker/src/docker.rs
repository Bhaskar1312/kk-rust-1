use bollard::Docker;
use bollard::errors::Error;
use bollard::models::ContainerSummary;
use bollard::models::ImageSummary;
use bollard::container::ListContainersOptions;
use bollard::container::StartContainerOptions;
use bollard::container::StopContainerOptions;
use bollard::image::ListImagesOptions;

pub struct DockerClient {
    docker: Docker,

}

impl DockerClient {
    pub fn new() -> Self {
        // let docker = Docker::connect_with_unix_defaults().unwrap();
        let docker = Docker::connect_with_local_defaults().unwrap();
        
        Self { docker: docker }
    }

    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = Some(ListContainersOptions::<String>{
            all,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;
        Ok(containers)
    }

    pub async fn list_images(&self) -> Result<Vec<ImageSummary>, Error>{
        let options = Some(ListImagesOptions::<String>{
            all : true,
            ..Default::default()
        });

        let images = self.docker.list_images(options).await?;
        Ok(images)
    }

    pub async fn start_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker.start_container(container_name, None::<StartContainerOptions<String>>).await?;
        Ok(())
    }

    pub async fn stop_container(&self, container_name: &str) -> Result<(), Error>{
        self.docker.stop_container(container_name, None::<StopContainerOptions>).await.unwrap_or_else(|e| {
            eprintln!("Error stopping container {}: {}", container_name, e);
        });
        Ok(())
    }
}

// docker context inspect // unix:///Users/{user}/.docker/run/docker.sock