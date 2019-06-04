fn main() {
    prost_build::compile_protos(
        &[
            "protos/footers.proto",
            "protos/metadata.proto",
            "protos/timestamp.proto",
        ],
        &["protos/"],
    )
    .unwrap();
}
