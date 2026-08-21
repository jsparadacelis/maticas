from time import sleep

from wifi import connect
from sensor import InternalTempSensor
from http_client import HttpClient

READ_INTERVAL_S = 10

connect()

sensor = InternalTempSensor()
client = HttpClient()

while True:
    data = sensor.read()
    status, body = client.send(data)
    print(data, "->", status, body)
    sleep(READ_INTERVAL_S)
