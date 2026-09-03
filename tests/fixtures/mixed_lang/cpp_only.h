/* Reached only from b.cpp, so warmed only as C++. a.c uses CHAR_LEAK
 * without including this header and gets it from the C union table, where
 * the C++-lexed body (one user-defined literal) must be re-lexed as C
 * (`'a'` + `C`, a call). */
#ifndef CPP_ONLY_H
#define CPP_ONLY_H
int helper(int);
#define C + helper(2)
#define CHAR_LEAK 'a'C
#endif
