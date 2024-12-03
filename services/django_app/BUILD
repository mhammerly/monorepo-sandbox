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
    name = "db_settings",
    imports = ["."],
    srcs = [
        "django_app/db_settings.py",
    ],
    deps = [
        "//third-party/python:psycopg",
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

py_library(
    name = "django_app",
    srcs = [
        "django_app/__init__.py",
        "django_app/asgi.py",
        "django_app/urls.py",
        "django_app/wsgi.py",
    ],
    deps = [
        ":settings",
        "//services/django_app/archive:archive",
    ],
    imports = ["."],
    visibility = ["//visibility:public"],
)
