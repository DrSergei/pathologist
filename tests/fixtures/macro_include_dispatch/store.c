#include "wrapper.h"

struct IDeviceIoService g_svc = {
    .object.objectId = 1,
    .Dispatch = DispatchToMessage,
};
