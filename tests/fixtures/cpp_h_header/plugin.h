#ifndef PLUGIN_H
#define PLUGIN_H

struct Plugin {
    virtual void OnEvent();
    void OnEventProxy();
};

struct Derived : Plugin {
    void OnEvent() override;
};

#endif
