#include "io_service.h"

void launch(struct IDeviceIoService *s) {
    s->Dispatch(0);
}
