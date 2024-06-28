from backend_app.models import User, Package, Locker, Paczkomat
from rest_framework.viewsets import GenericViewSet
from rest_framework.permissions import AllowAny, IsAdminUser
from rest_framework.response import Response
from rest_framework.decorators import action
from rest_framework.status import HTTP_403_FORBIDDEN, HTTP_200_OK, HTTP_201_CREATED, HTTP_404_NOT_FOUND
from backend_app.serializers import UserSerializer, PackageSerializer, LockerSerializer, PaczkomatSerializer
from django.db.utils import IntegrityError
from django.shortcuts import get_object_or_404
import requests
import json

class UserViewSet(GenericViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer

    @action(detail=False, methods=["post"], permission_classes=[AllowAny])
    def register(self, request):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        try:
            user = serializer.save()
            return Response("User Created")
        except IntegrityError:
            return Response(
                "Taki user już istnieje",
                status=HTTP_403_FORBIDDEN,
            )
        

class LockerViewSet(GenericViewSet):
    queryset = Locker.objects.all()
    serializer_class = LockerSerializer

    @action(detail=False, methods=['post'], permission_classes=[AllowAny])
    def add_locker(self, request):
        paczkomat = get_object_or_404(Paczkomat, id=request.data['paczkomat_id'])
        if len(Locker.objects.filter(paczkomat=paczkomat.id)) >= 5:
            return Response("W tym paczkomacie nie można dodawać więcej skrzynek", status=HTTP_403_FORBIDDEN)
        print(request.data)
        serializer = self.serializer_class(data=request.data)

        serializer.is_valid(raise_exception=True)
        serializer.save(paczkomat=paczkomat)
        return Response("Dodano skrzynkę", status=HTTP_201_CREATED)

    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def check_empty_lockers(self, request):
        queryset = Locker.objects.filter(empty=True)
        return Response(self.serializer_class(data=queryset, many=True).data, status=HTTP_200_OK)

class PackageViewSet(GenericViewSet):
    queryset = Package.objects.all()
    serializer_class = PackageSerializer

    @action(detail=False, methods=['post'])
    def create_package(self, request):
        serializer = self.serializer_class(data=request.data)
        serializer.is_valid(raise_exception=True)
        locker = Locker.objects.filter(empty=True)[:1][0]
        serializer.save(locker=locker, receiver=User.objects.get(id=request.data['receiver']))
        paczkomat = Paczkomat.objects.get(id=locker.paczkomat.id)
        print(f"http://{paczkomat.ip_address}:{paczkomat.port}/add_package")
        print(locker.locker_id)
        print(paczkomat.id)
        print(locker.paczkomat.id)
        requests.post(url=f"http://{paczkomat.ip_address}:{paczkomat.port}/add_package", data=json.dumps({"locker_id": str(locker.locker_id),"paczkomat_id": str(paczkomat.id)}), headers= {"Content-Type": "application/json"})
        return Response(f"Nadano przesyłkę do skrytki: {locker.locker_id}", status=HTTP_201_CREATED)
    
    @action(detail=True, methods=['put'])
    def change_emptyness(self, request, uuid):
        locker = get_object_or_404(Locker, locker_id=uuid)
        locker.empty = False
        locker.save()
        return Response("Zapisano", status=HTTP_200_OK)


    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def all_packages(self, request):
        return Response(self.serializer_class(data=self.queryset).data, status=HTTP_200_OK)
    

    @action(detail=False, methods=['get'], permission_classes=[IsAdminUser])
    def all_inside(self, request):
        return Response(self.serializer_class(data=Package.objects.filter(picked_up=False), many=True).data)

    @action(detail=False, methods=['patch'])
    def collect_package(self, request):
        package = Package.objects.filter(receiver=request.user, picked_up=False)[:1]

        if package.exists() is False:
            return Response("Użytkownik nie posiada żadnych paczek", status=HTTP_404_NOT_FOUND)

        locker = Locker.objects.get(id=package.locker)
        paczkomat = Paczkomat.objects.get(id=locker.paczkomat)
        requests.patch(url=f"{paczkomat.ip_address}:{paczkomat.port}/collect/", data={"id": paczkomat.id, "gpio": locker.locker_id}) # dokończyć


class PaczkomatViewSet(GenericViewSet):

    serializer_class = PaczkomatSerializer

    @action(detail=False, methods=['post'], permission_classes=[AllowAny])
    def add_paczkomat_or_check(self, request):
        if Paczkomat.objects.filter(id=request.data['id']).exists():
            paczkomat = get_object_or_404(Paczkomat, id=request.data['id'])
            if paczkomat.ip_address != request.data['ip_address'] or paczkomat.port != request.data["port"]:
                paczkomat.ip_address = request.data['ip_address']
                paczkomat.port = request.data['port']
                paczkomat.save()
                return Response("Zmieniona adres IP", status=HTTP_200_OK)
            else: 
                return Response("ALL GOOD", status=HTTP_200_OK)
        serializer = self.serializer_class(data=request.data)
        serializer.is_valid(raise_exception=True)
        serializer.save()
        return Response("Dodano paczkomat", status=HTTP_201_CREATED)
