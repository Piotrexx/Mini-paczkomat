from rest_framework import permissions

class IfWorker(permissions.BasePermission):
    def has_object_permission(self, request, view, obj):
        return request.user.is_worker