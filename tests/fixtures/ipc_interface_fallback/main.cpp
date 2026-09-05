// Stub with no handler method bodies — OnRemoteRequest switch calls
// inherited interface methods directly on `this`. Only OnRemoteRequest
// has a body; HasNext/GetNext are declared as overrides (no body) in the
// stub class.  This matches the FaultLogQueryResultStub pattern in hiview
// where the stub doesn't define the interface methods.

class IRemoteObject {
public:
    int SendRequest(int code, void *data, void *reply, void *option);
};

IRemoteObject *Remote();

// An unrelated same-named interface in another namespace must not be chosen
// by the fallback merely because it was indexed first.
namespace Other {
class IQueryResult {
public:
    virtual bool HasNext() = 0;
    virtual int GetNext() = 0;
};
}

// Interface: IPC methods (declared, no body — external in symbol table).
class IQueryResult {
public:
    virtual bool HasNext() = 0;
    virtual int GetNext() = 0;
};

// Stub: declares HasNext/GetNext as overrides (no body — external).
// OnRemoteRequest has a body. The switch calls the inherited interface
// methods directly on `this`.
class QueryResultStub : public IQueryResult {
public:
    bool HasNext() override;
    int GetNext() override;
    int OnRemoteRequest(int code, void *data, void *reply, void *option);
};

// Proxy: methods call SendRequest.
class QueryResultProxy {
public:
    bool HasNext();
    int GetNext();
};

int QueryResultStub::OnRemoteRequest(int code, void *data, void *reply, void *option) {
    switch (code) {
        case 1: return HasNext() ? 1 : 0;
        case 2: return GetNext();
        default: return -1;
    }
}

bool QueryResultProxy::HasNext() {
    IRemoteObject *remote = Remote();
    void *data = 0, *reply = 0, *option = 0;
    remote->SendRequest(1, data, reply, option);
    return false;
}
int QueryResultProxy::GetNext() {
    IRemoteObject *remote = Remote();
    void *data = 0, *reply = 0, *option = 0;
    remote->SendRequest(2, data, reply, option);
    return 0;
}

int main() {
    QueryResultProxy p;
    p.HasNext();
    return 0;
}
