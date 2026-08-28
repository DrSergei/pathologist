# Evaluation Report

**Date:** 2026-08-28  
**Binary:** current tree (`trace-cli` release)  
**Solver budget:** 800,000 pops (`TRACE_SOLVE_BUDGET_POPS`)  
**Machine:** Linux, 16 logical CPUs, `--jobs 8`, minimal SQLite export  

**Re-verified 2026-08-28:** all three corpora were re-analyzed fresh with the current tree
(`cargo test --workspace` green). Every hub and per-case target set is **unchanged**
(all names still resolved); the global metric tables below were refreshed to the current
counts. Two intended moves are now visible in the aggregates:
- the **`param_type_ids` merge fix** + **in-class template-method registration** collapse
  prototype/definition pairs, so total/external function counts are lower and calls that
  previously bound to a prototype now resolve `direct` (hiview direct +267 vs the old table);
- **run-to-run drift**: the parallel index is nondeterministic within a small band
  (observed on camera: 22,289 ± 29 functions, 43,312 ± 228 edges across identical binaries),
  so exact counts can wiggle between runs.

Each corpus is a separate section: **performance first**, then the **complete case list**. A case is file, line, function, and the full list of resolved function-pointer (or CHA virtual) targets from this binary.

C++ fixture coverage (`cpp_basic`, `cpp_dispatch`, `cpp_callable`, `cpp_flow`, …) lives under `tests/fixtures/` and is exercised by `cargo test`, not as a corpus below.

---

# 1. `drivers_hdf_core`

**Path:** `~/drivers_hdf_core`  
**Role:** OpenHarmony HDF kernel driver framework — C/C++ function-pointer dispatch  

## Performance

| Step | Time |
|------|-----:|
| Index | 3.7s |
| Analyze | 1.3s |
| Export | 0.7s |
| **Wall** | **5.9s** |

| Metric | Value |
|--------|------:|
| Files | 1,483 |
| Functions | 11,721 (9,382 defined / 2,339 external) |
| Call edges | 40,347 |
| Direct / indirect / external | 20,916 / **4,475** / 14,956 |
| Arg-flow edges | 32,400 |
| Parse warnings | 370 |
| `dlsym` PAG edges | 4 |

Sequential warm, then **wave-parallel PCH** (626 headers). Nested merge is **types/typedefs** from **direct** includes plus this header's preprocess `included_headers` (child units already nested-merged grandchild types). Each TU merges **symbols** from every include-graph-reachable header plus preprocess `included_headers`. After warm, preprocess `included_headers` are added as include-graph edges so a header is never PCH'd in the same wave as a nested type the raw `#include` scanner missed; headers that become reachable only then move from the orphan path into PCH. Include-graph **cycles** are indexed in order, not as a parallel leftover wave. That was the `DeviceNodeExtDispatch` 73→72 drop (`DispatchToMessage`): `hdf_wifi_core.c` designated `.object.objectId = 1, .Dispatch = DispatchToMessage` needs a complete `struct HdfObject` prefix inside `IDeviceIoService`. Parallel leaves used to intern that nested tag empty; sequential path-sort happened to PCH `hdf_object.h` first. With preprocess edges, waves keep all 73 names (including `DispatchToMessage`). `pch-done` 0.2s vs 1.0s sequential. Index also keeps a named-tag → richest-`TypeId` map (no scan of `types[]` on every intern), shares file/preprocessed text as `Arc<str>`, caches `canonicalize`, and builds each TU's header preamble from one PCH topo order (no per-TU Kahn sort or recanonicalize of graph keys).

Hub unique-indirect counts are unchanged vs the previous correct snapshot: `DeviceNodeExtDispatch` **73** (includes `DispatchToMessage`), `HdfDeviceLaunchNode` **125**, `HdfSbufReadBuffer` **2**, `StreamDispatch` **24**, `HdfCameraDispatch` **23**, `HdfPmDriverDispatch` **19**, `HdfObjectManagerGetObject` **18**, `PlatformDumperDump` **13**, `SetOption` **13**, `DeviceDriverBind` 122 edges / **106** names, `GpioOnDevEventReceive` 13 edges / **12** names. Leftovers: `HdfDeviceUnlaunchNode` **112** names, linux `WorkEntry` **20**. Global indirect is **4,475** (no hub names lost; the +1 indirect edge over the previous table is one new C++ overload record split, not a hub change). Because same-name overloads now stay distinct, a hub can have more `functions` rows than names; the counts above are **unique names** (call/export rows may be a few higher).

## Cases

### 1. `DeviceNodeExtDispatch` — HDF device-node dispatch hub

| Field | Value |
|-------|-------|
| File | `framework/core/common/src/hdf_device_node_ext.c` |
| Line | 20–50 |
| Function | `DeviceNodeExtDispatch` |
| Function-pointer sites | `deviceMethod->Dispatch` (line 47) |
| Resolved targets | **73** |

Central device IPC dispatch: `deviceMethod->Dispatch`.

**Resolved function-pointer targets:**

- `AdcManagerDispatch`
- `AdcTestDispatch`
- `BacklightDispatch`
- `CanServiceDispatch`
- `CanTestDispatch`
- `ClockManagerDispatch`
- `ClockTestDispatch`
- `ControlDispatch`
- `DacManagerDispatch`
- `DacTestDispatch`
- `DispatchAccel`
- `DispatchAls`
- `DispatchBarometer`
- `DispatchCommand`
- `DispatchGas`
- `DispatchGravity`
- `DispatchGyro`
- `DispatchHall`
- `DispatchHumidity`
- `DispatchLight`
- `DispatchMagnetic`
- `DispatchPedometer`
- `DispatchPpg`
- `DispatchProximity`
- `DispatchSensor`
- `DispatchTemperature`
- `DispatchToMessage`
- `DispatchVibrator`
- `GpioServiceDispatch`
- `GpioTestDispatch`
- `HdfCameraDispatch`
- `HdfDispDispatch`
- `HdfEnCoderDispatch`
- `HdfHIDDispatch`
- `HdfInfraredDispatch`
- `HdfKeventIoServiceDispatch`
- `HdfKeyDispatch`
- `HdfPmDriverDispatch`
- `HdfTestCaseProcess`
- `HdfTouchDispatch`
- `HdfUeventDriverDispatch`
- `HdmiIoDispatch`
- `HelperDriverDispatch`
- `I2cTestDispatch`
- `I3cTestDispatch`
- `MmcIoDispatch`
- `PcieBusTestDispatch`
- `PcieIoDispatch`
- `PcieTestDispatch`
- `PinIoManagerDispatch`
- `PinTestDispatch`
- `PwmIoDispatch`
- `PwmTestDispatch`
- `RtcIoDispatch`
- `RtcTestDispatch`
- `SampleDispatch`
- `SampleDriverDispatch`
- `SampleServiceDispatch`
- `SensorTestDispatch`
- `SpiIoDispatch`
- `SpiTestDispatch`
- `StreamDispatch`
- `TestDispatch`
- `TimerIoDispatch`
- `TimerTestDispatch`
- `UartIoDispatch`
- `UartTestDispatch`
- `UsbPnpManagerDispatch`
- `UsbPnpNotifyDispatch`
- `UsbTestPnpNotifyDispatch`
- `UsbnetAdapterDispatch`
- `WatchdogIoDispatch`
- `WatchdogTestDispatch`

### 2. `HandleRequestMessage` — WiFi command dispatch table

| Field | Value |
|-------|-------|
| File | `framework/model/network/wifi/platform/src/message/nodes/local_node.c` |
| Line | 32–51 |
| Function | `HandleRequestMessage` |
| Function-pointer sites | `messageDef->handler` (line 48) |
| Resolved targets | **56** |

WiFi command table: `messageDef->handler`.

**Resolved function-pointer targets:**

- `FuncNoLoad`
- `FuncSmallLoad`
- `WifiCmdAbortScan`
- `WifiCmdAddIf`
- `WifiCmdAssoc`
- `WifiCmdCancelRemainOnChannel`
- `WifiCmdChangeBeacon`
- `WifiCmdDelKey`
- `WifiCmdDisableEapol`
- `WifiCmdDisconnect`
- `WifiCmdDoResetChip`
- `WifiCmdEnableEapol`
- `WifiCmdGetAddr`
- `WifiCmdGetApBandwidth`
- `WifiCmdGetAssociatedStas`
- `WifiCmdGetChipId`
- `WifiCmdGetDevMacAddr`
- `WifiCmdGetDriverFlag`
- `WifiCmdGetHwFeature`
- `WifiCmdGetIfNamesByChipId`
- `WifiCmdGetNetDevInfo`
- `WifiCmdGetNetworkInfo`
- `WifiCmdGetPowerMode`
- `WifiCmdGetSignalPollInfo`
- `WifiCmdGetSupportCombo`
- `WifiCmdGetValidFreqsWithBand`
- `WifiCmdIsSupportCombo`
- `WifiCmdNewKey`
- `WifiCmdProbeReqReport`
- `WifiCmdReceiveEapol`
- `WifiCmdRemainOnChannel`
- `WifiCmdRemoveIf`
- `WifiCmdResetDriver`
- `WifiCmdScan`
- `WifiCmdSendAction`
- `WifiCmdSendEapol`
- `WifiCmdSetAp`
- `WifiCmdSetApWpsP2pIe`
- `WifiCmdSetClient`
- `WifiCmdSetCountryCode`
- `WifiCmdSetKey`
- `WifiCmdSetMacAddr`
- `WifiCmdSetMode`
- `WifiCmdSetNetdev`
- `WifiCmdSetPowerMode`
- `WifiCmdSetScanningMacAddress`
- `WifiCmdSetTxPower`
- `WifiCmdStaRemove`
- `WifiCmdStartChannelMeas`
- `WifiCmdStartPnoScan`
- `WifiCmdStopAp`
- `WifiCmdStopPnoScan`
- `WifiGetStationInfo`
- `WifiSendCmdIoctl`
- `WifiSendMlme`
- `WifiSetProjectionScreenParam`

### 3. `HdfDeviceLaunchNode` — Driver initialization

| Field | Value |
|-------|-------|
| File | `framework/core/host/src/hdf_device_node.c` |
| Line | 94–131 |
| Function | `HdfDeviceLaunchNode` |
| Function-pointer sites | `driverEntry->Init` (line 116) |
| Resolved targets | **125** |

Driver init table: `driverEntry->Init`.

**Resolved function-pointer targets:**

- `AccelInitDriver`
- `AdcManagerInit`
- `AdcTestInit`
- `AlsInitDriver`
- `AudioControlInit`
- `AudioDriverInit`
- `AudioHdmiCodecDriverInit`
- `AudioStreamInit`
- `AudioUsbCodecDriverInit`
- `AudioUsbDmaDriverInit`
- `BacklightInit`
- `BarometerInitDriver`
- `BlPwmEntryInit`
- `CanTestInit`
- `ClockManagerInit`
- `ClockTestInit`
- `DacManagerInit`
- `DacTestInit`
- `DummyI2cInit`
- `EdtFocalChipInit`
- `EmmcTestInit`
- `GasInitDriver`
- `GpioDriverInit`
- `GpioServiceInit`
- `GpioTestInit`
- `GravityInitDriver`
- `GyroInitDriver`
- `HallInitDriver`
- `HdfCameraDriverInit`
- `HdfDispEntryInit`
- `HdfDrmPanelEntryInit`
- `HdfEnCoderDriverInit`
- `HdfEthDriverInit`
- `HdfFocalChipInit`
- `HdfGoodixChipInit`
- `HdfHIDDriverInit`
- `HdfHelperDriverInit`
- `HdfInfraredDriverInit`
- `HdfInputManagerInit`
- `HdfKeventDriverInit`
- `HdfKeyDriverInit`
- `HdfPmDriverInit`
- `HdfPwmInit`
- `HdfSample1DriverInit`
- `HdfSampleDriverInit`
- `HdfSoftbusDriverInit`
- `HdfSpiDeviceInit`
- `HdfTestDriverInit`
- `HdfTouchDriverProbe`
- `HdfUartDeviceInit`
- `HdfUeventDriverInit`
- `HdfVirtualCanInit`
- `HdfWdtInit`
- `HdfWlanMainInit`
- `HdmiTestInit`
- `Hi35xxEntryInit`
- `Hi35xxMipiTxInit`
- `HiRtcInit`
- `HumidityInitDriver`
- `I2cDriverInit`
- `I2cManagerInit`
- `I2cTestInit`
- `I2sTestInit`
- `I3cManagerInit`
- `I3cTestInit`
- `Icn9700EntryInit`
- `Ili9881cBoeEntryInit`
- `InitLightDriver`
- `InitSensorDevManager`
- `InitSensorDriverTest`
- `InitVibratorDriver`
- `LcdkitEntryInit`
- `LinuxAdcInit`
- `LinuxClockInit`
- `LinuxEmmcInit`
- `LinuxGpioInit`
- `LinuxI2cInit`
- `LinuxRegulatorInit`
- `LinuxSdioInit`
- `MagneticInitDriver`
- `MipiCsiAdapterInit`
- `MipiCsiTestInit`
- `MipiDsiAdapterInit`
- `MipiDsiTestInit`
- `PanelEntryInit`
- `PcieBusTestInit`
- `PcieTestInit`
- `PcieVirtualAdapterInit`
- `PedometerInitDriver`
- `PinTestInit`
- `PlatformTestInit`
- `PpgInitDriver`
- `ProximityInitDriver`
- `PwmDriverInit`
- `PwmTestInit`
- `RegulatorManagerInit`
- `RegulatorTestInit`
- `RtcTestInit`
- `SampleUartDriverInit`
- `SdioTestInit`
- `SpiDriverInit`
- `SpiTestInit`
- `SspSt7789EntryInit`
- `TemperatureInitDriver`
- `TimerManagerInit`
- `TimerTestInit`
- `UartDriverInit`
- `UartTestInit`
- `UsbPnpManagerInit`
- `UsbPnpNotifyInit`
- `UsbTestPnpNotifyInit`
- `UsbnetAdapterInit`
- `VirtualAdcInit`
- `VirtualClockInit`
- `VirtualDacInit`
- `VirtualI3cInit`
- `VirtualPinInit`
- `VirtualPwmInit`
- `VirtualRegulatorInit`
- `VirtualSpiDeviceInit`
- `VirtualWatchdogInit`
- `WatchdogDriverInit`
- `WatchdogTestInit`
- `i2cDriverInit`
- `pinManagerInit`

### 4. `StreamDispatch` — Audio stream command dispatch

| Field | Value |
|-------|-------|
| File | `framework/model/audio/dispatch/src/audio_stream_dispatch.c` |
| Line | 1602–1614 |
| Function | `StreamDispatch` |
| Function-pointer sites | `g_streamDispCmdHandle[i]->func` (line 1609) |
| Resolved targets | **24** |

Audio stream command table `g_streamDispCmdHandle[i]->func`.

**Resolved function-pointer targets:**

- `StreamHostCaptureClose`
- `StreamHostCaptureOpen`
- `StreamHostCapturePause`
- `StreamHostCapturePrepare`
- `StreamHostCaptureResume`
- `StreamHostCaptureStart`
- `StreamHostCaptureStop`
- `StreamHostDspDecode`
- `StreamHostDspEncode`
- `StreamHostDspEqualizer`
- `StreamHostHwParams`
- `StreamHostMmapPositionRead`
- `StreamHostMmapPositionWrite`
- `StreamHostMmapRead`
- `StreamHostMmapWrite`
- `StreamHostRead`
- `StreamHostRenderClose`
- `StreamHostRenderOpen`
- `StreamHostRenderPause`
- `StreamHostRenderPrepare`
- `StreamHostRenderResume`
- `StreamHostRenderStart`
- `StreamHostRenderStop`
- `StreamHostWrite`

### 5. `BacklightDispatch` — Display brightness dispatch

| Field | Value |
|-------|-------|
| File | `framework/model/display/driver/backlight/hdf_bl.c` |
| Line | 398–412 |
| Function | `BacklightDispatch` |
| Function-pointer sites | `blCmdHandle` (line 411) |
| Resolved targets | **6** |

Backlight command table `blCmdHandle`.

**Resolved function-pointer targets:**

- `HdfGetBlDevList`
- `HdfGetCurrBrightness`
- `HdfGetDefBrightness`
- `HdfGetMaxBrightness`
- `HdfGetMinBrightness`
- `HdfSetBrightness`

### 6. `ControlDispatch` — Audio control dispatch

| Field | Value |
|-------|-------|
| File | `framework/model/audio/dispatch/src/audio_control_dispatch.c` |
| Line | 549–574 |
| Function | `ControlDispatch` |
| Function-pointer sites | `g_controlDispCmdHandle[i]->func` (line 570) |
| Resolved targets | **6** |

Audio control table `g_controlDispCmdHandle[i]->func`.

**Resolved function-pointer targets:**

- `ControlHostElemGetCard`
- `ControlHostElemInfo`
- `ControlHostElemList`
- `ControlHostElemRead`
- `ControlHostElemUnloadCard`
- `ControlHostElemWrite`

### 7. `RunDispatcher` — WiFi message dispatcher loop

| Field | Value |
|-------|-------|
| File | `framework/model/network/wifi/platform/src/message/message_dispatcher.c` |
| Line | 238–282 |
| Function | `RunDispatcher` |
| Function-pointer sites | `dispatcher->Ref` (line 253); `dispatcher->Disref` (line 258); `dispatcher->Disref` (line 276) |
| Resolved targets | **2** |

WiFi dispatcher loop; function-pointer deref of queued handlers.

**Resolved function-pointer targets:**

- `DisreferenceMessageDispatcher`
- `ReferenceMessageDispatcher`

### 8. `FinishEvent` — System event dispatcher

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/osal/src/osal_sysevent.c` |
| Line | 61–81 |
| Function | `FinishEvent` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 74) |
| Resolved targets | **5** |

Sys-event finish → registered dispatchers.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 9. `AdcOpen` — ADC open (user-space IPC)

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c` |
| Line | 30–77 |
| Function | `AdcOpen` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 60) |
| Resolved targets | **5** |

User-space ADC open; indirect through service `Dispatch`.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 10. `AdcRead` — ADC read (user-space IPC)

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c` |
| Line | 110–163 |
| Function | `AdcRead` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 146) |
| Resolved targets | **5** |

User-space ADC read; indirect through service `Dispatch`.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 11. `AdcClose` — ADC close (user-space IPC)

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/adc/adc_if_u.c` |
| Line | 79–108 |
| Function | `AdcClose` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 103) |
| Resolved targets | **5** |

User-space ADC close; indirect through service `Dispatch`.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 12. `AdcDeviceRead` — ADC core read

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/adc/adc_core.c` |
| Line | 306–333 |
| Function | `AdcDeviceRead` |
| Function-pointer sites | `device->ops->read` (line 330) |
| Resolved targets | **2** |

Driver-core ADC read: `device->ops->read`.

**Resolved function-pointer targets:**

- `AdcIioRead`
- `VirtualAdcRead`

### 13. `DeviceManagerDispatch` — Device manager dispatch

| Field | Value |
|-------|-------|
| File | `framework/core/common/src/devmgr_service_start.c` |
| Line | 66–106 |
| Function | `DeviceManagerDispatch` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Device-manager dispatch hub (direct calls only).

**Resolved function-pointer targets:** none.

### 14. `DevSvcManagerCreate` — Singleton service manager

| Field | Value |
|-------|-------|
| File | `framework/core/manager/src/devsvc_manager.c` |
| Line | 412–423 |
| Function | `DevSvcManagerCreate` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Singleton service-manager creation.

**Resolved function-pointer targets:** none.

### 15. `DevSvcManagerClntGetInstance` — Client singleton

| Field | Value |
|-------|-------|
| File | `framework/core/host/src/devsvc_manager_clnt.c` |
| Line | 146–155 |
| Function | `DevSvcManagerClntGetInstance` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Client singleton getter.

**Resolved function-pointer targets:** none.

### 16. `DevMgrUeventRuleCfgList` — Static uevent config list

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/manager/src/devmgr_uevent.c` |
| Line | 69–80 |
| Function | `DevMgrUeventRuleCfgList` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Static uevent config list.

**Resolved function-pointer targets:** none.

### 17. `DevSvcManagerExtStart` — Extended service manager start

| Field | Value |
|-------|-------|
| File | `framework/core/manager/src/devsvc_manager_ext.c` |
| Line | 129–165 |
| Function | `DevSvcManagerExtStart` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Extended service-manager start.

**Resolved function-pointer targets:** none.

### 18. `DevHostServiceStubDispatch` — Host service stub dispatch

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_stub.c` |
| Line | 80–111 |
| Function | `DevHostServiceStubDispatch` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Host-service stub IPC dispatch (direct).

**Resolved function-pointer targets:** none.

### 19. `DevHostServiceStubCreate` — Stub factory

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_stub.c` |
| Line | 123–135 |
| Function | `DevHostServiceStubCreate` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Stub factory.

**Resolved function-pointer targets:** none.

### 20. `DevHostServiceStubConstruct` — Stub construct

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_stub.c` |
| Line | 113–121 |
| Function | `DevHostServiceStubConstruct` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Stub construct.

**Resolved function-pointer targets:** none.

### 21. `DevHostServiceFullConstruct` — Full service constructor

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_full.c` |
| Line | 202–213 |
| Function | `DevHostServiceFullConstruct` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Full host-service constructor.

**Resolved function-pointer targets:** none.

### 22. `DevHostServiceFullDispatchMessage` — Message dispatch

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/devhost_service_full.c` |
| Line | 27–57 |
| Function | `DevHostServiceFullDispatchMessage` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Host-service message dispatch (direct).

**Resolved function-pointer targets:** none.

### 23. `GpioSetIrq` — GPIO IRQ configuration

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/gpio/gpio_if_u.c` |
| Line | 261–314 |
| Function | `GpioSetIrq` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 304) |
| Resolved targets | **5** |

GPIO IRQ configuration; userspace body calls `GpioRegListener`.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 24. `GetUartDeviceResource` — HCS config (uart_bes)

| Field | Value |
|-------|-------|
| File | `adapter/platform/uart/uart_bes.c` |
| Line | 510–564 |
| Function | `GetUartDeviceResource` |
| Function-pointer sites | `dri->GetUint32` (line 530); `dri->GetUint32` (line 534); `dri->GetUint32` (line 538); `dri->GetUint32` (line 542); `dri->GetUint32` (line 546); `dri->GetBool` (line 551); `dri->GetBool` (line 552) |
| Resolved targets | **2** |

HCS config: `dri->GetUint32` / `dri->GetBool`. This case is the `uart_bes` translation unit.

**Resolved function-pointer targets:**

- `HcsGetBool`
- `HcsGetUint32`

### 25. `GetUartDeviceResource` — HCS config (uart_stm32f4xx)

| Field | Value |
|-------|-------|
| File | `adapter/platform/uart/uart_stm32f4xx.c` |
| Line | 477–520 |
| Function | `GetUartDeviceResource` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

HCS config: `dri->GetUint32` / `dri->GetBool`. This case is the `uart_stm32` translation unit.

**Resolved function-pointer targets:** none.

### 26. `ChipDataHandle` — Touchscreen data (`fn_static`)

| Field | Value |
|-------|-------|
| File | `framework/model/input/driver/touchscreen/touch_ft5406.c` |
| Line | 115–162 |
| Function | `ChipDataHandle` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Touchscreen data path with `fn_static` (direct + `memset_s`).

**Resolved function-pointer targets:** none.

### 27. `AdcTestGetConfig` — ADC test configuration

| Field | Value |
|-------|-------|
| File | `framework/test/unittest/platform/common/adc_test.c` |
| Line | 27–79 |
| Function | `AdcTestGetConfig` |
| Function-pointer sites | `service->dispatcher->Dispatch` (line 50) |
| Resolved targets | **5** |

Test config retrieval; indirect through service `Dispatch`.

**Resolved function-pointer targets:**

- `DeviceManagerDispatch`
- `DeviceNodeExtDispatch`
- `DeviceSvcMgrDispatch`
- `HdfKIoServiceDispatch`
- `HdfSyscallAdapterDispatch`

### 28. `ClockManagerDispatch` — Clock platform dispatch

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/clock/clock_core.c` |
| Line | 762–801 |
| Function | `ClockManagerDispatch` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Clock platform dispatch (direct).

**Resolved function-pointer targets:** none.

### 29. `AudioCodecDevInit` — Audio codec device init

| Field | Value |
|-------|-------|
| File | `framework/model/audio/core/src/audio_host.c` |
| Line | 60–87 |
| Function | `AudioCodecDevInit` |
| Function-pointer sites | `codec->devData->Init` (line 78) |
| Resolved targets | **2** |

Audio codec `codec->devData->Init`.

**Resolved function-pointer targets:**

- `AudioHdmiCodecDeviceInit`
- `AudioUsbCodecDeviceInit`

### 30. `AudioDmaConfigChannel` — DMA channel configuration

| Field | Value |
|-------|-------|
| File | `framework/model/audio/common/src/audio_dma_base.c` |
| Line | 40–46 |
| Function | `AudioDmaConfigChannel` |
| Function-pointer sites | `data->ops->DmaConfigChannel` (line 43) |
| Resolved targets | **1** |

DMA config: `data->ops->DmaConfigChannel`.

**Resolved function-pointer targets:**

- `AudioUsbDmaConfigChannel`

### 31. `PlatformManagerTestAddAndDel` — Platform manager test (uniproton)

| Field | Value |
|-------|-------|
| File | `adapter/khdf/uniproton/test/sample_driver/src/platform_manager_test.c` |
| Line | 88–152 |
| Function | `PlatformManagerTestAddAndDel` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

Uniproton platform-manager test lifecycle.

**Resolved function-pointer targets:** none.

### 32. `HdfSbufReadBuffer` — C + C++ sbuf readBuffer

| Field | Value |
|-------|-------|
| File | `framework/utils/src/hdf_sbuf.c` |
| Line | 194–198 |
| Function | `HdfSbufReadBuffer` |
| Function-pointer sites | `sbuf->impl->readBuffer` (line 197) |
| Resolved targets | **2** |

C/C++ sbuf interop: `sbuf->impl->readBuffer` (FieldId guard: exactly 2).

**Resolved function-pointer targets:**

- `SbufMParcelImplReadBuffer`
- `SbufRawImplReadBuffer`

### 33. `HdfDeviceUnlaunchNode` — Driver teardown

| Field | Value |
|-------|-------|
| File | `framework/core/host/src/hdf_device_node.c` |
| Line | 183–222 |
| Function | `HdfDeviceUnlaunchNode` |
| Function-pointer sites | `driverEntry->Release` (line 200); `devNode->super->RemoveService` (line 209); `driverLoader->ReclaimDriver` (line 216) |
| Resolved targets | **112** |

Driver teardown: `driverEntry->Release`. Unique names **112** (original eval 116).

**Resolved function-pointer targets:**

- `AccelReleaseDriver`
- `AdcManagerRelease`
- `AdcTestRelease`
- `AlsReleaseDriver`
- `AudioControlRelease`
- `AudioDriverRelease`
- `AudioHdmiCodecDriverRelease`
- `AudioStreamRelease`
- `AudioUsbCodecDriverRelease`
- `AudioUsbDmaDriverRelease`
- `BarometerReleaseDriver`
- `CanTestRelease`
- `ClockManagerRelease`
- `ClockTestRelease`
- `DacManagerRelease`
- `DacTestRelease`
- `DummyI2cRelease`
- `EmmcTestRelease`
- `GasReleaseDriver`
- `GpioDriverRelease`
- `GpioServiceRelease`
- `GpioTestRelease`
- `GravityReleaseDriver`
- `GyroReleaseDriver`
- `HallReleaseDriver`
- `HdfCameraDriverRelease`
- `HdfDeviceNodeRemoveService`
- `HdfEncoderDriverRelease`
- `HdfEthDriverRelease`
- `HdfFocalChipRelease`
- `HdfGoodixChipRelease`
- `HdfHIDDriverRelease`
- `HdfHelperDriverRelease`
- `HdfInfraredDriverRelease`
- `HdfInputManagerRelease`
- `HdfKeventDriverRelease`
- `HdfPmDriverRelease`
- `HdfPwmRelease`
- `HdfSample1DriverRelease`
- `HdfSampleDriverRelease`
- `HdfSoftbusDriverRelease`
- `HdfSpiDeviceRelease`
- `HdfTestDriverRelease`
- `HdfTouchDriverRelease`
- `HdfUartDeviceRelease`
- `HdfUeventDriverRelease`
- `HdfVirtualCanRelease`
- `HdfWdtRelease`
- `HdfWlanDriverRelease`
- `HdmiTestRelease`
- `Hi35xxMipiTxRelease`
- `HiRtcRelease`
- `HumidityReleaseDriver`
- `I2cDriverRelease`
- `I2cManagerRelease`
- `I2cTestRelease`
- `I2sTestRelease`
- `I3cManagerRelease`
- `I3cTestRelease`
- `LinuxAdcRelease`
- `LinuxClockRelease`
- `LinuxEmmcRelease`
- `LinuxGpioRelease`
- `LinuxI2cRelease`
- `LinuxRegulatorRelease`
- `LinuxSdioRelease`
- `MagneticReleaseDriver`
- `MipiCsiAdapterRelease`
- `MipiDsiAdapterRelease`
- `PcieBusTestRelease`
- `PcieTestRelease`
- `PcieVirtualAdapterRelease`
- `PedometerReleaseDriver`
- `PinTestRelease`
- `PlatformTestRelease`
- `PpgReleaseDriver`
- `ProximityReleaseDriver`
- `PwmDriverRelease`
- `PwmTestRelease`
- `RegulatorManagerRelease`
- `RegulatorTestRelease`
- `ReleaseLightDriver`
- `ReleaseSensorDevManager`
- `ReleaseSensorDriverTest`
- `ReleaseVibratorDriver`
- `RtcTestRelease`
- `SampleUartDriverRelease`
- `SdioTestRelease`
- `SpiDriverRelease`
- `SpiTestRelease`
- `TemperatureReleaseDriver`
- `TimerManagerRelease`
- `TimerTestRelease`
- `UartDriverRelease`
- `UartTestRelease`
- `UsbPnpManagerRelease`
- `UsbPnpNotifyRelease`
- `UsbTestPnpNotifyRelease`
- `UsbnetAdapterRelease`
- `VirtualAdcRelease`
- `VirtualClockRelease`
- `VirtualDacRelease`
- `VirtualI3cRelease`
- `VirtualPinRelease`
- `VirtualPwmRelease`
- `VirtualRegulatorRelease`
- `VirtualSpiDeviceRelease`
- `VirtualWatchdogRelease`
- `WatchdogDriverRelease`
- `WatchdogTestRelease`
- `i2cDriverRelease`
- `pinManagerRelease`

### 34. `DeviceDriverBind` — Driver binding

| Field | Value |
|-------|-------|
| File | `framework/core/host/src/hdf_device_node.c` |
| Line | 65–92 |
| Function | `DeviceDriverBind` |
| Function-pointer sites | `driverEntry->Bind` (line 84) |
| Resolved targets | **106** |

Driver bind: `driverEntry->Bind`. **122** edges / **106** unique names (several edges share a callee).

**Resolved function-pointer targets:**

- `AccelBindDriver`
- `AdcManagerBind`
- `AdcTestBind`
- `AlsBindDriver`
- `AudioControlBind`
- `AudioDriverBind`
- `AudioHdmiCodecDriverBind`
- `AudioStreamBind`
- `AudioUsbCodecDriverBind`
- `AudioUsbDmaDriverBind`
- `BacklightBind`
- `BarometerBindDriver`
- `BindLightDriver`
- `BindSensorDevManager`
- `BindSensorDriverTest`
- `BindVibratorDriver`
- `CanTestBind`
- `ClockManagerBind`
- `ClockTestBind`
- `DacManagerBind`
- `DacTestBind`
- `DummyI2cBind`
- `EmmcTestBind`
- `GasBindDriver`
- `GpioDriverBind`
- `GpioServiceBind`
- `GpioTestBind`
- `GravityBindDriver`
- `GyroBindDriver`
- `HallBindDriver`
- `HdfCameraDriverBind`
- `HdfDispBind`
- `HdfEnCoderDriverBind`
- `HdfEthDriverBind`
- `HdfHIDDriverBind`
- `HdfHelperDriverBind`
- `HdfInfraredDriverBind`
- `HdfInputManagerBind`
- `HdfKeventDriverBind`
- `HdfKeyDriverBind`
- `HdfPmDriverBind`
- `HdfPwmBind`
- `HdfSample1DriverBind`
- `HdfSampleDriverBind`
- `HdfSoftbusDriverBind`
- `HdfSpiDeviceBind`
- `HdfTestDriverBind`
- `HdfTouchDriverBind`
- `HdfUartDeviceBind`
- `HdfUeventDriverBind`
- `HdfVirtualCanBind`
- `HdfWdtBind`
- `HdfWifiDriverBind`
- `HdmiTestBind`
- `HiRtcBind`
- `HumidityBindDriver`
- `I2cDriverBind`
- `I2cManagerBind`
- `I2cTestBind`
- `I2sTestBind`
- `I3cManagerBind`
- `I3cTestBind`
- `LinuxEmmcBind`
- `LinuxGpioBind`
- `LinuxI2cBind`
- `LinuxRegulatorBind`
- `LinuxSdioBind`
- `MagneticBindDriver`
- `MipiCsiAdapterBind`
- `MipiCsiTestBind`
- `MipiDsiAdapterBind`
- `MipiDsiTestBind`
- `PcieBusTestBind`
- `PcieTestBind`
- `PcieVirtualAdapterBind`
- `PedometerBindDriver`
- `PinTestBind`
- `PlatformTestBind`
- `PpgBindDriver`
- `ProximityBindDriver`
- `PwmDriverBind`
- `PwmTestBind`
- `RegulatorManagerBind`
- `RegulatorTestBind`
- `RtcTestBind`
- `SampleUartDriverBind`
- `SdioTestBind`
- `SpiDriverBind`
- `SpiTestBind`
- `TemperatureBindDriver`
- `TimerManagerBind`
- `TimerTestBind`
- `UartDriverBind`
- `UartTestBind`
- `UsbPnpManagerBind`
- `UsbPnpNotifyBind`
- `UsbTestPnpNotifyBind`
- `UsbnetAdapterBind`
- `VirtualPinBind`
- `VirtualPwmBind`
- `VirtualSpiDeviceBind`
- `VirtualWatchdogBind`
- `WatchdogDriverBind`
- `WatchdogTestBind`
- `i2cDriverBind`
- `pinManagerBind`

### 35. `HdfCameraDispatch` — Camera command dispatch

| Field | Value |
|-------|-------|
| File | `framework/model/camera/dispatch/src/camera_dispatch.c` |
| Line | 521–542 |
| Function | `HdfCameraDispatch` |
| Function-pointer sites | `g_cameraCmdHandle[i]->func` (line 538) |
| Resolved targets | **23** |

Camera command table `g_cameraCmdHandle[i].func`.

**Resolved function-pointer targets:**

- `CameraCmdCloseCamera`
- `CameraCmdEnumDevice`
- `CameraCmdEnumFmt`
- `CameraCmdGetAbility`
- `CameraCmdGetConfig`
- `CameraCmdGetCrop`
- `CameraCmdGetFPS`
- `CameraCmdGetFormat`
- `CameraCmdOpenCamera`
- `CameraCmdPowerDown`
- `CameraCmdPowerUp`
- `CameraCmdQueryConfig`
- `CameraCmdQueryMemory`
- `CameraCmdQueueInit`
- `CameraCmdReqMemory`
- `CameraCmdSetConfig`
- `CameraCmdSetCrop`
- `CameraCmdSetFPS`
- `CameraCmdSetFormat`
- `CameraCmdStreamDeQueue`
- `CameraCmdStreamOff`
- `CameraCmdStreamOn`
- `CameraCmdStreamQueue`

### 36. `PowerStateChange` — Power-state dispatch (4 sites)

| Field | Value |
|-------|-------|
| File | `framework/core/host/src/power_state_token.c` |
| Line | 58–90 |
| Function | `PowerStateChange` |
| Function-pointer sites | `stateToken->listener->Suspend` (line 67); `stateToken->listener->Resume` (line 72); `stateToken->listener->DozeSuspend` (line 77); `stateToken->listener->DozeResume` (line 82) |
| Resolved targets | **16** |

PM listener vtable: Suspend / Resume / DozeSuspend / DozeResume. Four sites × four listener families (**16** unique names).

**Resolved function-pointer targets:**

- `HdfPmHdfTestDozeResume`
- `HdfPmHdfTestDozeSuspend`
- `HdfPmHdfTestResume`
- `HdfPmHdfTestSuspend`
- `HdfPmSampleDozeResume`
- `HdfPmSampleDozeSuspend`
- `HdfPmSampleResume`
- `HdfPmSampleSuspend`
- `HdfPmTestDozeResume`
- `HdfPmTestDozeSuspend`
- `HdfPmTestResume`
- `HdfPmTestSuspend`
- `HdfSampleDozeResume`
- `HdfSampleDozeSuspend`
- `HdfSampleResume`
- `HdfSampleSuspend`

### 37. `HdfObjectManagerGetObject` — Object factory dispatch

| Field | Value |
|-------|-------|
| File | `framework/core/shared/src/hdf_object_manager.c` |
| Line | 11–22 |
| Function | `HdfObjectManagerGetObject` |
| Function-pointer sites | `targetCreator->Create` (line 16) |
| Resolved targets | **18** |

Object factory: `targetCreator->Create`.

**Resolved function-pointer targets:**

- `DevHostServiceCreate`
- `DevHostServiceStubCreate`
- `DevSvcManagerCreate`
- `DevSvcManagerExtCreate`
- `DevSvcManagerProxyCreate`
- `DevSvcManagerStubCreate`
- `DeviceNodeExtCreate`
- `DeviceServiceStubCreate`
- `DeviceTokenStubCreate`
- `DevmgrServiceCreate`
- `DevmgrServiceProxyCreate`
- `DevmgrServiceStubCreate`
- `DriverInstallerCreate`
- `DriverInstallerFullCreate`
- `HdfDeviceCreate`
- `HdfDeviceTokenCreate`
- `HdfDriverLoaderCreate`
- `HdfDriverLoaderFullCreate`

### 38. `SetOption` — Sensor option dispatch

| Field | Value |
|-------|-------|
| File | `framework/model/sensor/driver/common/src/sensor_device_manager.c` |
| Line | 216–231 |
| Function | `SetOption` |
| Function-pointer sites | `deviceInfo->ops->SetOption` (line 230) |
| Resolved targets | **13** |

Sensor `deviceInfo->ops.SetOption`.

**Resolved function-pointer targets:**

- `SetAccelOption`
- `SetAlsOption`
- `SetBarometerOption`
- `SetGasOption`
- `SetGravityOption`
- `SetGyroOption`
- `SetHallOption`
- `SetHumidityOption`
- `SetMagneticOption`
- `SetPedometerOption`
- `SetPpgOption`
- `SetProximityOption`
- `SetTemperatureOption`

### 39. `GpioOnDevEventReceive` — GPIO event callback

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/fwk/platform_listener_u.c` |
| Line | 121–149 |
| Function | `GpioOnDevEventReceive` |
| Function-pointer sites | `gpio->func` (line 146) |
| Resolved targets | **12** |

GPIO event callback: `gpio->func`. **13** edges / **12** unique names.

**Resolved function-pointer targets:**

- `GpioServiceIrqFunc`
- `GpioTestIrqHandler`
- `HallNorthPolarityIrqFunc`
- `HallSouthPolarityIrqFunc`
- `InfraredIrqHandle`
- `IrqHandle`
- `KeyIrqHandle`
- `PpgIrqHandler`
- `TestCaseGpioIrqHandler1`
- `TestCaseGpioIrqHandler2`
- `TestCaseGpioIrqHandler3`
- `TestCaseGpioIrqHandler4`

### 40. `HdfPmDriverDispatch` — PM driver test dispatch

| Field | Value |
|-------|-------|
| File | `framework/test/unittest/pm/hdf_pm_driver_test.c` |
| Line | 568–587 |
| Function | `HdfPmDriverDispatch` |
| Function-pointer sites | `g_testCases[cmdId]->testFunc` (line 581) |
| Resolved targets | **19** |

PM test driver `pdr->ops->Dispatch`.

**Resolved function-pointer targets:**

- `HdfPmTestBegin`
- `HdfPmTestEnd`
- `HdfPmTestOneDriverHundred`
- `HdfPmTestOneDriverOnce`
- `HdfPmTestOneDriverTen`
- `HdfPmTestOneDriverThousand`
- `HdfPmTestOneDriverTwice`
- `HdfPmTestThreeDriverHundred`
- `HdfPmTestThreeDriverHundredWithSync`
- `HdfPmTestThreeDriverOnce`
- `HdfPmTestThreeDriverSeqHundred`
- `HdfPmTestThreeDriverTen`
- `HdfPmTestThreeDriverThousand`
- `HdfPmTestThreeDriverTwice`
- `HdfPmTestTwoDriverHundred`
- `HdfPmTestTwoDriverOnce`
- `HdfPmTestTwoDriverTen`
- `HdfPmTestTwoDriverThousand`
- `HdfPmTestTwoDriverTwice`

### 41. `WorkEntry` — Workqueue dispatch (linux)

| Field | Value |
|-------|-------|
| File | `adapter/khdf/linux/osal/src/osal_workqueue.c` |
| Line | 51–63 |
| Function | `WorkEntry` |
| Function-pointer sites | `wrapper->workFunc` (line 57) |
| Resolved targets | **20** |

Linux workqueue: `work->func`. Unique names **20** (original eval 19; extra `AlsDataWorkEntry`).

**Resolved function-pointer targets:**

- `AccelDataWorkEntry`
- `AlsDataWorkEntry`
- `BarometerDataWorkEntry`
- `EsdWorkHandler`
- `EventQueueWorkEntry`
- `GasDataWorkEntry`
- `GravityDataWorkEntry`
- `GyroDataWorkEntry`
- `HallDataWorkEntry`
- `HumidityDataWorkEntry`
- `LightWorkEntry`
- `MagneticDataWorkEntry`
- `PedometerDataWorkEntry`
- `PpgDataWorkEntry`
- `ProximityDataWorkEntry`
- `SensorTestDataWorkEntry`
- `TemperatureDataWorkEntry`
- `TestDelayWorkEntry`
- `TestWorkEntry`
- `VibratorWorkEntry`

### 42. `PlatformDumperDump` — Platform dumper dispatch

| Field | Value |
|-------|-------|
| File | `framework/support/platform/src/fwk/platform_dumper_unopen.c` |
| Line | 21–25 |
| Function | `PlatformDumperDump` |
| Function-pointer sites | `pos->printFunc` (line 460) |
| Resolved targets | **13** |

Dumper type table: `ops->func`.

**Resolved function-pointer targets:**

- `DumperPrintCharInfo`
- `DumperPrintDoubleInfo`
- `DumperPrintFloatInfo`
- `DumperPrintInt16Info`
- `DumperPrintInt32Info`
- `DumperPrintInt64Info`
- `DumperPrintInt8Info`
- `DumperPrintRegisterInfo`
- `DumperPrintStringInfo`
- `DumperPrintUint16Info`
- `DumperPrintUint32Info`
- `DumperPrintUint64Info`
- `DumperPrintUint8Info`

### 43. `LoadIpcImpl` — dlsym IPC constructor load

| Field | Value |
|-------|-------|
| File | `framework/utils/src/hdf_sbuf.c` |
| Line | 76–106 |
| Function | `LoadIpcImpl` |
| Function-pointer sites | _none_ |
| Resolved targets | **0** |

`dlsym` of `"SbufObtainIpc"` / `"SbufBindIpc"` (call remains external libc).

**Resolved function-pointer targets:** none.

### 44. `HdfSbufTypedObtainCapacity` — sbuf obtain constructor

| Field | Value |
|-------|-------|
| File | `framework/utils/src/hdf_sbuf.c` |
| Line | 378–414 |
| Function | `HdfSbufTypedObtainCapacity` |
| Function-pointer sites | `constructor->obtain` (line 405) |
| Resolved targets | **3** |

Obtain constructor vtable after `dlsym` stores.

**Resolved function-pointer targets:**

- `SbufObtainIpc`
- `SbufObtainIpcHw`
- `SbufObtainRaw`

### 45. `DeviceServiceStubDispatch` — User-space IOService dispatch

| Field | Value |
|-------|-------|
| File | `adapter/uhdf2/host/src/device_service_stub.c` |
| Line | 26–60 |
| Function | `DeviceServiceStubDispatch` |
| Function-pointer sites | `ioService->Dispatch` (line 53) |
| Resolved targets | **73** |

Same `IDeviceIoService.Dispatch` field as case 1, from the UHDF2 stub. Target set is **identical** to `DeviceNodeExtDispatch` (verified), including `DispatchToMessage`.

**Resolved function-pointer targets:** same 73 names as case 1.

### 46. `HdfKIoServiceDispatch` — Kernel vnode IOService dispatch

| Field | Value |
|-------|-------|
| File | `framework/core/adapter/vnode/src/hdf_vnode_adapter.c` |
| Line | 56–71 |
| Function | `HdfKIoServiceDispatch` |
| Function-pointer sites | `kClient->client.device->service->Dispatch` (line 70) |
| Resolved targets | **73** |

Kernel vnode path onto the same `Dispatch` field. Target set is **identical** to case 1 (verified), including `DispatchToMessage`.

**Resolved function-pointer targets:** same 73 names as case 1.

### 47. `HdfObjectManagerFreeObject` — Object-factory Release

| Field | Value |
|-------|-------|
| File | `framework/core/shared/src/hdf_object_manager.c` |
| Line | 24–35 |
| Function | `HdfObjectManagerFreeObject` |
| Function-pointer sites | `targetCreator->Release` (line 34) |
| Resolved targets | **13** |

Teardown counterpart of case 37 (`Create` has 18; five creator-table slots set `Release = NULL` and correctly contribute no target).

**Resolved function-pointer targets:**

- `DevHostServiceRelease`
- `DevHostServiceStubRelease`
- `DevSvcManagerExtRelease`
- `DevSvcManagerProxyRelease`
- `DevSvcManagerRelease`
- `DeviceNodeExtRelease`
- `DeviceServiceStubRelease`
- `DeviceTokenStubRelease`
- `DevmgrServiceProxyRelease`
- `DevmgrServiceRelease`
- `HdfDeviceRelease`
- `HdfDeviceTokenRelease`
- `HdfDriverLoaderFullRelease`

### 48. `Enable` — Sensor ops Enable

| Field | Value |
|-------|-------|
| File | `framework/model/sensor/driver/common/src/sensor_device_manager.c` |
| Line | 162–169 |
| Function | `Enable` |
| Function-pointer sites | `deviceInfo->ops.Enable` (line 168) |
| Resolved targets | **13** |

All 13 production `deviceInfo->ops.Enable = Set*Enable` stores (accel/als/barometer/gas/gravity/gyro/hall/humidity/magnetic/pedometer/ppg/proximity/temperature). `SensorEnableTest` in the unittest file writes a **different** struct and is not a source for this site.

**Resolved function-pointer targets:**

- `SetAccelEnable`
- `SetAlsEnable`
- `SetBarometerEnable`
- `SetGasEnable`
- `SetGravityEnable`
- `SetGyroEnable`
- `SetHallEnable`
- `SetHumidityEnable`
- `SetMagneticEnable`
- `SetPedometerEnable`
- `SetPpgEnable`
- `SetProximityEnable`
- `SetTemperatureEnable`

### 49. `Disable` — Sensor ops Disable

| Field | Value |
|-------|-------|
| File | `framework/model/sensor/driver/common/src/sensor_device_manager.c` |
| Line | 171–179 |
| Function | `Disable` |
| Function-pointer sites | `deviceInfo->ops.Disable` (line 178) |
| Resolved targets | **13** |

Same 13 drivers as case 48, `Set*Disable` stores. Complete vs source.

**Resolved function-pointer targets:**

- `SetAccelDisable`
- `SetAlsDisable`
- `SetBarometerDisable`
- `SetGasDisable`
- `SetGravityDisable`
- `SetGyroDisable`
- `SetHallDisable`
- `SetHumidityDisable`
- `SetMagneticDisable`
- `SetPedometerDisable`
- `SetPpgDisable`
- `SetProximityDisable`
- `SetTemperatureDisable`

---

# 2. `hiviewdfx_hiview`

**Path:** `~/hiviewdfx_hiview`  
**Role:** OpenHarmony HiView DFX plugin platform — C++ virtual dispatch + preprocessor X-macros  

## Performance

| Step | Time |
|------|-----:|
| Index | 3.2s |
| Analyze | 0.1s |
| Export | 0.5s |
| **Wall** | **3.9s** |

| Metric | Value |
|--------|------:|
| Files | 1,424 |
| Functions | 10,349 (6,390 defined / 3,959 external) |
| Call edges | 19,845 |
| Direct / indirect / external | 4,323 / **10** / 15,512 |
| Arg-flow edges | 4,720 |
| Parse warnings | 462 |
| `dlsym` PAG edges | 1 |

The tree previously aborted with a preprocessor stack overflow on `PRIVATE_MESSAGE_TYPE`. Hide-set painting is what makes it finish. The **10** indirect edges are `$lambda` / JSON accessors, not the plugin pipeline pump. Typed virtual dispatch is recovered as **direct** CHA edges. Vs the previous snapshot: total/external function records are lower (the `param_type_ids` merge fix collapses prototypes into definitions) while **direct** edges rose (+267) and external fell (−322): calls that used to bind to an unmerged prototype now resolve to the defined body.

## Cases

### 1. `PRIVATE_MESSAGE_TYPE` — X-macro enumerator list (preprocessor)

| Field | Value |
|-------|-------|
| File | `base/include/defines.h` |
| Line | 39–70 |
| Function | `PRIVATE_MESSAGE_TYPE` |
| Dispatch site | _preprocessor; invoked from `event.h:127`_ |
| Resolved targets | **0** |

Not a call. Hide-set paints the first replacement token so the enum list expands as gcc does. Analysis of the tree completes (previously stack-overflowed). Same pattern: `PRIVATE_AUDIT_EVENT_TYPE`.

**Resolved function-pointer / virtual targets:** none.

### 2. `OHOS::HiviewDFX::Plugin::OnEventProxy` — Virtual plugin entry (CHA)

| Field | Value |
|-------|-------|
| File | `base/plugin.cpp` |
| Line | 55–83 |
| Function | `OHOS::HiviewDFX::Plugin::OnEventProxy` |
| Dispatch site | `OnEvent(dupEvent)` rewritten as implicit `this->OnEvent` (line 68) |
| Resolved targets | **23** |

**Pass.** CHA from static type `Plugin` emits **direct** edges to defined plugin `::OnEvent` overrides, including `Plugin::OnEvent` (`plugin.cpp:35`). Five other defined `::OnEvent` methods override `EventHandler`, not `Plugin`, and appear under `EventHandler::OnEventProxy` instead.

**Resolved targets:**

- `OHOS::HiviewDFX::BBoxDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample1::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample2::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample3::OnEvent`
- `OHOS::HiviewDFX::CrashValidator::OnEvent`
- `OHOS::HiviewDFX::DynamicLoadPluginExample::OnEvent`
- `OHOS::HiviewDFX::EventLogger::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample1::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample2::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample3::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample4::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample5::OnEvent`
- `OHOS::HiviewDFX::EventValidator::OnEvent`
- `OHOS::HiviewDFX::FaultDetectorManager::OnEvent`
- `OHOS::HiviewDFX::Faultlogger::OnEvent`
- `OHOS::HiviewDFX::FreezeDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::Plugin::OnEvent`
- `OHOS::HiviewDFX::PluginExample::OnEvent`
- `OHOS::HiviewDFX::PluginProxy::OnEvent`
- `OHOS::HiviewDFX::PrivacyController::OnEvent`
- `OHOS::HiviewDFX::SysEventDispatcher::OnEvent`
- `OHOS::HiviewDFX::SysEventStore::OnEvent`
- `OHOS::HiviewDFX::UsageEventReport::OnEvent`

### 3. `OHOS::HiviewDFX::PipelineEvent::OnContinue` — Pipeline pump

| Field | Value |
|-------|-------|
| File | `base/pipeline.cpp` |
| Line | 34–70 |
| Function | `OHOS::HiviewDFX::PipelineEvent::OnContinue` |
| Dispatch site | `pluginPtr->OnEventProxy` (after `auto pluginPtr = wp.lock()`) |
| Resolved targets | **0** |

**Fail** on the plugin dispatch: `auto` / `lock()` drops the `Plugin` type, so the site has 0 targets. Unqualified recursive `OnContinue()` **does** bind (direct).

**Resolved function-pointer / virtual targets:** none.

### 4. `OHOS::HiviewDFX::PluginFactory::GetPlugin` — Constructor registry

| Field | Value |
|-------|-------|
| File | `base/plugin_factory.cpp` |
| Line | 40–47 |
| Function | `OHOS::HiviewDFX::PluginFactory::GetPlugin` |
| Dispatch site | `info->getPluginObject()` (`std::function` field) |
| Resolved targets | **0** |

Unqualified `GetGlobalPluginInfo` binds (**Pass**). `getPluginObject` has **0** targets: constructors are registered through `std::map`, so no function address reaches this load.

**Resolved function-pointer / virtual targets:** none.

### 5. `OHOS::HiviewDFX::EventLogger::OnEvent` — Plugin body (same-class directs)

| Field | Value |
|-------|-------|
| File | `plugins/eventlogger/event_logger.cpp` |
| Line | 209–209 |
| Function | `OHOS::HiviewDFX::EventLogger::OnEvent` |
| Dispatch site | _no function-pointer site_ |
| Resolved targets | **0** |

**Pass** for same-class / event API directs (`IsValidEventParam`, `GetEventPid`, `UpdateDB`, …). STL / SDK / `Event::DownCastTo` / `ffrt::submit` remain external. No function-pointer dispatch.

**Resolved function-pointer / virtual targets:** none.

### 6. `OHOS::HiviewDFX::SysEventStore::OnEvent` — Event store plugin

| Field | Value |
|-------|-------|
| File | `plugins/event_store/sys_event_store.cpp` |
| Line | 123–160 |
| Function | `OHOS::HiviewDFX::SysEventStore::OnEvent` |
| Dispatch site | _no function-pointer site_ |
| Resolved targets | **0** |

Same-class calls bind. Nested `EventStore::…`, `TriggerExportEngine`, `TimeUtil`, `Parameter::*` stay external. No function-pointer dispatch.

**Resolved function-pointer / virtual targets:** none.

### 7. `inspect calls --from OnEventProxy` — inspect suffix match

| Field | Value |
|-------|-------|
| File | `base/plugin.cpp` |
| Line | 55–83 |
| Function | `inspect calls --from OnEventProxy` |
| Dispatch site | _CLI, not a call site_ |
| Resolved targets | **0** |

**Pass.** Suffix match lists `Plugin::OnEventProxy` and `EventHandler::OnEventProxy`. `--from Get_lugin` is empty (`LIKE` `_` escaped).

**Resolved function-pointer / virtual targets:** none.

### 8. `OHOS::HiviewDFX::PluginProxy::OnEvent` — Smart-pointer field receiver

| Field | Value |
|-------|-------|
| File | `base/plugin_proxy.cpp` |
| Line | 22–30 |
| Function | `OHOS::HiviewDFX::PluginProxy::OnEvent` |
| Dispatch site | `plugin_->OnEvent(event)` (line 28), field `shared_ptr<Plugin> plugin_` |
| Resolved targets | **23** |

**Pass.** Same CHA fan-out as case 2. Fixture: `cpp_smart_ptr_field_receiver_unwraps`.

**Resolved targets:**

- `OHOS::HiviewDFX::BBoxDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample1::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample2::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample3::OnEvent`
- `OHOS::HiviewDFX::CrashValidator::OnEvent`
- `OHOS::HiviewDFX::DynamicLoadPluginExample::OnEvent`
- `OHOS::HiviewDFX::EventLogger::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample1::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample2::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample3::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample4::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample5::OnEvent`
- `OHOS::HiviewDFX::EventValidator::OnEvent`
- `OHOS::HiviewDFX::FaultDetectorManager::OnEvent`
- `OHOS::HiviewDFX::Faultlogger::OnEvent`
- `OHOS::HiviewDFX::FreezeDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::Plugin::OnEvent`
- `OHOS::HiviewDFX::PluginExample::OnEvent`
- `OHOS::HiviewDFX::PluginProxy::OnEvent`
- `OHOS::HiviewDFX::PrivacyController::OnEvent`
- `OHOS::HiviewDFX::SysEventDispatcher::OnEvent`
- `OHOS::HiviewDFX::SysEventStore::OnEvent`
- `OHOS::HiviewDFX::UsageEventReport::OnEvent`

### 9. `OHOS::HiviewDFX::Plugin::DelayProcessEvent` — `std::bind` onto the work loop

| Field | Value |
|-------|-------|
| File | `base/plugin.cpp` |
| Line | 85–96 |
| Function | `OHOS::HiviewDFX::Plugin::DelayProcessEvent` |
| Dispatch site | `std::bind(&Plugin::OnEventProxy, this, event)` (line 93) |
| Resolved targets | **0** |

**Fail.** `std::bind` is external; no edge to `OnEventProxy`. `AddTimerEvent` is direct (`EventLoop` / `MockEventLoop`).

**Resolved function-pointer / virtual targets:** none.

### 10. `OHOS::HiviewDFX::EventLoop::ProcessEvent` — Packed vs typed handler

| Field | Value |
|-------|-------|
| File | `base/event_loop.cpp` |
| Line | 492–510 |
| Function | `OHOS::HiviewDFX::EventLoop::ProcessEvent` |
| Dispatch site | `event.handler->OnEventProxy` (line 498); `event->task` (496); `event->packagedTask` (504) |
| Resolved targets | **2** |

**Partial.** Typed handler CHA **Pass** (targets below). `event->task()` and `packagedTask` have **0** targets.

**Resolved targets:**

- `OHOS::HiviewDFX::EventHandler::OnEventProxy`
- `OHOS::HiviewDFX::Plugin::OnEventProxy`

### 11. `OHOS::HiviewDFX::Event::DownCastTo` — Template `DownCastTo<SysEvent>`

| Field | Value |
|-------|-------|
| File | `base/include/event.h` |
| Line | 201–205 |
| Function | `OHOS::HiviewDFX::Event::DownCastTo` |
| Dispatch site | 13 call sites (all external `Event::DownCastTo`) |
| Resolved targets | **0** |

**Fail.** Name-stripping does not instantiate the template, so the result is not typed as `SysEvent`.

**Resolved function-pointer / virtual targets:** none.

### 12. `ffrt::submit` — `ffrt::submit` deferred lambdas

| Field | Value |
|-------|-------|
| File | `plugins/ (e.g. passthrough_monitor.cpp:80)` |
| Line | 80–80 |
| Function | `ffrt::submit` |
| Dispatch site | 34 `ffrt::submit` sites (all external) |
| Resolved targets | **0** |

**Fail.** 357 `$lambda` functions exist; 7 have in-edges, none from `ffrt::submit`.

**Resolved function-pointer / virtual targets:** none.

### 13. `OHOS::HiviewDFX::UCollectUtil::GraphicMemoryCollectorImpl::GetGraphicUsage` — `dlopen` / `dlsym`

| Field | Value |
|-------|-------|
| File | `plugins/unified_collector/graphic_memory_collector_impl.cpp` |
| Line | 47–59 |
| Function | `OHOS::HiviewDFX::UCollectUtil::GraphicMemoryCollectorImpl::GetGraphicUsage` |
| Dispatch site | `dlsym(handler, GET_INSTANCE)` with name `"GetInstance"` |
| Resolved targets | **0** |

**Fail** for in-tree callees. The `dlsym` model is wired (1 PAG `dlsym` edge) but exact-name lookup is `"GetInstance"` while the export is stored as `OHOS::HiviewDFX::UCollectUtil::GetInstance`. `CallDllFunc` / `GetSymbol` pass `std::string::c_str()`, not a folded constant.

**Resolved function-pointer / virtual targets:** none.

### 14. `OHOS::HiviewDFX::Plugin::OnEvent` — Out-of-line `Plugin::OnEvent` body

| Field | Value |
|-------|-------|
| File | `base/plugin.cpp` |
| Line | 35–38 |
| Function | `OHOS::HiviewDFX::Plugin::OnEvent` |
| Dispatch site | _definition presence, not a dispatch site_ |
| Resolved targets | **0** |

**Pass.** `is_defined=1`. Predefined empty `__UNUSED` keeps the body. Participates in cases 2 and 8.

**Resolved function-pointer / virtual targets:** none.

### 15. `OHOS::HiviewDFX::EventHandler::OnEventProxy` — EventHandler CHA

| Field | Value |
|-------|-------|
| File | `base/include/event.h` |
| Line | 230–233 |
| Function | `OHOS::HiviewDFX::EventHandler::OnEventProxy` |
| Dispatch site | `OnEvent(event)` (line 232) |
| Resolved targets | **27** |

CHA from static type `EventHandler`. The 23 plugin `::OnEvent` names from case 2 plus four handlers that override `EventHandler` but not `Plugin`: `EventHandler::OnEvent`, `OverheadCalculateEventHandler`, `RealEventHandler`, `TestEventHandler`. Complete vs defined `::OnEvent` bodies under those two bases.

**Resolved targets:**

- `OHOS::HiviewDFX::BBoxDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample1::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample2::OnEvent`
- `OHOS::HiviewDFX::BundlePluginExample3::OnEvent`
- `OHOS::HiviewDFX::CrashValidator::OnEvent`
- `OHOS::HiviewDFX::DynamicLoadPluginExample::OnEvent`
- `OHOS::HiviewDFX::EventHandler::OnEvent`
- `OHOS::HiviewDFX::EventLogger::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample1::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample2::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample3::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample4::OnEvent`
- `OHOS::HiviewDFX::EventProcessorExample5::OnEvent`
- `OHOS::HiviewDFX::EventValidator::OnEvent`
- `OHOS::HiviewDFX::FaultDetectorManager::OnEvent`
- `OHOS::HiviewDFX::Faultlogger::OnEvent`
- `OHOS::HiviewDFX::FreezeDetectorPlugin::OnEvent`
- `OHOS::HiviewDFX::OverheadCalculateEventHandler::OnEvent`
- `OHOS::HiviewDFX::Plugin::OnEvent`
- `OHOS::HiviewDFX::PluginExample::OnEvent`
- `OHOS::HiviewDFX::PluginProxy::OnEvent`
- `OHOS::HiviewDFX::PrivacyController::OnEvent`
- `OHOS::HiviewDFX::RealEventHandler::OnEvent`
- `OHOS::HiviewDFX::SysEventDispatcher::OnEvent`
- `OHOS::HiviewDFX::SysEventStore::OnEvent`
- `OHOS::HiviewDFX::TestEventHandler::OnEvent`
- `OHOS::HiviewDFX::UsageEventReport::OnEvent`

### 16. `OHOS::HiviewDFX::PluginProxy::CanProcessEvent` — Field receiver CHA

| Field | Value |
|-------|-------|
| File | `base/plugin_proxy.cpp` |
| Line | 33–40 |
| Function | `OHOS::HiviewDFX::PluginProxy::CanProcessEvent` |
| Dispatch site | `plugin_->CanProcessEvent(event)` (line 39) |
| Resolved targets | **12** |

All 11 `Plugin::CanProcessEvent` overrides in tree plus the proxy itself. `Pipeline::CanProcessEvent` takes `PipelineEvent` and is a different function. Complete vs source.

**Resolved targets:**

- `OHOS::HiviewDFX::BundlePluginExample1::CanProcessEvent`
- `OHOS::HiviewDFX::BundlePluginExample2::CanProcessEvent`
- `OHOS::HiviewDFX::BundlePluginExample3::CanProcessEvent`
- `OHOS::HiviewDFX::EventProcessorExample1::CanProcessEvent`
- `OHOS::HiviewDFX::EventProcessorExample2::CanProcessEvent`
- `OHOS::HiviewDFX::EventProcessorExample3::CanProcessEvent`
- `OHOS::HiviewDFX::EventProcessorExample4::CanProcessEvent`
- `OHOS::HiviewDFX::EventProcessorExample5::CanProcessEvent`
- `OHOS::HiviewDFX::Faultlogger::CanProcessEvent`
- `OHOS::HiviewDFX::FreezeDetectorPlugin::CanProcessEvent`
- `OHOS::HiviewDFX::Plugin::CanProcessEvent`
- `OHOS::HiviewDFX::PluginProxy::CanProcessEvent`

### 17. `OHOS::HiviewDFX::PluginProxy::OnEventListeningCallback` — Listener CHA

| Field | Value |
|-------|-------|
| File | `base/plugin_proxy.cpp` |
| Line | 75–83 |
| Function | `OHOS::HiviewDFX::PluginProxy::OnEventListeningCallback` |
| Dispatch site | `plugin_->OnEventListeningCallback(msg)` (line 81) |
| Resolved targets | **8** |

**Complete for this analysis config.** Eight overrides remain after preprocess. `UnifiedCollector::OnEventListeningCallback` exists in source (`unified_collector.h:36`, `unified_collector.cpp:201`) but only under `#ifdef UNIFIED_COLLECTOR_TRACE_ENABLE`. GN sets that define when `hiview_unified_collector_trace_enable` is on (`plugins/unified_collector/BUILD.gn`). `trace analyze` does not pass OpenHarmony product flags, so the preprocessor strips both the declaration and the body. CHA then only sees `Plugin`'s empty default in `plugin.h`. Not a CHA miss: this run is the trace-disabled variant of the plugin. Passing `-D UNIFIED_COLLECTOR_TRACE_ENABLE` would include the override.

**Resolved targets:**

- `OHOS::HiviewDFX::BundlePluginExample3::OnEventListeningCallback`
- `OHOS::HiviewDFX::EventProcessorExample4::OnEventListeningCallback`
- `OHOS::HiviewDFX::FaultDetectorManager::OnEventListeningCallback`
- `OHOS::HiviewDFX::Faultlogger::OnEventListeningCallback`
- `OHOS::HiviewDFX::FreezeDetectorPlugin::OnEventListeningCallback`
- `OHOS::HiviewDFX::Plugin::OnEventListeningCallback`
- `OHOS::HiviewDFX::PluginProxy::OnEventListeningCallback`
- `OHOS::HiviewDFX::XperfPlugin::OnEventListeningCallback`

### 18. `OHOS::HiviewDFX::EventLoop::ProcessEvent` — Handler OnEventProxy

| Field | Value |
|-------|-------|
| File | `base/event_loop.cpp` |
| Line | 492–511 |
| Function | `OHOS::HiviewDFX::EventLoop::ProcessEvent` |
| Dispatch site | `event.handler->OnEventProxy(event.event)` (line 498) |
| Resolved targets | **2** |

Typed `EventHandler*` receiver CHA to the two `OnEventProxy` implementations (`EventHandler` inline, `Plugin` override). Fan-out from those proxies to `::OnEvent` is cases 2 and 15. (`event.task()` / packaged tasks remain unresolved — case 10.)

**Resolved targets:**

- `OHOS::HiviewDFX::EventHandler::OnEventProxy`
- `OHOS::HiviewDFX::Plugin::OnEventProxy`

### 19. `OHOS::HiviewDFX::PluginProxy::GetHandlerInfo` — Field receiver

| Field | Value |
|-------|-------|
| File | `base/plugin_proxy.cpp` |
| Line | 55–65 |
| Function | `OHOS::HiviewDFX::PluginProxy::GetHandlerInfo` |
| Dispatch site | `plugin_->GetHandlerInfo()` (line 62) |
| Resolved targets | **2** |

Only `Plugin` and `PluginProxy` define `GetHandlerInfo` in this tree. Complete vs source.

**Resolved targets:**

- `OHOS::HiviewDFX::Plugin::GetHandlerInfo`
- `OHOS::HiviewDFX::PluginProxy::GetHandlerInfo`

---

# 3. Camera and clang/test

Hang / stack-overflow checks, not dispatch-hub evals. PCH-style header IR is what lets these trees finish: camera previously hung in preprocess (diamond includes); `clang/test/Sema/deep_recursion.c` overflowed a rayon worker (now 16 MiB stacks + AST walk cap 512).

## `multimedia_camera_framework`

**Path:** `~/multimedia_camera_framework`

### Performance

| Step | Time |
|------|-----:|
| Index | 8.8s |
| Analyze | 0.3s |
| Export | 1.5s |
| **Wall** | **9.5s** |

| Metric | Value |
|--------|------:|
| Files | 1,593 |
| Functions | 22,289 (15,673 defined / 6,616 external) |
| Call edges | 43,312 |
| Direct / indirect / external | 12,644 / **105** / 30,563 |
| Arg-flow edges | 10,520 |
| Parse warnings | 776 |

Completes. The 105 indirect edges are almost all fuzzer `FuzzedDataProvider` calls; production dispatch is recovered as **direct** CHA. Five verified production cases follow. The lower totals vs the previous snapshot are the cumulative effect of the `param_type_ids` merge fix (prototype/definition collapse) plus the parallel-index nondeterminism band (±29 functions observed), not a resolution loss — every case target above is unchanged.

### Cases

### 1. `OHOS::CameraStandard::DeferredProcessing::Command::Do` — `Executing`

| Field | Value |
|-------|-------|
| File | `services/deferred_processing_service/src/base/command_server/command.cpp` |
| Line | 33–42 |
| Function | `OHOS::CameraStandard::DeferredProcessing::Command::Do` |
| Dispatch site | `Executing()` (line 37) |
| Resolved targets | **30** (29 defined overrides + pure-virtual `Command::Executing` external) |

`Executing` is pure virtual on `Command`. Source has 29 out-of-line overrides; all 29 resolve. `ServiceDiedCommand` has no `Executing` body (abstract) and is correctly absent.

**Resolved targets:**

- `OHOS::CameraStandard::DeferredProcessing::AddPhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::AddPhotoSessionCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::AddVideoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::AddVideoSessionCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::CancelProcessPhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::CancelProcessVideoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::Command::Executing`
- `OHOS::CameraStandard::DeferredProcessing::DeletePhotoSessionCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::DeleteVideoSessionCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::EventStatusChangeCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::NotifyJobChangedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::NotifyVideoJobChangedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::PhotoDiedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessFailedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessSuccessCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessTimeOutCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::PhotoSyncCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::ProcessCachePhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::ProcessPhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::ProcessVideoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::RemovePhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::RemoveVideoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::RestorePhotoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::RestoreVideoCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoDiedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessFailedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessSuccessCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessTimeOutCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoStateChangedCommand::Executing`
- `OHOS::CameraStandard::DeferredProcessing::VideoSyncCommand::Executing`

### 2. `OHOS::CameraStandard::DeferredProcessing::Command::Do` — `GetCommandName`

| Field | Value |
|-------|-------|
| File | `services/deferred_processing_service/src/base/command_server/command.cpp` |
| Line | 33–42 |
| Function | `OHOS::CameraStandard::DeferredProcessing::Command::Do` |
| Dispatch site | `GetCommandName()` (line 35) |
| Resolved targets | **31** |

`DECLARE_COMMAND` inline overrides plus `ServiceDiedCommand` (name only) and the pure-virtual `Command::GetCommandName` external. Matches every command class that exists in tree.

**Resolved targets:**

- `OHOS::CameraStandard::DeferredProcessing::AddPhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::AddPhotoSessionCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::AddVideoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::AddVideoSessionCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::CancelProcessPhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::CancelProcessVideoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::Command::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::DeletePhotoSessionCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::DeleteVideoSessionCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::EventStatusChangeCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::NotifyJobChangedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::NotifyVideoJobChangedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::PhotoDiedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessFailedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessSuccessCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::PhotoProcessTimeOutCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::PhotoSyncCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::ProcessCachePhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::ProcessPhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::ProcessVideoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::RemovePhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::RemoveVideoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::RestorePhotoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::RestoreVideoCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::ServiceDiedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoDiedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessFailedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessSuccessCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoProcessTimeOutCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoStateChangedCommand::GetCommandName`
- `OHOS::CameraStandard::DeferredProcessing::VideoSyncCommand::GetCommandName`

### 3. `OHOS::CameraStandard::CFilter::PrepareDone` — `DoPrepare`

| Field | Value |
|-------|-------|
| File | `mediastream/src/filter/cfilter.cpp` |
| Line | ~101 |
| Function | `OHOS::CameraStandard::CFilter::PrepareDone` |
| Dispatch site | `DoPrepare()` |
| Resolved targets | **18** |

All 17 `CFilter` subclasses that define `DoPrepare` plus the base. Complete vs `::DoPrepare(` in `mediastream/src/filter/`.

**Resolved targets:**

- `OHOS::CameraStandard::AudioCacheFilter::DoPrepare`
- `OHOS::CameraStandard::AudioCaptureFilter::DoPrepare`
- `OHOS::CameraStandard::AudioEncoderFilter::DoPrepare`
- `OHOS::CameraStandard::AudioForkFilter::DoPrepare`
- `OHOS::CameraStandard::AudioProcessFilter::DoPrepare`
- `OHOS::CameraStandard::CFilter::DoPrepare`
- `OHOS::CameraStandard::CinematicVideoCacheFilter::DoPrepare`
- `OHOS::CameraStandard::DemuxerFilter::DoPrepare`
- `OHOS::CameraStandard::ImageEffectFilter::DoPrepare`
- `OHOS::CameraStandard::MetaCacheFilter::DoPrepare`
- `OHOS::CameraStandard::MetaDataFilter::DoPrepare`
- `OHOS::CameraStandard::MovingPhotoAudioEncoderFilter::DoPrepare`
- `OHOS::CameraStandard::MovingPhotoMuxerFilter::DoPrepare`
- `OHOS::CameraStandard::MovingPhotoVideoEncoderFilter::DoPrepare`
- `OHOS::CameraStandard::MuxerFilter::DoPrepare`
- `OHOS::CameraStandard::SinkFilter::DoPrepare`
- `OHOS::CameraStandard::VideoCacheFilter::DoPrepare`
- `OHOS::CameraStandard::VideoEncoderFilter::DoPrepare`

### 4. `OHOS::CameraStandard::Pipeline::LinkFilters` — `LinkNext`

| Field | Value |
|-------|-------|
| File | `mediastream/src/pipeline/pipeline.cpp` |
| Line | ~243 |
| Function | `OHOS::CameraStandard::Pipeline::LinkFilters` |
| Dispatch site | `LinkNext` |
| Resolved targets | **18** |

Same 17 filter subclasses plus `CFilter::LinkNext`. Complete vs `::LinkNext(` in `mediastream/src/filter/`.

**Resolved targets:**

- `OHOS::CameraStandard::AudioCacheFilter::LinkNext`
- `OHOS::CameraStandard::AudioCaptureFilter::LinkNext`
- `OHOS::CameraStandard::AudioEncoderFilter::LinkNext`
- `OHOS::CameraStandard::AudioForkFilter::LinkNext`
- `OHOS::CameraStandard::AudioProcessFilter::LinkNext`
- `OHOS::CameraStandard::CFilter::LinkNext`
- `OHOS::CameraStandard::CinematicVideoCacheFilter::LinkNext`
- `OHOS::CameraStandard::DemuxerFilter::LinkNext`
- `OHOS::CameraStandard::ImageEffectFilter::LinkNext`
- `OHOS::CameraStandard::MetaCacheFilter::LinkNext`
- `OHOS::CameraStandard::MetaDataFilter::LinkNext`
- `OHOS::CameraStandard::MovingPhotoAudioEncoderFilter::LinkNext`
- `OHOS::CameraStandard::MovingPhotoMuxerFilter::LinkNext`
- `OHOS::CameraStandard::MovingPhotoVideoEncoderFilter::LinkNext`
- `OHOS::CameraStandard::MuxerFilter::LinkNext`
- `OHOS::CameraStandard::SinkFilter::LinkNext`
- `OHOS::CameraStandard::VideoCacheFilter::LinkNext`
- `OHOS::CameraStandard::VideoEncoderFilter::LinkNext`

### 5. `OHOS::CameraStandard::CaptureSession::AddOutput` — `CanAddOutput`

| Field | Value |
|-------|-------|
| File | `frameworks/native/camera/base/src/session/capture_session.cpp` |
| Line | ~1272 |
| Function | `OHOS::CameraStandard::CaptureSession::AddOutput` |
| Dispatch site | `CanAddOutput` |
| Resolved targets | **18** |

Base `CaptureSession::CanAddOutput` plus every session subclass that overrides it in `frameworks/native/camera/`. `CaptureSessionForSys` has no override (stitching calls it through inheritance). Complete vs `::CanAddOutput(` definitions.

**Resolved targets:**

- `OHOS::CameraStandard::ApertureVideoSession::CanAddOutput`
- `OHOS::CameraStandard::CaptureSession::CanAddOutput`
- `OHOS::CameraStandard::FluorescencePhotoSession::CanAddOutput`
- `OHOS::CameraStandard::HighResPhotoSession::CanAddOutput`
- `OHOS::CameraStandard::MacroPhotoSession::CanAddOutput`
- `OHOS::CameraStandard::MacroVideoSession::CanAddOutput`
- `OHOS::CameraStandard::NightSession::CanAddOutput`
- `OHOS::CameraStandard::PanoramaSession::CanAddOutput`
- `OHOS::CameraStandard::PhotoSession::CanAddOutput`
- `OHOS::CameraStandard::PhotoSessionForSys::CanAddOutput`
- `OHOS::CameraStandard::PortraitSession::CanAddOutput`
- `OHOS::CameraStandard::ProfessionSession::CanAddOutput`
- `OHOS::CameraStandard::QuickShotPhotoSession::CanAddOutput`
- `OHOS::CameraStandard::ScanSession::CanAddOutput`
- `OHOS::CameraStandard::SlowMotionSession::CanAddOutput`
- `OHOS::CameraStandard::StitchingPhotoSession::CanAddOutput`
- `OHOS::CameraStandard::VideoSession::CanAddOutput`
- `OHOS::CameraStandard::VideoSessionForSys::CanAddOutput`

## clang/test (llvm-project)

`--jobs 8`, `--timeout-secs 180`. Check: no hang, no stack overflow.

| Subtree | TUs | Index | Analyze | Export | Result |
|---------|----:|------:|--------:|-------:|--------|
| `Preprocessor` | 371 | 1.0s | 0.0s | 0.1s | completes |
| `Lexer` | 138 | 0.2s | 0.0s | 0.0s | completes |
| `Parser` | 325 | 1.4s | 0.0s | 0.2s | completes |
| `CXX` | 918 | 0.5s | 0.0s | 0.1s | completes |
| `Sema` | 1,379 | 3.7s | 0.1s | 0.4s | completes (includes `deep_recursion.c`) |

---

# Appendix — Re-runnable regression checks

`scripts/eval_check.py` re-analyzes the three corpora fresh and asserts:

1. **Global metrics** — files, functions (defined/external), call edges by resolution
   (direct/indirect/external), arg-flow, diagnostics, `dlsym` PAG edges. Diagnostics,
   `dlsym`, and **indirect** edges must match **exactly** (they are correctness
   invariants); bulk function/edge/arg-flow totals use tolerance bands because the
   parallel index drifts a little run-to-run.
2. **Dispatch-site checks (exact, name-based)** — the 12 HDF hubs
   (`DeviceNodeExtDispatch` 73 … `WorkEntry` 20, linux `osal_workqueue.c`), the 7
   hiview CHA/fn-ptr sites (`Plugin::OnEventProxy`→23 … `GetHandlerInfo`→2 at line 62),
   and the 5 camera cases (`Command::Do` 31/30, `CFilter::PrepareDone` 18,
   `Pipeline::LinkFilters` 18, `CaptureSession::AddOutput` 18). These are the
   eval-report correctness numbers, guarded against silent drift.
3. **C++-slice production probes** — defined overload groups split by scalar type
   (hiview ≥120, camera ≥240), template member call sites that carry resolution
   records (camera ≥15 distinct `…<…>` callee texts), and a **calibration probe**:
   external-class template sites (`MetaHdr` `Set<Tag>`/`Get<Tag>`) must stay
   unresolved (~1,178) instead of degrading into noise edges.

```bash
python3 scripts/eval_check.py                 # all three corpora, 800k pops, --jobs 8
python3 scripts/eval_check.py hdf camera      # subset
```

Exit 0 = all checks pass (current: **64 checks, 0 failures**).
