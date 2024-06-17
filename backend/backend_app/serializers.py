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
    receiver = UserSerializer(read_only=True)
    class Meta:
        model = Package
        fields = "__all__"
        read_only_fields = ("package_code", "place", "date_addressed")

