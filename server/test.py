from flask import Flask, request, send_file, make_response
import os
import hashlib

tokens = {"test":"6285a343ccf1afb2226397569425e49a"}

app = Flask(__name__)

@app.route("/api/update",  methods=['POST'])
def update():
    if ("token" in request.json):
        t = request.json["token"]
        if t in tokens:
            url = tokens.pop(t)

            resp = make_response("", 201)
            resp.headers["Location"] = "/api/images/" + url
            return resp

        return "", 204
    return "", 400

@app.route("/api/images/<filename>")
def image(filename):
    if (filename in os.listdir("files")):
        return send_file(f"files/{filename}")
    return "", 404

@app.route("/api/new_token_image/<token>", methods=["POST"])
def token_image(token):
    if "file" not in request.files:
        print("Nya1")
        return "", 400

    file = request.files['file']

    if file.filename == '':
        print("Nya2")
        return "", 400

    BUF_SIZE = 2048
    md5 = hashlib.md5()

    while True:
        data = file.read(BUF_SIZE)
        if not data:
            break
        md5.update(data)

    file.seek(0)
    file.save("files/" + md5.hexdigest())

    tokens[token] = md5.hexdigest()

    return "", 200

app.run()