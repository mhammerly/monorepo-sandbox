load("@rules_rust//rust:defs.bzl", "rust_binary")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "main",
    edition = "2021",
    srcs = ["main.rs"],
    deps = [
        "@crates.io//:ferris-says",
    ],
)

load("@rules_pyo3//pyo3:defs.bzl", "pyo3_extension")

pyo3_extension(
    name = "return_foo",
    edition = "2021",
    srcs = ["ext.rs"],
    crate_root = "ext.rs",
    imports = ["."],
)
