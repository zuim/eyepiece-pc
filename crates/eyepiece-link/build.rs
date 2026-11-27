fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("protoc unavailable");
    std::env::set_var("PROTOC", protoc);
    prost_build::compile_protos(&["../../contract/eyepiece-rpc.proto"], &["../../contract"])
        .expect("compile protocol");
    println!("cargo:rerun-if-changed=../../contract/eyepiece-rpc.proto");
}
