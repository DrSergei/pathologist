#include "shared.h"
#include "c_only.h"
int helper(int x) { return x; }
int c_raw(void) { return RAW_VAL; }
int c_char(void) { return CHAR_VAL; }
int c_leak(void) { return CHAR_LEAK; }
