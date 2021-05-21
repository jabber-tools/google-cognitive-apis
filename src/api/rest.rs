//! This module wraps structs used for respective REST Google APIs.
//! The main purpouse of these structs is to provide JSON based configuration
//! capabilities and subsequent conversion into GRPC structs, i.e. provide way how to configure different
//! Google APIs with JSON structs that can be then converted into their GRPC counterparts.
pub mod google;
