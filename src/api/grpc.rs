//! This module wraps underlying GRPC stubs and organizes them into nested module structure.
macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!("../grpc_stubs/", concat!($package, ".rs")));
    };
}

pub mod google {

    pub mod rpc {
        include_proto!("google.rpc");
    }

    pub mod longrunning {
        include_proto!("google.longrunning");
    }

    pub mod r#type {
        include_proto!("google.r#type");
    }

    pub mod cloud {

        pub mod texttospeech {

            pub mod v1 {
                #[cfg(any(feature = "default", feature = "google-cloud-texttospeech-v1",))]
                include_proto!("google.cloud.texttospeech.v1");
            }

            pub mod v1beta1 {
                #[cfg(any(feature = "default", feature = "google-cloud-texttospeech-v1beta1",))]
                include_proto!("google.cloud.texttospeech.v1beta1");
            }
        }

        pub mod speechtotext {

            pub mod v1 {
                #[cfg(any(feature = "default", feature = "google-cloud-speechtotext-v1",))]
                include_proto!("google.cloud.speech.v1");
            }

            pub mod v1p1beta1 {
                #[cfg(any(feature = "default", feature = "google-cloud-speechtotext-v1p1beta1",))]
                include_proto!("google.cloud.speech.v1p1beta1");
            }

            pub mod v2 {
                #[cfg(any(feature = "default", feature = "google-cloud-speechtotext-v2",))]
                include_proto!("google.cloud.speech.v2");
            }
        }

        pub mod dialogflow {

            pub mod v2 {
                #[cfg(any(feature = "default", feature = "google-cloud-dialogflow-v2",))]
                include_proto!("google.cloud.dialogflow.v2");
            }

            pub mod v2beta1 {
                #[cfg(any(feature = "default", feature = "google-cloud-dialogflow-v2beta1",))]
                include_proto!("google.cloud.dialogflow.v2beta1");
            }

            pub mod cx {
                pub mod v3 {
                    #[cfg(any(feature = "default", feature = "google-cloud-dialogflow-v2",))]
                    include_proto!("dialogflow_cx/google.cloud.dialogflow.cx.v3");
                }

                pub mod v3beta1 {
                    #[cfg(any(feature = "default", feature = "google-cloud-dialogflow-v2",))]
                    include_proto!("dialogflow_cx/google.cloud.dialogflow.cx.v3beta1");
                }
            }
        }
    }
}
