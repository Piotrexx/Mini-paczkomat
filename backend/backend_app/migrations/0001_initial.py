# Generated by Django 5.0.1 on 2024-06-19 11:31

import django.db.models.deletion
from django.conf import settings
from django.db import migrations, models


class Migration(migrations.Migration):

    initial = True

    dependencies = [
        ('auth', '0012_alter_user_first_name_max_length'),
    ]

    operations = [
        migrations.CreateModel(
            name='Locker',
            fields=[
                ('locker_id', models.IntegerField(primary_key=True, serialize=False)),
                ('empty', models.BooleanField(default=True)),
            ],
        ),
        migrations.CreateModel(
            name='Paczkomat',
            fields=[
                ('id', models.UUIDField(editable=False, primary_key=True, serialize=False)),
                ('ip_address', models.GenericIPAddressField(protocol='IPv4')),
            ],
        ),
        migrations.CreateModel(
            name='User',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('password', models.CharField(max_length=128, verbose_name='password')),
                ('last_login', models.DateTimeField(blank=True, null=True, verbose_name='last login')),
                ('email', models.EmailField(max_length=254, unique=True)),
                ('first_name', models.CharField(max_length=50)),
                ('last_name', models.CharField(max_length=100)),
                ('date_joined', models.DateTimeField(auto_now_add=True, verbose_name='date joined')),
                ('is_staff', models.BooleanField(default=False)),
                ('is_worker', models.BooleanField(default=False)),
                ('is_superuser', models.BooleanField(default=False)),
                ('groups', models.ManyToManyField(blank=True, help_text='The groups this user belongs to. A user will get all permissions granted to each of their groups.', related_name='user_set', related_query_name='user', to='auth.group', verbose_name='groups')),
                ('user_permissions', models.ManyToManyField(blank=True, help_text='Specific permissions for this user.', related_name='user_set', related_query_name='user', to='auth.permission', verbose_name='user permissions')),
            ],
            options={
                'abstract': False,
            },
        ),
        migrations.CreateModel(
            name='Package',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('package_name', models.CharField(max_length=100)),
                ('date_addressed', models.DateTimeField(auto_now_add=True)),
                ('picked_up', models.BooleanField(default=False)),
                ('locker', models.ForeignKey(on_delete=django.db.models.deletion.CASCADE, related_name='locker', to='backend_app.locker')),
                ('receiver', models.ForeignKey(on_delete=django.db.models.deletion.CASCADE, related_name='receiver', to=settings.AUTH_USER_MODEL)),
            ],
        ),
        migrations.AddField(
            model_name='locker',
            name='paczkomat',
            field=models.ForeignKey(on_delete=django.db.models.deletion.CASCADE, to='backend_app.paczkomat'),
        ),
    ]
