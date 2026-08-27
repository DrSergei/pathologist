#include "param.h"

int Handler(int gpio, void *data) {
    (void)gpio;
    (void)data;
    return 0;
}

void set_irq(struct GpioParam *p, GpioIrqFunc func) {
    p->func = func;
}
