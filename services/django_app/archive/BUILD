load("@rules_python//python:defs.bzl", "py_library")

filegroup(
    name = "templates",
    srcs = glob(["**/*.html"]),
    visibility = ["//visibility:public"],
)

filegroup(
    name = "migrations",
    srcs = glob(["migrations/*.py"]),
)

py_library(
    name = "models",
    srcs = ["models.py"],
    imports = [".."],
    deps = [
        "//third-party/python:django",
    ],
    visibility = ["//visibility:public"],
)

py_library(
    name = "archive",
    srcs = glob(["*.py"]),
    data = [":templates", ":migrations", "//services/ingest:ingest_proto"],
    imports = [".."],
    deps = [
        "//archive:pyo3_archive",
        "//config:py_config",
        "//third-party/python:google-cloud-pubsub",
        "//services/ingest:ingest_py_proto",
    ],
    visibility = ["//visibility:public"],
)

