from django.apps import AppConfig

from google.api_core.exceptions import AlreadyExists
from google.cloud.pubsub import SchemaServiceClient
from google.pubsub_v1 import PublisherClient
from google.pubsub_v1.types import Schema

import py_config

import os

class ArchiveConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'archive'

    def ready(self):
        project_id = py_config.gcp_project_id()
        topic_id = py_config.gcp_topic_id()

        publisher = PublisherClient()
        topic_path = publisher.topic_path(project_id, topic_id)
        create_topic_request = {"name": topic_path}
        
        # Emulator doesn't support protobuf schemas
        using_emulator = "PUBSUB_EMULATOR_HOST" in os.environ
        if not using_emulator:
            schema_id = py_config.gcp_schema_id()

            # TODO don't hardcode absolute path
            proto_file = "/django_app/manage.runfiles/_main/services/ingest/proto/ingest.proto"
            with open(proto_file, "rb") as f:
                proto_source = f.read().decode("utf-8")

            schema_client = SchemaServiceClient()
            schema_path = schema_client.schema_path(project_id, schema_id)
            schema = Schema(name=schema_path, type_=Schema.Type.PROTOCOL_BUFFER, definition=proto_source)

            try:
                result = schema_client.create_schema(
                    request={"parent": f"projects/{project_id}", "schema": schema, "schema_id": schema_id},
                )
                print(f"Created a schema using a Protobuf schema file:\n{result}")
            except AlreadyExists:
                print(f"{schema_id} already exists")
            except Exception as e:
                print(f"Some other error: {e}")
                return

            create_topic_request["schema_settings"] = {
                "schema": schema_path,
                "encoding": "BINARY",
                # TODO what are these IDs actually
                "first_revision_id": "0",
                "last_revision_id": "0",
            }

        try:
            topic = publisher.create_topic(request=create_topic_request)
            print(f"Created topic with schema:\n{topic}")
        except AlreadyExists:
            print(f"{topic_id} already exists")


