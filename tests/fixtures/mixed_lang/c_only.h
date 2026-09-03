/* Reached only from a.c, so warmed only as C. b.cpp uses RAW_LEAK without
 * including this header and gets it from the C++ union table, where the
 * C-lexed body (`R` + `"(x)"`) must be re-lexed as C++ (one raw string,
 * no call). */
#ifndef C_ONLY_H
#define C_ONLY_H
int helper(int);
#define R helper(1) +
#define RAW_LEAK R"(x)"[0]
#endif
