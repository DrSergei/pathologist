/* Regression fixture for issue #1: #if/#elif conditional evaluation.
 * Markers are chosen so substring checks cannot collide. */
#define FEATURE 1

#if defined(FEATURE)
int feature_on = 1;
#endif

#if !defined(FEATURE)
int feature_off = 2;
#endif

#if 0
int b1 = 1;
#elif 1
int b2 = 2;
#elif 0
int b3 = 3;
#else
int b4 = 4;
#endif

#if defined(FEATURE) && FEATURE >= 1
int compound_ok = 5;
#endif

#define GE(a, b) ((a) >= (b))
#if GE(3, 2)
int fnlike_ok = 6;
#endif
