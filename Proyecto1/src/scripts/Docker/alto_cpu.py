# Este script consume mucha CPU realizando cálculos intensivos
import time
import math
for i in range(1000000):
    math.sqrt(12345)
    time.sleep(0.1)
