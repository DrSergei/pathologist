// C++ overload resolution by scalar type + template member calls.
class FieldValue {
public:
  int GetNumber() { return 42; }
  long GetNumber(long v) { return v; }
  template <typename T> T GetNumber() { return T(); }
};

struct Box {
  template <typename T> T read() { return T(); }
};

int f(int x) { (void)x; return 0; }
int f(double x) { (void)x; return 0; }
int f(short x) { (void)x; return 0; }
int f(int x, int y) { (void)x; (void)y; return 0; }

int main() {
  FieldValue fv;
  int n = fv.GetNumber();
  long l = fv.GetNumber(7L);
  int tn = fv.GetNumber<int>();
  f(1);
  f(1.5);
  short s = 0;
  f(s);
  f(1, 2);
  Box b;
  int r = b.read<short>();
  return n + (int)l + tn + r;
}