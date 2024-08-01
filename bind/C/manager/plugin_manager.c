#include <plugin_manager.h>
#include <string.h>
#include <dlfcn.h>
#include <plugin_base.h>
#include <stdlib.h>

#define PLUGIN_NAME_SIZE        128
#define PLUGIN_ARRAY_SIZE       5

typedef struct 
{
    void *handle;
    char name [PLUGIN_NAME_SIZE + 1];
    plugin_base_t *base;
    bool loaded;
} plugin_data_t;

typedef struct 
{
    plugin_data_t list [PLUGIN_ARRAY_SIZE];
    uint8_t amount;
} plugin_array_t;

struct plugin_manager_t
{
    plugin_array_t plugins;
    bool initialized;
};

static plugin_data_t *plugin_manager_get (plugin_manager_t *object, const char *device);

plugin_manager_t *plugin_manager_create (void)
{
    plugin_manager_t *pm = calloc (1, sizeof (plugin_manager_t));

    if (pm != NULL)
    {
        pm->initialized = true;
    }

    return pm;
}

bool plugin_manager_load (plugin_manager_t *object, char *name, char *path)
{
    bool status = false;

    if (object != NULL &&
        object->initialized == true &&
        name != NULL &&
        path != NULL &&
        object->plugins.amount < PLUGIN_ARRAY_SIZE)
    {
        plugin_data_t *data = &object->plugins.list [object->plugins.amount];

        data->handle = dlopen (path, RTLD_LAZY);
        if (data->handle != NULL)
        {
            plugin_base_t *(*plugin_create) (void) = dlsym (data->handle, "create_plugin");
            if (dlerror () == NULL)
            {
                data->base = plugin_create ();
                strncpy (data->name, name, PLUGIN_NAME_SIZE);
                data->loaded = true;

                object->plugins.amount ++;

                status = true;
            }
        }
    }

    return status;
}

bool plugin_manager_close (plugin_manager_t *object)
{
    bool status = false;

    if (object != NULL && object->initialized == true)
    {
        free (object);

        status = true;
    }

    return status;
}

bool plugin_manager_read (plugin_manager_t *object, const char *device, void *data, size_t size)
{
    bool status = false;

    plugin_data_t *plugin = plugin_manager_get (object, device);

    if (plugin != NULL)
    {
        status = plugin->base->read (plugin->base, data, size);
    }

    return status;
}

bool plugin_manager_write (plugin_manager_t *object, const char *device, void *data, size_t size)
{
    bool status = false;

    plugin_data_t *plugin = plugin_manager_get (object, device);

    if (plugin != NULL)
    {
        status = plugin->base->write (plugin->base, data, size);
    }

    return status;
}

bool plugin_manager_cleanup (plugin_manager_t *object, const char *device)
{
    bool status = false;

    plugin_data_t *plugin = plugin_manager_get (object, device);

    if (plugin != NULL)
    {
        status = plugin->base->cleanup (plugin->base);
    }

    return status;
}

static plugin_data_t *plugin_manager_get (plugin_manager_t *object, const char *device)
{
    plugin_data_t *plugin = NULL;

    if (object != NULL &&
        object->initialized == true &&
        device != NULL &&
        strlen (device) > 0)
    {
        for (uint8_t i = 0; i < object->plugins.amount; i ++)
        {

            plugin_data_t *__plugin = &object->plugins.list [i];

            if (strcmp (device, __plugin->name) == 0)
            {
                plugin = __plugin;
                break;
            }
        }
    }

    return plugin;
}