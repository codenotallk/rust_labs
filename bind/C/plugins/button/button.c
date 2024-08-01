#include <button.h>
#include <stdio.h>
#include <string.h>

static bool button_read (void *object, void *data, size_t size);
static bool button_write (void *object, void *data, size_t size);
static bool button_cleanup (void *object);

bool button_init (button_t *object)
{
    bool status = false;

    if (object != NULL)
    {
        memset (object, 0, sizeof (button_t));

        object->base.object    = object;
        object->base.read      = button_read;
        object->base.write     = button_write;
        object->base.cleanup   = button_cleanup;

        object->gpio = 3;
        object->name = "button";

        status = true;
    }
}

plugin_base_t *create_plugin (void)
{
    plugin_base_t *plugin = NULL;

    button_t *button = calloc (1, sizeof (button_t));

    if (button != NULL && button_init (button) == true)
    {
        plugin = &button->base;
    }

    return plugin;
}

static bool button_read (void *object, void *data, size_t size)
{
    button_t *button = (button_t *) object;
    (void) data;
    (void) size;

    printf ("Reading data from device %s gpio %d\n", button->name, button->gpio);

    return true;
}

static bool button_write (void *object, void *data, size_t size)
{
    (void) object;
    (void) data;
    (void) size;

    return false;
}

static bool button_cleanup (void *object)
{
    bool status = false;
    button_t *button = (button_t *) object;

    if (button != NULL)
    {
        free (button);
        status = true;
    }

    return status;
}