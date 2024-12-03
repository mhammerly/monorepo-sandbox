A sandbox to tinker with monorepo configuration, distributed system interfaces,
and development experiences.

Implements an unserious webpage-archiving service with a few components:
- `services/django_app`: a website where you can enqueue a site to be archived
- `services/ingest`: a Rust service that creates the archive
- `services/storage`: a Python service that saves the archive to the database

`django_app` talks to `ingest` by way of a Pub/Sub queue. `ingest` talks to
`storage` via gRPC, and `storage` connects to a PostgreSQL database. There's a
`docker-compose.yml` file in the workspace root that sets up containers for the
services (provided they've been built), postgres, and the Pub/Sub emulator.

# Setting up

### Container artifact repository
Create a container registry on your chosen host (e.g. GCR) and configure its URI
in two places:
- the `container_repo` constant in `_config.bzl`. For instance:
  ```
  # _config.bzl
  container_repo = "<your-location>-docker.pkg.dev/<your-project>/<your-repo>"
  ```
- the `$CONTAINER_REPO` environment variable (same value as above)

If you don't want to push/pull your images from anywhere you still need to put a
stub value in `_config.bzl`:
```
# _config.bzl
container_repo = None
```

### Remote build cache (optional)
Set up your preferred build cache backend (e.g. GCS) and set the relevant
options in `.bazelrc.local`. For example:
```
# .bazelrc.local
build --remote_cache=https://storage.googleapis.com/<your-bucket-name> --google_default_credentials
```

### GCP configuration (optional)
The default env var values in `.env` will allow you to run the service with the
Pub/Sub emulator, but if you want to run against a real Pub/Sub queue you will
need to set some details in environment variables:
- (required) `GCP_PROJECT_ID`
- (optional) `GCP_TOPIC_ID` (default `ingest-topic`)
- (optional) `GCP_SCHEMA_ID` (default `ingest-schema`)
- (optional) `GCP_SUBSCRIPTION_ID` (default `ingest-subscription`)

These values are loaded in `docker-compose.yml` and accessed in the services so
you can plug in your own GCP demo project details.

### Building and running
The fastest way to run the service is by building/loading a single-arch version
of each service's container image:
```bash
$ bazelisk run //services/storage/docker:load_single --platforms=//tools/platforms:linux_amd64
$ bazelisk run //services/django_app/docker:load_single --platforms=//tools/platforms:linux_amd64
$ bazelisk run //services/ingest/docker:load_single --platforms=//tools/platforms:linux_amd64
```
You can use `linux_aarch64` instead if you are on an arm64 machine.

After building and/or pushing the images, you can start the service:
```bash
$ docker compose up postgres pubsub --detach
$ docker compose up django ingest storage
```

# Monorepo "features"
For an elevator pitch of a build system like Bazel, see ["Why Bazel?"](https://bazel.build/about/why).
For the most part, the same benefits can be realized with similar build systems
like [Pants](https://www.pantsbuild.org/) or [Buck2](https://buck2.build/). The
gist is that they provide build parallelization, incremental builds, caching,
cross-compilation, dependency graph manipulation, and more, generically over
different languages. The downside being that they're less turn-key.

This repository has no innovations of its own; rather, it demonstrates how to
use off-the-shelf components and some community wisdom to support a variety of
non-trivial build tasks.

Generally everything is configured in the root `MODULE.bazel`, the sub-modules
that it `include()`s, and various `.bzl` files in the `tools` subdirectory tree.
Some things may be set up in `WORKSPACE` if they don't support bzlmod yet.

## Python projects
`services/django/BUILD` and `services/storage/BUILD` contain straightforward
uses of the Python rules from `rules_python`.

Samples:
```python
load("@rules_python//python:defs.bzl", "py_binary", "py_library")
py_binary(
    name = "manage",
    srcs = ["manage.py"],
    imports = ["."],
    deps = [
        "//third-party/python:django",
        "//services/django_app:django_app",
        "//services/django_app/archive:archive",
    ],
    visibility = ["//visibility:public"],
)

py_library(
    name = "settings",
    srcs = ["django_app/settings.py"],
    imports = ["."],
    deps = [":db_settings"],
    visibility = ["//visibility:public"],
)
```

`services/django/BUILD` and `services/django/archive/BUILD` show a way (albeit a
messy one) to break a Django project + apps into Bazel targets.

### Third-party `pip`/PyPI dependencies
Packages from `pip`/PyPI are exposed under `//third-party/python` (for example,
`//third-party/python:django`). To add a new package, update the `PACKAGES`
constant in `third-party/python/defs.bzl` and run the update script:
```bash
$ bazelisk run //third-party/python:requirements.update
```

Packages must have a precompiled wheel uploaded to PyPI to work here.

Under the hood, each package is exported several times from per-platform PyPI
repositories so we can get the right wheels for different target platforms. Each
platform-specific version of the package is initially exposed with a label like
`@pypi-x86_64_linux//django` but for convenience a single `alias()` rule is set
up which points at the correct platform-specific version of the package. The
`PACKAGES` constant is used to both generate a `requirements.in` file and the
`alias`es.

NOTE: The generated `requirements.txt` lock file is built without consideration
for different target platforms. Wheels will still be downloaded for the right
target platform, but if a package supposed to have a certain dependency only on
Linux or on aarch64, that won't be reflected in the `requirements.txt` file.

Sample of depending on a PyPI package:
```python
py_library(
    name = "my_lib",
    srcs = glob(["*.py"]),
    deps = [
        "//third-party/python:django",
        "//third-party/python:psycopg",
    ]
)
```

## Rust projects
`services/ingest/BUILD` contains a straightforward definition of a Rust binary
using the `rust_binary` rule from `rules_rust`. `archive/BUILD` and
`config/BUILD` contain straightforward uses of `rust_library` from the same.

Samples:
```python
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

rust_binary(
    name = "main",
    srcs = ["src/main.rs"],
    deps = [
        "@crates.io//:google-cloud-pubsub",
        "@crates.io//:google-cloud-googleapis",
        ...
    ],
    visibility = ["//visibility:public"],
)

rust_library(
    name = "archive",
    srcs = ["src/lib.rs"],
    deps = [
        "@crates.io//:web-archive",
    ],
)
```

### Third-party crates.io dependencies
Packages from https://crates.io are exposed by adding them to a dummy Rust
project's `Cargo.toml` and running `cargo build` to generate a `Cargo.lock`
file. The dummy project's `Cargo.toml` is at `/third-party/rust/Cargo.toml`.

NOTE: This process may need to be changed to handle per-platform dependencies
or features.

Sample of depending on a crates.io package:
```python
rust_binary(
    name = "my_lib",
    srcs = ["main.rs"],
    deps = [
        "@crates.io//:ferris-says",
    ],
)
```

## PyO3 extensions
`config/BUILD` and `archive/BUILD` show how to use the `pyo3_extension` rule
from `rules_pyo3` to define a Python native extension written in Rust.

Sample:
```python
load("@rules_pyo3//pyo3:defs.bzl", "pyo3_extension")

pyo3_extension(
    name = "pyo3_archive",
    srcs = ["src/lib.rs"],
    crate_features = ["pyo3"],
    imports = ["."],
    deps = [
        "@crates.io//:web-archive",
    ],
)
```
`pyo3_extension` targets are dependend on just like regular `python_library`
targets. In this case, the Python module's name will be `pyo3_archive` (because
of the module name in `src/lib.rs`, not necessarily the target name) and, due to
the `imports = ["."]` keyword argument, it can be imported directly as
`pyo3_archive` (as opposed to something like `long.path.preceeding.pyo3_archive`).

## Protobufs, gRPC
Rust and Python bindings are automatically generated for protobufs using the
custom `multilang_proto_library()` macro defined in `tools/proto.bzl`. See
examples in `services/storage/BUILD` and `services/ingest/BUILD`.

Sample:
```python
load("//tools:proto.bzl", "multilang_proto_library")

multilang_proto_library(
    name = "ingest",
    srcs = ["proto/ingest.proto"],
)
```

The above macro invocation will generate a few targets:
- `:ingest_proto`, a `proto_library` (from `@protobuf`) for the protobuf schema
- `:ingest_py_proto`: a `py_proto_library` (from `@grpc`) for Python schema
  bindings
- `:ingest_py_grpc`: a `py_grpc_library` (from `@grpc`) for Python gRPC bindings
  for any services defined in the protobuf schema
- `:ingest_rs_proto`: a `rust_prost_library` (from `@rules_rust`) for Rust
  bindings for the protobuf schema as well as gRPC service traits.

For samples of using the generated bindings from Python see `services/storage/main.py`.
For samples of using the generated bindings from Rust see `services/ingest/src/main.rs`.

## Docker/OCI container images
`rules_oci` provides rules for building, pulling, and pushing docker/OCI images
with Bazel. See samples in `services/django_app/docker/BUILD` (the corresponding
files for the other services are essentially identical).

Build each service's image for a specific architecture:
```bash
$ bazelisk run //services/storage/docker:load_single --platforms=//tools/platforms:linux_amd64
$ bazelisk run //services/django_app/docker:load_single --platforms=//tools/platforms:linux_amd64
$ bazelisk run //services/ingest/docker:load_single --platforms=//tools/platforms:linux_amd64
```

Build a multi-arch image for each service:
```bash
$ bazelisk run //services/storage/docker:load_multiarch
$ bazelisk run //services/django_app/docker:load_multiarch
$ bazelisk run //services/ingest/docker:load_multiarch
```

If you have a container registry configured, push the multi-arch image for each
service:
```
$ bazelisk run //services/storage/docker:push
$ bazelisk run //services/django_app/docker:push
$ bazelisk run //services/ingest/docker:push
```

### Third-party images
Third-party images from `gcr.io` or `docker.io` can be exposed with the `oci`
Bazel extension. To expose a new image, add a call to `oci.pull` for it in
`/third-party/docker/docker.MODULE.bazel`. The root module (`/MODULE.bazel`)
uses `include()` (Bazel 7.2) to incorporate the contents of our docker-specific
`docker.MODULE.bazel`.

Each third-party image is exposed under a name that includes the image name and
target architecture (e.g. `@python-slim_linux_amd64`). The `rules_oci` rules
also expose a platform-agnostic `@python-slim` that should resolve to the
correct underlying image for the current target platform.

Example of depending on an exposed base iamge:
```python
oci_image(
    name = "django_image",
    base = "@python-slim",
    tars = [":django_layer"],
    entrypoint = ["/bin/sh", "-c", "/django_app/manage migrate && /django_app/manage runserver 0.0.0.0:8000"],
)
```

### Third-party package managers
Packages from Debian repos can be exposed for inclusion in Docker images with
the `apt` plugin from `rules_distroless`. To add a new package, modify
`/third-party/deb/bullseye.yaml` and run the following to update the lock file:
```bash
$ bazelisk run "@bookworm//:lock"
```

Each package is exposed under a name that includes the package name and target
architecture (e.g. `@bookworm//libpq-dev/amd64`). If you are using a deb package
in a situation where we support multiple platforms, you'll have to use `select()`
to resolve the correct one.

Each package exposes a `:data` target which can be used in `pkg_tar` archives or
directly as layers in `oci_image`.

Example of depending on an exposed debian package:
```python
pkg_tar(
    name = "django_layer",
    srcs = ["//services/django_app:manage"] + select({
        "//tools/platforms:is_aarch64_linux": ["@bookworm//libpq-dev/arm64:data"],
        "//tools/platforms:is_x86_64_linux": ["@bookworm//libpq-dev/amd64:data"],
    }),
    package_dir = "django_app",
    include_runfiles = True,
)
```

## Cross-compilation
Assuming the host machine is some sort of modern Mac, the Python code, Rust
code, and container images in this repository can be built for a variety of
different platforms:
- `//tools/platforms:linux_amd64`
- `//tools/platforms:linux_aarch64`
- `//tools/platforms:macos_amd64` (except container images)
- `//tools/platforms:macos_aarch64` (except container images)

Many developer machines these days are `macos_aarch64` Macbooks while the
typical CI runner is probably `linux_amd64`. While `macos_aarch64` can run a
`linux_amd64` container image with Docker, it will spew warnings about the
architecture mismatch. Containers aside, some of this code could also be built
and run on the host machine or distributed as a developer tool. Whatever the
reason, this repository is set up so a (presumably `macos_aarch64`) host can
cross-compile for a few different targets.

### PyPI
As explained above, we expose PyPI packages multiple times in platform-specific
PyPI "repositories". This allows us to download the correct version of, for
instance, `psycopg[binary]` and its native extension which is precompiled for
different architectures.

### Rust
The `extra_target_triples` argument to `rust.toolchain()` in `MODULE.bazel`
handles this for the most part.

### LLVM sysroots
The LLVM toolchain itself can be reused for different target platforms, but we
need to provide a "sysroot" appropriate for each target. That all gets set up in
`tools/llvm.MODULE.bazel`. This comes into play when cross-compiling Rust code
in... certain scenarios.

### Building a vendored OpenSSL
Cross-compiling OpenSSL is a pain. Sometimes a Rust project can use `rustls` or
`boringssl` or some other alternative, but in this case a dependency explicitly
selects `openssl`.

The solution has two parts:
- `third-party/rust/cratesio.MODULE.bazel` adds an annotation to `openssl-sys`
  to compile a "vendored" copy of OpenSSL
- `third-party/openssl/openssl.BUILD` uses `rules_foreign_cc` to teach Bazel
  how to run the `make`-based build process for OpenSSL with some configurations

This is a particularly fussy example of needing to cross-compile C/C++ code with
the LLVM toolchains described above.

## Remote build cache
Bazel supports using an HTTP API to `PUT`/`GET` build artifacts from a cache
to improve build speed. A populated remote cache makes a considerable difference
for this project because it compiles OpenSSL and protobuf things for multiple
platforms which can take a while.

| target | empty remote cache | populated remote cache |
| ------ | ------------------ | ---------------------- |
| `//services/storage/docker:load_multiarch` | 25m 32s | 1m 48s |
| `//services/django_app/docker:load_multiarch` | 4m 14s | 41s |
| `//services/ingest/docker:load_multiarch` | 4m 17s | 10s |

Measurements were taken by running the following series of commands twice in a
row, once to populate the cache and once to try reading from it:
```bash
$ bazelisk clean
$ bazelisk run //services/storage/docker:load_multiarch
$ bazelisk run //services/django_app/docker:load_multiarch
$ bazelisk run //services/ingest/docker:load_multiarch
```

The cache backend used was a GCS bucket. Afterwards, the cache took up about
3.75GB:
```bash
$ gcloud storage du --summarize --readable-sizes --total
3.74GiB      gs://<redacted>
```

# Testing

TODO write some

## Locally

TODO should just be `bazelisk test //path/to/target`. Can bazel somehow
associate a library with its tests like Buck does?

## "Smart" testing in CI
TODO
- For each file modified in the commit, get the target it belongs to:
  ```bash
  file_label=$(bazel query /path/to/file.py)
  bazel query "attr('srcs', $file_label, ${file_label//:*I/}/...)"
  ```
- Building on the above `bazel query` invocation, get the reverse dependencies
  for that target, N=3 layers deep:
  ```bash
  bazel query "rdeps(//..., attr('srcs', $file_label, ${file_label//:*/}/...), 3)"
  ```
- Building on the above again, filter the reverse dependencies to get only tests
  ```bash
  bazel query "kind(test, rdeps(//..., attr('srcs', $file_label, ${file_label//:*/}/...), 3))"
  ```

TODO see if this query can be performed on a set of files at the same time

# Local development
The development workflow is not great.

## Django migrations
```bash
# Start postgres
docker compose up postgres

$ Making migrations
$ bazelisk run //services/django_app:manage makemigrations
Migrations for 'archive':                                                                                                                                                                                    │oading.
  /private/var/tmp/_bazel_matt/9d85013bc7892b9289e73ab698a99aad/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/django_app/manage.runfiles/_main/django_app/archive/migrations/0001_initial.py
    + Create model ArchivedSite
$ cp /private/var/tmp/_bazel_matt/9d85013bc7892b9289e73ab698a99aad/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/django_app/manage.runfiles/_main/django_app/archive/migrations/*.py django_app/archive/migrations

# Running migrations
$ bazelisk run //services/django_app:manage migrate
```
TODO: Make creating migrations less awful.


## Hot-reloading
- Need to figure something nicer out for Django
- Check out https://github.com/bazelbuild/bazel-watcher

Outside of the container, `manage.py` has been modified to `cd` into the Bazel workspace directory rather
than an output directory so it will look at the actual source and hot-reload when
there are changes. This works for changing Python sources and HTML templates but
it will not keep in sync if there are changes to `BUILD` files.

## Linting
TODO

## IDE integrations
TODO

### rust-analyzer
There is some rudimentary support: https://bazelbuild.github.io/rules_rust/rust_analyzer.html
Work is in progress on a better integration: https://github.com/bazelbuild/rules_rust/issues/2755

### Python
TODO

# Miscellany

## Code coverage
TODO
