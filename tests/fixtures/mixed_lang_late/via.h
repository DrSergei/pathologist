/* Reached only from b.cpp. The raw `#include` scanner cannot see the
 * macro-spelled include below, so the include graph learns that late.h is
 * reachable from a C++ unit only after this header has been preprocessed
 * during the warm pass. */
#ifndef VIA_H
#define VIA_H
#define LATE_H "late.h"
#include LATE_H
#endif
