fn main() {
    prost_build::compile_protos(&["proto/hello.proto"], &["proto"]).unwrap();
}
