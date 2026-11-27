fn main() {
    let protoc = protoc_bin_vendored::protoc_bin_path().expect("protoc unavailable");
    std::env::set_var("PROTOC", protoc);
    // the bridge protocol (defined here) and tinkerglass' control protocol (a copy of
    // tinkerglass' contract/tinkerglass-control.proto)
    prost_build::compile_protos(
        &["../../contract/eyepiece-rpc.proto", "../../contract/tinkerglass-control.proto"],
        &["../../contract"],
    )
    .expect("compile protocol");
    println!("cargo:rerun-if-changed=../../contract/eyepiece-rpc.proto");
    println!("cargo:rerun-if-changed=../../contract/tinkerglass-control.proto");
}
