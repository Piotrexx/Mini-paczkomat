import random
from backend_app import models

def get_package_safe_code() -> int:
    random_code = random.randint(1000, 9999)
    while models.Package.objects.filter(package_code=random_code, picked_up=False).exists():
        random_code = random.randint(1000, 9999)
    return random_code