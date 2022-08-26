//! Contains common utility & convenience functions.
//! All functions here are intended for internal use only.
use crate::errors::Result as CogResult;
use crate::CERTIFICATES;
use gouth::Builder;
use std::sync::Arc;
use tonic::{
    metadata::{Ascii, AsciiMetadataValue, MetadataValue},
    service::Interceptor,
    transport::{Certificate, Channel, ClientTlsConfig},
    Status,
};

#[derive(Clone)]
pub(crate) struct TokenInterceptor(Arc<String>);
impl TokenInterceptor {
    fn new(token_header_val: Arc<String>) -> TokenInterceptor {
        TokenInterceptor(token_header_val)
    }
}
pub fn new_interceptor(token_header_val: Arc<String>) -> TokenInterceptor {
    TokenInterceptor::new(token_header_val)
}
impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let mut req = request;
        let meta_result =
            MetadataValue::<Ascii>::from_str(&self.0);
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

// /// Convenience function to return tonic Interceptor
// /// (see https://docs.rs/tonic/0.4.3/tonic/struct.Interceptor.html)
// #[allow(clippy::rc_buffer)]
// pub(crate) fn new_interceptor(token_header_val: Arc<String>) -> Result<MyInterceptor> {
//     let interceptor = tonic::Interceptor::new(move |mut req: tonic::Request<()>| {
//         let meta_result = MetadataValue::from_str(&token_header_val);

//         return match meta_result {
//             Ok(meta) => {
//                 req.metadata_mut().insert("authorization", meta);

//                 // TBD: half-close operation probably needs to be somehow implemented in interceptor
//                 // by adding specific metadata to last message
//                 //
//                 // https://grpclib.readthedocs.io/en/latest/client.html#grpclib.client.Stream.send_message
//                 // https://grpclib.readthedocs.io/en/latest/client.html
//                 // req.metadata().append() ???
//                 Ok(req)
//             }
//             Err(some_error) => Err(tonic::Status::internal(format!(
//                 "new_interceptor: Error when getting MetadataValue from token {:?}",
//                 some_error
//             ))),
//         };
//     });
//     Ok(interceptor)
// }

/// Creates new GRPC channel to *.googleapis.com API
/// Domain name and channel URL (like texttospeech.googleapis.com & https://texttospeech.googleapis.com)
/// is provided as input. Optionally timeout in seconds can be specified.
pub(crate) async fn new_grpc_channel(
    domain_name: &'static str,
    channel_url: &'static str,
    timeout_secs: Option<u64>,
) -> CogResult<Channel> {
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

/// Returns google token (String value) from
/// Google Cloud Platform project JSON credentials (provided as String).
#[allow(clippy::rc_buffer)]
pub(crate) fn get_token(google_credentials: impl AsRef<str>) -> CogResult<Arc<String>> {
    let token = Builder::new().json(google_credentials).build()?;
    let token_header_val: Arc<String> = token.header_value()?;
    Ok(token_header_val)
}
