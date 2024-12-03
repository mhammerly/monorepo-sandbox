load("//tools:proto.bzl", "multilang_proto_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")

multilang_proto_library(
    name = "ingest",
    srcs = ["proto/ingest.proto"],
)

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
        "@crates.io//:google-cloud-pubsub",
        "@crates.io//:google-cloud-googleapis",
        "@crates.io//:google-cloud-gax",
        "@crates.io//:tokio",
        "@crates.io//:tokio-util",
        ":ingest_rs_proto",
        "@crates.io//:prost",
        "//services/storage:storage_rs_proto",
        "@crates.io//:tonic",
        "//archive:archive",
        "//config:rust_config",
    ],
    visibility = ["//visibility:public"],
)
