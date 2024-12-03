# https://github.com/bazelbuild/rules_rust/issues/1519#issuecomment-2459048249

load("@//tools/platforms:defs.bzl", "platforms")
load("@rules_foreign_cc//foreign_cc:defs.bzl", "configure_make")

filegroup(
    name = "all_srcs",
    srcs = glob(
        include = ["**"],
        exclude = ["*.bazel"],
    ),
)

# https://wiki.openssl.org/index.php/Compilation_and_Installation
CONFIGURE_OPTIONS = select({
    "@{}".format(platforms.linux_amd64.config): ["linux-x86_64"],
    "@{}".format(platforms.linux_aarch64.config): ["linux-aarch64"],
    "@{}".format(platforms.macos_amd64.config): ["darwin64-x86_64", "-static"],
    "@{}".format(platforms.macos_aarch64.config): ["darwin64-arm64", "-static"],
}) + [
    "--libdir=lib64", # `third-party/rust/cratesio.MODULE.bazel` assumes this
    "no-comp",
    "no-idea",
    "no-weak-ssl-ciphers",
    "no-shared",
]

MAKE_TARGETS = [
    "build_libs",
    "install_dev",
]

configure_make(
    name = "openssl",
    args = ["-j12"],
    configure_command = "config",
    configure_in_place = True,
    configure_options = CONFIGURE_OPTIONS,
    lib_name = "openssl",
    lib_source = ":all_srcs",
    env = select({
        "@{}".format(platforms.macos_amd64.config): { "AR": "" },
        "@{}".format(platforms.macos_aarch64.config): { "AR": "" },
        "@//conditions:default": {},
    }),
    out_lib_dir = "lib64",
    out_shared_libs = [],
    out_static_libs = ["libssl.a"],
    targets = MAKE_TARGETS,
    visibility = ["//visibility:public"],
)

filegroup(
    name = "gen_dir",
    srcs = [":openssl"],
    output_group = "gen_dir",
    visibility = ["//visibility:public"],
)
