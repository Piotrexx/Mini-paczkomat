from rest_framework.serializers import ModelSerializer
from rest_framework import serializers
from backend_app.models import User, Package, Locker, Paczkomat


class UserSerializer(ModelSerializer):
    class Meta:
        model = User
        fields = "__all__"


class LockerSerializer(ModelSerializer):
    paczkomat_id = serializers.UUIDField()
    class Meta:
        model = Locker
        fields = ("locker_id", "gpio", "paczkomat_id")
        

class PackageSerializer(ModelSerializer):
    class Meta:
        model = Package
        fields = ("package_name", "receiver","locker", "date_addressed")
        read_only_fields = ("date_addressed","locker") # "package_code",


class PaczkomatSerializer(ModelSerializer):
    class Meta:
        model = Paczkomat
        fields = "__all__"