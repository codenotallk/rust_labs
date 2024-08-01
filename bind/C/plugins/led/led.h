#ifndef LED_H_
#define LED_H_

#include <plugin_base.h>
#include <stdint.h>

typedef struct 
{
    plugin_base_t base;
    char *name;
    uint32_t gpio;
} led_t;

bool led_init (led_t *object);

plugin_base_t *create_plugin (void);

#endif/* LED_H_ */
