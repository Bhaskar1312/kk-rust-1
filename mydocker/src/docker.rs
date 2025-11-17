use bollard::Docker;
use bollard::errors::Error;
use bollard::models::ContainerSummary;
use bollard::container::ListContainersOptions;

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
}

// docker context inspect // unix:///Users/{user}/.docker/run/docker.sock