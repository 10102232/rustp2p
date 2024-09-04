fn main() {
    std::fs::create_dir_all("src/protocol/proto").unwrap();
    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir("src/protocol/proto")
        .inputs(&["proto/message.proto"])
        .include("proto")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
