pub mod internal_pipeline_server;
use crate::grpc::internal_grpc::internal_pipeline_service_client::InternalPipelineServiceClient;
use dozer_types::models::api_config::ApiInternal;
use tonic::transport::Channel;

pub async fn init_internal_pipeline_client(
    config: ApiInternal,
) -> Result<InternalPipelineServiceClient<Channel>, Box<dyn std::error::Error>> {
    let address = format!("http://{:}:{:}", config.host, config.port);
    let client = InternalPipelineServiceClient::connect(address).await?;
    Ok(client)
}