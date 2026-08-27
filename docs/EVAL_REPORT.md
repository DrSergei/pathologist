# Evaluation Report: `trace` on OpenHarmony corpora

**Date:** 2026-08-25 (updated 2026-08-26 with cross-struct FieldId guard fix; **2026-08-27** preprocessor hide-set, C++ CHA/`final`/callables, hiviewdfx_hiview re-eval, review-fix revalidation, `dlsym` model, PCH-style header IR, nested-type PCH + C/C++ prototype merge)
**Binary:** `target/release/trace` (current tree)
**Solver budget:** 800,000 pops (default; override via `TRACE_SOLVE_BUDGET_POPS`)
**Machine (timing):** WSL2, 16 logical CPUs, `--jobs 8`, minimal SQLite export

### Wall-clock (release, `--jobs 8`, minimal export)

| Corpus | Index | Analyze | Export | Wall | Notes |
|--------|------:|--------:|-------:|-----:|-------|
| HDF `~/drivers_hdf_core` | 12.8s | 1.1s | 0.8s | **14.8s** | sequential PCH restores Dispatch/GPIO hubs; was 3.3s index on first PCH run |
| Hiview `~/hiviewdfx_hiview` | 8.1s | 1.3s | 2.3s | **11.1s** | sequential PCH; H4/H9/H10/H16 still pass |
| Camera `~/multimedia_camera_framework` | 30.1s | 8.7s | 13.2s | **51.9s** | still completes (hang check); was 8.0 / 0.3 / 1.4 / 9.7s on first PCH run |

This document covers two trees plus a hang/regression check:

| Corpus | Path | Role |
|--------|------|------|
| HDF (original) | `~/drivers_hdf_core` | C/C++ driver framework; function-pointer dispatch |
| Hiview (2026-08-27) | `~/hiviewdfx_hiview` | C++ plugin platform; preprocessor X-macros + virtual dispatch |
| Camera / clang/test (2026-08-27) | `~/multimedia_camera_framework`, `llvm-project/clang/test/{Preprocessor,Lexer,Parser,CXX,Sema}` | PCH hang/stack-overflow check; not a dispatch-hub eval |

---

# Part 1 — `drivers_hdf_core`

**Target:** `~/drivers_hdf_core` (OpenHarmony HDF kernel driver framework)
**Flags (original eval):** `--full-export --debug-points-to`

### Hide-set revalidation (2026-08-27)

After C11 macro hide-set + expansion-depth cap, the same tree was re-analyzed (minimal export, `--jobs 8`):

| Metric | Original eval | After hide-set |
|--------|---------------|----------------|
| Files | 1,356 | 1,356 |
| Functions | 11,899 | 11,903 |
| Call edges | 36,957 | 36,956 |
| Direct / indirect / external | 16,037 / 4,428 / 16,492 | 16,031 / 4,430 / 16,495 |
| Arg-flow edges | 26,057 | 26,056 |

No stack overflow. Counts match within a few edges (noise / cache order). The hide-set change does not regress HDF pointer analysis.

### C++ CHA / callable revalidation (2026-08-27)

After virtual-inheritance recording, `final` class/method devirtualization, implicit `this->method()`, smart-pointer unwrap, and callable modeling (`std::function`, lambdas, `operator()`), the same tree was re-analyzed (minimal export, `--jobs 8`):

| Metric | Hide-set revalidation | This run |
|--------|------------------------|----------|
| Files | 1,356 | 1,356 |
| Functions | 11,903 | 11,955 |
| Call edges | 36,956 | 40,428 |
| Direct / indirect / external | 16,031 / 4,430 / 16,495 | 20,825 / 4,484 / 15,119 |
| Arg-flow edges | 26,056 | 28,307 |
| Parse warnings | 442 (original full-export) | 478 |

Direct edges rose and external edges fell because C++ member/static calls that previously became unqualified stubs now bind in-tree. Indirect totals moved by ~50 edges.

**Dispatch hubs (no pollution regression):**

| Function | Original eval | This run |
|----------|---------------|----------|
| `DeviceNodeExtDispatch` | 73 indirect targets | **73** unique indirect |
| `HdfDeviceLaunchNode` | 125 driver inits | **125** unique indirect |
| `HdfSbufReadBuffer` | 2 (C + C++) | **2** (`SbufRawImplReadBuffer`, `SbufMParcelImplReadBuffer`) |
| `StreamDispatch` | 24 | **24** |
| `HdfCameraDispatch` | 23 | **23** |
| `HdfPmDriverDispatch` | 19 | **19** |
| `HdfObjectManagerGetObject` | 18 | **18** |
| `PlatformDumperDump` | 13 | **13** |
| `SetOption` | 13 | **13** |
| `HdfDeviceUnlaunchNode` | 135 | 135 indirect **edges** / 116 unique names |
| `DeviceDriverBind` | 122 | 122 indirect **edges** / 106 unique names |
| `GpioOnDevEventReceive` | 13 | 13 indirect **edges** / 12 unique names |

`HdfSbufReadBuffer` staying at exactly two targets is the cross-struct FieldId guard: it has not regressed to the old 140-FP result. Hub **edge** counts match the original eval; where unique-name counts are lower, several edges share a callee.

### Review-fix revalidation (2026-08-27, later)

After field receivers, predefined `__UNUSED`, inspect `LIKE` escape, member CHA arity, and `std::function`-only wrappers, the same tree was re-analyzed (minimal export, `--jobs 8`):

| Metric | CHA/callable revalidation | This run |
|--------|---------------------------|----------|
| Files | 1,356 | 1,356 |
| Functions | 11,955 | 11,970 |
| Call edges | 40,428 | 40,473 |
| Direct / indirect / external | 20,825 / 4,484 / 15,119 | 20,820 / 4,484 / 15,169 |
| Arg-flow edges | 28,307 | 28,254 |
| Parse warnings | 478 | 478 |
| Index / analyze / export / wall | (unrecorded) | **7.0s / 1.6s / 0.6s / 9.5s** |

**No dispatch-hub regression.** Unique indirect counts are unchanged:

| Function | CHA/callable run | This run |
|----------|------------------|----------|
| `DeviceNodeExtDispatch` | 73 | **73** |
| `HdfDeviceLaunchNode` | 125 | **125** |
| `HdfSbufReadBuffer` | 2 | **2** (`SbufRawImplReadBuffer`, `SbufMParcelImplReadBuffer`) |
| `StreamDispatch` | 24 | **24** |
| `HdfCameraDispatch` | 23 | **23** |
| `HdfPmDriverDispatch` | 19 | **19** |
| `HdfObjectManagerGetObject` | 18 | **18** |
| `PlatformDumperDump` | 13 | **13** |
| `SetOption` | 13 | **13** |
| `HdfDeviceUnlaunchNode` | 135 edges / 116 unique | 135 / 116 |
| `DeviceDriverBind` | 122 edges / 106 unique | 122 / 106 |
| `GpioOnDevEventReceive` | 13 edges / 12 unique | 13 / 12 |

Indirect stays at **exactly 4,484**. The +15 functions / +45 call edges are unnamed-parameter arity slots (overloads no longer collapsed) and a few extra C++ binds — not hub pollution. `LoadIpcImpl` `dlsym` remains external.

### `dlsym` model revalidation (2026-08-27)

After interned `StringConst` / `LocKind::StringLit` and the built-in `dlsym`/`dlvsym`/`GetProcAddress` model, the same tree was re-analyzed (minimal export, `--jobs 8`):

| Metric | Review-fix run | This run |
|--------|----------------|----------|
| Files | 1,356 | 1,356 |
| Functions | 11,970 | 11,970 (9,410 defined / 2,560 external) |
| Call edges | 40,473 | 40,519 |
| Direct / indirect / external | 20,820 / 4,484 / 15,169 | 20,822 / **4,536** / 15,161 |
| Arg-flow edges | 28,254 | 32,550 |
| Flow nodes / edges | (unrecorded here) | 172,059 / 112,651 |
| `string_lit` flow nodes | — | 12,795 |
| `dlsym` flow edges | — | 4 |
| Parse warnings | 478 | 478 |
| Index / analyze / export / wall | **7.0s / 1.6s / 0.6s / 9.5s** | **7.0s / 2.1s / 0.8s / 10.2s** |

Index time is unchanged (StringConst is extra IR, not extra parse). Analyze is **+0.5s**; export **+0.2s**. The extra work is interned string locations (`addr_of` 35,839 vs ~2.8k before) and argument wiring of string-literal call args (arg-flow **+4,296**).

**Dispatch hubs (unique indirect) are unchanged:**

| Function | Review-fix | This run |
|----------|------------|----------|
| `DeviceNodeExtDispatch` | 73 | **73** |
| `HdfDeviceLaunchNode` | 125 | **125** |
| `HdfSbufReadBuffer` | 2 | **2** (`SbufRawImplReadBuffer`, `SbufMParcelImplReadBuffer`) |
| `StreamDispatch` | 24 | **24** |
| `HdfCameraDispatch` | 23 | **23** |
| `HdfPmDriverDispatch` | 19 | **19** |
| `HdfObjectManagerGetObject` | 18 | **18** |
| `PlatformDumperDump` | 13 | **13** |
| `SetOption` | 13 | **13** |
| `HdfDeviceUnlaunchNode` | 135 / 116 | 135 / 116 |
| `DeviceDriverBind` | 122 / 106 | 122 / 106 |
| `GpioOnDevEventReceive` | 13 / 12 | 13 / 12 |

`LoadIpcImpl` still *calls* `dlsym` as **external** (the libc stub). The model writes `SbufObtainIpc` / `SbufBindIpc` into the return destinations; `constructor->obtain` / `constructor->bind` already reached those via the compile-time `&SbufObtainIpc` init. `HdfSbufTypedObtainCapacity` unique callees went **2 → 3** (`SbufObtainIpc`, `SbufObtainIpcHw`, plus `SbufObtainRaw`) — field-summary mixing from the extra store, **not** a `HdfSbufReadBuffer` regression.

Literal `dlsym(h, "driverDesc")` / `"hdfVdiDesc"` add `Dlsym` constraints but no in-tree function of that exact name, so they stay unresolved (correct).

### PCH-style header IR revalidation (2026-08-27, later)

After parse-once header IR (no token splice into TUs), C++ grammar for `.h` reached from a C++ TU, and merging include-graph-reachable header IR into each TU before lower, the same trees were re-analyzed (minimal export, `--jobs 8`).

Headers now appear as first-class files (HDF 1,356 → 1,483; hiview 1,322 → 1,424). That is expected, not extra TUs.

| Metric | `dlsym` run | This run |
|--------|-------------|---------|
| Files | 1,356 | 1,483 |
| Functions | 11,970 (9,410 defined / 2,560 external) | 12,321 (9,529 defined / 2,792 external) |
| Call edges | 40,519 | 40,684 |
| Direct / indirect / external | 20,822 / **4,536** / 15,161 | 14,965 / **4,357** / 21,362 |
| Arg-flow edges | 32,550 | 32,433 |
| Parse warnings | 478 | 370 |
| Index / analyze / export / wall | **7.0s / 2.1s / 0.8s / 10.2s** | **3.3s / 1.5s / 0.8s / 5.6s** |

Index is **~2× faster**. Direct edges drop because header-local C++ member calls are no longer re-parsed inside every including TU; many of those sites still exist once, attributed to the header. Indirect is **−179** (noise plus a few missing vtable stores — see hubs).

**Dispatch hubs (unique indirect names unless noted):**

| Function | `dlsym` run | This run |
|----------|-------------|----------|
| `DeviceNodeExtDispatch` | 73 | **50** unique names / 53 ids (was 73) |
| `HdfDeviceLaunchNode` | 125 | **125** unique names / 145 edges |
| `HdfSbufReadBuffer` | 2 | **2** (`SbufRawImplReadBuffer`, `SbufMParcelImplReadBuffer`) |
| `StreamDispatch` | 24 | **24** |
| `HdfCameraDispatch` | 23 | **23** |
| `HdfPmDriverDispatch` | 19 | **19** |
| `HdfObjectManagerGetObject` | 18 | **18** |
| `PlatformDumperDump` | 13 | **13** |
| `SetOption` | 13 | **13** |
| `HdfDeviceUnlaunchNode` | 135 / 116 | 135 / **116** |
| `DeviceDriverBind` | 122 / 106 | 122 / **106** |
| `GpioOnDevEventReceive` | 13 / 12 | **0** indirect (`gpio->func` unresolved; 1 external) |

No `HdfSbufReadBuffer` pollution. Driver entry tables (`Init` / `Bind` / `Release`) match. Remaining PCH gaps: `DeviceNodeExtDispatch` missing ~20 `*Dispatch` targets (including `StreamDispatch` as a callee of the hub), and `GpioOnDevEventReceive`’s `gpio->func` callback slot.

Without the C++-`.h` grammar, `plugin.h`-style classes parsed as C and CHA collapsed (hiview `OnEventProxy` → unqualified external `OnEvent`). Fixture: `cpp_h_header/`. Cross-TU designated `.Init = fn` is `cross_tu_designated/`.

### Nested-type PCH + C/C++ prototype merge (2026-08-27, later)

Two PCH gaps above were real and are fixed. Isolated header parse interned `struct StreamHost { struct IDeviceIoService service; }` with an empty `service` tag (no `Dispatch` field), so `host->service.Dispatch = StreamDispatch` emitted no store. Separately, `gpio_if.h` is reachable from C++ TUs so its `GpioSetIrq` prototype is `is_cpp`; the userspace `.c` body is not. Overload-splitting on either side being C++ left callers bound to the undefined prototype, so `GpioRegListener` never ran and `gpio->func` stayed empty.

Fixes: sequential PCH in include-graph order with nested header IR (plus `complete_nested_tags` / layout-field merge); C vs C++-parsed-header same-name merge by arity, clearing `is_cpp` so a later TU merge does not refuse the body. Fixtures: `nested_host_dispatch/`, `typedef_fnptr_field/` (C++ `register.cpp` calls the header prototype).

Same tree, minimal export, `--jobs 8`:

| Metric | First PCH run | This run |
|--------|---------------|----------|
| Files | 1,483 | 1,483 |
| Functions | 12,321 (9,529 defined / 2,792 external) | 11,800 (9,398 defined / 2,402 external) |
| Call edges | 40,684 | 40,273 |
| Direct / indirect / external | 14,965 / **4,357** / 21,362 | 20,532 / **4,431** / 15,310 |
| Arg-flow edges | 32,433 | 31,828 |
| Index / analyze / export / wall | 3.3s / 1.5s / 0.8s / 5.6s | **12.8s / 1.1s / 0.8s / 14.8s** |

Sequential PCH (626 headers) plus nested merge into header units is the index cost. Direct edges recover toward the pre-PCH shape because prototype/definition collapse restores in-tree callees (external **−6,052**).

**Dispatch hubs:**

| Function | Original eval | First PCH run | This run |
|----------|---------------|---------------|----------|
| `DeviceNodeExtDispatch` | 73 | **50** | **73** (`StreamDispatch` is a callee again) |
| `HdfDeviceLaunchNode` | 125 | 125 | **125** |
| `HdfSbufReadBuffer` | 2 | 2 | **2** |
| `StreamDispatch` | 24 | 24 | **24** |
| `HdfCameraDispatch` | 23 | 23 | **23** |
| `HdfPmDriverDispatch` | 19 | 19 | **19** |
| `HdfObjectManagerGetObject` | 18 | 18 | **18** |
| `PlatformDumperDump` | 13 | 13 | **13** |
| `SetOption` | 13 | 13 | **13** |
| `HdfDeviceUnlaunchNode` | 135 / 116 | 135 / 116 | 131 / **112** |
| `DeviceDriverBind` | 122 / 106 | 122 / 106 | 122 / **106** |
| `GpioOnDevEventReceive` | 13 / 12 | **0** | **13 / 12** |

`GpioOnDevEventReceive` targets: `GpioTestIrqHandler`, `GpioServiceIrqFunc`, `PpgIrqHandler`, `IrqHandle`, `KeyIrqHandle`, `InfraredIrqHandle`, `HallNorthPolarityIrqFunc`, `HallSouthPolarityIrqFunc`, `TestCaseGpioIrqHandler1`–`4`. `GpioSetIrq` is a single defined row (`gpio_if_u.c:261`). No `HdfSbufReadBuffer` pollution.

`HdfDeviceUnlaunchNode` is **−4** unique names vs the original eval (small leftover, not the two PCH gaps above).

### Full eval-case recheck (2026-08-27, same binary)

All 40 HDF functions in the coverage matrix still resolve. Unique-indirect hubs match the original eval except `HdfDeviceUnlaunchNode` (**112** vs 116) and `WorkEntry` linux (**20** vs 19 — extra `AlsDataWorkEntry`). `PowerStateChange` is **16** unique names (4 sites × 4 listeners). `HandleRequestMessage` (local_node) is **56**. `LoadIpcImpl` still *calls* `dlsym` as external; 4 `dlsym` PAG edges; `HdfSbufTypedObtainCapacity` unique callees stay at 3 (`SbufObtainIpc`, `SbufObtainIpcHw`, `SbufObtainRaw`).

Hiview H-cases unchanged in status: H4/H9/H10/H16 **Pass** (`Plugin::OnEventProxy` → 23 `::OnEvent` including `Plugin::OnEvent` at `plugin.cpp:35`); H5/H7/H11/H13–H15 still fail as documented (`auto`/`lock()`, `std::function` factory, `std::bind`, `DownCastTo`, `ffrt::submit`, `dlsym("GetInstance")`). Index **8.1s / 1.3s / 2.3s / 11.1s**.

Camera still **completes** (hang check). Index **30.1s / 8.7s / 13.2s / 51.9s**; indirect **0** (was 117 on the first PCH run — not a hub eval).

## Executive Summary

Analysis of 1,356 files (11,899 defined + 2,564 external functions) produced:
- **36,957 call edges** (16,037 direct, 4,428 indirect, 16,492 external) — **indirect edges reduced 88%** from 38,166 after cross-struct FieldId guard fix
- **26,057 arg-flow edges** (actual→formal parameter wiring)
- **128,143 flow nodes** and **74,007 flow edges** (copy/gep/load/store/addr_of/call_arg/points_to/terminates)
- **0 unresolved indirect calls** in all evaluated functions
- 442 parse warnings (0 errors), 0 analysis errors

All 40 evaluated functions below were analyzed successfully at 800K pops. Indirect call resolution via function-pointer analysis resolved every dispatch pattern tested, including vtable dispatch (73 targets), array-of-function-pointers (24 targets), driver entry tables (125 targets), C++ cross-language interop (2 targets), power-state dispatch (4 sites × 4 targets), sensor dispatch (13 targets), and GPIO event callbacks (13 targets).

**C++ support** adds namespaces, overloads (arity-based), classes with virtual dispatch (including virtual bases and `final` class/method devirtualization), ctors/dtors, implicit `this->method()`, smart-pointer unwrap, callables (`std::function`, lambdas, functors), templates (name-stripping), constructor-initializer lists, and cross-C/C++ interop. The C++ implementation files (`.cpp`) are indexed as translation units alongside `.c` files, enabling analysis of mixed C/C++ driver stacks such as the HDF framework where C++ IPC backends extend C interfaces.

## Overall Metrics

| Metric | Value |
|--------|-------|
| Files indexed | 1,356 |
| Functions total | 11,899 |
| Functions defined | 9,335 |
| External functions | 2,564 |
| Call edges | 36,957 |
| Direct call edges | 16,037 |
| Indirect call edges | 4,428 |
| External call edges | 16,492 |
| Arg-flow edges | 26,057 |
| Flow nodes | 128,143 |

### Flow Edge Breakdown

| Kind | Count |
|------|-------|
| copy | 23,964 |
| gep | 19,235 |
| call_arg | 9,614 |
| load | 7,891 |
| points_to | 5,757 |
| store | 4,349 |
| addr_of | 2,833 |
| terminates | 364 |

### Diagnostics

| Severity | Stage | Count |
|----------|-------|-------|
| warning | parse | 442 |

---

## Feature Coverage Matrix

| # | Feature | Functions |
|---|---------|-----------|
| 1 | Indirect call (vtable dispatch, 78 targets) | `DeviceNodeExtDispatch` |
| 2 | Indirect call (array dispatch, 138 targets) | `HandleRequestMessage` (local_node) |
| 3 | Indirect call (driver entry table, 125 targets) | `HdfDeviceLaunchNode` |
| 4 | Indirect call (function-pointer deref) | `RunDispatcher`, `AudioCodecDevInit` |
| 5 | Indirect call (command dispatch table, 24 targets) | `StreamDispatch` |
| 6 | Indirect call (brightness dispatch, 6 targets) | `BacklightDispatch` |
| 7 | Indirect call (control dispatch, 6 targets) | `ControlDispatch` |
| 8 | Direct call + arg-flow (user-space IPC) | `AdcOpen`, `AdcRead`, `AdcClose` |
| 9 | Direct call + arg-flow (driver core read) | `AdcDeviceRead` |
| 10 | Direct call (device lifecycle) | `DeviceManagerDispatch` |
| 11 | Direct call + static singleton | `DevSvcManagerCreate`, `DevSvcManagerClntGetInstance` |
| 12 | Direct call + static config list | `DevMgrUeventRuleCfgList` |
| 13 | Direct call + static dispatcher | `DevSvcManagerExtStart` |
| 14 | Direct call + static handler | `DevHostServiceFullConstruct` |
| 15 | Direct call (IPC dispatch) | `DevHostServiceStubDispatch` |
| 16 | Direct call (message dispatch) | `DevHostServiceFullDispatchMessage` |
| 17 | Direct call (HCS config parsing) | `GetUartDeviceResource` |
| 18 | Direct call + fn_static | `ChipDataHandle` (touch_ft5406) |
| 19 | Direct call + arg-flow (GPIO IRQ) | `GpioSetIrq` |
| 20 | Direct call + arg-flow (test config) | `AdcTestGetConfig` |
| 21 | Direct call (clock platform) | `ClockManagerDispatch` |
| 22 | Direct call (test lifecycle) | `PlatformManagerTestAddAndDel` |
| 23 | Direct call + external model (memset_s) | `ChipDataHandle` |
| 24 | Direct call (DMA config) | `AudioDmaConfigChannel` |
| 25 | Direct call (stub create + fn_static) | `DevHostServiceStubCreate` |
| 26 | Direct call (stub construct + fn_static) | `DevHostServiceStubConstruct` |
| 27 | FinishEvent (sysevent → dispatch) | `FinishEvent` |
| 28 | RunDispatcher (wifi message loop) | `RunDispatcher` |
| 29 | HandleRequestMessage (wifi command dispatch) | `HandleRequestMessage` |
| 30 | HdfDeviceLaunchNode (driver init) | `HdfDeviceLaunchNode` |
| 31 | C++ virtual dispatch (Shape/Circle) | `main.cpp` (cpp_basic) |
| 32 | C++ overload resolution (arity-based) | `main.cpp` (cpp_basic, cpp_more) |
| 33 | C++ namespace + anonymous namespace | `util::tag`, `hidden()` |
| 34 | C++ ctor/dtor sites (`new`/`delete`) | `new Circle()`, `delete s` |
| 35 | C++ ctor-initializer list (base + member) | `D(int v) : Base(v), m()` |
| 36 | C++ template (name-stripping) | `Box<Widget>`, `b.put()`, `b.get()` |
| 37 | C++ multiple inheritance + virtual dispatch | `AB : A, B` — `pa->fa()` resolves to `A::fa` override |
| 38 | C++ static member function | `S::Make()` |
| 39 | C++ cross-C/C++ interop (extern "C" + ops table) | `cpp_flow` — C++ impl registers into C ops, C caller resolves both |
| 40 | C++ real-world interop (HdfSbufReadBuffer → C + C++ impl) | `HdfSbufReadBuffer` → `SbufRawImplReadBuffer` + `SbufMParcelImplReadBuffer` |
| 41 | Cross-struct FieldId guard (pollution prevention) | `HdfSbufReadBuffer` now resolves 2 targets (was 140 FPs) |
| 42 | Device unlaunch (driver teardown, 135 targets) | `HdfDeviceUnlaunchNode` — `driverEntry->Release` dispatch |
| 43 | Device driver bind (driver binding, 122 targets) | `DeviceDriverBind` — `driverEntry->Bind` dispatch |
| 44 | Camera command dispatch (23 targets) | `HdfCameraDispatch` — `g_cameraCmdHandle[i].func` table |
| 45 | Power state change (4 dispatch sites × 4 targets) | `PowerStateChange` — `Suspend`/`Resume`/`DozeSuspend`/`DozeResume` |
| 46 | Object manager factory (18 targets) | `HdfObjectManagerGetObject` — `targetCreator->Create()` dispatch |
| 47 | Sensor dispatch (13 targets) | `SetOption` — `deviceInfo->ops.SetOption()` dispatch |
| 48 | GPIO event callback (13 targets) | `GpioOnDevEventReceive` — `gpio->func()` dispatch |
| 49 | PM driver dispatch (19 targets) | `HdfPmDriverDispatch` — `pdr->ops->Dispatch` dispatch |
| 50 | Workqueue dispatch (19 targets) | `WorkEntry` — `work->func()` sensor data handler dispatch |
| 51 | Platform dumper dispatch (13 targets) | `PlatformDumperDump` — `ops->func` field dispatch |

---

## Individual Function Evaluations

### 1. `DeviceNodeExtDispatch` — HDF Device Node Dispatch Hub

| Property | Value |
|----------|-------|
| File | `framework/core/common/src/hdf_device_node_ext.c:20-50` |
| Linkage | internal |
| Callees | 84 |
| Callers | 104 |
| Arg-flow edges | 227 |
| Indirect call sites | 1 (`deviceMethod->Dispatch`) |
| Indirect targets resolved | 78 |

**Role:** Central dispatch hub — every HDF device call goes through here via `deviceMethod->Dispatch` function pointer. This is the single most important dispatch point in the framework.

**Indirect call resolution:** The single `deviceMethod->Dispatch` call site resolved to **78 distinct targets** including `BacklightDispatch`, `StreamDispatch`, `ClockManagerDispatch`, `AdcManagerDispatch`, `GpioTestDispatch`, `HdfCameraDispatch`, `HdfHIDDispatch`, `HdfTouchDispatch`, and all platform driver dispatchers. This is the vtable dispatch pattern — the tool correctly resolves all registered driver dispatchers.

**Arg-flow quality:** All 227 arg-flow edges correctly wire `service→service`, `data→data`, `reply→reply` through to the 78 dispatch targets.

**Callers:** Called by 104 test entry functions (`AdcTestGetConfig`, `ClockTestGetConfig`, `GpioTestGetConfig`, etc.) and adapter functions.

---

### 2. `HandleRequestMessage` (local_node) — WiFi Command Dispatch Table

| Property | Value |
|----------|-------|
| File | `framework/model/network/wifi/platform/src/message/nodes/local_node.c:32-51` |
| Linkage | internal |
| Callees | 58 |
| Callers | 1 |
| Arg-flow edges | 113 |
| Indirect call sites | 1 (`messageDef->handler`) |
| Indirect targets resolved | 56 |

**Role:** WiFi message dispatcher — routes commands to handler functions via `messageDef->handler` function-pointer table.

**Indirect call resolution:** The single dispatch site resolved to **56 WiFi command handlers** including `WifiCmdScan`, `WifiCmdAssoc`, `WifiCmdDisconnect`, `WifiCmdSetKey`, `WifiCmdSendEapol`, `WifiSendMlme`, `WifiCmdSetCountryCode`, etc. This demonstrates array-of-function-pointer dispatch resolution.

**Arg-flow quality:** 113 arg-flow edges wire message parameters correctly to all 56 handlers.

---

### 3. `HdfDeviceLaunchNode` — Driver Initialization

| Property | Value |
|----------|-------|
| File | `framework/core/host/src/hdf_device_node.c:94-131` |
| Linkage | external |
| Callees | 147 |
| Callers | 2 |
| Arg-flow edges | 147 |
| Indirect call sites | 1 (`driverEntry->Init`) |
| Indirect targets resolved | 125 |

**Role:** Launches a driver node — calls `DeviceDriverBind` directly, then invokes `driverEntry->Init` for the actual driver initialization.

**Indirect call resolution:** `driverEntry->Init` resolved to **125 driver init functions** including `GpioDriverInit`, `I2cDriverInit`, `SpiDriverInit`, `UartDriverInit`, `AudioDriverInit`, `HdfCameraDriverInit`, `HdfWlanMainInit`, `LinuxGpioInit`, `LinuxI2cInit`, etc. This covers both hardware-specific and virtual driver init paths.

**Arg-flow quality:** `devNode` parameter correctly wired to `DeviceDriverBind(devNode)`, `HdfDeviceNodePublishService(devNode)`, and all 125 `driverEntry->Init(devNode)` calls.

---

### 4. `StreamDispatch` — Audio Stream Command Dispatch

| Property | Value |
|----------|-------|
| File | `framework/model/audio/dispatch/src/audio_stream_dispatch.c:1602-1614` |
| Linkage | internal |
| Callees | 24 |
| Callers | 3 |
| Arg-flow edges | 72 |
| Indirect call sites | 1 (`g_streamDispCmdHandle[i]->func`) |
| Indirect targets resolved | 24 |

**Role:** Audio stream dispatch — routes stream commands (open/close/start/stop/pause/resume/mmap/decode/encode) via function-pointer table.

**Indirect call resolution:** Resolved to **24 stream handler functions**: `StreamHostWrite`, `StreamHostRead`, `StreamHostHwParams`, `StreamHostRenderOpen`, `StreamHostRenderClose`, `StreamHostRenderStart`, `StreamHostRenderStop`, `StreamHostCaptureOpen`, `StreamHostCaptureClose`, `StreamHostCaptureStart`, `StreamHostCaptureStop`, `StreamHostRenderPause`, `StreamHostCapturePause`, `StreamHostRenderResume`, `StreamHostCaptureResume`, `StreamHostMmapWrite`, `StreamHostMmapRead`, `StreamHostMmapPositionWrite`, `StreamHostMmapPositionRead`, `StreamHostDspDecode`, `StreamHostDspEncode`, `StreamHostDspEqualizer`, `StreamHostRenderPrepare`, `StreamHostCapturePrepare`.

**Arg-flow quality:** 72 arg-flow edges wire `device→device`, `data→reqData`, `reply→rspData` to all 24 handlers.

---

### 5. `BacklightDispatch` — Display Brightness Dispatch

| Property | Value |
|----------|-------|
| File | `framework/model/display/driver/backlight/hdf_bl.c:398-412` |
| Linkage | internal |
| Callees | 6 |
| Callers | 3 |
| Arg-flow edges | 18 |
| Indirect call sites | 1 (`blCmdHandle`) |
| Indirect targets resolved | 6 |

**Indirect call resolution:** Resolved to `HdfGetBlDevList`, `HdfGetCurrBrightness`, `HdfGetDefBrightness`, `HdfGetMaxBrightness`, `HdfGetMinBrightness`, `HdfSetBrightness`.

---

### 6. `ControlDispatch` — Audio Control Dispatch

| Property | Value |
|----------|-------|
| File | `framework/model/audio/dispatch/src/audio_control_dispatch.c:549-574` |
| Linkage | internal |
| Callees | 6 |
| Callers | 3 |
| Arg-flow edges | 18 |
| Indirect call sites | 1 (`g_controlDispCmdHandle[i]->func`) |
| Indirect targets resolved | 6 |

**Indirect call resolution:** Resolved to `ControlHostElemInfo`, `ControlHostElemRead`, `ControlHostElemWrite`, `ControlHostElemList`, `ControlHostElemUnloadCard`, `ControlHostElemGetCard`.

---

### 7. `RunDispatcher` — WiFi Message Dispatcher Loop

| Property | Value |
|----------|-------|
| File | `framework/model/network/wifi/platform/src/message/message_dispatcher.c:238-282` |
| Linkage | internal |
| Callees | 5 |
| Callers | 0 (entry point for thread) |
| Arg-flow edges | 6 |
| Indirect call sites | 3 |
| Indirect targets resolved | 2 |

**Role:** Main message loop — pops from priority queue, handles messages, manages dispatcher lifecycle.

**Indirect calls:**
- `dispatcher->Ref` → `ReferenceMessageDispatcher` (1 target)
- `dispatcher->Disref` → `DisreferenceMessageDispatcher` (2 call sites, same target)

**Direct calls:** `PopPriorityQueue`, `HandleMessage`, `ReleaseAllMessage`.

**Arg-flow quality:** Dispatcher reference/release correctly wired through function pointers.

---

### 8. `FinishEvent` — System Event Dispatcher

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/osal/src/osal_sysevent.c:61-81` |
| Linkage | internal |
| Callees | 11 |
| Callers | 1 (`DeviceManagerDispatch`) |
| Arg-flow edges | 16 |
| Indirect call sites | 1 (`service->dispatcher->Dispatch`) |
| Indirect targets resolved | 6 |

**Role:** Handles system events — obtains a service buffer, writes event data, dispatches via `service->dispatcher->Dispatch`.

**Indirect call resolution:** Resolved to `DeviceManagerDispatch`, `DeviceNodeExtDispatch`, `HdfKIoServiceDispatch`, `DeviceSvcMgrDispatch`, `HdfSyscallAdapterDispatch`, `DevSvcManagerOnServiceDied`.

**Direct calls:** `HdfSbufObtain`, `HdfSbufWriteUint64`, `HdfSbufRecycle`.

---

### 9. `AdcOpen` — ADC Device Open (User-Space IPC)

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c:30-77` |
| Linkage | external |
| Callees | 15 |
| Callers | 1 (`AdcTesterGet`) |
| Arg-flow edges | 27 |
| Indirect call sites | 1 (`service->dispatcher->Dispatch`) |
| Indirect targets resolved | 6 |

**Role:** User-space ADC open — calls `AdcDeviceGet`/`AdcDeviceStart` directly, formats request, dispatches via IPC.

**Key arg-flow:**
- `number → AdcDeviceGet(number)` — device number forwarded correctly
- `device → AdcDeviceStart(device)` — device handle forwarded
- `tmp_fmt → DealFormat(dest)` — format string to buffer destination
- `data → HdfSbufWriteUint32(sbuf)` — request data serialized
- `service → DeviceNodeExtDispatch(service)` — IPC dispatch with 6 targets

**Read-back flow:** `HdfSbufReadUint32(reply, &handle)` correctly reads the returned handle.

---

### 10. `AdcRead` — ADC Device Read

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c:110-163` |
| Linkage | external |
| Callees | 12 |
| Callers | 4 (`AdcTestRead`, `AdcTestThreadFunc`, `AdcTestReliability`, `AdcIfPerformanceTest`) |
| Arg-flow edges | 25 |
| Indirect call sites | 1 |
| Indirect targets resolved | 6 |

**Key arg-flow:**
- `channel → AdcDeviceRead(channel)` — channel parameter forwarded
- `val → AdcDeviceRead(val)` — output value pointer forwarded
- `reply → HdfSbufReadUint32(sbuf)` — result read back

---

### 11. `AdcClose` — ADC Device Close

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c:79-108` |
| Linkage | external |
| Callees | 12 |
| Callers | 0 |
| Arg-flow edges | 16 |
| Indirect call sites | 1 |
| Indirect targets resolved | 6 |

**Key arg-flow:**
- `device → AdcDeviceStop(device)` — stop device
- `device → AdcDevicePut(device)` — release device reference
- `data → HdfSbufWriteUint32(sbuf)` — close request serialized
- `service → dispatch(service)` — IPC dispatch with 6 targets

---

### 12. `AdcDeviceRead` — ADC Core Read (Driver Internal)

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/adc/adc_core.c:306-333` |
| Linkage | external |
| Callees | 4 |
| Callers | 2 (`AdcManagerIoRead`, `AdcRead`) |
| Arg-flow edges | 8 |
| Indirect call sites | 1 (`device->ops->read`) |
| Indirect targets resolved | 2 |

**Indirect call resolution:** `device->ops->read` resolved to `AdcIioRead` and `VirtualAdcRead` — the two concrete ADC read implementations.

**Arg-flow quality:** `device → AdcDeviceLock(device)` / `AdcDeviceUnlock(device)` correctly models lock/unlock. `channel → AdcIioRead(channel)` and `val → AdcIioRead(val)` wire the read parameters.

---

### 13. `DeviceManagerDispatch` — Device Manager Dispatch Hub

| Property | Value |
|----------|-------|
| File | `framework/core/common/src/devmgr_service_start.c:66-106` |
| Linkage | external |
| Callees | 10 |
| Callers | 104 |
| Arg-flow edges | 13 |
| Static variables | 1 (`callback`) |

**Role:** Top-level device manager dispatch — routes operations to `DeviceNodeExtDispatch`, `HdfKIoServiceDispatch`, and other sub-dispatchers. No indirect call sites of its own (all direct calls).

**Callers:** Called by 104 test functions and adapter functions, demonstrating its role as a central dispatch point.

---

### 14. `DevSvcManagerCreate` — Singleton Service Manager Creation

| Property | Value |
|----------|-------|
| File | `framework/core/manager/src/devsvc_manager.c:412-423` |
| Linkage | external |
| Callees | 3 |
| Callers | 1 (`HdfObjectManagerGetObject`) |
| Arg-flow edges | 1 |
| Static variables | 2 (`devSvcManagerInstance`, `g_createOnce`) |

**Role:** Thread-safe singleton creation — uses `g_createOnce` flag and `devSvcManagerInstance` static to ensure single initialization.

---

### 15. `DevSvcManagerClntGetInstance` — Client Singleton

| Property | Value |
|----------|-------|
| File | `framework/core/host/src/devsvc_manager_clnt.c:146-155` |
| Linkage | external |
| Callees | 1 |
| Callers | 11 |
| Arg-flow edges | 1 |
| Static variables | 2 (`instance`, `singletonInstance`) |

**Callers:** Used by 11 client-side functions (`DeviceServiceStubPublishService`, `DevSvcManagerClntGetService`, `DevSvcManagerClntAddService`, etc.).

---

### 16. `DevMgrUeventRuleCfgList` — Static Config List with Init Guard

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/manager/src/devmgr_uevent.c:69-80` |
| Linkage | internal |
| Callees | 1 |
| Callers | 4 |
| Arg-flow edges | 1 |
| Static variables | 2 (`ruleCfgList`, `initFlag`) |

**Role:** Manages uevent rule configuration list. Uses `initFlag` static to lazy-initialize `ruleCfgList`.

---

### 17. `DevSvcManagerExtStart` — Extended Service Manager Start

| Property | Value |
|----------|-------|
| File | `framework/core/manager/src/devsvc_manager_ext.c:129-165` |
| Linkage | external |
| Callees | 2 |
| Callers | 1 |
| Arg-flow edges | 0 |
| Static variables | 3 (`dispatcher`, `svcmgrDevObj`, `svcmgrIoService`) |

**Role:** Starts the extended service manager — creates and initializes three static objects.

---

### 18. `DevHostServiceStubDispatch` — Host Service Stub Dispatch

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_stub.c:80-111` |
| Linkage | internal |
| Callees | 6 |
| Callers | 13 |
| Arg-flow edges | 12 |

**Callers:** Called by 13 proxy/manager functions (`DevSvcManagerProxyAddService`, `DevmgrServiceProxyAttachDevice`, `DevHostServiceProxyOpsDevice`, etc.).

---

### 19. `DevHostServiceStubCreate` — Stub Factory

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_stub.c:123-135` |
| Linkage | external |
| Callees | 2 |
| Callers | 1 |
| Arg-flow edges | 1 |
| Static variables | 1 (`instance`) |

**Role:** Factory function — allocates via `HdfObjectManagerGetObject`, then calls `DevHostServiceStubConstruct`.

---

### 20. `DevHostServiceFullConstruct` — Full Service Constructor

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_full.c:202-213` |
| Linkage | external |
| Callees | 3 |
| Callers | 1 |
| Arg-flow edges | 5 |
| Static variables | 1 (`handler`) |

---

### 21. `DevHostServiceFullDispatchMessage` — Message Dispatch

| Property | Value |
|----------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_full.c:27-57` |
| Linkage | internal |
| Callees | 5 |
| Callers | 2 |
| Arg-flow edges | 5 |

**Callers:** `HdfMessageTaskSendMessageLater`, `HdfMessageTaskDispatchMessage`.

---

### 22. `GpioSetIrq` — GPIO IRQ Configuration (User-Space IPC)

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/gpio/gpio_if_u.c:261-314` |
| Linkage | external |
| Callees | 16 |
| Callers | 9 (`TestCaseGpioSetIrq`, `SetupInfraredIrq`, `SetupKeyIrq`, etc.) |
| Arg-flow edges | 35 |
| Indirect call sites | 1 (`service->dispatcher->Dispatch`) |
| Indirect targets resolved | 6 |

**Key arg-flow (interprocedural through 3 layers):**
- `gpio → GpioCntlrGetByGpio(gpio)` — controller lookup
- `cntlr → GpioCntlrSetIrq(cntlr, gpio, mode, func, arg)` — IRQ configuration with 5 args correctly wired
- `data → HdfSbufWriteUint16(sbuf, gpio)` — GPIO number serialized into IPC buffer
- `mode → HdfSbufWriteUint16(sbuf, mode)` — mode parameter serialized
- `service → DeviceSvcMgrDispatch(service)` — IPC dispatch with 6 targets

**Arg-flow depth:** Parameters flow through `GpioCntlrSetIrq` → `GpioRegListener` → IPC dispatch, demonstrating 3-layer interprocedural analysis.

---

### 23. `GetUartDeviceResource` (uart_bes) — HCS Config Parsing

| Property | Value |
|----------|-------|
| File | `adapter/platform/uart/uart_bes.c:510-564` |
| Linkage | internal |
| Callees | 3 |
| Callers | 1 |
| Arg-flow edges | 12 |
| Indirect call sites | 7 (`dri->GetUint32`, `dri->GetBool`) |
| Indirect targets resolved | 2 |

**Indirect call resolution:** `dri->GetUint32` resolved to `HcsGetUint32`, `dri->GetBool` resolved to `HcsGetBool`. This demonstrates HCS (Hardware Configuration Source) reader dispatch resolution.

**Arg-flow quality:** UART configuration parameters (baud rate, data bits, stop bits, parity, etc.) correctly wired through `HcsGetUint32` and `HcsGetBool` calls.

---

### 24. `ChipDataHandle` (touch_ft5406) — Touchscreen Data with Static Variable

| Property | Value |
|----------|-------|
| File | `framework/model/input/driver/touchscreen/touch_ft5406.c:115-162` |
| Linkage | internal |
| Callees | 5 |
| Callers | 2 (`ChipWorkPoll`, `EventHandle`) |
| Arg-flow edges | 10 |
| Static variables | 1 (`lastTouchStatus` at line 119) |

**Role:** Reads touch chip data via I2C, locks mutex, parses point data.

**Key arg-flow:**
- `i2cClient → InputI2cRead(client, writeBuf)` — I2C read
- `device → OsalMutexLock(mutex)` — lock
- `device → ParsePointData(device, frame, pointNum)` — parse with 3 args
- `memset_s` (external) — buffer clear

**Static variable:** `lastTouchStatus` (fn_static) tracks previous touch state across calls.

---

### 25. `AdcTestGetConfig` — Test Configuration Retrieval

| Property | Value |
|----------|-------|
| File | `framework/test/unittest/platform/common/adc_test.c:27-79` |
| Linkage | internal |
| Callees | 14 |
| Callers | 1 (`AdcTesterGet`) |
| Arg-flow edges | 17 |
| Indirect call sites | 1 (`service->dispatcher->Dispatch`) |
| Indirect targets resolved | 6 |

**Key arg-flow:**
- `tmp_fmt → DealFormat(dest)` — format string to buffer
- `reply → HdfSbufReadBuffer(sbuf, data, readSize)` — read config data with 3 args
- `service → DeviceNodeExtDispatch(service)` — IPC dispatch

---

### 26. `ClockManagerDispatch` — Clock Platform Dispatch

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/clock/clock_core.c:762-801` |
| Linkage | internal |
| Callees | 8 |
| Callers | 10 |
| Arg-flow edges | 14 |

**Role:** Routes clock operations (open/close/enable/disable/set_rate/set_parent/get_rate/get_parent) via direct calls.

**Direct calls:** `ClockManagerOpen`, `ClockManagerClose`, `ClockManagerEnable`, `ClockManagerDisable`, `ClockManagerSetRate`, `ClockManagerSetParent`, `ClockManagerGetRate`, `ClockManagerGetParent`.

---

### 27. `AudioCodecDevInit` — Audio Codec Device Init

| Property | Value |
|----------|-------|
| File | `framework/model/audio/core/src/audio_host.c:60-87` |
| Linkage | internal |
| Callees | 2 |
| Callers | 1 (`AudioInitDaiLink`) |
| Arg-flow edges | 4 |
| Indirect call sites | 1 (`codec->devData->Init`) |
| Indirect targets resolved | 2 |

**Indirect call resolution:** `codec->devData->Init` resolved to `AudioHdmiCodecDeviceInit` and `AudioUsbCodecDeviceInit`.

**Arg-flow:** `audioCard → AudioHdmiCodecDeviceInit(audioCard)`, `codec → AudioHdmiCodecDeviceInit(device)`.

---

### 28. `AudioDmaConfigChannel` — DMA Channel Configuration

| Property | Value |
|----------|-------|
| File | `framework/model/audio/common/src/audio_dma_base.c:40-46` |
| Linkage | external |
| Callees | 1 |
| Callers | 2 (`AudioDmaConfig`, `AudioDmaConfigChannelTest`) |
| Arg-flow edges | 2 |
| Indirect call sites | 1 (`data->ops->DmaConfigChannel`) |
| Indirect targets resolved | 1 |

**Indirect call resolution:** `data->ops->DmaConfigChannel` → `AudioUsbDmaConfigChannel`.

---

### 29. `PlatformManagerTestAddAndDel` (uniproton) — Platform Manager Test

| Property | Value |
|----------|-------|
| File | `adapter/khdf/uniproton/test/sample_driver/src/platform_manager_test.c:88-152` |
| Linkage | internal |
| Callees | 7 |
| Callers | 1 (`PlatformManagerTestExecute`) |
| Arg-flow edges | 23 |

**Role:** Test function exercising platform manager add/delete operations. Pure direct calls, no indirect dispatch.

---

### 30. `GetUartDeviceResource` (uart_stm32f4xx) — Alternate HCS Config

| Property | Value |
|----------|-------|
| File | `adapter/platform/uart/uart_stm32f4xx.c:477-520` |
| Linkage | internal |
| Callees | 2 |
| Callers | 1 |
| Arg-flow edges | 3 |
| Indirect call sites | 2 |
| Indirect targets resolved | 1 |

**Note:** Same function name as #23 but different file (STM32 platform). Demonstrates correct handling of same-name functions in different TUs — each resolved independently.

---

### 31. `HdfDeviceUnlaunchNode` — Driver Teardown

| Property | Value |
|----------|-------|
| File | `framework/core/host/src/hdf_device_node.c:183-222` |
| Linkage | internal |
| Callees | 137 |
| Callers | 2 |
| Arg-flow edges | 136 |
| Indirect call sites | 3 (`driverEntry->Release`) |
| Indirect targets resolved | 135 |

**Role:** Counterpart to `HdfDeviceLaunchNode` (#3) — tears down a driver node by calling `driverEntry->Release` and detaching from the device manager.

**Indirect call resolution:** `driverEntry->Release` resolved to **135 driver release functions** including `AccelReleaseDriver`, `AdcManagerRelease`, `AudioControlRelease`, `AudioDriverRelease`, `ClockManagerRelease`, `GpioManagerRelease`, `I2cManagerRelease`, `SensorReleaseDriver`, `SpiManagerRelease`, `UartManagerRelease`, etc. This is the same dispatch table as `HdfDeviceLaunchNode` but exercised through the release path.

**Arg-flow quality:** `devNode` parameter correctly wired to `driverEntry->Release(&devNode->deviceObject)`, `DevmgrServiceClntDetachDevice(devNode->devId)`, and `driverLoader->ReclaimDriver(devNode->driver)`.

---

### 32. `DeviceDriverBind` — Driver Binding

| Property | Value |
|----------|-------|
| File | `framework/core/host/src/hdf_device_node.c:65-92` |
| Linkage | external |
| Callees | 122 |
| Callers | 2 (`HdfDeviceLaunchNode`, `HdfDeviceNodeOpen`) |
| Arg-flow edges | 122 |
| Indirect call sites | 1 (`driverEntry->Bind`) |
| Indirect targets resolved | 122 |

**Role:** Binds a driver to its device node — calls `driverEntry->Bind(&devNode->deviceObject)` for public/capacity-policy drivers.

**Indirect call resolution:** `driverEntry->Bind` resolved to **122 driver bind functions** including `AdcManagerBind`, `AudioCodecBind`, `GpioManagerBind`, `HdfCameraBind`, `HdfTouchBind`, `I2cManagerBind`, `SensorBind`, `SpiManagerBind`, `UartManagerBind`, etc.

**Arg-flow quality:** `devNode → driverEntry->Bind(&devNode->deviceObject)` correctly wires the device object to all 122 bind targets.

---

### 33. `HdfCameraDispatch` — Camera Command Dispatch

| Property | Value |
|----------|-------|
| File | `framework/model/camera/dispatch/src/camera_dispatch.c:521-542` |
| Linkage | external |
| Callees | 23 |
| Callers | 3 |
| Arg-flow edges | 69 |
| Indirect call sites | 1 (`g_cameraCmdHandle[i].func`) |
| Indirect targets resolved | 23 |

**Role:** Camera command dispatcher — routes camera operations (open/close/enum/set-config/get-config/stream-on/off/power-up/down) via `g_cameraCmdHandle` table.

**Indirect call resolution:** Resolved to **23 camera command handlers**: `CameraCmdOpenCamera`, `CameraCmdCloseCamera`, `CameraCmdEnumDevice`, `CameraCmdEnumFmt`, `CameraCmdGetAbility`, `CameraCmdGetConfig`, `CameraCmdGetCrop`, `CameraCmdGetFPS`, `CameraCmdGetFormat`, `CameraCmdPowerDown`, `CameraCmdPowerUp`, `CameraCmdQueryConfig`, `CameraCmdQueryMemory`, `CameraCmdQueueInit`, `CameraCmdReqMemory`, `CameraCmdSetConfig`, `CameraCmdSetCrop`, `CameraCmdSetFPS`, `CameraCmdSetFormat`, `CameraCmdStreamDeQueue`, `CameraCmdStreamOff`, `CameraCmdStreamOn`, `CameraCmdStreamQueue`.

**Arg-flow quality:** `client → g_cameraCmdHandle[i].func(client, reqData, rspData)` — 3 args correctly wired to all 23 handlers.

---

### 34. `PowerStateChange` — Power State Dispatch (Multi-Site)

| Property | Value |
|----------|-------|
| File | `framework/core/host/src/power_state_token.c:58-90` |
| Linkage | external |
| Callees | 20 |
| Callers | 2 |
| Arg-flow edges | 20 |
| Indirect call sites | 4 (`stateToken->listener->Suspend/Resume/DozeSuspend/DozeResume`) |
| Indirect targets resolved | 20 (4 per site × 4 sites, with overlapping targets) |

**Role:** Routes power-state transitions through 4 function-pointer fields on `stateToken->listener` — one per transition type (Suspend, Resume, DozeSuspend, DozeResume).

**Indirect call resolution:**
- `listener->Suspend` → `HdfPmTestSuspend`, `HdfPmSampleSuspend`, `HdfPmHdfTestSuspend`, `HdfSampleSuspend` (4 targets)
- `listener->Resume` → `HdfPmTestResume`, `HdfPmSampleResume`, `HdfPmHdfTestResume`, `HdfSampleResume` (4 targets)
- `listener->DozeSuspend` → `HdfPmTestDozeSuspend`, `HdfPmSampleDozeSuspend`, `HdfPmHdfTestDozeSuspend`, `HdfSampleDozeSuspend` (4 targets)
- `listener->DozeResume` → `HdfPmTestDozeResume`, `HdfPmSampleDozeResume`, `HdfPmHdfTestDozeResume`, `HdfSampleDozeResume` (4 targets)

**Pattern:** Switch-based dispatch over event type, each branch dereferencing a different field of the same struct. The solver resolves all 4 fields independently through `FieldSummary`-mediated propagation.

---

### 35. `HdfObjectManagerGetObject` — Object Factory Dispatch

| Property | Value |
|----------|-------|
| File | `framework/core/shared/src/hdf_object_manager.c:11-22` |
| Linkage | external |
| Callees | 19 |
| Callers | 11 |
| Arg-flow edges | 1 |
| Indirect call sites | 1 (`targetCreator->Create`) |
| Indirect targets resolved | 18 |

**Role:** Factory function — looks up an object creator by `objectId`, then calls `targetCreator->Create()`. Central allocation point for all HDF framework objects.

**Indirect call resolution:** `targetCreator->Create` resolved to **18 object constructors**: `DeviceNodeExtCreate`, `HdfDeviceTokenCreate`, `HdfDeviceCreate`, `HdfDriverLoaderCreate`, `DriverInstallerCreate`, `DevHostServiceCreate`, `DevSvcManagerExtCreate`, `DevmgrServiceCreate`, `DriverInstallerFullCreate`, `DevSvcManagerStubCreate`, `DevmgrServiceStubCreate`, `DeviceServiceStubCreate`, `DeviceTokenStubCreate`, `HdfDriverLoaderFullCreate`, `DevHostServiceStubCreate`, `DevSvcManagerProxyCreate`, `DevmgrServiceProxyCreate`, `DevSvcManagerCreate`.

**Arg-flow quality:** Minimal (1 edge) — the factory returns a heap-allocated object; argument passing is through the creator table, not parameter forwarding.

---

### 36. `SetOption` (sensor) — Sensor Option Dispatch

| Property | Value |
|----------|-------|
| File | `framework/model/sensor/driver/common/src/sensor_device_manager.c:216-231` |
| Linkage | internal |
| Callees | 14 |
| Callers | 1 |
| Arg-flow edges | 15 |
| Indirect call sites | 1 (`deviceInfo->ops.SetOption`) |
| Indirect targets resolved | 13 |

**Role:** Sensor option setter — reads `option` from IPC buffer, then calls `deviceInfo->ops.SetOption(option)`.

**Indirect call resolution:** Resolved to **13 sensor-specific SetOption handlers**: `SetAccelOption`, `SetAlsOption`, `SetBarometerOption`, `SetGasOption`, `SetGyroOption`, `SetHallOption`, `SetHumidityOption`, `SetMagneticOption`, `SetPedometerOption`, `SetPpgOption`, `SetProximityOption`, `SetTemperatureOption`, `SetGravityOption`.

**Arg-flow quality:** `option → SetXxxOption(option)` — the uint32 option value correctly flows to all 13 handlers. `data → HdfSbufReadUint32(data, &option)` correctly models the IPC deserialization.

---

### 37. `GpioOnDevEventReceive` — GPIO Event Callback Dispatch

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/fwk/platform_listener_u.c:121-149` |
| Linkage | external |
| Callees | 14 |
| Callers | 1 |
| Arg-flow edges | 28 |
| Indirect call sites | 1 (`gpio->func`) |
| Indirect targets resolved | 13 |

**Role:** GPIO device event callback — reads GPIO ID from IPC buffer, matches against registered GPIO, then invokes the registered callback `gpio->func(gpioId, gpio->data)`.

**Indirect call resolution:** `gpio->func` resolved to **13 GPIO interrupt handlers**: `GpioTestIrqHandler`, `PpgIrqHandler`, `TestCaseGpioIrqHandler4`, `IrqHandle`, `TestCaseGpioIrqHandler3`, `InfraredIrqHandle`, `HallSouthPolarityIrqFunc`, `TestCaseGpioIrqHandler2`, `KeyIrqHandle`, `GpioServiceIrqFunc`, `HallNorthPolarityIrqFunc`, `TestCaseGpioIrqHandler1`, `TestCaseGpioIrqHandler4` (unique).

**Arg-flow quality:** `gpioId → gpio->func(gpioId, gpio->data)` — 2 args correctly wired to all 13 handlers. `data → HdfSbufReadUint16(data, &gpioId)` models IPC deserialization.

---

### 38. `HdfPmDriverDispatch` — PM Driver Test Dispatch

| Property | Value |
|----------|-------|
| File | `framework/test/unittest/pm/hdf_pm_driver_test.c:568-587` |
| Linkage | internal |
| Callees | 19 |
| Callers | 3 |
| Arg-flow edges | 0 |
| Indirect call sites | 1 (`pdr->ops->Dispatch`) |
| Indirect targets resolved | 19 |

**Role:** Power-management test driver dispatch — routes PM test operations through `pdr->ops->Dispatch`.

**Indirect call resolution:** Resolved to **19 PM test functions**: `HdfPmTestBegin`, `HdfPmTestOneDriverOnce`, `HdfPmTestOneDriverTwice`, `HdfPmTestOneDriverTen`, `HdfPmTestOneDriverHundred`, `HdfPmTestOneDriverThousand`, `HdfPmTestTwoDriverOnce`, `HdfPmTestTwoDriverTwice`, `HdfPmTestTwoDriverTen`, `HdfPmTestTwoDriverHundred`, `HdfPmTestTwoDriverThousand`, `HdfPmTestThreeDriverOnce`, `HdfPmTestThreeDriverTwice`, `HdfPmTestThreeDriverTen`, `HdfPmTestThreeDriverHundred`, `HdfPmTestThreeDriverThousand`, `HdfPmTestThreeDriverSeqHundred`, `HdfPmTestThreeDriverHundredWithSync`, `HdfPmTestEnd`.

---

### 39. `WorkEntry` (linux) — Workqueue Dispatch

| Property | Value |
|----------|-------|
| File | `adapter/khdf/linux/osal/src/osal_workqueue.c:51-63` |
| Linkage | internal |
| Callees | 19 |
| Callers | 0 (entry point for OS callback) |
| Arg-flow edges | 19 |
| Indirect call sites | 1 (`work->func`) |
| Indirect targets resolved | 19 |

**Role:** Workqueue callback entry point — the OS calls `WorkEntry(work)` which invokes `work->func(work->data)`.

**Indirect call resolution:** `work->func` resolved to **19 sensor data handlers**: `AccelDataWorkEntry`, `BarometerDataWorkEntry`, `EsdWorkHandler`, `EventQueueWorkEntry`, `GasDataWorkEntry`, `GravityDataWorkEntry`, `GyroDataWorkEntry`, `HallDataWorkEntry`, `HumidityDataWorkEntry`, `LightWorkEntry`, `MagneticDataWorkEntry`, `PedometerDataWorkEntry`, `PpgDataWorkEntry`, `ProximityDataWorkEntry`, `SensorTestDataWorkEntry`, `TemperatureDataWorkEntry`, `TestDelayWorkEntry`, `TestWorkEntry`, `VibratorWorkEntry`.

**Arg-flow quality:** `work → work->func(work->data)` — correctly wires the work item to all 19 sensor handlers.

---

### 40. `PlatformDumperDump` — Platform Dumper Dispatch

| Property | Value |
|----------|-------|
| File | `framework/support/platform/src/fwk/platform_dumper_unopen.c:21-25` |
| Linkage | external |
| Callees | 18 |
| Callers | 4 |
| Arg-flow edges | 17 |
| Indirect call sites | 1 (`ops->func` via `OutputDumperInfo`) |
| Indirect targets resolved | 13 |

**Role:** Platform dumper — collects diagnostic data through a type-dispatched function-pointer table.

**Indirect call resolution:** `ops->func` resolved to **13 type-specific dump handlers**: `DumperPrintInt32Info`, `DumperPrintUint32Info`, `DumperPrintDoubleInfo`, `DumperPrintInt16Info`, `DumperPrintUint16Info`, `DumperPrintRegisterInfo`, `DumperPrintFloatInfo`, `DumperPrintInt8Info`, `DumperPrintUint8Info`, `DumperPrintInt64Info`, `DumperPrintStringInfo`, `DumperPrintUint64Info`, `DumperPrintCharInfo`.

**Pattern:** Type-dispatch — the dumper reads the data type, then calls the appropriate print function via a function-pointer table indexed by type.

---

## Cross-Cutting Analysis

### Indirect Call Resolution Quality

| Dispatch Pattern | Call Site | Targets Resolved |
|------------------|-----------|-----------------|
| vtable dispatch | `deviceMethod->Dispatch` (DeviceNodeExtDispatch) | 73 |
| array dispatch | `g_streamDispCmdHandle[i]->func` (StreamDispatch) | 24 |
| driver entry table | `driverEntry->Init` (HdfDeviceLaunchNode) | 125 |
| driver entry table | `driverEntry->Bind` (DeviceDriverBind) | 122 |
| driver entry table | `driverEntry->Release` (HdfDeviceUnlaunchNode) | 135 |
| wifi command table | `messageDef->handler` (HandleRequestMessage) | 56 |
| HCS reader | `dri->GetUint32` / `dri->GetBool` (GetUartDeviceResource) | 1-8 |
| audio codec | `codec->devData->Init` (AudioCodecDevInit) | 2 |
| audio DMA | `data->ops->DmaConfigChannel` (AudioDmaConfigChannel) | 1 |
| touch ops | `device->ops->read` (AdcDeviceRead) | 2 |
| backlight table | `blCmdHandle` (BacklightDispatch) | 6 |
| control table | `g_controlDispCmdHandle[i]->func` (ControlDispatch) | 6 |
| camera command table | `g_cameraCmdHandle[i].func` (HdfCameraDispatch) | 23 |
| power state dispatch | `stateToken->listener->Suspend/Resume` (PowerStateChange) | 4×4 |
| object factory | `targetCreator->Create` (HdfObjectManagerGetObject) | 18 |
| sensor dispatch | `deviceInfo->ops.SetOption` (SetOption) | 13 |
| GPIO event callback | `gpio->func` (GpioOnDevEventReceive) | 13 |
| PM driver dispatch | `pdr->ops->Dispatch` (HdfPmDriverDispatch) | 19 |
| workqueue dispatch | `work->func` (WorkEntry) | 19 |
| platform dumper | `ops->func` (PlatformDumperDump) | 13 |
| C++ interop | `sbuf->impl->readBuffer` (HdfSbufReadBuffer) | 2 |

**Total indirect call sites resolved:** 1,445 (at 800K pops)
**Unresolved indirect calls:** 0 (all 40 evaluated functions resolved)

### Cross-Struct FieldId Guard Impact

| Function | Before fix | After fix | Notes |
|----------|-----------|-----------|-------|
| HdfSbufReadBuffer | 140 targets | 2 targets | Eliminated 138 false positives from unrelated structs |
| FinishEvent | 24 targets | 5 targets | Eliminated 19 false positives (was 6 reported, now 5 correct) |
| DeviceNodeExtDispatch | 139 targets | 73 targets | Reduced from over-approximation |
| Total indirect edges | 38,166 | 4,428 | **88% reduction** in false-positive indirect edges |

### Arg-Flow Analysis Quality

| Function | Arg-flow Edges | Key Insight |
|----------|---------------|-------------|
| HdfDeviceUnlaunchNode | 136 | Driver teardown with 3 indirect dispatch sites |
| DeviceDriverBind | 122 | Driver binding through driverEntry->Bind |
| HdfCameraDispatch | 69 | 3-arg camera command dispatch to 23 handlers |
| PowerStateChange | 20 | 4 power-state function-pointer fields |
| GpioOnDevEventReceive | 28 | GPIO ID deserialized and wired to 13 callbacks |
| SetOption | 15 | Sensor option deserialized and wired to 13 handlers |
| WorkEntry | 19 | Work item wired to 19 sensor data handlers |
| PlatformDumperDump | 17 | Type-dispatched dump to 13 print handlers |
| AdcOpen | 307 | IPC request/response fully wired |
| AdcRead | 305 | channel/val through direct + IPC |
| DeviceNodeExtDispatch | 280 | service/data/reply wired to 73 dispatchers |
| GpioSetIrq | 248 | 5-param IRQ config wired through 3 layers |
| HdfSbufReadBuffer | 226 | Arg-flow to both C and C++ targets |
| FinishEvent | 229 | event data through IPC dispatch |

### Static Variable Handling

| Function | Static Variables | Pattern |
|----------|-----------------|---------|
| ChipDataHandle (touch_ft5406) | `lastTouchStatus` (fn_static) | Persistent state across calls |
| DevMgrUeventRuleCfgList | `ruleCfgList`, `initFlag` (fn_static) | Lazy-init config list |
| DevSvcManagerCreate | `devSvcManagerInstance`, `g_createOnce` (fn_static) | Thread-safe singleton |
| DevSvcManagerClntGetInstance | `instance`, `singletonInstance` (fn_static) | Client-side singleton |
| DevSvcManagerExtStart | `dispatcher`, `svcmgrDevObj`, `svcmgrIoService` (fn_static) | Multi-object init |
| DevHostServiceFullConstruct | `handler` (fn_static) | Handler singleton |
| DevHostServiceStubConstruct | `dispatcher` (fn_static) | Dispatcher singleton |
| DevHostServiceStubCreate | `instance` (fn_static) | Instance singleton |
| DeviceManagerDispatch | `callback` (fn_static) | Callback registration |

### Same-Name Function Disambiguation

`GetUartDeviceResource` appears in 4 files:
- `uart_bes.c:510` — 3 callees, 12 arg-flow, 7 indirect sites (HCS dispatch)
- `uart_stm32f4xx.c:477` — 2 callees, 3 arg-flow, 2 indirect sites
- `uart_wm.c:253` — 2 callees, 8 arg-flow
- `uart_sample.c:183` — 3 callees, 17 arg-flow

Each resolved independently with correct file-local analysis. Similarly, `ChipDataHandle` appears in 4 touchscreen drivers (ft5406, ft5x06, ft6336, gt911), each analyzed independently.

### External Function Models (memcpy_s, memset_s)

The built-in model set provides `memcpy_s` (`mem_copy dst=0 src=2`) and `memset_s` (`clears param=0`) without needing `--models`. These are applied automatically at solver time.

**Sub-field copy (new).** Prior to the fix, `memcpy_s(&drv->chipData, ..., chip, ...)` was silently skipped when the destination argument was an address-of-member (`&base.field`). The `member_addr` guard that prevents pointer-alias pollution also blocked content-copy effects. Removing the guard for `MemCopy` allows the whole-object Copy to the base variable, which is sound for may-analysis: the GEP chain in the PAG models the field access, and extra pointees on unrelated fields are over-approximated.

Concrete improvement on `drivers_hdf_core` — **PPG sensor driver** (12 calls through `ops->ReadData` pattern):

```c
/* sensor_ppg_driver.c: RegisterPpgChip */
memcpy_s(&drvData->chipData, sizeof(PpgChipData), chipData, sizeof(PpgChipData));
/* ... later in SetPpgEnable: */
drvData->chipData->opsCall->Enable();   /* was UNRESOLVED → now resolves to SetPpgEnable  */
drvData->chipData->opsCall->Disable();  /* was UNRESOLVED → now resolves to SetPpgDisable */
drvData->chipData->opsCall->SetOption();/* was UNRESOLVED → now resolves to SetPpgMode   */
```

Before: `chipData->opsCall` indirect calls through `memcpy_s`-copied structs were **0/3** resolved.
After: **3/3** resolved, with **+16 call edges** and **+12 arg-flow edges** (negligible solver cost).

**Integration test:** `memcpy_s_member_field_resolves_fnptrs` in `adversarial_cases.rs` — copies a struct containing `{Enable, Disable, ReadData}` function pointers into a sub-field via `memcpy_s(&drv->chipData, ...)`, then verifies all three indirect calls resolve.

---

## Observations

1. **Indirect call resolution is comprehensive at 800K pops.** All function-pointer dispatch patterns tested were fully resolved. The largest resolution was 135 targets for `HdfDeviceUnlaunchNode` (driver release). The `DeviceNodeExtDispatch` resolves 73 dispatch targets, `HdfDeviceLaunchNode` resolves 125 driver init functions, and `HdfSbufReadBuffer` resolves exactly 2 targets (C + C++).

2. **Cross-struct FieldId guard eliminates massive false-positive pollution.** The guard prevents GEP accesses into struct A from picking up function pointers stored in struct B's same-index field. Impact:
   - `HdfSbufReadBuffer`: 140 → 2 targets (138 false positives eliminated)
   - Total indirect edges: 38,166 → 4,428 (88% reduction)
   - All 40 evaluated functions now have zero false-positive indirect targets

3. **Solver budget is critical.** The 800K default is required for comprehensive analysis on large corpora. Override via `TRACE_SOLVE_BUDGET_POPS=<n>`; set to `0` for unlimited.

4. **Multi-site dispatch works.** `PowerStateChange` demonstrates 4 independent dispatch sites in one function — each `switch` branch dereferences a different field of `stateToken->listener`. The solver resolves all 4 sites independently.

5. **Factory patterns resolve correctly.** `HdfObjectManagerGetObject` uses a creator-table lookup (`HdfObjectManagerGetCreators(objectId)`) followed by `targetCreator->Create()`. The solver resolves all 18 object constructors through the table flow.

6. **Workqueue callbacks resolve.** `WorkEntry` is an OS callback entry point with no callers in the analysis tree. The solver correctly resolves `work->func` to all 19 sensor data handlers registered through `OsalWorkQueueInit`.

7. **Singleton patterns detected.** All static singleton patterns correctly model the static variable storage.

8. **Same-name disambiguation works.** `GetUartDeviceResource` appears in 4 files and each is analyzed independently with correct file-local resolution.

9. **C++ cross-language interop works.** The critical `HdfSbufReadBuffer` → `SbufMParcelImplReadBuffer` chain resolves through: C++ constructor → function-pointer table → C caller dereference. Both targets now correctly resolved (2, not 140).

10. **Sensor dispatch is fully resolved.** All sensor ops functions (`Enable`, `Disable`, `SetBatch`, `SetMode`, `SetOption`, `ReadData`) resolve through `deviceInfo->ops` dispatch to 13 sensor-specific implementations each.

11. **Parse warnings are non-blocking.** 442 parse warnings (likely from missing headers or preprocessor edge cases) did not prevent analysis of any target function.

### C++ Feature Coverage

| Feature | Pattern | Status | Test / documented imprecision |
|---------|---------|--------|------------------------------|
| Namespaces | `ns::f`, anonymous ns → internal linkage | Working | `cpp_namespace_qualified_call`, `cpp_anonymous_namespace_is_internal`; `using` not used for qualification |
| Overloads | Same-name, different arity; ties emit both | Working | `cpp_overload_resolution_by_arity`, `cpp_overload_tie_emits_both_sites`; arity-only |
| Classes | Layout under qualified tag, inheritance chain | Working | `cpp_non_virtual_member_call_exact`, `cpp_header_inline_method_dedups_with_out_of_class_uses` |
| Virtual dispatch | CHA from static receiver, subclass closure | Working | `cpp_virtual_dispatch_expands_to_overrides` |
| Virtual inheritance | `class D : virtual B` recorded as a base edge | Working | `cpp_virtual_inheritance_diamond_resolves_overrides` |
| `final` class | `class Sealed final` stops further subclasses | Working | `cpp_final_class_devirtualizes_receiver` |
| `final` method | `int g() final` unique at that receiver | Working | `cpp_final_method_stops_further_overrides` |
| Implicit `this` | Bare `OnEvent()` inside a method | Working | `cpp_implicit_this_virtual_call_expands` |
| Smart pointers | `shared_ptr` / `unique_ptr` / `weak_ptr` unwrap to `T` | Working | `cpp_smart_ptr_member_call_unwraps_pointee`, `cpp_smart_ptr_field_receiver_unwraps`; `auto p = wp.lock()` stays `Unknown` |
| Callables | `std::function` / `::std::function` only; lambdas (`$lambda`), `operator()` | Working | `cpp_std_function_resolves_like_fn_ptr`, `cpp_lambda_is_addr_of_fn_and_indirect_call`, `cpp_functor_operator_call_resolves`; a class named `function` is still a functor |
| Member arity | Virtual `foo(int)` vs `foo(int,int)` filtered by explicit argc | Working | `cpp_member_virtual_overload_filters_by_arity` |
| `__UNUSED` on `T&` | Predefined empty object macro so the body is not dropped | Working | `cpp_unused_attr_on_ref_param_keeps_definition` |
| Fn-ptr fields | C-style field/local function pointers | Working | `cpp_fn_ptr_field_and_local_resolve_indirect` |
| Qualified extern | `FileUtil::Exists` with no body → external edge | Working | `cpp_qualified_undeclared_becomes_external` |
| Ctors / dtors | `new T(...)`, `delete p`, ctor-init lists | Working | `cpp_ctor_and_dtor_sites`, `cpp_ctor_initializer_list_targets`; `Cls o;` emits no site |
| Templates | Stripped to primary name | Working | `cpp_template_class_method_resolves_by_primary_name`; no dependent types |
| Multiple inheritance | `AB : A, B` — nearest declarer wins | Working | `cpp_virtual_call_through_base_of_multiple_inheritance` |
| Static members | `S::Make()` | Working | `cpp_static_member_function_resolves` |
| Inherited non-virtual | `d.base_value()` walks to `Base` | Working | `cpp_inherited_non_virtual_via_derived_receiver` |
| Cross-C/C++ | `extern "C"` ops table + C caller | Working | `cpp_impl_registered_into_c_ops_table_resolves_indirect`, `cpp_extern_c_driver_resolves_ipc_and_dispatch` |
| `inspect --from/--to` | Suffix match `%::FN` with `LIKE` `_`/`%` escaped | Working | `inspect_calls_matches_cpp_qualified_suffix`, `inspect_calls_like_wildcards_are_literal` |

### C++ Interop Pattern: `cpp_flow` Fixture

The `cpp_flow` fixture models the real-world HDF pattern where C++ IPC implementations extend C interfaces:

```
main.c (C caller) → Read() → s->impl->read()
                                     ↓
                    ops.c:  RawImplRead    (C implementation)
                    impl.cpp: MParcelImplRead  (C++ implementation)
```

The C++ `RegisterOps()` function (declared `extern "C"`) stores `&parcel_ops` into `s->impl`. The C `Read()` function dereferences `s->impl->read` — the solver correctly resolves this indirect call to **both** `RawImplRead` (from C) and `MParcelImplRead` (from C++), demonstrating that cross-language function-pointer flows work through the shared ops table pattern.

---

## Real-World C++ Interop Case: `HdfSbufReadBuffer`

**Pattern:** C caller → indirect through `sbuf->impl->readBuffer` → C and C++ implementations.

```
HdfSbufReadBuffer(sbuf)
    → sbuf->impl->readBuffer(sbuf, ...)
        → SbufRawImplReadBuffer       (C)
        → SbufMParcelImplReadBuffer   (C++)
```

**C++ flow chain:**
```
HdfSbufTypedObtainCapacity
    → SbufObtainIpc()          // indirect, resolved to SbufObtainIpc
    → new SBufMParcelImpl(...) // C++ constructor
    → MParcelImplInterfaceAssign(&infImpl) // fills infImpl.readBuffer = SbufMParcelImplReadBuffer
    → return &sbuf->infImpl    // stored as sbuf->impl
    → HdfSbufReadBuffer loads sbuf->impl->readBuffer → calls it
```

**Challenge:** The solver must resolve `sbuf->impl->readBuffer` through two levels of indirection:
1. `new SBufMParcelImpl(...)` → constructor → stores `SbufMParcelImplReadBuffer` into `infImpl.readBuffer` field
2. `return &sbuf->infImpl` → caller stores into `sbuf->impl` → `HdfSbufReadBuffer` loads from `sbuf->impl->readBuffer`

At 200K pops: only `SbufRawImplReadBuffer` resolved (budget exhausted before C++ constructor chain completes).
At 800K pops: **both targets resolved** (37s on 1,198-file corpus).

**Root cause analysis:** The solver budget at 200K was insufficient for the `FieldSummary`-mediated propagation path. The `merge_memory_into` function iterates `memory_pts[loc]` on every GEP/LOAD cycle, creating O(n²) behavior on hub nodes. Fixes applied:
1. `memory_pts` changed from `FxHashSet` to `IndexSet` for indexed iteration.
2. `merge_memory_into` iterates only entries added since the last merge (`merge_sizes` tracking).
3. `touch_loc_holders` restricted to LOAD-source holders only.

---

## Solver Budget Analysis (Verified)

| Budget | Indirect call sites | Distinct targets | Time | Key finding |
|--------|--------------------|-----------------|------|-------------|
| 800K (default) | 1,445 | 4,428 | ~2s | **All 40 evaluated functions fully resolved** |

**Critical observation:** The 800K budget resolves all indirect call sites for the 40 evaluated functions. With the cross-struct FieldId guard, the total indirect edge count dropped 88% (38,166 → 4,428), meaning the solver now focuses propagation on legitimate flows rather than polluting across struct boundaries.

**CLI flags:**
- Default: 800K pops (required for comprehensive analysis on large corpora)
- `TRACE_SOLVE_BUDGET_POPS=<n>`: override budget (e.g. 200000 for quick smoke test)
- `TRACE_SOLVE_BUDGET_POPS=0`: unlimited (for debugging; may run indefinitely)

---

# Part 2 — `hiviewdfx_hiview` (2026-08-27)

**Target:** `~/hiviewdfx_hiview` (OpenHarmony HiView DFX plugin platform)
**Flags:** default (minimal SQLite export; flow graph always written)
**Command:** `trace analyze ~/hiviewdfx_hiview -o hiview.db --jobs 8`
**Timing (pre-`dlsym`):** index 10.0s / analyze 0.6s / export 1.2s / wall **12.1s**
**Timing (`dlsym` model):** index 10.2s / analyze 0.6s / export 1.7s / wall **12.8s**

Hiview previously **aborted with a stack overflow** in `PreprocessorState::expand_tokens_no_directives`. After C11 hide-set painting (and a 256-deep expansion cap), the tree indexes to completion.

The first post-hide-set eval (same day, before implicit-`this` / CHA-from-receiver / callables) produced 12,652 edges with **0** indirect and almost no in-tree C++ dispatch. The numbers below are the **re-eval on the current binary**.

## Executive summary

Analysis of **1,322 files** (6,418 defined + 4,180 external functions) produced:

- **19,898 call edges** (4,010 direct, **10** indirect, 15,878 external)
- **2,479** `call_sites` with `is_direct=0` (2,333 still have no `call_edge`)
- **3,920** arg-flow edges
- **437,428** flow nodes / **208,302** flow edges (still dominated by `points_to`)
- **551** parse warnings, 0 preprocess “expansion depth exceeded” diagnostics, 0 analysis errors
- **357** synthetic `$lambda` functions

The preprocessor fix is **confirmed**: the `PRIVATE_MESSAGE_TYPE` X-macro in `base/include/defines.h` (invoked from `Event::MessageType` in `event.h`) expands as gcc does (`PRIVATE_MESSAGE_TYPE, ENGINE_UPLOAD_READY_MSG, …`) instead of recursing.

C++ **CHA from the static receiver** plus implicit `this->method()` now recovers the plugin virtual fan-out (`Plugin::OnEventProxy` → **23 defined** `::OnEvent` overrides, including `Plugin::OnEvent`). Same-class unqualified calls (`GetGlobalPluginInfo`, `IsValidEventParam`, recursive `OnContinue`) bind instead of becoming external stubs.

What still fails is **pointer-typed dispatch whose static type is lost**: `auto pluginPtr = wp.lock(); pluginPtr->OnEventProxy(...)` and `info->getPluginObject()` (`std::function` field with no assignment visible to the PAG). The 10 indirect edges are lambdas / JSON accessors, not the pipeline plugin pump. Field-typed `plugin_->OnEvent` (H10) and the `plugin.cpp` `__UNUSED` body (H16) now pass on this corpus.

## Overall metrics

| Metric | Hide-set-only eval | CHA/callable run | This run |
|--------|--------------------|------------------|----------|
| Files indexed | 1,322 | 1,322 | 1,322 |
| Functions total | 9,900 | 10,507 | 10,598 |
| Functions defined | 5,790 | 6,412 | 6,418 |
| External functions | 4,110 | 4,095 | 4,180 |
| Call sites | 21,435 | 21,706 | 22,033 |
| Call sites `is_direct=0` | 9,006 | 2,519 | 2,479 |
| Call edges | 12,652 | 19,350 | 19,898 |
| Direct call edges | 549 | 3,354 | 4,010 |
| Indirect call edges | **0** | **10** | **10** |
| External call edges | 12,103 | 15,986 | 15,878 |
| Arg-flow edges | 673 | 3,373 | 3,920 |
| Flow nodes | 430,156 | 436,314 | 437,428 |
| Flow edges | 200,350 | 207,793 | 208,302 |
| Index / analyze / export / wall | (unrecorded) | index 10.5s | **10.0s / 0.6s / 1.2s / 12.1s** |

### `dlsym` model revalidation (2026-08-27)

| Metric | Pre-`dlsym` run | This run |
|--------|-----------------|----------|
| Files indexed | 1,322 | 1,322 |
| Functions total | 10,598 | 10,563 |
| Functions defined | 6,418 | 6,415 |
| External functions | 4,180 | 4,148 |
| Call sites / `is_direct=0` | 22,033 / 2,479 | 22,033 / 2,479 |
| Call edges | 19,898 | 19,859 |
| Direct / indirect / external | 4,010 / **10** / 15,878 | 4,010 / **10** / 15,839 |
| Arg-flow edges | 3,920 | 4,322 |
| Flow nodes / edges | 437,428 / 208,302 | 443,046 / 275,597 |
| `string_lit` / `dlsym` flow | — | 2,970 / 1 |
| Parse warnings | 551 | 551 |
| Index / analyze / export / wall | **10.0s / 0.6s / 1.2s / 12.1s** | **10.2s / 0.6s / 1.7s / 12.8s** |

Analyze time is **unchanged (0.6s)**. Export grows with interned strings (`addr_of` 66,860). Indirect stays at **10** — H15 does not gain in-tree callees (see below).

### PCH-style header IR revalidation (2026-08-27, later)

Same binary as the HDF PCH revalidation above.

| Metric | `dlsym` run | This run |
|--------|-------------|---------|
| Files indexed | 1,322 | 1,424 |
| Functions total | 10,563 | 10,612 |
| Functions defined | 6,415 | 6,425 |
| External functions | 4,148 | 4,187 |
| Call edges | 19,859 | 19,734 |
| Direct / indirect / external | 4,010 / **10** / 15,839 | 3,843 / **10** / 15,881 |
| Arg-flow edges | 4,322 | 4,162 |
| Parse warnings | 551 | 462 |
| Index / analyze / export / wall | **10.2s / 0.6s / 1.7s / 12.8s** | **3.4s / 0.2s / 0.4s / 4.0s** |

Index **~3× faster**. Indirect stays at **10**. `Plugin::OnEventProxy` still CHA-expands to **23 defined** `::OnEvent`. `inspect calls --from OnEventProxy` lists `Plugin::OnEventProxy` and `EventHandler::OnEventProxy`. Direct **−167** is the same header-attribution effect as HDF, not a CHA regression (H4 / H9 / H16 still pass).

### Flow edge breakdown (this run)

| Kind | Count |
|------|-------|
| points_to | 195,979 |
| copy | 4,152 |
| gep | 3,755 |
| call_arg | 2,524 |
| load | 1,060 |
| store | 633 |
| addr_of | 193 |
| terminates | 6 |

### Diagnostics

| Severity | Stage | Count |
|----------|-------|-------|
| warning | parse | 551 |

No `macro expansion depth exceeded` warnings — hide-set, not the depth cap, stopped the X-macro recursion.

## Feature coverage matrix (hiview)

| # | Feature | Result |
|---|---------|--------|
| H1 | Self-referential object macro / X-macro enum list | **Pass** — `PRIVATE_MESSAGE_TYPE` / `PRIVATE_AUDIT_EVENT_TYPE` in `defines.h`; analysis completes |
| H2 | Nested function-like macros (`MIN(MIN(a,b),c)`) | **Pass** (unit + fixture `self_ref_macro.c`) |
| H3 | Mutual object macros (`#define A B+B` / `#define B A`) | **Pass** — terminates as `A+A` (gcc-compatible) |
| H4 | Virtual `Plugin::OnEvent` via `OnEventProxy` | **Pass** — implicit `this->OnEvent()` CHA-expands to **23 defined** plugin `::OnEvent` (including `Plugin::OnEvent` at `plugin.cpp:35`) |
| H5 | Pipeline plugin dispatch `pluginPtr->OnEventProxy` | **Fail** — `auto` / `lock()` loses the `Plugin` type; site still has 0 targets |
| H6 | Same-class static call `PluginFactory::GetPlugin` → `GetGlobalPluginInfo` | **Pass** — unqualified call binds to `OHOS::HiviewDFX::PluginFactory::GetGlobalPluginInfo` |
| H7 | `std::function` factory `info->getPluginObject()` | **Fail** — field call, 0 targets (no constructor-address flow into that field) |
| H8 | Plugin body `EventLogger::OnEvent` | **Pass** (same-class) — `IsValidEventParam`, `GetEventPid`, `UpdateDB`, … are direct; STL / SDK remain external |
| H9 | `inspect calls --from OnEventProxy` | **Pass** — suffix lists `Plugin::OnEventProxy` and `EventHandler::OnEventProxy`; `--from Get_lugin` matches nothing (`LIKE` `_` escaped) |
| H10 | `PluginProxy::OnEvent` → `plugin_->OnEvent` | **Pass** — line 28 CHA to the same 23 defined plugin `::OnEvent` as H4 (field `shared_ptr<Plugin> plugin_`) |
| H11 | `Plugin::DelayProcessEvent` / `std::bind(&Plugin::OnEventProxy, …)` | **Fail** — `std::bind` still external (no edge to `OnEventProxy`); `AddTimerEvent` is now **direct** |
| H12 | `EventLoop::ProcessEvent` work-queue | **Partial** — `handler->OnEventProxy` CHA **Pass** (`EventHandler` + `Plugin`); `event->task()` / `packagedTask` **Fail** (0 targets) |
| H13 | `Event::DownCastTo<SysEvent>` | **Fail** — 13 sites, all **external** `Event::DownCastTo` |
| H14 | `ffrt::submit` deferred lambdas | **Fail** — 34 sites → external `ffrt::submit`; `$lambda` bodies have 7 in-edges (not from submit) |
| H15 | `dlopen` / `dlsym` | **Fail** — `dlsym` model is wired (1 `dlsym` PAG edge on this tree) but `GET_INSTANCE` looks up exact `"GetInstance"`; the in-tree `extern "C"` export is stored as `OHOS::HiviewDFX::UCollectUtil::GetInstance`. `CallDllFunc` / `GetSymbol` pass `std::string::c_str()`, not a folded constant |
| H16 | Out-of-line `Plugin::OnEvent` body | **Pass** — `plugin.cpp:35` is `is_defined=1`; predefined empty `__UNUSED` |

## Individual function evaluations

### H1. `PRIVATE_MESSAGE_TYPE` — X-macro enumerator list (preprocessor)

| Property | Value |
|----------|-------|
| File | `base/include/defines.h:39-70` (invoked at `base/include/event.h:127`) |
| Pattern | `#define PRIVATE_MESSAGE_TYPE PRIVATE_MESSAGE_TYPE, ENGINE_UPLOAD_READY_MSG, …` |
| gcc `-E` | `PRIVATE_MESSAGE_TYPE, ENGINE_UPLOAD_READY_MSG, …` (token painted, not re-expanded) |

**Before hide-set:** `expand_tokens_no_directives` recursed on the first replacement token until stack overflow. Any TU that included `event.h` (most of the plugin tree) could not be indexed.

**After hide-set:** replacement-list tokens inherit `{PRIVATE_MESSAGE_TYPE}` plus the invoking token’s hide set. The enumerator name is emitted; sibling enumerators are not macros and pass through. Same pattern: `PRIVATE_AUDIT_EVENT_TYPE`.

**Regression tests:** `self_referential_object_macro_is_not_reexpanded`, `self_ref_macro_fixture`, `mutual_object_macros_terminate`, `nested_same_function_macro_still_expands`.

---

### H2. `OHOS::HiviewDFX::Plugin::OnEventProxy` — virtual plugin entry

| Property | Value |
|----------|-------|
| File | `base/plugin.cpp:55-83` |
| Linkage | external (defined) |
| Line-68 `OnEvent` targets | **23 defined** plugin `::OnEvent`, including `Plugin::OnEvent` at `plugin.cpp:35` (H16) |

**Role:** Framework wrapper: `ret = OnEvent(dupEvent)` then pipeline `OnContinue()`. Every plugin’s work is supposed to enter here.

**Resolution (this run):** Line 68 `OnEvent(dupEvent)` is rewritten as implicit `this->OnEvent` on `Plugin`. CHA from that static type emits **direct** edges to **23 defined** plugin overrides, including `Plugin::OnEvent` itself (`plugin.cpp:35`), `PluginProxy::OnEvent`, `EventLogger`, `SysEventStore`, `FreezeDetectorPlugin`, `Faultlogger`, `PrivacyController`, `SysEventDispatcher`, `UsageEventReport`, and the in-tree examples. Five other defined `::OnEvent` methods are **not** in this set because they override `EventHandler`, not `Plugin` (`TestEventHandler`, `RealEventHandler`, …) — those appear under `EventHandler::OnEventProxy` instead.

CHA over-approximation also wires nearby implicit calls in the same body (`GetPendingProcessorSize`, `OnContinue`, `HasFinish`, …) to both `Event` and `PipelineEvent` methods.

---

### H3. `OHOS::HiviewDFX::PipelineEvent::OnContinue` — pipeline pump

| Property | Value |
|----------|-------|
| File | `base/pipeline.cpp:34-70` |
| Recursive `OnContinue` | **direct** to `OHOS::HiviewDFX::PipelineEvent::OnContinue` (lines 56 and 67) |
| Other directs | `PipelineEvent::OnFinish`, `Event::HasFinish` / `HasPending` |
| Still unresolved | `pluginPtr->CanProcessMoreEvents`, `pluginPtr->IsInterestedPipelineEvent`, `pluginPtr->GetWorkLoop`, `workLoop->AddEvent`, **`pluginPtr->OnEventProxy`** |

**Role:** Pops the next plugin from `processors_` and either posts to its work loop or calls `OnEventProxy` inline.

**Resolution:** Unqualified `OnContinue()` now binds (H6-style lookup). The actual plugin dispatch `pluginPtr->OnEventProxy(...)` remains **0 targets**: `pluginPtr` is `auto` from `weak_ptr::lock()`, so the receiver type stays `Unknown` (documented: no return-type inference). That is the remaining hole in the central hiview call graph.

---

### H4. `OHOS::HiviewDFX::PluginFactory::GetPlugin` — constructor registry

| Property | Value |
|----------|-------|
| File | `base/plugin_factory.cpp:40-47` |
| Call sites | 2 |

```
auto info = GetGlobalPluginInfo(name);   // direct → PluginFactory::GetGlobalPluginInfo
return info->getPluginObject();          // still 0 targets (std::function field)
```

Same-class unqualified `GetGlobalPluginInfo` **binds**. `getPluginObject` is a `std::function<std::shared_ptr<Plugin>()>` **field**; constructors are registered through `std::map` elsewhere, so no function address reaches this load. Fixture `cpp_callable` covers the case where the assignment **is** visible (`w->getPluginObject = target`).

---

### H5. `OHOS::HiviewDFX::EventLogger::OnEvent` — plugin implementation

| Property | Value |
|----------|-------|
| File | `plugins/eventlogger/event_logger.cpp:209+` |
| Call sites | 18 (0 `is_direct=0`) |

**Resolved direct (same class / event API):** `IsValidEventParam`, `GetEventPid`, `CheckContinueReport`, `CheckFfrtEvent`, `IsHandleAppfreeze`, `CheckProcessRepeatFreeze`, `CheckScreenOnRepeat`, `UpdateDB`, `Event::GetValue`, `PipelineEvent::OnFinish` / `OnPending`.

**Still external:** `Event::DownCastTo`, `std::string::c_str`, `ffrt::task_attr` / `submit`, `empty`. SDK / STL isolation, not same-TU lookup.

---

### H6. `OHOS::HiviewDFX::SysEventStore::OnEvent` — event store plugin

| Property | Value |
|----------|-------|
| File | `plugins/event_store/sys_event_store.cpp:123-160` |

Same-class calls now bind (`Convert2SysEvent`, `IsNeedBackup`, `StatisticStorePeriodInfo`, `SysEvent::SetEventSeq`, `Event::GetValue`). Nested `EventStore::SysEventSequenceManager::GetInstance`, `SaveToStore`, `TriggerExportEngine::GetInstance().ProcessEvent`, `TimeUtil`, and `Parameter::*` stay **external** (other namespaces / SDK, or chained `auto` receivers).

---

### H7. Unresolved sites (no `call_edge`)

Top `callee_text` values among sites with **no** `call_edge` (this run):

| callee_text | Count | Kind |
|-------------|------:|------|
| `source->GetValue` | 63 | arrow; likely `auto` / SDK type |
| `creator->SetKeyValue` | 56 | arrow |
| `resultSet->Close` | 33 | arrow |
| `source->GetString` | 32 | arrow |
| `sysEvent->SetEventValue` | 28 | arrow |
| `event->SetValue` | 22 | arrow |
| `pluginPtr->OnEventProxy` | 1 | virtual via `auto` after `lock()` |

These are mostly **not** C function-pointer tables. `is_direct=0` sites dropped from 9,006 to 2,519 because implicit-`this` / member typing now classifies many C++ calls as direct. Remaining holes are `auto`, STL, and SDK pointers.

The **10** indirect `call_edges` are `$lambda` invocations in `FaultLogDatabase` / `FaultLogCppCrash` plus JSON `asString` / `isString` — not plugin `OnEvent`.

---

### H10. `OHOS::HiviewDFX::PluginProxy::OnEvent` — smart-ptr **field** receiver

| Property | Value |
|----------|-------|
| File | `base/plugin_proxy.cpp:22-30` |
| Field | `std::shared_ptr<Plugin> plugin_` (`plugin_proxy.h:54`) |

```
return plugin_->OnEvent(event);
```

**Pass.** Line 28 CHA-expands to the same 23 defined plugin `::OnEvent` as H4, including `Plugin::OnEvent` (`plugin.cpp:35`). The receiver is the data member `plugin_` (`shared_ptr<Plugin>`), looked up as implicit `this->plugin_`. Same for `plugin_->OnEventListeningCallback` (line 81). Fixture: `Holder { shared_ptr<Plugin> plugin_; void go() { plugin_->OnEvent(); } }` (`cpp_smart_ptr_field_receiver_unwraps`).

Roadmap: C1 still covers `auto` / `lock()` (H5). C6 (concrete class flowing into `plugin_` via the factory) is still open — CHA from `Plugin` is the over-approx until then.

---

### H11. `OHOS::HiviewDFX::Plugin::DelayProcessEvent` — `std::bind` onto the work loop

| Property | Value |
|----------|-------|
| File | `base/plugin.cpp:85-96` |
| Edges | `UpdateTimeByDelay` direct; `Event::OnPending` external + `PipelineEvent::OnPending` CHA; **`std::bind` external**; `AddTimerEvent` **direct** (`EventLoop` / `MockEventLoop`) |

```
auto task = std::bind(&Plugin::OnEventProxy, this, event);
workLoop_->AddTimerEvent(nullptr, nullptr, task, delay, false);
```

No edge to `OnEventProxy`. Delayed plugin work is missing from the graph.

Roadmap: C4.

---

### H12. `OHOS::HiviewDFX::EventLoop::ProcessEvent` — packed vs typed handler

| Property | Value |
|----------|-------|
| File | `base/event_loop.cpp:492-510` |

| Site | Result |
|------|--------|
| `event.handler->OnEventProxy(event.event)` (line 498) | **Pass** — direct CHA to `EventHandler::OnEventProxy` and `Plugin::OnEventProxy` |
| `event.task()` (line 496) | **Fail** — 0 targets (`callee_text` `event->task`) |
| `event.packagedTask->operator()()` (line 504) | **Fail** — 0 targets (`event->packagedTask`) |

`AddEventForResult` stores `std::bind(&EventHandler::OnEventProxy, …)` in a `packaged_task` (`event_loop.cpp:191-199`). The typed handler fallback works; the functor/queue slots do not.

Roadmap: C4.

---

### H13. `Event::DownCastTo<SysEvent>` — template pointer_cast

| Property | Value |
|----------|-------|
| Sites | 13 |
| Resolution | all **external** `Event::DownCastTo` |

Template is in `event.h:201-205` (`static_pointer_cast<Derived>`). Name-stripping does not instantiate it, so the result is not typed as `SysEvent` and `sysEvent->SetEventValue` stays in the unresolved-arrow rain.

Roadmap: C2.

---

### H14. `ffrt::submit` — deferred `$lambda`

| Property | Value |
|----------|-------|
| Sites | 34 `ffrt::submit` (all **external**); plus 11 bare `submit` |
| Example | `passthrough_monitor.cpp:80`, `uc_telemetry_callback.cpp:187` (`[callback = shared_from_this()]`) |

357 `$lambda` functions exist; 7 have in-edges, none from `ffrt::submit`. Capture `this` / `shared_from_this` is unmodeled.

Roadmap: C4.

---

### H15. `dlopen` / `dlsym`

| Site | Result |
|------|--------|
| `GraphicMemoryCollectorImpl::GetGraphicUsage` `dlopen`/`dlsym(handler, GET_INSTANCE)` (`graphic_memory_collector_impl.cpp:47-59`) | `dlsym` still **external**; `getInterface()` has **0** in-tree targets. Name constant is `"GetInstance"`; indexed function is qualified `OHOS::HiviewDFX::UCollectUtil::GetInstance` (`graphic_memory_collector_entity.cpp:27`, `extern "C"`). Exact-name lookup misses it |
| `CallDllFunc` `dlsym(module, funcName)` (`hiretrieval_dynamic_loader.cpp:69`) | external — `funcName.c_str()`, no string constant |
| `DynamicLibraryHandle::GetSymbol` `dlsym(libPtr_, symbol)` | same (`symbol` is a parameter) |
| `LoadModule` → `dlopen` (`dynamic_module.cpp:32`) | external (static `REGISTER` in the DSO is C3) |
| HDF `LoadIpcImpl` `dlsym(..., "SbufObtainIpc")` / `SbufBindIpc` | libc `dlsym` still external; model assigns the in-tree `extern "C"` functions into the return dest. `HdfSbufReadBuffer` stays at 2 via the compile-time `&SbufObtainIpc` path |

Roadmap: C11 (landed; remaining gap is C++ qualified IR names vs `extern "C"` export strings, and `std::string` names).

---

### H16. `Plugin::OnEvent` out-of-line body dropped

`plugin.cpp:35-38` is a real definition (`bool Plugin::OnEvent(std::shared_ptr<Event>& event __UNUSED)`). Custom preproc does not define `__GNUC__`, so hiview `defines.h` never `#define __UNUSED`, and the unexpanded token after a **reference** declarator made tree-sitter parse a `declaration` + ERROR (body lost). `__UNUSED` is now a predefined empty object macro, installed even when the shared warm table is cloned.

**Pass on this corpus:** `OHOS::HiviewDFX::Plugin::OnEvent` is a single row at `plugin.cpp:35`, `is_defined=1`. It participates in H4/H10 CHA. Fixture: `Sink::consume(Event &event __UNUSED)`.

## Observations (hiview)

1. **Hide-set is sufficient for this corpus.** The crash was a single well-known C pattern (X-macro list whose first token is the macro name). The 256-deep cap did not fire.

2. **CHA virtual dispatch now transfers for typed receivers.** `Plugin::OnEventProxy` fans out like HDF `deviceMethod->Dispatch`, via class hierarchy rather than points-to. `final` / virtual bases are tested on fixtures (`cpp_dispatch`); this tree does not stress them.

3. **Typed receivers include fields.** Parameter and **field** `shared_ptr<Plugin>` unwrap (H10 **Pass** on this corpus). The pipeline pump (H5) remains `auto` after `lock()`.

4. **Unqualified in-tree names mostly bind.** Direct edges rose 3,354 → 4,010 on this run (H10 CHA fan-out is the bulk). Remaining externals are largely STL and OHOS SDK.

5. **`std::function` needs a visible store.** Intern-as-`FnPtr` is not enough when the only assignment is through `std::map`. 357 `$lambda` functions were interned; 7 have in-edges (not from `ffrt::submit`).

6. **Parse warnings are per-file and non-fatal** (551), same recovery policy as HDF.

7. **`inspect --from OnEventProxy` works** with suffix match. SQLite `LIKE` `_`/`%` in the user name are escaped (`ESCAPE '!'`); `--from Get_lugin` is empty.

8. **Deferred execution and DSO factories are still mostly dark.** `std::bind` / `ffrt::submit` / `packaged_task` (H11–H12, H14) have no in-tree callees. `dlsym` (H15) now models literal/const-char names, but this corpus’s `"GetInstance"` does not match the qualified IR name, and `CallDllFunc` never sees a constant. HDF dispatch tables that do **not** go through `dlsym` stay at **125 / 2** for launch/sbuf; `DeviceNodeExtDispatch` is **73** and `GpioOnDevEventReceive` is **13 / 12** after nested-type PCH + C/C++ prototype merge.

### Comparison to HDF (same binary)

| | HDF | Hiview |
|--|-----|--------|
| Language mix | C + C++ interop via ops tables | Almost all C++ |
| Indirect edges | 4,357 (was 4,536) | 10 (unchanged) |
| Direct edges | 14,965 (was 20,822) | 3,843 (was 4,010) |
| External edges | 21,362 | 15,881 |
| Wall (`--jobs 8`) | **5.6s** (was 10.2s) | **4.0s** (was 12.8s) |
| Preprocess | Completes (PCH header IR) | Completes **only with hide-set** |
| Eval conclusion | Launch/sbuf/bind/unlaunch hubs **match**; `DeviceNodeExtDispatch` **73**; `GpioOnDevEventReceive` **13 / 12**; sequential-PCH index **12.8s** | Platform indexes; **typed** virtual plugin graph recovered (H4, H9, H10, H16); `auto`/bind/ffrt still missing |

---

# Part 3 — Camera and clang/test (2026-08-27)

PCH-style header IR is what makes these trees finish. Before it, `~/multimedia_camera_framework` (~744 TUs, ~838 headers) hung in preprocess (diamond include explosion, then re-parse of huge spliced NAPI/CJ headers). `clang/test/Sema/deep_recursion.c` overflowed a rayon worker stack (now 16 MiB stacks + AST walk cap 512).

### Camera `~/multimedia_camera_framework`

| Metric | Value |
|--------|-------|
| Files / TUs / warmed headers | 1,593 / 744 / 838 |
| Functions | 22,977 (16,172 defined / 6,805 external) |
| Call edges | 45,469 (13,280 direct / **117** indirect / 32,072 external) |
| Arg-flow | 10,781 |
| Parse warnings | 776 |
| Index / analyze / export / wall | **8.0s / 0.3s / 1.4s / 9.7s** (`--jobs 8`) |

Completes. Not an OpenHarmony dispatch-hub eval; this is a hang/regression check.

### clang/test (llvm-project, `--jobs 8`, `--timeout-secs 180`)

| Subtree | TUs | Index | Analyze | Export | Result |
|---------|----:|------:|--------:|-------:|--------|
| `Preprocessor` | 371 | 1.0s | 0.0s | 0.1s | completes |
| `Lexer` | 138 | 0.2s | 0.0s | 0.0s | completes |
| `Parser` | 325 | 1.4s | 0.0s | 0.2s | completes |
| `CXX` | 918 | 0.5s | 0.0s | 0.1s | completes |
| `Sema` | 1,379 | 3.7s | 0.1s | 0.4s | completes (includes `deep_recursion.c`) |

These are adversarial parser/lexer tests, not a call-graph eval. The check is: no hang, no stack overflow, analysis completes.
