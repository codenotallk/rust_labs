#include <led.h>
#include <stdio.h>
#include <string.h>

static bool led_read (void *object, void *data, size_t size);
static bool led_write (void *object, void *data, size_t size);
static bool led_cleanup (void *object);

bool led_init (led_t *object)
{
    bool status = false;

    if (object != NULL)
    {
        memset (object, 0, sizeof (led_t));

        object->base.object    = object;
        object->base.read      = led_read;
        object->base.write     = led_write;
        object->base.cleanup   = led_cleanup;

        object->gpio = 3;
        object->name = "led";

        status = true;
    }
}

plugin_base_t *create_plugin (void)
{
    plugin_base_t *plugin = NULL;

    led_t *led = calloc (1, sizeof (led_t));

    if (led != NULL && led_init (led) == true)
    {
        plugin = &led->base;
    }

    return plugin;
}

static bool led_read (void *object, void *data, size_t size)
{
    led_t *led = (led_t *) object;
    (void) data;
    (void) size;

    printf ("Reading data from device %s gpio %d\n", led->name, led->gpio);

    return true;
}

static bool led_write (void *object, void *data, size_t size)
{
    led_t *led = (led_t *) object;
    (void) data;
    (void) size;

    printf ("Writing data on device %s gpio %d\n", led->name, led->gpio);

    return true;
}

static bool led_cleanup (void *object)
{
    bool status = false;
    led_t *led = (led_t *) object;

    if (led != NULL)
    {
        free (led);
        status = true;
    }

    return status;
}