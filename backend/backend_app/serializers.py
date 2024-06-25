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
    receiver = UserSerializer(read_only=True)
    locker = LockerSerializer()
    class Meta:
        model = Package
        fields = "__all__"
        read_only_fields = ("package_code", "date_addressed", "locker")


class PaczkomatSerializer(ModelSerializer):
    class Meta:
        model = Paczkomat
        fields = "__all__"