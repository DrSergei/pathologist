class MockInterface {
public:
    MOCK_METHOD(int, LinkNext, (int value), (override));
    MOCK_METHOD((std::pair<int, int>), Pair, (), ());
    MOCK_METHOD(int, Peek, (), (const, override));
    MOCK_METHOD(bool, CheckMap, ((std::map<int, double>), bool), (override));
    MOCK_METHOD((void (*)(int)), GetHandler, (), (override));
    MOCK_METHOD0(Start, int());
    MOCK_METHOD1(Attach, int(int value));
    MOCK_CONST_METHOD2(Inspect, int(int left, int right));
    MOCK_METHOD0(GetCallback, void (*())(int));
    MOCK_METHOD1_WITH_CALLTYPE(STDMETHODCALLTYPE, Send, int(int value));
    MOCK_CONST_METHOD0_WITH_CALLTYPE(STDMETHODCALLTYPE, Poll, int());
    MOCK_METHOD(void, OnLinked,
        (const std::shared_ptr<Filter>& filter, StreamType out),
        (override));
    MOCK_METHOD((void (Registry<int>::*)(int)), GetMemberPtr, (), (override));
    MOCK_METHOD(int, Wait, (), (noexcept(is_nothrow<const int&>::value)));
};
