import requests
from dotenv import load_dotenv
import os
import socket


decision = bool(int(input("Create 0. new or 1. check ? 0/1")))
load_dotenv()
uuid = os.getenv("uuid") # uuid paczkomatu wygenerowane wcześniej

s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.connect(("8.8.8.8", 80))
ip_address=s.getsockname()[0]
s.close() # adres IP w sieci lokalnej (może kiedyś przerobię to na sięć publiczą)

BASE_URL = os.getenv("server_url")

if decision:
    requests.patch(url=f"{BASE_URL}/ip/check/", data={"id": uuid, "ip_address": ip_address})

else:
    requests.post(url=f"{BASE_URL}/paczkomat/add_paczkomat/", data={"id": uuid, "ip_address": ip_address})



