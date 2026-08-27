#include "entry.h"

void my_init(void) {}
void my_bind(void) {}

struct DriverEntry g_entry = {
    .Init = my_init,
    .Bind = my_bind,
};
