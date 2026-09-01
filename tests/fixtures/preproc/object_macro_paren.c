/* Object macros whose replacement starts with `(` (C11 6.10.3p10: a
 * definition is function-like only when `(` immediately follows the name). */
#define HALF (.5)
#define ORIGIN (.x = 0, .y = 0)
#define VALUE 42
#define ALIAS (VALUE)
#define WRAP (x) x
#define SQUARE(x) ((x) * (x))
#define SPLICED\
(x) (x)

struct point { int x; int y; };

double half = HALF;
struct point origin = ORIGIN;
int alias = ALIAS;
int wrap = WRAP(1);
int square = SQUARE(3);
int spliced = SPLICED(7);
int after;
