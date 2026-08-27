#include "param.h"

void fire(struct GpioParam *p) {
    p->func(1, 0);
}
