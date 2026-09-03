/* Included directly by a.c and, through `#include LATE_H` in via.h, by
 * b.cpp. The scanner-built include graph sees only the C edge, so the
 * first warm pass lexes this header as C; once via.h's macro include is
 * discovered the header is parsed as C++ and must be re-preprocessed as
 * C++ too, or the C text (`R` expanded to a helper call) is handed to the
 * C++ parser and `late_raw` grows a call that no C++ compiler sees. */
#ifndef LATE_H_
#define LATE_H_
int helper(int);
#define R helper(1) +
static inline int late_raw(void) { return R"(x)"[0]; }
#endif
