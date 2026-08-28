// Pointer casts must rank against pointer overloads, and unresolvable
// member values must keep the conservative candidate set.
int f(int x) { (void)x; return 0; }
int f(int* p) { (void)p; return 0; }
int f(char x) { (void)x; return 0; }
int f(char* p) { (void)p; return 0; }
int f(int** pp) { (void)pp; return 0; }

struct Holder {
  int val;
};
int g(int x) { (void)x; return 0; }
int g(Holder h) { (void)h; return 0; }

int main() {
  int i = 0;
  char c = 0;
  char* pc = &c;
  int* pi = &i;
  int** pp = &pi;
  f(i);
  f((int*)&i);
  f(c);
  f((char*)&c);
  f(pc);
  f(pi);
  f(pp);
  f((int**)&pi);
  Holder gh;
  Holder* hp = &gh;
  g(42);
  g(gh.val);
  g(hp->val);
  return i + c;
}