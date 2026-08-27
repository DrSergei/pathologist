#include "layout.hpp"

extern struct IoService g_svc;

void launch(void) { g_svc.Dispatch(0); }
