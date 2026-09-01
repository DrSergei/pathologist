# Preprocessor specification

trace implements its own C preprocessor in `trace-preproc`. It runs before tree-sitter parsing so `#include` and `#define` are resolved without invoking gcc/clang.

## API

```rust
pub fn preprocess_file(path: &Path, opts: &PreprocessOptions) -> Result<PreprocessResult>;

pub struct PreprocessResult {
    pub output: String,
    pub line_map: LineMap,
    pub diagnostics: Vec<Diagnostic>,
}
```

CLI equivalents: `--include PATH`, `-D NAME=VALUE`.

## Role in the pipeline

```mermaid
flowchart LR
  Raw[Raw .c on disk]
  IG[IncludeGraph]
  PP[preprocess_file]
  Out[Expanded source]
  TS[tree-sitter parse]

  Raw --> IG
  IG --> PP --> Out --> TS
```

- **`IncludeGraph`** (`trace-parse/src/deps.rs`) scans project files for `#include` directives, builds dependency edges, discovers include directories, and marks which files need preprocessing.
- Preprocessed output is **cached** per file (parallel cache fill when `--jobs > 1`).
- If preprocessing fails hard, parse may fall back to reading raw source (diagnostic recorded).

## Phases

### P0 (implemented)

| Feature | Notes |
|---------|-------|
| Comments | `//`, `/* */` |
| Literals | String and character literals preserved |
| `#include "..."` / `<...>` | Include path stack + `--include` |
| `#include` macro operand | C11 6.10.2: if the tokens are not already `"..."` / `<...>`, the rest of the line is macro-expanded and must then form a header-name (`#include FOO` with `#define FOO "n.h"`) |
| `#define` | Object-like and **function-like** (non-variadic) |
| Macro rescanning | Function-like macros invoked inside another macro's expansion are expanded too (C11 6.10.3.4); uninvoked function-like names are emitted verbatim |
| Macro hide set | Replacement-list tokens are painted with the macro name (and the invoking token's hide set) so self-referential macros such as `#define FOO FOO, BAR` terminate; nested `MIN(MIN(a,b),c)` still expands because argument tokens are not painted |
| Expansion depth cap | 256 nested expansions; further expansion is skipped with a warning (backstop if hide-set does not apply) |
| Runaway caps | Per-file limits (defaults): 64 nested `#include`s, 32 MiB live output, 8M token-loop iterations (macro rescan included). Exceeding output/token budget stops that file with an error diagnostic; include-depth skips the nested include. CLI `--timeout-secs N` aborts the whole process. |
| `##` token pasting | In macro bodies after argument substitution; chained pastes (`a ## b ## c`) collapse left to right, a dangling `##` with no operand is dropped |
| Conditionals | `#ifdef`, `#ifndef`, `#if` / `#elif` / `#else` / `#endif`. `#if` conditions get full constant-expression evaluation: the `defined X` / `defined(X)` operator is resolved over unexpanded tokens (C11 6.10.1p4), object **and** function-like macros expand (hide-set painted, depth-capped), and the result is parsed with C operator precedence (`?:`, `\|\|`, `&&`, bitwise, `==`/`!=`, relationals, shifts, arithmetic, unary `!`/`~`/`-`/`+`, parens). Integer literals accept `0x`/`0b`/octal prefixes and `u`/`U`/`l`/`L` suffixes; arithmetic models 64-bit intmax_t/uintmax_t with the usual arithmetic conversions (an operand mixed with an unsigned one converts to unsigned, so `-1 < 1U` is false; `>>` is arithmetic for signed, logical for unsigned; a literal is unsigned when suffixed `u`/`U` or too large for intmax_t). Identifiers surviving expansion evaluate to 0; malformed expressions (trailing tokens, unbalanced parens) conservatively skip the branch; per chain at most one branch activates. `\`-newline continuations inside conditions are spliced. Conditions in skipped groups are not evaluated (and malformed `#ifdef` operands there are tolerated). Condition macro expansion runs under its own budget (64K tokens / 1M steps); exceeding it warns and conservatively skips the branch. `#elif` after `#else` warns and is ignored. |
| `#line` | Location tracking in `LineMap` |
| `#undef` | |
| Predefined | `__FILE__`, `__LINE__`; builtin fallback macros for headers the indexed tree does not ship (see [Builtin fallback macros](#builtin-fallback-macros)) |
| Token spacing | No space before `)` / `]`; space between `>` and `&` / `*` so `operator()` and `shared_ptr<T> &p` survive re-lexing |

### P1 (planned)

- `#pragma once` / include-guard detection
- Variadic macros (`...`, `__VA_ARGS__`)
- `#` stringize operator

### P2 (planned)

- `_Pragma`, additional standard predefined macros

## Builtin fallback macros

Code is indexed without a real toolchain, so macros whose definitions live in
headers the tree does not ship (gtest, Linux kernel headers, `<inttypes.h>`)
survive preprocessing, produce tree-sitter ERROR nodes, and can drop whole
functions from the index (`docs/PARSE_FAILURES.md` catalogs the impact on the
eval corpora). The preprocessor installs fallback definitions for the common
offenders:

| Macros | Fallback | Failure mode avoided |
|--------|----------|----------------------|
| `__UNUSED` | empty | `T &x __UNUSED` breaks the function definition |
| `__user`, `__iomem`, `__percpu`, `__rcu`, `__force`, `__init`, `__exit`, `__initdata`, `__exitdata`, `__read_mostly` | empty | kernel address-space/section annotations are syntax errors in declarators (`char __user *buf`, `int __init foo(void)`) |
| `PRI[diuxXo](8\|16\|32\|64)` | format-specifier string literal (e.g. `PRIu64` → `"llu"`) | `"%" PRIu64` leaves an identifier between string literals |
| `container_of(ptr, type, member)` | `((type *)(void *)(ptr))` | a type keyword in expression position; the fallback keeps the pointer flow and target type |
| `HWTEST(a, b, level)`, `HWTEST_F`, `HWTEST_P` | `static void a##_##b()` | gtest/OpenHarmony test macros followed by a body are unparseable, losing every test body in a file |

Semantics — a fallback is a definition of last resort, never an answer to
"is this defined?":

- Each fallback is installed only when the name is not already defined; any
  real definition — CLI `-D`, source or header `#define`, a cached include's
  macro delta — **overrides** it.
- Fallbacks behave as **undefined** throughout conditional evaluation: they do
  not satisfy `#ifdef` / `#ifndef` / `defined()` — so the ubiquitous guard
  idiom (`#ifndef container_of` + `#define container_of(...)`) takes its
  branch and the tree's genuine definition wins — and inside a `#if`
  expression they stay unexpanded identifiers evaluating to 0 (`#if 1 ||
  __init` is true; an empty expansion would mangle the expression). A source
  `#define` of the name then makes it a normal macro.
- Installation happens per preprocess, after cloning the shared warm table, so
  fallbacks apply even in warm-cache runs (and stay overridden if the warm
  table carries a real definition). CLI `-D` defines absent from the warm
  table are re-applied first-wins so they beat fallbacks in that path too.
- The include-expansion cache records the **ordered log** of `#define` /
  `#undef` directives a header executed (nested replays included), not a
  before/after table diff, and replays it through the same mutation helpers
  live directives use — so a cache hit and a cache miss agree on macro state,
  fallbacks included (see [Macro operations in cached
  entries](#macro-operations-in-cached-entries)).

## LineMap

The preprocessor records mappings from **output byte offsets** to original `(file, line, col)` in `LineMap`.

**Current behavior:** tree-sitter parses **preprocessed** source; IR spans (`Span` in `trace-ir`) are resolved through the `LineMap` to original `(file, line, col)` — `#include`d code attributes to its header, TU-local code keeps its original pre-expansion position, and macro-expanded code attributes to the expansion site's origin (identical coordinates when nothing was expanded). Cached `#include` expansions store their own sub-`LineMap`, which is spliced back on replay so origins survive caching.

The `LineMap` must keep byte-accurate offset mapping when extending the preprocessor.

## Include resolution

For `#include "header.h"` / `#include <header.h>`:

1. Directory of the including file
2. Paths from `IncludeGraph.include_dirs` (discovered + `--include`)
3. Error diagnostic if not found

Only **project-local** files under the analysis root are linked; system headers outside the tree are not resolved unless present in the project.

## Include graph and header indexing

| Behavior | Notes |
|----------|-------|
| `needs_preprocess` set | Files with `#include` edges (or included by another) run through the preprocessor |
| `source_cache` | Reuse file text while scanning `#include` edges |
| Reachable headers | Preprocessed file-locally, parsed/lowered **once** (PCH-style header IR), then merged into TUs |
| Orphan headers | Project `.h` never reached from any `.c` are indexed as their own units (may contain calls) |
| Parallel index | Header IR, orphan headers, and `.c` TUs: parallel parse/lower, sequential merge |

### Determinism

Indexing output must be identical across runs of the same tree. Two mechanisms guarantee this:

- **Macro warm pass** runs sequentially over C-reachable headers in canonical (`index_order`) order. Each header is warmed under a **fresh macro table** seeded only from command-line defines; the per-header final states are merged into a union table handed to later phases. Sharing one accumulating table across headers let include guards defined by earlier-warmed headers starve later headers' expansions (the starved text was then frozen into the expansion cache). Dedup between headers comes from the shared expansion cache, not from shared guard state.
- **Expansion-cache freeze**: during parallel phases the include-expansion cache is read-only (`PreprocessOptions::frozen_expansion_cache`). Hits replay warm-pass entries (produced deterministically); misses expand inline under each TU's own macro/guard state and are *not* inserted — first-writer-wins inserts would make results scheduling-dependent.

Translation units inherit the **union** of all warm-pass macro states: cached expansions replay without executing their `#define` directives, so TU-local code still needs those macros.

### Header IR (PCH-style)

Indexing sets `inline_include_bodies = false`. Nested cacheable `#include`s replay **macros and include-once state** but do not copy header tokens into the consumer's live output. Each header's preprocessed text is therefore file-local.

After the warm pass, reachable headers are parsed and lowered **once**. PCH order uses the include graph **plus preprocess `included_headers`** (macro includes the raw scanner misses). Independent leaves may run in parallel waves; a header is never in the same wave as a nested include it needs. Include **cycles** are not a parallel wave: leftovers are indexed in include-graph order so nested layouts stay visible. Nested `#include` IR merges **types and typedefs** from **direct** includes (plus this header's preprocess `included_headers`) so `struct StreamHost { struct IDeviceIoService service; }` sees `Dispatch`, and `GpioIrqFunc func` sees the typedef, without copying every descendant's functions/flow into ancestor units. Child PCH units already nested-merged grandchild types. Parallel isolation *without* those preprocess edges interned empty tags / `Int` and dropped field stores (`DeviceNodeExtDispatch` lost `DispatchToMessage`, `GpioOnDevEventReceive` lost `gpio->func`).

Headers that become reachable only after those preprocess edges are added join the PCH set (and leave the orphan path) so translation units can merge their prototypes.

Translation units parse only their own remainder and merge already-built header `UnitIndex`es for every header **reachable** via the include graph, plus preprocessor `included_headers` (a cached splice can omit a nested path from the graph edge, and types-only nested PCH does not copy nested prototypes into ancestors). That merge is **symbols only** (types + prototypes): header call sites and flow are already in the global program from PCH. Merge also rewrites leftover incomplete nested tags. That is the analogue of a PCH / clangd preamble. Merging only direct includes dropped `DispatchToMessage` from `DeviceNodeExtDispatch` (designated `.Dispatch =` in `hdf_wifi_core.c` when `sidecar.h` was not in that TU's `included_headers`).

Grammar follows the including language, not the extension alone: `.hpp`/`.hh`/`.hxx`/`.inl`/`.ipp` always use the C++ parser; a `.h` uses C++ if any C++ TU can reach it via the include graph, otherwise C. (Before PCH, header tokens were spliced into the TU and parsed with that TU's grammar, so `plugin.h` included from `plugin.cpp` was already C++.)

Standalone `preprocess_file` still inlines by default so a single-file expansion remains self-contained.

### Macro operations in cached entries

A cached expansion replays its text **without** executing the `#define`s it contains, so a header whose body *invokes* macros defined by an earlier-included header would starve: at warm time the dependency was processed inline (fine), but a consumer warmed later splices the dependency's cached body and never learns its macros. Therefore each `IncludeExpansion` records the **ordered log** of `#define` / `#undef` directives its processing executed, nested replays included (`IncludeExpansion::ops`). An ordered log rather than a table diff: a diff cannot represent a no-op `#undef` (name absent at capture, defined in a later consumer) or an undef-then-redefine of a name present at both boundaries.

`splice_cached` replays the log through the same mutation helpers live directives use, so a cache hit and a cache miss agree on everything a directive touches: the local table (a replayed `#define` overwrites, like live execution), builtin-fallback marks, the shared table under `accumulate_macros`, and the op log feeding an enclosing cached header's own entry.

### Cache self-containment

Cached expansions are flat text — nested `#include`s inside an entry were already resolved when the entry was built. An entry built while a nested header was already in this run's include-once set would otherwise freeze *without* that header's content, permanently hiding its definitions from every consumer routed through the entry.

Re-splicing the nested cached blob into **live output** on every such skip exponentiates on diamond include graphs (each copy contains previous copies). Instead, the skip is recorded on the in-progress cache frame and the nested expansion is **embedded only into that frame's cache entry** at the `#include` site. Live output stays unique per file; frozen-phase guard-skips stay silent as before. Duplicate definitions inside a cache entry are still harmless downstream (merge deduplicates same-origin entities; re-declarations remain valid C).

Entries also record which files they claim (`IncludeExpansion.files`); files whose expansion emitted nothing are not claimed, so symbol-scope registration (`headers_of`) does not attribute phantom contributions. A cached-header include whose entire body was skipped emits a visible Warning during non-frozen phases ("resolved include expanded to nothing") — silence here is how starvation bugs historically went unnoticed.

`index_order` itself is canonical: input files are sorted and dependents are visited in sorted order, so unordered `HashSet`/`HashMap` iteration cannot leak into processing order.

### Include-dir self-sufficiency

Project headers must resolve **without manual `-I` flags**: `discover_include_dirs` adds the root, every discovered header's parent directory, and every directory named `include`; a unique-basename fallback resolves names that match exactly one project file. Analyzing a tree root (e.g. an entire source checkout) therefore needs no include-path configuration.

Manual `-I` remains appropriate only for things the tool cannot discover:

- headers **outside** the analyzed root (system SDKs, vendored deps, sibling trees) — when analyzing a subdirectory whose dependencies live elsewhere;
- **platform selection**: when several dirs contain same-basename twins (e.g. per-OS adapter layers), `-I` order picks the intended one — discovery order is sorted-path and not platform-aware;
- paired with `-D` for the matching platform macros (e.g. `-D __LITEOS__`).

**Limitation:** The raw include scanner only sees literal `#include "..."` / `<...>` lines (no macro expansion). After the warm pass, preprocess `included_headers` — including headers reached via `#include FOO` — are added as graph edges, so those files join PCH instead of staying orphan. Headers excluded by `#if 0` in the preprocessor but visible in the raw graph are treated as reachable and not indexed separately — if the TU also omits them at preprocess time, calls in those headers can be missed.

## Error recovery

| Condition | Behavior |
|-----------|----------|
| Unknown `#directive` | Warning, skip line |
| Missing include | Error on TU |
| Unterminated `#if` | Error at EOF |
| Macro-argument parse failure | Warning `preprocess stopped in <file>`; output produced so far is kept |
| Preprocess failure (hard error) | Diagnostic; unit falls back to raw read |

A mid-run stop inside ONE nested header must not invalidate the whole TU: indexing keeps the truncated-but-LineMap-consistent prefix rather than falling back to raw source, because raw text drops every `#include`d declaration and feeds the parser unexpanded function-like macros. The stop message names the file where processing stopped so downstream tools can report the truncation point.

## Unsupported (v1)

- `_Pragma`
- `#import` (Objective-C)
- `#warning` / `#error` (partially recognized)
- Full C11 macro prescan/rescan semantics
- System include paths outside project tree (unless copied into tree)

## Testing

- Unit tests: `trace-preproc/src/`
- Integration fixtures: `tests/fixtures/preproc/` (including `self_ref_macro.c` for C11 hide-set / X-macro lists, `include_macro.c` for `#include FOO`)

See [ARCHITECTURE.md](ARCHITECTURE.md) for how preprocessing fits the full workflow.
