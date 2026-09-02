#define STR(x) #x
#define VALUE 42
#define LOG(fmt, ...) log_write(#fmt, __VA_ARGS__)
#define CHECK(cond) do { if (!(cond)) fail(#cond, __LINE__); } while (0)
#define ALL(...) #__VA_ARGS__

const char *a = STR(hello);
const char *b = STR(a + b);
const char *c = STR("quoted \"inner\"");
const char *d = STR(VALUE);
const char *g = ALL(p, q);

void f(void)
{
    LOG(hello %d, 1);
    CHECK(x > 0 && ptr != NULL);
}
