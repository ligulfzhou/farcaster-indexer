fn main() {
    tonic_build::compile_protos("../protobuf/rpc.proto").unwrap()
}
