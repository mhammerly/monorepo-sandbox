from django.urls import path, re_path

from . import views

urlpatterns = [
    path('create', views.create, name='create'),
    re_path(r'preview/(?P<site>.+)$', views.preview, name='preview'),
    path('', views.index, name='index'),
]
