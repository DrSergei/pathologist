class FooTest : public testing::Test {
public:
    void SetUp();
};

void FooTest::SetUp() {}

HWTEST_F(FooTest, Bar, TestSize.Level1)
{
    int value = Helper(1);
    (void)value;
}
