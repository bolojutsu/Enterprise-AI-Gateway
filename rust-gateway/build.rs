fn main() {
    tonic_build::compile_protos("../proto/gateway.proto").expect("Failed to compile protos");
}
