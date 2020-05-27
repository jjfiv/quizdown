from flask import Flask, Response, jsonify
import mimetypes
import subprocess

mimetypes.add_type("application/wasm", ".wasm")

app = Flask(__name__, static_folder="pkg")


@app.route("/")
def index():
    # exec = subprocess.run(
    #    ["wasm-pack", "build", "--target", "web"], stderr=subprocess.PIPE
    # )
    # if exec.returncode != 0:
    #    return exec.stdout
    with open("index.html") as fp:
        index_html = fp.read()
    return Response(index_html, mimetype="text/html")


if __name__ == "__main__":
    app.run(host="0.0.0.0", debug=True, port=5001)
