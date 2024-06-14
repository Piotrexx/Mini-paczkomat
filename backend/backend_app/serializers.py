from rest_framework.serializers import ModelSerializer
from backend_app.models import User, Package, Place


class UserSerializer(ModelSerializer):
    class Meta:
        model = User
        fields = "__all__"


class PlaceSerializer(ModelSerializer):
    class Meta:
        model = Place
        fields = "__all__"

class PackageSerializer(ModelSerializer):
    class Meta:
        model = Package
        fields = "__all__"

