from backend_app.models import User, Package, Place, Paczkomat
from rest_framework.viewsets import GenericViewSet
from rest_framework.permissions import AllowAny, IsAuthenticated, IsAdminUser
from backend_app.permissions import IfWorker
from rest_framework.response import Response
from rest_framework.decorators import action
from rest_framework.status import HTTP_403_FORBIDDEN, HTTP_200_OK, HTTP_201_CREATED
from backend_app.serializers import UserSerializer, PackageSerializer, PlaceSerializer, PaczkomatSerializer
from django.db.utils import IntegrityError
from uuid import uuid4
from django.shortcuts import get_object_or_404


class UserViewSet(GenericViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer

    @action(detail=False, methods=["post"], permission_classes=[AllowAny])
    def register(self, request):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        try:
            user = serializer.save()
        except IntegrityError:
            return Response(
                "Taki user już istnieje",
                status=HTTP_403_FORBIDDEN,
            )
        

class PlaceViewSet(GenericViewSet):
    queryset = Place.objects.all()
    serializer_class = PlaceSerializer

    @action(detail=False, methods=['post'], permission_classes=[IsAdminUser])
    def add_place(self, request):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exceptions=True)
        serializer.save()
        return Response(serializer.data)

    @action(detail=False, methods=['get'], permission_classes={IfWorker})
    def check_empty_places(self, request):
        queryset = Place.objects.filter(empty=True)
        return Response(self.serializer_class(data=queryset, many=True).data, status=HTTP_200_OK)

class PackageViewSet(GenericViewSet):
    queryset = Package.objects.all()
    serializer_class = PackageSerializer

    @action(detail=False, methods=['post'], permission_classes=[IsAuthenticated])
    def create_package(self, request):
        serializer = self.serializer_class(data=request.data)
        serializer.is_valid(raise_exception=True)
        serializer.save(package_code=uuid4(), place=Place.objects.filter(empty=True)[:1])
        return Response("Nadano przesyłkę", status=HTTP_201_CREATED)
    

    @action(detail=False, methods=['get'], permission_classes=[IfWorker])
    def all_packages(self, request):
        return Response(self.serializer_class(data=self.queryset).data, status=HTTP_200_OK)
    

    @action(detail=False, methods=['get'], permission_classes=[IfWorker])
    def all_inside(self, request):
        return Response(self.serializer_class(data=Package.objects.filter(picked_up=False), many=True).data)
    

class CheckIP(GenericViewSet):

    @action(detail=False, methods=['PATCH'], permission_classes=[AllowAny])
    def check(self, request):
        paczkomat = get_object_or_404(Paczkomat, id=request.data['id'])
        if paczkomat.ip_address != request.data['ip_address']:
            paczkomat.ip_address = request.data['ip_address']
            paczkomat.save()
            return Response("Zmieniona adres IP", status=HTTP_200_OK)
        return Response("Wszystko jest ok", status=HTTP_200_OK)
    

class Paczkomat(GenericViewSet):

    serializer_class = PaczkomatSerializer

    @action(detail=False, methods=['post'], permission_classes=[IfWorker])
    def add_paczkomat(self, request):
        serializer = self.serializer_class(data=request.data)
        serializer.is_valid(raise_exception=True)
        serializer.save()
        return Response("Dodano paczkomat")