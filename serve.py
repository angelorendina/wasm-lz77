import http.server
import socketserver
import sys

class MyHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        return http.server.SimpleHTTPRequestHandler.do_GET(self)

handler_object = MyHttpRequestHandler

try:
    PORT = int(sys.argv[1])
except:
    PORT = 8080

class MyServer(socketserver.TCPServer):
    allow_reuse_address = True

my_server = MyServer(("", PORT), handler_object)

print("[serve.py] Started server on localhost:" + str(PORT))

try:
    my_server.serve_forever()
except KeyboardInterrupt:
    print("[serve.py] Stopping server on locahost:" + str(PORT))
finally:
    my_server.server_close()
