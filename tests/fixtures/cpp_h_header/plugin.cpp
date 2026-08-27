#include "plugin.h"

void Plugin::OnEventProxy() { OnEvent(); }
void Plugin::OnEvent() {}
void Derived::OnEvent() {}

static void drive(Plugin *p) { p->OnEventProxy(); }
