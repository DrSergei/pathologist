struct HdfDev {
    int flags;
    struct HdfDev* next;
};

static long DevRead(struct file* filep, char __user* buf, unsigned long long len)
{
    struct HdfDev* dev = container_of(filep, struct HdfDev, next);
    printf("read %" PRIu64 " bytes\n", len);
    return dev->flags;
}

static int __init DevInit(void)
{
    return 0;
}

static void __exit DevExit(void)
{
}
