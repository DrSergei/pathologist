#ifndef DEVICE_H
#define DEVICE_H

/* Raw include scanner only matches #"..." / <...>. This nested type
 * must still PCH before this header so designated `.object.objectId`
 * sees a complete HdfObject prefix. */
#define OBJECT_HDR "object.h"
#include OBJECT_HDR

struct IDeviceIoService {
    struct HdfObject object;
    int (*Dispatch)(int);
};

#endif
