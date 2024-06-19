from backend_app.models import User, Package, Locker, Paczkomat
from rest_framework.viewsets import GenericViewSet
from rest_framework.permissions import AllowAny, IsAuthenticated, IsAdminUser
from rest_framework.response import Response
from rest_framework.decorators import action
from rest_framework.status import HTTP_403_FORBIDDEN, HTTP_200_OK, HTTP_201_CREATED
from backend_app.serializers import UserSerializer, PackageSerializer, LockerSerializer, PaczkomatSerializer
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
        

class LockerViewSet(GenericViewSet):
    queryset = Locker.objects.all()
    serializer_class = LockerSerializer

    @action(detail=False, methods=['post'], permission_classes=[IsAdminUser])
    def add_locker(self, request):
        paczkomat = get_object_or_404(Paczkomat, id=request.data['paczkomat'])
        if len(Locker.objects.filter(paczkomat=paczkomat.id)) >= 5:
            return Response("W tym paczkomacie nie można dodawać więcej skrzynek", status=HTTP_403_FORBIDDEN)

        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exceptions=True)
        serializer.save()
        return Response("Dodano skrzynkę", status=HTTP_201_CREATED)

    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def check_empty_lockers(self, request):
        queryset = Locker.objects.filter(empty=True)
        return Response(self.serializer_class(data=queryset, many=True).data, status=HTTP_200_OK)

class PackageViewSet(GenericViewSet):
    queryset = Package.objects.all()
    serializer_class = PackageSerializer

    @action(detail=False, methods=['post'], permission_classes=[IsAuthenticated])
    def create_package(self, request):
        serializer = self.serializer_class()
        serializer.is_valid(raise_exception=True)
        serializer.save(locker=Locker.objects.filter(empty=True)[:1], receiver=User.objects.get(id=request.data['receiver']))
        return Response(f"Nadano przesyłkę do skrytki: {serializer.data['locker']}", status=HTTP_201_CREATED)
    

    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def all_packages(self, request):
        return Response(self.serializer_class(data=self.queryset).data, status=HTTP_200_OK)
    

    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def all_inside(self, request):
        return Response(self.serializer_class(data=Package.objects.filter(picked_up=False), many=True).data)
    

class CheckIPViewSet(GenericViewSet):

    @action(detail=False, methods=['PATCH'], permission_classes=[AllowAny])
    def check(self, request):
        paczkomat = get_object_or_404(Paczkomat, id=request.data['id'])
        if paczkomat.ip_address != request.data['ip_address']:
            paczkomat.ip_address = request.data['ip_address']
            paczkomat.save()
            return Response("Zmieniona adres IP", status=HTTP_200_OK)
        return Response("Wszystko jest ok", status=HTTP_200_OK)
    

class PaczkomatViewSet(GenericViewSet):

    serializer_class = PaczkomatSerializer

    @action(detail=False, methods=['post'], permission_classes=[AllowAny])
    def add_paczkomat(self, request):
        serializer = self.serializer_class(data=request.data)
        serializer.is_valid(raise_exception=True)
        serializer.save()
        return Response("Dodano paczkomat")