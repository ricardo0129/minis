mod pb;

use pb::hello::HelloRequest;
use prost::Message;

fn main() {
    let req = HelloRequest {
        name: "Ricky".to_string(),
    };

    let mut buf = vec![];
    // Serialize
    req.encode(&mut buf).unwrap();

    // Deserialize
    let decoded = HelloRequest::decode(&*buf).unwrap();

    println!("Decoded: {:?}", decoded);
}
