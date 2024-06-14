from django.contrib import admin
from django.urls import path, include
from backend_app import views
from rest_framework import routers

router = routers.DefaultRouter()
router.register(r'user', views.UserViewSet, 'user')


urlpatterns = [
    path('admin/', admin.site.urls),
    path('', include(router.urls)),
    path('auth/', include('backend_app.urls'))
]
