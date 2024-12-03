# key: platform name
# value: list of constraint values
_platforms = {
    "linux_amd64": [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
    "linux_aarch64": [
        "@platforms//os:linux",
        "@platforms//cpu:aarch64",
    ],
    "macos_amd64": [
        "@platforms//os:macos",
        "@platforms//cpu:x86_64",
    ],
    "macos_aarch64": [
        "@platforms//os:macos",
        "@platforms//cpu:aarch64",
    ],
}

def _platform_struct(platform):
    return struct(
        name = platform,
        label = "//tools/platforms:{}".format(platform),
        config = "//tools/platforms:is_{}".format(platform),
        pypi_repo = "@pypi-{}".format(platform),
    )

# Single place for platform-related strings so they aren't hand-written
# everywhere with mistakes.
platforms = struct(**{
    platform: _platform_struct(platform)
    for platform in _platforms.keys()
})

# List of platforms in `platforms`. Useful for dictionary comprehensions for
# `select()`s:
# select({
#    platform.config: "@{}//{}".format(package.pypi_repo, pkg_name)
#    for platform in platform_list
# })
platform_list = [getattr(platforms, k) for k in dir(platforms) if k not in ("to_json", "to_proto")]

def setup_platforms_and_configs():
    for platform, constraint_values in _platforms.items():
        native.platform(
            name = platform,
            constraint_values = constraint_values,
            visibility = ["//visibility:public"],
        )

        native.config_setting(
            name = "is_" + platform,
            constraint_values = constraint_values,
            visibility = ["//visibility:public"],
        )

