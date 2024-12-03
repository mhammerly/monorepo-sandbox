from django.db import models

class ArchivedSite(models.Model):
    archive_id = models.AutoField(primary_key=True)
    archive_url = models.TextField()
    archive_content = models.TextField()

    created = models.DateTimeField(auto_now_add=True)
