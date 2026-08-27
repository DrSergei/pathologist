#include "layout.hpp"

int DispatchToMessage(int);

struct IoService g_svc;

void store(void) { g_svc.Dispatch = DispatchToMessage; }
