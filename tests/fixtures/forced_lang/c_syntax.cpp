/* A `.cpp` file that is only valid C: `class` is an ordinary identifier
 * here and `'a'C` is the char literal followed by the `C` macro. Indexed
 * with `PreprocessOptions::with_language(C)` it must be lexed AND parsed
 * as C (one decision), so `use_class` calls `class` and `c_char` calls
 * `helper` through the macro rather than seeing a user-defined literal. */
int helper(int);
#define C + helper(2)
int class(int x) { return x; }
int use_class(void) { return class(1); }
int c_char(void) { return 'a'C; }
