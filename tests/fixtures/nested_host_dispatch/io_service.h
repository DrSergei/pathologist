#ifndef IO_SERVICE_H
#define IO_SERVICE_H

struct IDeviceIoService {
    int (*Dispatch)(int);
};

#endif
