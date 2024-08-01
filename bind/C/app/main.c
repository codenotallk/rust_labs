#include <plugin_manager.h>
#include <stdio.h>

int main (void)
{
    plugin_manager_t *pm = plugin_manager_create ();

    plugin_manager_load (pm, "button", "../lib/libbutton.so");
    plugin_manager_load (pm, "led", "../lib/libled.so");

    plugin_manager_read (pm, "led", NULL, 0);
    plugin_manager_write (pm, "led", NULL, 0);

    plugin_manager_read (pm, "button", NULL, 0);
    plugin_manager_write (pm, "button", NULL, 0);

    return 0;
}
