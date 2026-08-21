from machine import ADC


class Sensor:
    def read(self):
        raise NotImplementedError


class InternalTempSensor(Sensor):
    """Sensor de temperatura integrado en el RP2040 (ADC canal 4)."""

    _CONVERSION_FACTOR = 3.3 / 65535

    def __init__(self):
        self._adc = ADC(4)

    def read(self):
        voltage = self._adc.read_u16() * self._CONVERSION_FACTOR
        # Formula del datasheet del RP2040 (seccion 4.9.5).
        temperature_c = 27 - (voltage - 0.706) / 0.001721
        return {"temperature_c": round(temperature_c, 2)}
