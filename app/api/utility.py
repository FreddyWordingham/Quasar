import os

from . import settings


def get_plugin_list(catagory: str):
    """
    List the plugin files in the given catagory.
    """

    return [
        os.path.splitext(filename)[0]
        for filename in os.listdir(os.path.join(settings.PLUGIN_DIR, catagory))
    ]
