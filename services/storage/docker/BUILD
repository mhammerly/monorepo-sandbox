load("//:config.bzl", "container_repo")
load("//tools/platforms:defs.bzl", "platforms")
load("//tools:docker.bzl", "multiarch_image")
load("@rules_oci//oci:defs.bzl", "oci_load", "oci_image", "oci_image_index", "oci_push")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

pkg_tar(
    name = "storage_layer",
    srcs = ["//services/storage:main"],
    package_dir = "storage",
    include_runfiles = True,
)

oci_image(
    name = "image",
    base = "@python-slim",
    tars = [":storage_layer"],
    entrypoint = ["/bin/sh", "-c", "/storage/main"],
)

# $ bazel run //services/storage:load_specific --platforms //tools/platforms:linux_aarch64
# $ docker run -p 8000:8000 --rm <container-repo>/storage:latest
oci_load(
    name = "load_specific",
    image = ":image",
    repo_tags = ["{}/storage:latest".format(container_repo)],
)

multiarch_image(
    name = "_multiarch_image",
    image = ":image",
    platforms = [
        platforms.linux_amd64.label,
        platforms.linux_aarch64.label,
    ],
)

oci_image_index(
    name = "multiarch_image",
    images = [":_multiarch_image"],
)

# $ bazel run //services/storage:load_multiarch
# $ docker run -p 8000:8000 --rm <container-repo>/storage:latest
oci_load(
    name = "load_multiarch",
    image = ":multiarch_image",
    repo_tags = ["{}/storage:latest".format(container_repo)],
    format = "oci",
)

oci_push(
    name = "push",
    image = ":multiarch_image",
    repository = "{}/storage".format(container_repo),
    remote_tags = ["latest"],
)
