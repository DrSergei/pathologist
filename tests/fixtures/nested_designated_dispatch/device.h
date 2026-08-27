#ifndef DEVICE_H
#define DEVICE_H

#include "object.h"

struct IDeviceIoService {
    struct HdfObject object;
    int (*Dispatch)(int);
};

#endif
