from flask import Flask, request

app = Flask(__name__)

@app.route("/api/update",  methods=['POST'])
def update():
    if ("token" in request.json):
        print("token:", request.json["token"])
    return "test", 200

app.run()