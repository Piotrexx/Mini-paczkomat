from rest_framework import permissions
from .models import Paczkomat
from django.contrib.gis.measure import D
from django.contrib.gis.geos import Point

class DistancePermission(permissions.BasePermission):
    def has_object_permission(self, request, view, obj):
        Paczkomat.objects.filter(
                location_point__distance_lte=(
                    Point(float(request.data["lat"]), float(request.data["lon"])),
                    D(m=int(3)),
                )).exists()
