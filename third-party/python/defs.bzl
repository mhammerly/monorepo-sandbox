load("//tools/platforms:defs.bzl", "platforms", "platform_list")
load("@rules_python//python:pip.bzl", "compile_pip_requirements")

# Key is a package name
# Value is None or a dict:
# {
#     "extras": ["binary"],
#     "version": (">=", "3.0.1"),
# }
PACKAGES = {
    "django": None,
    "django-admin": None,
    "psycopg": {
        "extras": ["binary"],
    },
    "google-cloud-pubsub": None,
    "protobuf": None,
    "grpcio": None,
}

def generate_requirements_in():
    reqs_lines = []
    for package, baggage in PACKAGES.items():
        version_str = ""
        extras_str = ""

        if baggage and "version" in baggage:
            op, version = baggage["version"]
            version_str = "{}{}".format(op, version)
        if baggage and "extras" in baggage:
            extras_str = "[{}]".format(",".join(baggage["extras"]))

        reqs_lines.append("{}{}{}".format(package, extras_str, version_str))

    native.genrule(
        name = "requirements_in",
        outs = ["requirements.in"],
        cmd = """
cat << EOF > $@
{}
EOF
""".format("\n".join(reqs_lines)),
    )

def generate_pypi_aliases():
    for package in PACKAGES:
        normalized_name = package.replace("-", "_").replace(".", "_").lower()
        native.alias(
            name = package,
            actual = select({
                platform.config: "{}//{}".format(platform.pypi_repo, normalized_name)
                for platform in platform_list
            }),
            visibility = ["//visibility:public"],
        )
