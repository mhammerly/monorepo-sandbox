load("//:config.bzl", "container_repo")
load("//tools/platforms:defs.bzl", "platforms")
load("//tools:docker.bzl", "multiarch_image")
load("@rules_oci//oci:defs.bzl", "oci_load", "oci_image", "oci_image_index", "oci_push")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")

pkg_tar(
    name = "django_layer",
    srcs = ["//services/django_app:manage"],
    package_dir = "django_app",
    include_runfiles = True,
)

oci_image(
    name = "image",
    base = "@python-slim",
    tars = [":django_layer"],
    # We don't actually have `manage.py` and the regular project hierarchy. We
    # have this `manage` executable and its runfiles. Another solution is needed
    # for local development that supports hot-reloading.
    entrypoint = ["/bin/sh", "-c", "/django_app/manage migrate && /django_app/manage runserver 0.0.0.0:8000"],
    target_compatible_with = ["@platforms//os:linux"],
)

# $ bazel run //services/django_app:load_specific --platforms //tools/platforms:linux_aarch64
# $ docker run -p 8000:8000 --rm <container-repo>/django:latest
oci_load(
    name = "load_specific",
    image = ":image",
    repo_tags = ["{}/django:latest".format(container_repo)],
)

multiarch_image(
    name = "_multiarch_image",
    image = ":image",
    platforms = [
        platforms.linux_amd64.label,
#        platforms.linux_aarch64.label, # TODO fix multiplatform for django
    ],
)

oci_image_index(
    name = "multiarch_image",
    images = [":_multiarch_image"],
)

# $ bazel run //services/django_app:load_multiarch
# $ docker run -p 8000:8000 --rm <container-repo>/django:latest
oci_load(
    name = "load_multiarch",
    image = ":multiarch_image",
    repo_tags = ["{}/django:latest".format(container_repo)],
    format = "oci",
)

oci_push(
    name = "push",
    image = ":multiarch_image",
    repository = "{}/django".format(container_repo),
    remote_tags = ["latest"],
)
