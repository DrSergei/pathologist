#define HILOG_DEBUG(label, fmt, args...) printf(fmt, ##args)
#define DECORATOR_HILOG(op, fmt, args...) op("L", fmt, ##args)
#define MEDIA_DEBUG_LOG(fmt, ...) DECORATOR_HILOG(HILOG_DEBUG, fmt, ##__VA_ARGS__)
#define COUNT 42

void f(void)
{
    MEDIA_DEBUG_LOG("plain");
    MEDIA_DEBUG_LOG("num %d", COUNT);
    MEDIA_DEBUG_LOG("str %s", "reason");
}
