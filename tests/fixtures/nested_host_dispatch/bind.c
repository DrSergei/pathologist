#include "host.h"

void StreamDispatch(int x) { (void)x; }

void bind(struct StreamHost *h) {
    h->service.Dispatch = StreamDispatch;
}
