import orjson
import cowsay

cowsay.cow(orjson.dumps({"hello": "world"}))
#print(orjson.dumps({"hello": "world"}))
