import requests
from dotenv import load_dotenv
import os
import socket


decision = bool(input("Create 0. new or 1. check ? 0/1"))

if decision:
    url = "/ip/check/"
else:
    url = "/paczkomat/add_paczkomat/"

load_dotenv()
uuid = os.getenv("uuid") # uuid paczkomatu wygenerowane wcześniej
ip_address = socket.gethostbyname(socket.gethostname()) # adres IP w sieci lokalnej (może kiedyś przerobię to na sięć publiczą)



requests.patch(url=f"{os.getenv("server_url")}{url}", data={"id": uuid, "ip_address": ip_address})
