#include "entry.h"

extern struct DriverEntry g_entry;

void launch(void) {
    g_entry.Init();
    g_entry.Bind();
}
