#!/usr/bin/python3
import os
import subprocess
import time
from sys import platform
os.chdir('article')

reloadscript = """
		<script>

			function getTimestamp() {
				return fetch("timestamp.txt")
					.then(response => response.text())
					.then(function (data) { 
						return data;
					})
			}

			var timestamp = "";
			getTimestamp().then(ts => timestamp = ts);

			
			setInterval(function(){
				getTimestamp().then(function (newts) {
					if(newts != timestamp){ 
						document.location.reload(); 
					}
				});
			}, 2000);
		</script>
	</body>
"""

def regenerate():
    print("REGENERATING WEBSITE")
    f = open("ch_template.html","r", encoding="utf-8")
    template_lines = f.readlines()

    if platform == "win32":
        ch_html = subprocess.run(['cmd','/C', r'type ch.md | pulldown-cmark -F -S'.encode('utf-8')], capture_output=True, encoding='UTF-8').stdout
    else:
        ch_html = subprocess.run(['bash', '-c', 'cat ch.md | pulldown-cmark -F -S'], capture_output=True, encoding='UTF-8').stdout

    # Write final version
    out = open("ch.html", "w", encoding="utf-8")

    for line in template_lines:
        processed_line = line.replace("CONTENTS", ch_html)
        out.write(processed_line)

    out.close()

    # Write development version
    out = open("ch_dev.html", "w", encoding="utf-8")

    for line in template_lines:
        processed_line = line.replace("CONTENTS", ch_html).replace("</body>", reloadscript)
        out.write(processed_line)

    out.close()


    timestampout = open("timestamp.txt", "w", encoding="utf-8")
    timestampout.write(str(time.monotonic_ns()))
    timestampout.close()


# Monitor modified files
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
class MyHandler(FileSystemEventHandler):
    def on_modified(self, event):
        print(event.src_path[2:])
        if ("ch.md" in event.src_path) or  \
                ("ch_template.html" in event.src_path) or \
                ("ch.css" in event.src_path):
            print("Change in tracked file detected")
            print(event)
            regenerate()



regenerate()

event_handler = MyHandler()
observer = Observer()
observer.schedule(event_handler, path='.', recursive=False)
observer.start()



# Serve contents
import http.server
import socketserver
import socket

PORT = 8000

class MyTCPServer(socketserver.TCPServer):
    def server_bind(self):
        self.socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.socket.bind(self.server_address)

Handler = http.server.SimpleHTTPRequestHandler

httpd = MyTCPServer(("", PORT), Handler)
print("Serving on port", PORT)
httpd.serve_forever()
