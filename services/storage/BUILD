load("//tools:proto.bzl", "multilang_proto_library")
load("@rules_python//python:defs.bzl", "py_binary")

multilang_proto_library(
    name = "storage",
    srcs = ["proto/storage.proto"],
    deps = [
        "@protobuf//:timestamp_proto",
    ],
)

py_binary(
    name = "main",
    srcs = ["main.py"],
    imports = ["."],
    deps = [
        ":storage_py_proto",
        ":storage_py_grpc",
        "//services/django_app:settings",
        "//services/django_app/archive:models",
        "//third-party/python:grpcio",
    ],
    visibility = ["//visibility:public"],
)
