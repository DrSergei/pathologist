/* A `.c` file that is only valid C++: a member function definition and a
 * raw string literal. Indexed with `PreprocessOptions::with_language(Cpp)`
 * it must be lexed AND parsed as C++ (one decision), so `S::m` is a
 * method, `use_m` calls it and `R"(x)"` is one literal, not a call to the
 * `R` macro. */
int helper(int);
#define R helper(1) +
struct S {
    int m() { return R"(x)"[0]; }
};
int use_m() {
    S s;
    return s.m();
}
