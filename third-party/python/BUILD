load(":defs.bzl", "generate_requirements_in", "generate_pypi_aliases")
load("@rules_python//python:pip.bzl", "compile_pip_requirements")

generate_requirements_in()

# This requirements file does not consider the target platform at all. If there
# are OS-specific or arch-specific dependencies, they could be wrong.
compile_pip_requirements(
    name = "requirements",
    srcs = ["//third-party/python:requirements.in"],
)

generate_pypi_aliases()
