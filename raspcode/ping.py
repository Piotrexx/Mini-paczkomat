import requests
from dotenv import load_dotenv
import os
import socket

load_dotenv()
uuid = os.getenv("uuid") # uuid paczkomatu wygenerowane wcześniej
ip_address = socket.gethostbyname(socket.gethostname()) # adres IP w sieci lokalnej (może kiedyś przerobię to na sięć publiczą)



requests.patch(url=os.getenv("server_url"), data={"id": uuid, "ip_address": ip_address})
