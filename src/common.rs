use crate::api::grpc::google::cloud::dialogflow::v2beta1::sessions_client::SessionsClient;
use crate::errors::Result;
use crate::CERTIFICATES;
use std::sync::Arc;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

/// Convenience function to return tonic Interceptor
/// (see https://docs.rs/tonic/0.4.3/tonic/struct.Interceptor.html)
pub fn new_interceptor(token_header_val: Arc<String>) -> Result<tonic::Interceptor> {
    let interceptor = tonic::Interceptor::new(move |mut req: tonic::Request<()>| {
        let meta_result = MetadataValue::from_str(&token_header_val);

        return match meta_result {
            Ok(meta) => {
                req.metadata_mut().insert("authorization", meta);
                Ok(req)
            }
            Err(some_error) => Err(tonic::Status::internal(format!(
                "new_interceptor: Error when getting MetadataValue from token {:?}",
                some_error
            ))),
        };
    });
    Ok(interceptor)
}

/// Creates new GRPC channel to speech.googleapis.com API
pub async fn new_grpc_channel(
    domain_name: &'static str,
    channel_url: &'static str,
    timeout_secs: Option<u64>,
) -> Result<Channel> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name(domain_name);

    return if let Some(timeout) = timeout_secs {
        Ok(Channel::from_static(channel_url)
            .tls_config(tls_config.clone())?
            .timeout(std::time::Duration::from_secs(timeout))
            .connect()
            .await?)
    } else {
        Ok(Channel::from_static(channel_url)
            .tls_config(tls_config.clone())?
            .connect()
            .await?)
    };
}
