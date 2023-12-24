use prost::Message;
use std::time::SystemTime;

use crate::greeter::HelloRequest;

// Include the `helloworld` module, which is generated from helloworld.proto.
pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
}

pub fn create_request(name: &str) -> HelloRequest {
    return HelloRequest {
        name: name.to_owned(),
        sent_utc: Some(prost_types::Timestamp::from(SystemTime::now())),
    };
}

pub fn serialize_request(request: &HelloRequest) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(request.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    request.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_request(buf: &[u8]) -> Result<HelloRequest, prost::DecodeError> {
    HelloRequest::decode(buf)
}

fn main() {
    let request = create_request("Mike");
    println!("Request: {:?}", request);

    let buf = serialize_request(&request);
    println!("Serialized: {:?}", &buf);

    let r = deserialize_request(&buf).unwrap();
    println!("Deserialized: {:?}", r);
}
