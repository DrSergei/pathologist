#ifndef PARAM_H
#define PARAM_H

#include "irq.h"

struct GpioParam {
    int gpio;
    void *data;
    GpioIrqFunc func;
};

int Handler(int gpio, void *data);
void set_irq(struct GpioParam *p, GpioIrqFunc func);

#endif
