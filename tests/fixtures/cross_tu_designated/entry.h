#ifndef ENTRY_H
#define ENTRY_H

struct DriverEntry {
    int (*Init)(void);
    int (*Bind)(void);
};

void my_init(void);
void my_bind(void);

#endif
