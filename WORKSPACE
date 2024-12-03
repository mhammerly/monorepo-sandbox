# # # # # # # #
# # PyO3 setup
# # # # # # # #
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_pyo3",
    integrity = "sha256-ObyBGmMiWQb9WYBnWX1RyXNssALyplEVD5iLbWLDOO4=",
    urls = ["https://github.com/abrisco/rules_pyo3/releases/download/0.0.6/rules_pyo3-v0.0.6.tar.gz"],
)

load("@rules_pyo3//pyo3:repositories.bzl", "register_pyo3_toolchains", "rules_pyo3_dependencies")
rules_pyo3_dependencies()
register_pyo3_toolchains()

load("@rules_pyo3//pyo3:repositories_transitive.bzl", "rules_pyo3_transitive_deps")
rules_pyo3_transitive_deps()
