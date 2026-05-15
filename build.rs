fn main() {
    // ChainRemote: .proto 변경 시 cargo 가 build.rs 자동 재실행 → 바인딩 재생성.
    // 접두사 "cargo:" 가 없으면 무시되므로 명시.
    println!("cargo:rerun-if-changed=protos/message.proto");
    println!("cargo:rerun-if-changed=protos/rendezvous.proto");

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
