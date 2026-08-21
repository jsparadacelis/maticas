import urequests

from wifi import connect

SERVER_URL = "https://maticas-server-production.up.railway.app/"

connect()

resp = urequests.post(SERVER_URL, data="hello from pico")
print("status:", resp.status_code)
print(resp.text)
resp.close()
