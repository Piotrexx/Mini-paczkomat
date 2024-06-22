import uuid
from django.db import models
from django.contrib.auth.models import User
from django.contrib.auth.models import AbstractBaseUser, BaseUserManager
from django.contrib.auth.models import PermissionsMixin
from django.core.validators import MinValueValidator, MaxValueValidator

class CustomUserManager(BaseUserManager):
    def create(self, **kwargs):
        return self.create_user(**kwargs)

    def create_user(self, email, first_name, last_name, password, **kwargs):
        if not email or not first_name or not last_name or not password:
            return ValueError("The First Name is required ")

        email = self.normalize_email(email)
        user = self.model(
            email=email,
            first_name=first_name,
            last_name=last_name,
            **kwargs,
        )
        user.set_password(password)
        user.save(using=self._db)
        return user

    def create_superuser(self, email, password=None, **extra_fields):
        extra_fields.setdefault("is_staff", True)
        extra_fields.setdefault("is_superuser", True)
        extra_fields.setdefault("is_worker", True)
        return self.create_user(email=email, password=password, **extra_fields)



class User(AbstractBaseUser, PermissionsMixin):
    email = models.EmailField(unique=True)
    first_name = models.CharField(max_length=50)
    last_name = models.CharField(max_length=100)
    date_joined = models.DateTimeField("date joined", auto_now_add=True, editable=False)
    is_staff = models.BooleanField(default=False)
    is_worker = models.BooleanField(default=False)
    is_superuser = models.BooleanField(default=False)
    objects = CustomUserManager()

    USERNAME_FIELD = "email"
    REQUIRED_FIELDS = ["first_name", "last_name"]

    
    # def has__package(self):
    #     return Package.objects.get()

class Paczkomat(models.Model):
    id = models.UUIDField(primary_key=True)
    ip_address = models.GenericIPAddressField(protocol='IPv4')
    port = models.IntegerField(validators=[MinValueValidator(8001), MaxValueValidator(9000)], default=None, null=True)

class Locker(models.Model):
    locker_id = models.UUIDField(primary_key=True)
    gpio = models.IntegerField()
    empty = models.BooleanField(default=True)
    paczkomat = models.ForeignKey(Paczkomat, on_delete=models.CASCADE)


class Package(models.Model):
    # package_code = models.UUIDField(primary_key=True, editable=False)
    package_name = models.CharField(max_length=100)
    receiver = models.ForeignKey(User, on_delete=models.CASCADE, related_name="receiver")
    locker = models.ForeignKey(Locker, on_delete=models.CASCADE, related_name="locker")
    date_addressed = models.DateTimeField(auto_now_add=True)
    picked_up = models.BooleanField(default=False)