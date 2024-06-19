import requests
from dotenv import load_dotenv
import os
import socket


decision = bool(int(input("Create 0. new or 1. check ? 0/1")))
load_dotenv()
uuid = os.getenv("uuid") # uuid paczkomatu wygenerowane wcześniej
ip_address = socket.gethostbyname(socket.gethostname()) # adres IP w sieci lokalnej (może kiedyś przerobię to na sięć publiczą)
BASE_URL = os.getenv("server_url")

if decision:
    requests.patch(url=f"{BASE_URL}/ip/check/", data={"id": uuid, "ip_address": ip_address})

else:
    requests.post(url=f"{BASE_URL}/paczkomat/add_paczkomat/", data={"id": uuid, "ip_address": ip_address})



