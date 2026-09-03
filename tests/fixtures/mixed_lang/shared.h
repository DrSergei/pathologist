/* Included from both a.c and b.cpp. The macro bodies below lex
 * differently in the two languages: in C, `R` and `C` are identifiers
 * (macros here, so RAW_VAL and CHAR_VAL both call helper); in C++,
 * `R"(x)"` is a raw string literal and `'a'C` a user-defined literal (no
 * call in either). The indexer warms this header once per language so
 * each unit replays the tokenization its own lexer would produce. */
#ifndef SHARED_H
#define SHARED_H
int helper(int);
#define R helper(1) +
#define C + helper(2)
#define RAW_VAL R"(x)"[0]
#define CHAR_VAL 'a'C
#endif
