from rest_framework.permissions import AllowAny
from rest_framework_simplejwt.views import TokenRefreshView, TokenObtainPairView
from django.urls import path


urlpatterns = [
    path('login/', TokenObtainPairView(permissions_classes=[AllowAny]).as_view(), name='login'),
    path('token/refresh/', TokenRefreshView(permissions_classes=[AllowAny]).as_view(), name='token_refresh'),
]