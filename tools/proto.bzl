load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")
load("@grpc//bazel:python_rules.bzl", "py_grpc_library", "py_proto_library")

def multilang_proto_library(name, srcs, deps = [], generate_rust=True, generate_python=True, visibility=["//visibility:public"]):
    proto_name = "{}_proto".format(name)
    proto_library(
        name = proto_name,
        srcs = srcs,
        deps = deps,
        visibility = visibility,
    )
    if generate_python:
        py_proto_name = "{}_py_proto".format(name)
        py_proto_library(
            name = py_proto_name,
            deps = [":{}".format(proto_name)],
            visibility = visibility,
        )
        py_grpc_library(
            name = "{}_py_grpc".format(name),
            srcs = [":{}".format(proto_name)],
            deps = [":{}".format(py_proto_name)],
            visibility = visibility,
        )
    if generate_rust:
        rust_prost_library(
            name = "{}_rs_proto".format(name),
            proto = "{}_proto".format(name),
            visibility = visibility,
        )
