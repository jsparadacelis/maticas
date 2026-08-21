from machine import Pin
from time import sleep

# On the Pico W the onboard LED is wired to the WiFi chip, not a plain
# GPIO, so it's addressed by name ("LED") instead of a pin number.
led = Pin("LED", Pin.OUT)

while True:
    led.toggle()
    sleep(10)
