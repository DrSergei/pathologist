#include "param.h"

// C++ TU that only sees the header prototype. If `set_irq` does not merge
// with the C definition (header parsed as C++ vs `.c` body), this call
// binds to an undefined stub and `p->func` never receives Handler.
void register_it(struct GpioParam *p) {
    set_irq(p, Handler);
}
