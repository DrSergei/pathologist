/* dlsym / GetProcAddress: string constants become function-pointer targets. */

int target(void)
{
    return 1;
}

int other(void)
{
    return 2;
}

void *dlopen(const char *path, int flags);
void *dlsym(void *handle, const char *name);
void *dlvsym(void *handle, const char *name, const char *version);
void *GetProcAddress(void *module, const char *name);

typedef int (*fn_t)(void);

static const char *g_name = "target";

void *wrap(void *h, const char *name)
{
    return dlsym(h, name);
}

void call_literal(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)dlsym(h, "target");
    f();
}

void call_var(void)
{
    void *h = dlopen("x.so", 0);
    const char *n = "target";
    fn_t f = (fn_t)dlsym(h, n);
    f();
}

void call_copy(void)
{
    void *h = dlopen("x.so", 0);
    const char *n = "target";
    const char *m = n;
    fn_t f = (fn_t)dlsym(h, m);
    f();
}

void call_concat(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)dlsym(h, "ta" "rget");
    f();
}

void call_wrap(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)wrap(h, "target");
    f();
}

void call_global(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)dlsym(h, g_name);
    f();
}

void call_getproc(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)GetProcAddress(h, "target");
    f();
}

void call_cast_invoke(void)
{
    void *h = dlopen("x.so", 0);
    void *p = dlsym(h, "target");
    ((fn_t)p)();
}

void call_missing(void)
{
    void *h = dlopen("x.so", 0);
    fn_t f = (fn_t)dlsym(h, "not_a_symbol");
    if (f) {
        f();
    }
}

void call_unknown(void)
{
    void *h = dlopen("x.so", 0);
    const char *n;
    fn_t f = (fn_t)dlsym(h, n);
    if (f) {
        f();
    }
}
