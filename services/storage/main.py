import django
django.setup()

from concurrent import futures
import grpc

from archive import models
from services.storage.proto import storage_pb2, storage_pb2_grpc
from google.protobuf.timestamp_pb2 import Timestamp

class StorageService(storage_pb2_grpc.StorageServiceServicer):
    def SaveArchive(self, request, context):
        print("SaveArchive: ", request.url)
        archive = models.ArchivedSite.objects.create(archive_url=request.url, archive_content=request.content)
        created = Timestamp()
        created.FromDatetime(archive.created)
        return storage_pb2.SaveResponse(id=1, created_at=created)

    def LoadArchive(self, request, context):
        print("LoadArchive: ", request.id)
        result = models.ArchivedSite.get(pk=request.id)
        return storage_pb2.LoadResponse(content=result.content, created_at=result.created)

    def CheckExists(self, request, context):
        print("CheckExists: ", request.url)
        ids = models.ArchivedSite.objects.filter(url=request.url).values_list('id', flat=True)
        return storage_pb2.ExistsResponse(ids=list(ids))

class Interceptor(grpc.ServerInterceptor):
    def intercept_service(self, continuation, handler_call_details):
        print("incoming request")
        try:
            result = continuation(handler_call_details)
            print("finished processing request")
            return result
        except Exception as e:
            print("exception while processing request")
            return

def main():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10), interceptors = (Interceptor(),))
    storage_pb2_grpc.add_StorageServiceServicer_to_server(StorageService(), server)
    server.add_insecure_port("0.0.0.0:50051")
    server.start()
    print("storage service started, listening on 0.0.0.0:50051")
    server.wait_for_termination()
    print("terminated")

if __name__ == "__main__":
    main()
