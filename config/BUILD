load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_pyo3//pyo3:defs.bzl", "pyo3_extension")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "rust_config",
    srcs = ["src/lib.rs"],
)

pyo3_extension(
    name = "py_config",
    srcs = ["src/pyo3.rs"],
    crate_root = "src/pyo3.rs",
    imports = ["."],
    deps = [
        ":rust_config",
    ],
)

