import network
from time import sleep

from env import load_env

_env = load_env()
SSID = _env["SSID"]
PASSWORD = _env["PASSWORD"]


def connect(ssid=SSID, password=PASSWORD, timeout=15):
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)

    if not wlan.isconnected():
        print(f"Conectando a {ssid}...")
        wlan.connect(ssid, password)

        for _ in range(timeout * 2):
            if wlan.isconnected():
                break
            sleep(0.5)

    if not wlan.isconnected():
        raise RuntimeError("No se pudo conectar al WiFi")

    print("Conectado:", wlan.ifconfig())
    return wlan


if __name__ == "__main__":
    connect()
