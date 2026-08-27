#include "device.h"

extern struct IDeviceIoService g_svc;

void launch(void) { g_svc.Dispatch(0); }
