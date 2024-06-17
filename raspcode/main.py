import RPi.GPIO as GPIO
import time

diodes = [4,27,22]

for gpio in diodes:
    GPIO.setmode(GPIO.BCM)
    GPIO.setup(gpio,GPIO.OUT)
    GPIO.output(gpio,GPIO.HIGH)
    time.sleep(1)
    GPIO.output(gpio,GPIO.LOW)

