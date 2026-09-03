// C++11 raw string literals (issue #14): the shapes the eval corpora use —
// JSON templates with inner quotes, `~`-delimited literals whose bodies
// contain `)"`, a literal spanning lines, a regex full of backslashes and
// parentheses, encoding prefixes, and a raw string passed through macros —
// plus ordinary encoding-prefixed string and character literals, and
// user-defined literals whose suffix must stay glued to the literal.
#define STR(x) #x
#define ID(x) x
#define JSON_TMPL R"~({"domain_":")~"

const char* plain = R"(a "quoted" b)";
const char* delim = R"~(logPath:)~";
const char* regex = R"(=((".*?")|(\S*)))";
const char* multi = R"~({
  "file": "/data/log/test",
  "pc": "0x1234"
})~";
const char* wide = LR"(w "x")";
const char* utf8 = u8R"(y "z")";
const char* concat = R"~({"k":")~" "v" R"~("})~";
const char* viaMacro = ID(R"(p, "q")");
const char* fromMacro = JSON_TMPL;
const char* stringized = STR(R"(a "b")");
auto json = R"({"k":1})"_json;
auto jsonViaMacro = ID(R"~({"k":2})~"_json);
auto sec = "text"s + 10_s;
wchar_t wc = L'x';
char16_t c16 = u'y';
// `R` that is not a raw-string prefix stays an identifier.
int R = 1;
int Rect = R + 1;
