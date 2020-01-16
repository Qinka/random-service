import pymongo
from flask import Flask, redirect, request, make_response, abort, render_template, Response, send_file
from datetime import datetime
import json

def test_auth(col, token, right):
    rt = col.find_one({"token": token})
    print(rt)
    if 'expire' in rt and rt['expire'].time != 0:
        if rt['expire'].time < datetime.utcnow().timestamp():
            return abort(Response(response=json.dumps(dict(code = 4, message = f"Invailed token ``{token}''", data = None)), status = 403))
    if right in rt['right']:
        return None
    else:
        return abort(no_auth)

no_auth = Response(response=json.dumps(dict(code = 5, message = "No authorization", data = None)), status = 403)