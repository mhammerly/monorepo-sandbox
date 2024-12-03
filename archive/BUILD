load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_pyo3//pyo3:defs.bzl", "pyo3_extension")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "archive",
    srcs = ["src/lib.rs"],
    deps = [
        "@crates.io//:web-archive",
    ],
)

pyo3_extension(
    name = "pyo3_archive",
    srcs = ["src/lib.rs"],
    crate_features = ["pyo3"],
    imports = ["."],
    deps = [
        "@crates.io//:web-archive",
    ],
)
