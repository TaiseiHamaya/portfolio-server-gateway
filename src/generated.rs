#![allow(unused_imports, nonstandard_style, dead_code)]

#[path = "./generated/server/proto.rs"]
pub mod proto_server;

#[path = "./generated/client/proto.rs"]
pub mod proto_client;

use crate::network::proto::proto;

macro_rules! impl_proto_vector3_from {
    ($type_a:path) => {
        impl From<$type_a> for proto::Vector3 {
            fn from(value: $type_a) -> Self {
                let mut result = proto::Vector3::new();
                result.set_x(value.x);
                result.set_y(value.y);
                result.set_z(value.z);
                result
            }
        }
        impl From<proto::Vector3> for $type_a {
            fn from(value: proto::Vector3) -> Self {
                $type_a {
                    x: value.x(),
                    y: value.y(),
                    z: value.z(),
                }
            }
        }
    };
}

impl_proto_vector3_from!(proto_client::Vector3);
impl_proto_vector3_from!(proto_server::Vector3);
