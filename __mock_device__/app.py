import time
from counterfit_connection import CounterFitConnection
from counterfit_shims_grove.grove_light_sensor_v1_2 import GroveLightSensor
from counterfit_shims_grove.grove_led import GroveLed
import paho.mqtt.client as mqtt
import json

id = '<client-id>'

client_metrics_topic = id + '/<feature>-metrics'
server_command_topic = id + '/<feature>-commands'
client_name = id

mqtt_client = mqtt.Client(client_name)
mqtt_client.connect('test.mosquitto.org')

mqtt_client.loop_start()

for i in range(5):
    message = {

    }
    mqtt_client.publish(client_metrics_topic, json.dumps(message))
    print(i)

    time.sleep(1)