import ujson
import urequests

from env import load_env

SERVER_URL = load_env()["SERVER_URL"]


class HttpClient:
    def __init__(self, base_url=SERVER_URL):
        self._base_url = base_url

    def send(self, data, path=""):
        url = self._base_url + path
        response = urequests.post(
            url,
            data=ujson.dumps(data),
            headers={"Content-Type": "application/json"},
        )
        try:
            return response.status_code, response.text
        finally:
            response.close()
