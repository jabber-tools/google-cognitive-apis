//! Contains common utility & convenience functions.
//! All functions here are intended for internal use only.
use crate::errors::Result;
use crate::CERTIFICATES;
use gouth::Builder;
use std::sync::Arc;
use tonic::{
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Certificate, Channel, ClientTlsConfig},
    Status,
};

#[derive(Clone)]
pub struct TokenInterceptor(Arc<String>);
impl TokenInterceptor {
    fn new(token_header_val: Arc<String>) -> TokenInterceptor {
        TokenInterceptor(token_header_val)
    }
}
pub fn new_interceptor(token_header_val: Arc<String>) -> TokenInterceptor {
    TokenInterceptor::new(token_header_val)
}
impl Interceptor for TokenInterceptor {
    fn call(
        &mut self,
        request: tonic::Request<()>,
    ) -> core::result::Result<tonic::Request<()>, Status> {
        let mut req = request;
        #[allow(deprecated)]
        let meta_result = MetadataValue::<Ascii>::from_str(&self.0);
        return match meta_result {
            Ok(meta) => {
                req.metadata_mut().insert("authorization", meta);

                // TBD: half-close operation probably needs to be somehow implemented in interceptor
                // by adding specific metadata to last message
                //
                // https://grpclib.readthedocs.io/en/latest/client.html#grpclib.client.Stream.send_message
                // https://grpclib.readthedocs.io/en/latest/client.html
                // req.metadata().append() ???
                Ok(req)
            }
            Err(some_error) => Err(tonic::Status::internal(format!(
                "new_interceptor: Error when getting MetadataValue from token {:?}",
                some_error
            ))),
        };
    }
}

/// Creates new GRPC channel to *.googleapis.com API
/// Domain name and channel URL (like texttospeech.googleapis.com & https://texttospeech.googleapis.com)
/// is provided as input. Optionally timeout in seconds can be specified.
pub(crate) async fn new_grpc_channel(
    domain_name: &'static str,
    channel_url: &'static str,
    timeout_secs: Option<u64>,
) -> Result<Channel> {
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name(domain_name);

    if let Some(timeout) = timeout_secs {
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
    }
}

/// Returns google token (String value) from
/// Google Cloud Platform project JSON credentials (provided as String).
#[allow(clippy::rc_buffer)]
pub(crate) fn get_token(google_credentials: impl AsRef<str>) -> Result<Arc<String>> {
    let token = Builder::new().json(google_credentials).build()?;
    let token_header_val: Arc<String> = token.header_value()?;
    Ok(token_header_val)
}
