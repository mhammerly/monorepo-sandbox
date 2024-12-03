import urllib

from django.http import HttpResponse, HttpResponseRedirect
from django.urls import reverse
from django.shortcuts import render
from google.pubsub_v1 import PublisherClient, PublishRequest
from google.pubsub_v1.types import PublishRequest, PubsubMessage

import py_config
from .models import ArchivedSite

from services.ingest.proto import ingest_pb2
from pyo3_archive import archive

def index(request):
    # TODO: list archives
    return HttpResponse(archive("https://about.codecov.io"))

def create(request):
    if request.method == "GET":
        return render(request, "archive/create.html")
    else:
        url = request.POST["url"]
        content = archive(url)

        ingest_request = ingest_pb2.IngestRequest()
        ingest_request.url = url
        data = ingest_request.SerializeToString()

        project_id = py_config.gcp_project_id()
        topic_id = py_config.gcp_topic_id()

        publisher = PublisherClient()
        topic_path = publisher.topic_path(project_id, topic_id)
        request = PublishRequest(
            topic=topic_path,
            messages=[PubsubMessage(data=data)],
        )

        publish_response = publisher.publish(request)
        print(f"Published message ID: {publish_response}", flush=True)
        print(f"Published to topic {topic_path}");

        return HttpResponseRedirect(reverse("index"))

def preview(request, site):
    url = urllib.parse.unquote(site)
    content = archive(url)
    return HttpResponse(content)
