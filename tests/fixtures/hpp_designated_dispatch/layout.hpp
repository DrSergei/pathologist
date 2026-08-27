#ifndef LAYOUT_HPP
#define LAYOUT_HPP

struct IoService {
    int objectId;
    int (*Dispatch)(int);
};

#endif
