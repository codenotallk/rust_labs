#ifndef PLUGIN_BASE_H_
#define PLUGIN_BASE_H_

#include <stdbool.h>
#include <stdlib.h>

typedef struct 
{
    void *object;
    bool (*read) (void *object, void *data, size_t size);
    bool (*write) (void *object, void *data, size_t size);
    bool (*cleanup) (void *object);
} plugin_base_t;

#endif/* PLUGIN_BASE_H_ */
