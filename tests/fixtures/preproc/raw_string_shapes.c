// The C twin of raw_string.cpp: valid C in which the identifiers that
// would start a C++11 raw string or end a user-defined literal are macros,
// and the literals next to them are separate preprocessing tokens (C11
// 6.4). Lexed with the C++ rules, `R"(x)"` and `'a'C` would each become
// one token and the macros would never expand.
#define R const char *s =
#define C + 1
#define _s , 2
R"(x)";
int n = 'a'C;
int m[] = { 1 "y"_s };
/* Encoding prefixes are C too and stay glued to their literal. */
const wchar_t *w = L"w";
const char *u = u8"s";
int c16 = u'y';
