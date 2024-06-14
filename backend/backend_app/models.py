from django.db import models
from django.contrib.auth.models import User


class Place(models.Model):
    place_id = models.IntegerField(primary_key=True)
    empty = models.BooleanField(default=True)

class Package(models.Model):
    package_name = models.CharField(max_length=100)
    receiver = models.ForeignKey(User, on_delete=models.CASCADE, related_name="receiver")
    place = models.ForeignKey(Place, on_delete=models.CASCADE, related_name="place")
    date_addressed = models.DateTimeField(auto_now_add=True)
    in_paczkomat = models.BooleanField(default=False)




