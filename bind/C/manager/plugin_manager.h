#ifndef PLUGIN_MANAGER_H_
#define PLUGIN_MANAGER_H_

#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

typedef struct plugin_manager_t plugin_manager_t;

plugin_manager_t *plugin_manager_create (void);
bool plugin_manager_load (plugin_manager_t *object, char *name, char *path);
bool plugin_manager_close (plugin_manager_t *object);

bool plugin_manager_read (plugin_manager_t *object, const char *device, void *data, size_t size);
bool plugin_manager_write (plugin_manager_t *object, const char *device, void *data, size_t size);
bool plugin_manager_cleanup (plugin_manager_t *object, const char *device);

#endif/* PLUGIN_MANAGER_H_ */
