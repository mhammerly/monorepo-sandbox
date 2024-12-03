load("@rules_python//python:defs.bzl", "py_binary")

py_binary(
    name = "main",
    srcs = ["main.py"],
    deps = [
        "//third-party/python:orjson",
        "//third-party/python:cowsay",
    ],
)

py_test(
    name = "main-test",
    srcs = [],
    deps = [
        "//python:main",
    ],
)

test_suite(
    name = "suite",
    tests = [
        "//python:main-test",
    ],
)

py_binary(
    name = "call_ext",
    srcs = ["call_ext.py"],
    deps = [
        "//rust:return_foo",
    ],
)
