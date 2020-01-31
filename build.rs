fn main() {
    prost_build::compile_protos(
        &[
            "protos/footer.proto",
            "protos/header.proto",
            "protos/metadata.proto",
            "protos/timestamp.proto",
        ],
        &["protos/"],
    )
    .unwrap();
}
