#ifndef BUTTON_H_
#define BUTTON_H_

#include <plugin_base.h>
#include <stdint.h>

typedef struct 
{
    plugin_base_t base;
    char *name;
    uint32_t gpio;
} button_t;

bool button_init (button_t *object);

plugin_base_t *create_plugin (void);

#endif/* BUTTON_H_ */
