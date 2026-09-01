# Parse failures — eval corpora

Files that fail tree-sitter parsing during `trace analyze`. Regenerate TSV with:

```bash
trace analyze <ROOT> -o /tmp/out.db --jobs 8
cargo run -p trace-cli --release --example parse_failures -- <ROOT> --from-db /tmp/out.db > /tmp/parse_failures.tsv
python3 scripts/gen_parse_failures_report.py
```

## Overview

| Corpus | Root | Failing files | Top category |
|--------|------|--------------:|--------------|
| `drivers_hdf_core` | `/private/tmp/corpora/drivers_hdf_core` | 291 | generic ERROR nodes (mixed C++ constructs) |
| `hiviewdfx_hiview` | `/private/tmp/corpora/hiviewdfx_hiview` | 344 | generic ERROR nodes (mixed C++ constructs) |
| `multimedia_camera_framework` | `/private/tmp/corpora/multimedia_camera_framework` | 695 | generic ERROR nodes (mixed C++ constructs) |

## Cross-corpus category totals

| Category | HDF | Hiview | Camera | Total |
|----------|----:|-------:|-------:|------:|
| generic ERROR nodes (mixed C++ constructs) | 288 | 279 | 662 | 1229 |
| other / mixed | 0 | 54 | 1 | 55 |
| missing type identifiers (often macro-expanded types) | 1 | 7 | 17 | 25 |
| gtest/HWTEST macros (`missing ;`) | 2 | 2 | 15 | 19 |
| extern template instantiations | 0 | 2 | 0 | 2 |

## drivers_hdf_core

Generated from `trace analyze /private/tmp/corpora/drivers_hdf_core` (291 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 291

### Failure categories

| Category | Files |
|----------|------:|
| generic ERROR nodes (mixed C++ constructs) | 288 |
| gtest/HWTEST macros (`missing ;`) | 2 |
| missing type identifiers (often macro-expanded types) | 1 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `adapter/khdf/hongmeng/osal/src/osal_cdev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 2 | `adapter/khdf/hongmeng/osal/src/osal_thread.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 3 | `adapter/khdf/linux/manager/src/hdf_kevent.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 4 | `adapter/khdf/linux/model/camera/src/contig_dma.c` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 5 | `adapter/khdf/linux/model/camera/src/sg_dma.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 6 | `adapter/khdf/linux/model/camera/src/virtual_malloc.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 7 | `adapter/khdf/linux/model/storage/emmc_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 8 | `adapter/khdf/linux/model/storage/sdio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 9 | `adapter/khdf/linux/model/usb/host/src/usb_net_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 150 |
| 10 | `adapter/khdf/linux/model/usb/host/src/usb_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 11 | `adapter/khdf/linux/platform/adc/adc_iio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 12 | `adapter/khdf/linux/platform/clock/clock_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 13 | `adapter/khdf/linux/platform/fwk/platform_trace.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 14 | `adapter/khdf/linux/platform/gpio/gpio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 15 | `adapter/khdf/linux/platform/i2c/i2c_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 16 | `adapter/khdf/linux/platform/mipi_csi/mipi_csi_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 17 | `adapter/khdf/linux/platform/mipi_csi/mipi_v4l2_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 18 | `adapter/khdf/linux/platform/mipi_dsi/mipi_drm_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 19 | `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 20 | `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_hi35xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 21 | `adapter/khdf/linux/platform/pwm/pwm_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 22 | `adapter/khdf/linux/platform/pwm/pwm_hi35xx_linux.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 23 | `adapter/khdf/linux/platform/regulator/regulator_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 24 | `adapter/khdf/linux/platform/rtc/rtc_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 25 | `adapter/khdf/linux/platform/spi/spi_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 26 | `adapter/khdf/linux/platform/watchdog/watchdog_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 27 | `adapter/khdf/linux/test/platform/i2c/i2c_adapter_dummy.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 28 | `adapter/khdf/liteos/model/storage/src/mmc/mmc_block_lite.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 29 | `adapter/khdf/liteos/model/storage/src/mtd/mtd_block_lite.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 30 | `adapter/khdf/liteos/model/storage/src/mtd/mtd_char_lite.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 31 | `adapter/khdf/liteos/model/usb/host/src/usb_pnp_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 32 | `adapter/khdf/liteos/model/usb/host/src/usb_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 33 | `adapter/khdf/liteos/model/usb/host/src/usb_test_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 34 | `adapter/khdf/liteos/osal/src/osal_firmware.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 35 | `adapter/khdf/liteos/platform/include/gpio_dev.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 36 | `adapter/khdf/liteos/platform/src/gpio_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 37 | `adapter/khdf/liteos/platform/src/i2c_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 38 | `adapter/khdf/liteos/platform/src/platform_trace.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 39 | `adapter/khdf/liteos/platform/src/spi_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 40 | `adapter/khdf/liteos/platform/src/uart_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 41 | `adapter/khdf/liteos_m/test/sample_driver/src/sample_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 42 | `adapter/khdf/uniproton/test/sample_driver/src/platform_device_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 99 |
| 43 | `adapter/khdf/uniproton/test/sample_driver/src/platform_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 44 | `adapter/khdf/uniproton/test/sample_driver/src/platform_manager_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 45 | `adapter/platform/can/can_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 46 | `adapter/platform/gpio/gpio_asr.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 47 | `adapter/platform/gpio/gpio_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 48 | `adapter/platform/gpio/gpio_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 49 | `adapter/platform/gpio/gpio_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 50 | `adapter/platform/i2c/i2c_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 51 | `adapter/platform/i2c/i2c_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 52 | `adapter/platform/i2c/i2c_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 53 | `adapter/platform/mipi_dsi/mipi_drm_imx8mm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 54 | `adapter/platform/pwm/pwm_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 55 | `adapter/platform/pwm/pwm_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 56 | `adapter/platform/pwm/pwm_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 57 | `adapter/platform/spi/spi_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 58 | `adapter/platform/spi/spi_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 59 | `adapter/platform/spi/spi_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 60 | `adapter/platform/uart/uart_asr.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 61 | `adapter/platform/uart/uart_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 62 | `adapter/platform/uart/uart_gr5xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 63 | `adapter/platform/uart/uart_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 64 | `adapter/platform/uart/uart_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 65 | `adapter/platform/watchdog/watchdog_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 66 | `adapter/platform/watchdog/watchdog_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 67 | `adapter/platform/watchdog/watchdog_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 68 | `adapter/uhdf2/hdi/src/idevmgr_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 69 | `adapter/uhdf2/hdi/src/servstat_listener.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 70 | `adapter/uhdf2/hdi/src/servstat_listener_stub.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 71 | `adapter/uhdf2/hdi/src/stub_collector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 72 | `adapter/uhdf2/hdi/test/buffer_handle/native_buffer_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 73 | `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 74 | `adapter/uhdf2/host/devhost.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 75 | `adapter/uhdf2/host/src/devhost_service_full.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 76 | `adapter/uhdf2/host/src/device_service_stub.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 77 | `adapter/uhdf2/host/src/devsvc_manager_proxy.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 78 | `adapter/uhdf2/host/src/hdf_device_thread.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 79 | `adapter/uhdf2/host/test/unittest/sample1_driver/sample1_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 80 | `adapter/uhdf2/host/test/unittest/sample_driver/sample_driver.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 81 | `adapter/uhdf2/ipc/src/hdf_remote_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 82 | `adapter/uhdf2/manager/src/devmgr_service_full.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 83 | `adapter/uhdf2/manager/src/devmgr_uevent.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 84 | `adapter/uhdf2/manager/src/devsvc_manager_stub.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 85 | `adapter/uhdf2/osal/test/unittest/common/sample_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 86 | `adapter/uhdf2/security/src/hdf_security.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 87 | `adapter/uhdf2/shared/src/dev_attribute_serialize.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 88 | `framework/core/adapter/syscall/src/hdf_syscall_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 89 | `framework/core/adapter/vnode/src/hdf_vnode_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 90 | `framework/core/common/src/hdf_attribute_macro.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 91 | `framework/core/host/src/devhost_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 92 | `framework/core/host/src/hdf_device_node.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 93 | `framework/core/host/src/hdf_device_object.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 94 | `framework/core/manager/src/devhost_service_clnt.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 95 | `framework/core/manager/src/devmgr_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 96 | `framework/core/manager/src/devsvc_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 97 | `framework/core/manager/test/unittest/common/hdf_ioservice_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 98 | `framework/core/manager/test/unittest/common/hdf_sbuf_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 99 | `framework/model/audio/core/src/audio_host.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 100 | `framework/model/audio/dispatch/src/audio_control_dispatch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 101 | `framework/model/audio/dispatch/src/audio_stream_dispatch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 102 | `framework/model/audio/hdmi/src/audio_hdmi_codec_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 103 | `framework/model/audio/usb/src/audio_usb_codec_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 104 | `framework/model/audio/usb/src/audio_usb_dma_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 105 | `framework/model/audio/usb/src/audio_usb_endpoints.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 106 | `framework/model/audio/usb/src/audio_usb_mixer.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 107 | `framework/model/camera/parser/src/camera_config_parser.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 72 |
| 108 | `framework/model/camera/utils/src/camera_utils.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 109 | `framework/model/display/driver/adapter_soc/hi35xx_disp.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 110 | `framework/model/display/driver/backlight/hdf_bl.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 111 | `framework/model/display/driver/backlight/pwm_bl.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 112 | `framework/model/display/driver/hdf_disp.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 113 | `framework/model/display/driver/hdf_drm_panel.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 114 | `framework/model/display/driver/lcdkit/lcdkit_parse_config.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 44 |
| 115 | `framework/model/display/driver/lcdkit/lite_lcdkit.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 116 | `framework/model/display/driver/panel/ili9881_st_5p5.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 117 | `framework/model/display/driver/panel/ili9881c_boe.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 118 | `framework/model/display/driver/panel/mipi_icn9700.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 119 | `framework/model/display/driver/panel/ssp_st7789.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 120 | `framework/model/input/driver/hdf_encoder.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 121 | `framework/model/input/driver/hdf_encoder.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 122 | `framework/model/input/driver/hdf_hid_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 123 | `framework/model/input/driver/hdf_infrared.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 124 | `framework/model/input/driver/hdf_input_device_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 125 | `framework/model/input/driver/hdf_key.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 126 | `framework/model/input/driver/hdf_touch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 127 | `framework/model/input/driver/input_config_parser.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 65 |
| 128 | `framework/model/input/driver/touchscreen/touch_ft5406.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 129 | `framework/model/input/driver/touchscreen/touch_ft5x06.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 130 | `framework/model/input/driver/touchscreen/touch_ft6336.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 131 | `framework/model/input/driver/touchscreen/touch_gt911.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 132 | `framework/model/misc/dsoftbus/src/hdf_dsoftbus_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 133 | `framework/model/misc/dsoftbus/src/module_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 134 | `framework/model/misc/light/driver/src/light_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 135 | `framework/model/misc/vibrator/driver/src/vibrator_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 136 | `framework/model/misc/vibrator/driver/src/vibrator_haptic.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 137 | `framework/model/misc/vibrator/driver/src/vibrator_parser.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 138 | `framework/model/network/ethernet/src/hdf_eth_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 139 | `framework/model/network/wifi/core/components/p2p/p2p.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 140 | `framework/model/network/wifi/core/components/softap/ap.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 141 | `framework/model/network/wifi/core/components/sta/sta.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 30 |
| 142 | `framework/model/network/wifi/core/hdf_wifi_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 143 | `framework/model/network/wifi/core/module/wifi_base.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 53 |
| 144 | `framework/model/network/wifi/platform/src/message/message_dispatcher.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 145 | `framework/model/network/wifi/platform/src/message/sidecar.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 146 | `framework/model/sensor/driver/accel/sensor_accel_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 147 | `framework/model/sensor/driver/accel/sensor_gravity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 148 | `framework/model/sensor/driver/als/sensor_als_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 149 | `framework/model/sensor/driver/barometer/sensor_barometer_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 150 | `framework/model/sensor/driver/common/src/sensor_config_parser.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 151 | `framework/model/sensor/driver/common/src/sensor_device_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 152 | `framework/model/sensor/driver/gas/sensor_gas_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 153 | `framework/model/sensor/driver/gyro/sensor_gyro_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 154 | `framework/model/sensor/driver/hall/sensor_hall_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 155 | `framework/model/sensor/driver/humidity/sensor_humidity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 156 | `framework/model/sensor/driver/magnetic/sensor_magnetic_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 157 | `framework/model/sensor/driver/pedometer/sensor_pedometer_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 158 | `framework/model/sensor/driver/ppg/sensor_ppg_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 159 | `framework/model/sensor/driver/proximity/sensor_proximity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 160 | `framework/model/sensor/driver/temperature/sensor_temperature_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 161 | `framework/model/storage/src/mmc/mmc_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 162 | `framework/model/storage/src/mmc/mmc_protocol.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 163 | `framework/model/storage/src/mmc/mmc_sdio.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 164 | `framework/model/storage/src/mtd/mtd_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 165 | `framework/model/storage/src/mtd/mtd_spi_common.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 166 | `framework/sample/platform/uart/src/uart_sample.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 167 | `framework/support/platform/include/fwk/platform_device.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 168 | `framework/support/platform/include/fwk/platform_errno.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 169 | `framework/support/platform/include/fwk/platform_trace.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 170 | `framework/support/platform/src/adc/adc_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 171 | `framework/support/platform/src/can/can_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 172 | `framework/support/platform/src/can/can_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 173 | `framework/support/platform/src/clock/clock_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 174 | `framework/support/platform/src/dac/dac_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 175 | `framework/support/platform/src/dma/dmac_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 176 | `framework/support/platform/src/fwk/platform_device.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 177 | `framework/support/platform/src/fwk/platform_dumper.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 178 | `framework/support/platform/src/fwk/platform_listener_u.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 179 | `framework/support/platform/src/fwk/platform_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 180 | `framework/support/platform/src/fwk/platform_queue.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 181 | `framework/support/platform/src/fwk/platform_trace_unopen.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 182 | `framework/support/platform/src/gpio/gpio_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 183 | `framework/support/platform/src/gpio/gpio_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 184 | `framework/support/platform/src/gpio/gpio_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 185 | `framework/support/platform/src/hdmi/hdmi_cec.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 186 | `framework/support/platform/src/hdmi/hdmi_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 187 | `framework/support/platform/src/hdmi/hdmi_dfm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 188 | `framework/support/platform/src/hdmi/hdmi_edid.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 189 | `framework/support/platform/src/hdmi/hdmi_frl.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 190 | `framework/support/platform/src/hdmi/hdmi_infoframe.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 191 | `framework/support/platform/src/hdmi/hdmi_scdc.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 192 | `framework/support/platform/src/i2c/i2c_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 193 | `framework/support/platform/src/i2c/i2c_if.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 194 | `framework/support/platform/src/i2c/i2c_if_u.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 195 | `framework/support/platform/src/i3c/i3c_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 196 | `framework/support/platform/src/i3c/i3c_if.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 197 | `framework/support/platform/src/pin/pin_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 198 | `framework/support/platform/src/pwm/pwm_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 199 | `framework/support/platform/src/regulator/regulator_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 58 |
| 200 | `framework/support/platform/src/regulator/regulator_if.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 201 | `framework/support/platform/src/regulator/regulator_tree_mgr.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 202 | `framework/support/platform/src/rtc/rtc_base.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 203 | `framework/support/platform/src/spi/spi_if_u.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 204 | `framework/support/platform/src/timer/timer_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 205 | `framework/support/platform/src/timer/timer_if_u.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 206 | `framework/support/platform/src/uart/uart_if_u.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 207 | `framework/support/platform/src/uart/uart_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 208 | `framework/test/unittest/common/hdf_main_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 209 | `framework/test/unittest/manager/sample_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 210 | `framework/test/unittest/model/audio/src/audio_host_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 211 | `framework/test/unittest/model/audio/src/hdf_audio_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 212 | `framework/test/unittest/model/network/wifi/unittest/message/hdf_single_node_message_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 213 | `framework/test/unittest/model/network/wifi/unittest/net/hdf_netbuf_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 214 | `framework/test/unittest/model/usb/device/src/hdf_usb_device_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 215 | `framework/test/unittest/model/usb/host/src/usb_raw_io.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 216 | `framework/test/unittest/osal/osal_all_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 217 | `framework/test/unittest/platform/common/adc_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 218 | `framework/test/unittest/platform/common/adc_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 219 | `framework/test/unittest/platform/common/can_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 49 |
| 220 | `framework/test/unittest/platform/common/clock_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 221 | `framework/test/unittest/platform/common/clock_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 222 | `framework/test/unittest/platform/common/dac_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 223 | `framework/test/unittest/platform/common/dac_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 224 | `framework/test/unittest/platform/common/emmc_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 225 | `framework/test/unittest/platform/common/gpio_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 226 | `framework/test/unittest/platform/common/gpio_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 227 | `framework/test/unittest/platform/common/hdmi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 228 | `framework/test/unittest/platform/common/i2c_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 229 | `framework/test/unittest/platform/common/i2c_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 230 | `framework/test/unittest/platform/common/i2s_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 231 | `framework/test/unittest/platform/common/i3c_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 232 | `framework/test/unittest/platform/common/i3c_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 233 | `framework/test/unittest/platform/common/mipi_csi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 234 | `framework/test/unittest/platform/common/mipi_dsi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 235 | `framework/test/unittest/platform/common/pcie_bus_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 236 | `framework/test/unittest/platform/common/pcie_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 237 | `framework/test/unittest/platform/common/pcie_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 238 | `framework/test/unittest/platform/common/pin_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 239 | `framework/test/unittest/platform/common/pin_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 240 | `framework/test/unittest/platform/common/platform_device_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 98 |
| 241 | `framework/test/unittest/platform/common/platform_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 242 | `framework/test/unittest/platform/common/platform_event_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 243 | `framework/test/unittest/platform/common/platform_manager_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 244 | `framework/test/unittest/platform/common/platform_queue_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 245 | `framework/test/unittest/platform/common/pwm_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 246 | `framework/test/unittest/platform/common/pwm_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 247 | `framework/test/unittest/platform/common/regulator_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 248 | `framework/test/unittest/platform/common/rtc_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 249 | `framework/test/unittest/platform/common/rtc_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 250 | `framework/test/unittest/platform/common/sdio_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 251 | `framework/test/unittest/platform/common/spi_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 252 | `framework/test/unittest/platform/common/spi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 253 | `framework/test/unittest/platform/common/timer_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 254 | `framework/test/unittest/platform/common/timer_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 255 | `framework/test/unittest/platform/common/uart_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 256 | `framework/test/unittest/platform/common/uart_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 257 | `framework/test/unittest/platform/common/watchdog_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 258 | `framework/test/unittest/platform/common/watchdog_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 259 | `framework/test/unittest/platform/config/can_test_config.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 260 | `framework/test/unittest/platform/virtual/adc_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 261 | `framework/test/unittest/platform/virtual/clock_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 262 | `framework/test/unittest/platform/virtual/dac_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 263 | `framework/test/unittest/platform/virtual/i3c_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 264 | `framework/test/unittest/platform/virtual/pcie_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 265 | `framework/test/unittest/platform/virtual/pin_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 266 | `framework/test/unittest/platform/virtual/pwm_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 267 | `framework/test/unittest/platform/virtual/regulator_linux_current_virtual_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 268 | `framework/test/unittest/platform/virtual/regulator_linux_voltage_virtual_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 269 | `framework/test/unittest/platform/virtual/regulator_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 270 | `framework/test/unittest/platform/virtual/spi_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 271 | `framework/test/unittest/platform/virtual/watchdog_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 272 | `framework/test/unittest/pm/hdf_pm_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 273 | `framework/test/unittest/sensor/hdf_sensor_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 274 | `framework/test/unittest/uevent/devmgr_uevent_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 275 | `framework/test/unittest/utils/hcs_parser/unittest/hcs_macro_cases.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 64 |
| 276 | `framework/test/unittest/wifi/hdf_wifi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 277 | `framework/tools/hdi-gen/ast/ast.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 53 |
| 278 | `framework/tools/hdi-gen/lexer/lexer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 279 | `framework/tools/hdi-gen/lexer/token.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 280 | `framework/tools/hdi-gen/parser/parser.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 281 | `framework/tools/hdi-gen/util/logger.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 282 | `framework/tools/hdi-gen/util/logger.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 283 | `framework/tools/hdi-gen/util/string_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 284 | `framework/tools/hdi-gen/util/string_builder.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 285 | `framework/tools/hdi-gen/util/string_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 286 | `framework/tools/hdi-gen/util/string_helper.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 287 | `framework/utils/src/hcs_parser/hcs_tree_if.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 66 |
| 288 | `framework/utils/src/hdf_sbuf.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 83 |
| 289 | `framework/utils/src/hdf_sbuf_impl_raw.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 290 | `interfaces/inner_api/hdi/iservstat_listener_hdi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 291 | `interfaces/inner_api/utils/hdf_trace.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |

### Per-file details

#### `adapter/khdf/hongmeng/osal/src/osal_cdev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 202 | 113 | `ERROR` | `udkDev-> name` |
| 251 | 128 | `ERROR` | `dev-> name` |
| 284 | 128 | `ERROR` | `dev-> name` |

#### `adapter/khdf/hongmeng/osal/src/osal_thread.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 134 | 107 | `ERROR` | `wrapper-> cpuId` |

#### `adapter/khdf/linux/manager/src/hdf_kevent.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 426 | 66 | `missing ;` | `` |
| 426 | 106 | `ERROR` | `=(` |
| 426 | 141 | `ERROR` | `)` |

#### `adapter/khdf/linux/model/camera/src/contig_dma.c`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 41 | `missing ;` | `` |

#### `adapter/khdf/linux/model/camera/src/sg_dma.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 404 | 45 | `ERROR` | `*` |
| 404 | 68 | `ERROR` | `*` |

#### `adapter/khdf/linux/model/camera/src/virtual_malloc.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 95 | 31 | `ERROR` | `*` |
| 95 | 54 | `ERROR` | `*` |

#### `adapter/khdf/linux/model/storage/emmc_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 162 | 61 | `missing ;` | `` |
| 162 | 101 | `ERROR` | `=(` |
| 162 | 131 | `ERROR` | `)` |

#### `adapter/khdf/linux/model/storage/sdio_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 504 | 125 | `ERROR` | `cntlr-> index` |
| 577 | 61 | `missing ;` | `` |
| 577 | 101 | `ERROR` | `=(` |
| 577 | 131 | `ERROR` | `)` |

#### `adapter/khdf/linux/model/usb/host/src/usb_net_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 244 | `ERROR` | `,` |
| 75 | 195 | `ERROR` | `stats64-> rx_packets` |
| 75 | 222 | `ERROR` | `stats64-> rx_bytes` |
| 75 | 326 | `ERROR` | `,` |
| 78 | 195 | `ERROR` | `stats64-> tx_packets` |
| 78 | 222 | `ERROR` | `stats64-> tx_bytes` |
| 78 | 326 | `ERROR` | `,` |
| 86 | 195 | `ERROR` | `stats64-> rx_packets` |
| 86 | 222 | `ERROR` | `stats64-> rx_bytes` |
| 86 | 326 | `ERROR` | `,` |
| 89 | 195 | `ERROR` | `stats64-> tx_packets` |
| 89 | 222 | `ERROR` | `stats64-> tx_bytes` |
| 89 | 326 | `ERROR` | `,` |
| 108 | 191 | `ERROR` | `skb-> protocol` |
| 108 | 291 | `ERROR` | `,` |
| 114 | 186 | `ERROR` | `,` |
| 120 | 249 | `ERROR` | `,` |
| 122 | 249 | `ERROR` | `,` |
| 180 | 181 | `ERROR` | `,` |
| 191 | 181 | `ERROR` | `,` |
| … | … | … | *(130 more)* |

#### `adapter/khdf/linux/model/usb/host/src/usb_pnp_notify.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 792 | 63 | `missing ;` | `` |
| 792 | 103 | `ERROR` | `=(` |
| 792 | 135 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/adc/adc_iio_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 192 | 123 | `ERROR` | `adcDevice-> deviceNum` |
| 270 | 65 | `missing ;` | `` |
| 270 | 105 | `ERROR` | `=(` |
| 270 | 139 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/clock/clock_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 208 | 121 | `ERROR` | `clockDevice-> deviceIndex` |
| 266 | 124 | `ERROR` | `clockDevice-> deviceIndex` |
| 349 | 67 | `missing ;` | `` |
| 349 | 107 | `ERROR` | `=(` |
| 349 | 143 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/fwk/platform_trace.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 4 | 87 | `ERROR` | `, . . .` |

#### `adapter/khdf/linux/platform/gpio/gpio_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 220 | 130 | `ERROR` | `chip-> base` |
| 222 | 130 | `ERROR` | `chip-> base` |
| 237 | 145 | `ERROR` | `cntlr-> start` |
| 242 | 145 | `ERROR` | `cntlr-> start` |
| 254 | 115 | `ERROR` | `HdfDeviceGetServiceName(device)` |
| 267 | 132 | `ERROR` | `chip-> base` |
| 269 | 132 | `ERROR` | `chip-> base` |
| 275 | 127 | `ERROR` | `chip-> base` |
| 280 | 146 | `ERROR` | `cntlr-> start` |
| 303 | 66 | `missing ;` | `` |
| 303 | 106 | `ERROR` | `=(` |
| 303 | 141 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/i2c/i2c_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 135 | 117 | `ERROR` | `cntlr-> busId` |
| 172 | 65 | `missing ;` | `` |
| 172 | 105 | `ERROR` | `=(` |
| 172 | 139 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/mipi_csi/mipi_csi_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 249 col 158 (missing )) ` |
| 22 | 54 | `ERROR` | `, . . .` |
| 94 | 129 | `ERROR` | `dev-> name` |
| 249 | 226 | `missing "` | `` |

#### `adapter/khdf/linux/platform/mipi_csi/mipi_v4l2_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 282 | 136 | `ERROR` | `cfg-> inputDataType` |
| 288 | 62 | `ERROR` | `attr-> inputMode` |
| 304 | 122 | `ERROR` | `camera-> link_freqs` |
| 305 | 143 | `ERROR` | `mode-> fll_def` |
| 478 | 69 | `missing ;` | `` |
| 478 | 109 | `ERROR` | `=(` |
| 478 | 147 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/mipi_dsi/mipi_drm_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 96 | 170 | `ERROR` | `cmd-> dataType` |
| 105 | 173 | `ERROR` | `cmd-> dataType` |
| 115 | 150 | `ERROR` | `cmd-> dataType` |
| 150 | 169 | `ERROR` | `cmd-> dataType` |
| 157 | 149 | `ERROR` | `cmd-> dataType` |
| 245 | 69 | `missing ;` | `` |
| 245 | 109 | `ERROR` | `=(` |
| 245 | 147 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 453 | 24 | `ERROR` | `struct` |
| 456 | 24 | `ERROR` | `struct` |

#### `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_hi35xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 634 | 127 | `ERROR` | `cmdInfo-> cmdSize` |
| 639 | 142 | `ERROR` | `cmdInfo-> cmdSize` |
| 683 | 161 | `ERROR` | `intSt1 .u32` |
| 712 | 160 | `ERROR` | `pldData .u32` |
| 967 | 124 | `ERROR` | `devCfg-> laneId[i]` |
| 973 | 128 | `ERROR` | `devCfg-> outputMode` |
| 978 | 127 | `ERROR` | `devCfg-> videoMode` |
| 985 | 130 | `ERROR` | `devCfg-> outputFormat` |
| 1032 | 119 | `ERROR` | `cmdInfo-> devno` |
| 1038 | 126 | `ERROR` | `cmdInfo-> cmdSize` |
| 1105 | 123 | `ERROR` | `getCmdInfo-> devno` |
| 1109 | 130 | `ERROR` | `getCmdInfo-> getDataSize` |
| 1231 | 63 | `missing ;` | `` |
| 1231 | 103 | `ERROR` | `=(` |
| 1231 | 135 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/pwm/pwm_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 164 | `ERROR` | `0xff` |
| 47 | 162 | `ERROR` | `config-> number` |
| 133 | 52 | `missing ;` | `` |
| 133 | 92 | `ERROR` | `=(` |
| 133 | 113 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/pwm/pwm_hi35xx_linux.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 114 | `ERROR` | `state-> polarity` |
| 28 | 133 | `ERROR` | `state-> period` |
| 32 | 150 | `ERROR` | `state-> duty_cycle` |
| 40 | 135 | `ERROR` | `pwm-> state .polarity` |
| 44 | 135 | `ERROR` | `pwm-> state .period` |
| 48 | 131 | `ERROR` | `pwm-> state .duty_cycle` |
| 55 | 172 | `ERROR` | `state-> period` |
| 76 | 167 | `ERROR` | `state-> period` |

#### `adapter/khdf/linux/platform/regulator/regulator_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 126 | `ERROR` | `node-> regulatorInfo .name` |
| 59 | 124 | `ERROR` | `info-> devName` |
| 61 | 130 | `ERROR` | `info-> devName` |
| 83 | 115 | `ERROR` | `node-> regulatorInfo .name` |
| 107 | 112 | `ERROR` | `node-> regulatorInfo .name` |
| 132 | 140 | `ERROR` | `node-> regulatorInfo .name` |
| 136 | 120 | `ERROR` | `info-> devName` |
| 153 | 115 | `ERROR` | `node-> regulatorInfo .name` |
| 174 | 111 | `ERROR` | `node-> regulatorInfo .name` |
| 195 | 115 | `ERROR` | `node-> regulatorInfo .name` |
| 215 | 111 | `ERROR` | `node-> regulatorInfo .name` |
| 291 | 106 | `ERROR` | `regNode-> regulatorInfo .name` |
| 326 | 131 | `ERROR` | `regNode-> regulatorInfo .name` |
| 348 | 115 | `ERROR` | `info-> devName` |
| 355 | 119 | `ERROR` | `info-> supplyName` |
| 395 | 146 | `ERROR` | `regNode-> regulatorInfo .name` |
| 497 | 71 | `missing ;` | `` |
| 497 | 111 | `ERROR` | `=(` |
| 497 | 151 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/rtc/rtc_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 200 | 118 | `ERROR` | `HdfDeviceGetServiceName(device)` |
| 225 | 60 | `missing ;` | `` |
| 225 | 100 | `ERROR` | `=(` |
| 225 | 129 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/spi/spi_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 280 | 122 | `ERROR` | `GetSpiDevName(& spidev-> dev)` |
| 283 | 125 | `ERROR` | `GetSpiDevName(& spidev-> dev)` |
| 284 | 136 | `ERROR` | `spidev-> master-> bus_num` |
| 528 | 58 | `missing ;` | `` |
| 528 | 98 | `ERROR` | `=(` |
| 528 | 125 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/watchdog/watchdog_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 228 | 115 | `ERROR` | `HdfDeviceGetServiceName(obj)` |
| 267 | 57 | `missing ;` | `` |
| 267 | 97 | `ERROR` | `=(` |
| 267 | 123 | `ERROR` | `)` |

#### `adapter/khdf/linux/test/platform/i2c/i2c_adapter_dummy.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 106 | 67 | `missing ;` | `` |
| 106 | 107 | `ERROR` | `=(` |
| 106 | 143 | `ERROR` | `)` |

#### `adapter/khdf/liteos/model/storage/src/mmc/mmc_block_lite.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 32 | `ERROR` | `struct Vnode` |
| 35 | 33 | `ERROR` | `struct Vnode` |
| 41 | 36 | `ERROR` | `struct Vnode` |
| 41 | 71 | `ERROR` | `char` |
| 75 | 37 | `ERROR` | `struct Vnode` |
| 75 | 69 | `ERROR` | `unsigned char` |
| 105 | 36 | `ERROR` | `struct Vnode` |
| 105 | 62 | `ERROR` | `struct geometry` |
| 130 | 44 | `ERROR` | `struct Vnode` |
| 179 | 37 | `ERROR` | `struct Vnode` |
| 199 | 21 | `ERROR` | `struct` |
| 209 | 30 | `ERROR` | `int` |

#### `adapter/khdf/liteos/model/storage/src/mtd/mtd_block_lite.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 5 | 35 | `ERROR` | `struct Vnode` |
| 11 | 36 | `ERROR` | `struct Vnode` |
| 17 | 39 | `ERROR` | `struct Vnode` |
| 17 | 74 | `ERROR` | `char` |
| 27 | 40 | `ERROR` | `struct Vnode` |
| 27 | 72 | `ERROR` | `unsigned char` |
| 37 | 39 | `ERROR` | `struct Vnode` |
| 37 | 65 | `ERROR` | `struct geometry` |
| 44 | 40 | `ERROR` | `struct Vnode` |

#### `adapter/khdf/liteos/model/storage/src/mtd/mtd_char_lite.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 28 | `ERROR` | `struct file` |
| 50 | 29 | `ERROR` | `struct file` |
| 80 | 32 | `ERROR` | `struct file` |
| 80 | 57 | `ERROR` | `char` |
| 152 | 33 | `ERROR` | `struct file` |
| 152 | 64 | `ERROR` | `char` |
| 224 | 31 | `ERROR` | `struct file` |
| 364 | 29 | `ERROR` | `struct file` |
| 381 | 21 | `ERROR` | `struct` |
| 385 | 21 | `ERROR` | `struct` |
| 386 | 22 | `ERROR` | `struct` |
| 416 | 31 | `ERROR` | `struct file` |
| 416 | 56 | `ERROR` | `LosVmMapRegion` |
| 503 | 115 | `ERROR` | `mtdDevice-> type` |

#### `adapter/khdf/liteos/model/usb/host/src/usb_pnp_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 64 | `missing ;` | `` |
| 85 | 104 | `ERROR` | `=(` |
| 85 | 137 | `ERROR` | `)` |

#### `adapter/khdf/liteos/model/usb/host/src/usb_pnp_notify.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 882 | 63 | `missing ;` | `` |
| 882 | 103 | `ERROR` | `=(` |
| 882 | 135 | `ERROR` | `)` |

#### `adapter/khdf/liteos/model/usb/host/src/usb_test_pnp_notify.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 67 | `missing ;` | `` |
| 59 | 107 | `ERROR` | `=(` |
| 59 | 143 | `ERROR` | `)` |

#### `adapter/khdf/liteos/osal/src/osal_firmware.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 118 | `ERROR` | `,(long long` |
| 48 | 204 | `missing ;` | `` |
| 48 | 220 | `ERROR` | `)` |

#### `adapter/khdf/liteos/platform/include/gpio_dev.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 1 | `ERROR` | `typedef struct GpioBitInfo{ unsigned int groupnumber ; unsigned int bitnumber ;  unsigned char value ; unsigned char dir…` |

#### `adapter/khdf/liteos/platform/src/gpio_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 119 | `ERROR` | `info-> direction` |
| 37 | 119 | `ERROR` | `info-> direction` |
| 46 | 116 | `ERROR` | `info-> value` |
| 54 | 117 | `ERROR` | `info-> value` |
| 89 | 120 | `ERROR` | `info .bitnumber` |
| 93 | 121 | `ERROR` | `info .groupnumber` |
| 97 | 128 | `ERROR` | `info .bitnumber` |

#### `adapter/khdf/liteos/platform/src/i2c_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 266 | 130 | `ERROR` | `wrap .nmsgs` |

#### `adapter/khdf/liteos/platform/src/platform_trace.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 87 | `ERROR` | `, . . .` |

#### `adapter/khdf/liteos/platform/src/spi_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 112 | `ERROR` | `device-> csNum` |
| 16 | 118 | `ERROR` | `device-> csNum` |

#### `adapter/khdf/liteos/platform/src/uart_dev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 206 | 97 | `ERROR` | `host-> num` |
| 208 | 118 | `ERROR` | `host-> num` |
| 214 | 121 | `ERROR` | `host-> num` |

#### `adapter/khdf/liteos_m/test/sample_driver/src/sample_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 58 | 63 | `missing ;` | `` |
| 58 | 103 | `ERROR` | `=(` |
| 58 | 135 | `ERROR` | `)` |

#### `adapter/khdf/uniproton/test/sample_driver/src/platform_device_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 227 | `ERROR` | `#(` |
| 19 | 305 | `ERROR` | `)` |
| 21 | 217 | `ERROR` | `#(` |
| 21 | 225 | `ERROR` | `(0)` |
| 21 | 285 | `ERROR` | `)` |
| 25 | 230 | `ERROR` | `#(` |
| 25 | 248 | `ERROR` | `(NULL)` |
| 25 | 311 | `ERROR` | `)` |
| 29 | 227 | `ERROR` | `#(` |
| 29 | 305 | `ERROR` | `)` |
| 31 | 217 | `ERROR` | `#(` |
| 31 | 225 | `ERROR` | `(0)` |
| 31 | 285 | `ERROR` | `)` |
| 52 | 227 | `ERROR` | `#(` |
| 52 | 305 | `ERROR` | `)` |
| 56 | 245 | `ERROR` | `#(` |
| 56 | 264 | `ERROR` | `(refCntBeforeGet+ 1)` |
| 56 | 341 | `ERROR` | `)` |
| 60 | 227 | `ERROR` | `#(` |
| 60 | 305 | `ERROR` | `)` |
| … | … | … | *(79 more)* |

#### `adapter/khdf/uniproton/test/sample_driver/src/platform_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 63 | `missing ;` | `` |
| 45 | 103 | `ERROR` | `=(` |
| 45 | 135 | `ERROR` | `)` |

#### `adapter/khdf/uniproton/test/sample_driver/src/platform_manager_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 70 | 228 | `ERROR` | `#(` |
| 70 | 306 | `ERROR` | `)` |
| 73 | 246 | `ERROR` | `#(` |
| 73 | 265 | `ERROR` | `(refCntBeforeAdd+ 1)` |
| 73 | 342 | `ERROR` | `)` |
| 78 | 228 | `ERROR` | `#(` |
| 78 | 306 | `ERROR` | `)` |
| 82 | 231 | `ERROR` | `#(` |
| 82 | 312 | `ERROR` | `)` |
| 87 | 231 | `ERROR` | `#(` |
| 87 | 312 | `ERROR` | `)` |
| 95 | 228 | `ERROR` | `#(` |
| 95 | 306 | `ERROR` | `)` |
| 101 | 228 | `ERROR` | `#(` |
| 101 | 306 | `ERROR` | `)` |
| 107 | 246 | `ERROR` | `#(` |
| 107 | 342 | `ERROR` | `)` |
| 111 | 228 | `ERROR` | `#(` |
| 111 | 243 | `ERROR` | `(NULL)` |
| 111 | 306 | `ERROR` | `)` |
| … | … | … | *(30 more)* |

#### `adapter/platform/can/can_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 339 | 58 | `missing ;` | `` |
| 339 | 98 | `ERROR` | `=(` |
| 339 | 125 | `ERROR` | `)` |

#### `adapter/platform/gpio/gpio_asr.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 357 | 99 | `ERROR` | `gpioCntlr-> start` |
| 389 | 61 | `missing ;` | `` |
| 389 | 101 | `ERROR` | `=(` |
| 389 | 131 | `ERROR` | `)` |

#### `adapter/platform/gpio/gpio_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 83 | 61 | `missing ;` | `` |
| 83 | 101 | `ERROR` | `=(` |
| 83 | 131 | `ERROR` | `)` |
| 248 | 99 | `ERROR` | `gpioCntlr-> start` |

#### `adapter/platform/gpio/gpio_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 106 | 61 | `missing ;` | `` |
| 106 | 101 | `ERROR` | `=(` |
| 106 | 131 | `ERROR` | `)` |
| 341 | 99 | `ERROR` | `gpioCntlr-> start` |

#### `adapter/platform/gpio/gpio_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 321 | 99 | `ERROR` | `gpioCntlr-> start` |
| 367 | 61 | `missing ;` | `` |
| 367 | 101 | `ERROR` | `=(` |
| 367 | 131 | `ERROR` | `)` |

#### `adapter/platform/i2c/i2c_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 60 | `missing ;` | `` |
| 20 | 100 | `ERROR` | `=(` |
| 20 | 129 | `ERROR` | `)` |

#### `adapter/platform/i2c/i2c_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 62 | `missing ;` | `` |
| 39 | 102 | `ERROR` | `=(` |
| 39 | 133 | `ERROR` | `)` |

#### `adapter/platform/i2c/i2c_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 60 | `missing ;` | `` |
| 35 | 100 | `ERROR` | `=(` |
| 35 | 129 | `ERROR` | `)` |

#### `adapter/platform/mipi_dsi/mipi_drm_imx8mm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 191 | 69 | `missing ;` | `` |
| 191 | 109 | `ERROR` | `=(` |
| 191 | 147 | `ERROR` | `)` |

#### `adapter/platform/pwm/pwm_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 127 | 60 | `missing ;` | `` |
| 127 | 100 | `ERROR` | `=(` |
| 127 | 129 | `ERROR` | `)` |

#### `adapter/platform/pwm/pwm_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 256 | 60 | `missing ;` | `` |
| 256 | 100 | `ERROR` | `=(` |
| 256 | 129 | `ERROR` | `)` |

#### `adapter/platform/pwm/pwm_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 199 | 60 | `missing ;` | `` |
| 199 | 100 | `ERROR` | `=(` |
| 199 | 129 | `ERROR` | `)` |

#### `adapter/platform/spi/spi_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 476 | 60 | `missing ;` | `` |
| 476 | 100 | `ERROR` | `=(` |
| 476 | 129 | `ERROR` | `)` |

#### `adapter/platform/spi/spi_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 403 | 99 | `ERROR` | `g_parseHcsMap[i]` |
| 478 | 60 | `missing ;` | `` |
| 478 | 100 | `ERROR` | `=(` |
| 478 | 129 | `ERROR` | `)` |

#### `adapter/platform/spi/spi_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 284 | 60 | `missing ;` | `` |
| 284 | 100 | `ERROR` | `=(` |
| 284 | 129 | `ERROR` | `)` |

#### `adapter/platform/uart/uart_asr.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 645 | 59 | `missing ;` | `` |
| 645 | 99 | `ERROR` | `=(` |
| 645 | 127 | `ERROR` | `)` |

#### `adapter/platform/uart/uart_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 129 | `ERROR` | `,(int` |
| 85 | 209 | `missing ;` | `` |
| 85 | 229 | `ERROR` | `)` |
| 109 | 129 | `ERROR` | `,(int` |
| 109 | 209 | `missing ;` | `` |
| 109 | 229 | `ERROR` | `)` |
| 323 | 61 | `missing ;` | `` |
| 323 | 101 | `ERROR` | `=(` |
| 323 | 131 | `ERROR` | `)` |
| 383 | 99 | `ERROR` | `uartDevice-> uartId` |

#### `adapter/platform/uart/uart_gr5xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 708 | 59 | `missing ;` | `` |
| 708 | 99 | `ERROR` | `=(` |
| 708 | 127 | `ERROR` | `)` |

#### `adapter/platform/uart/uart_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 271 | 61 | `missing ;` | `` |
| 271 | 101 | `ERROR` | `=(` |
| 271 | 131 | `ERROR` | `)` |
| 326 | 106 | `ERROR` | `uartDevice-> uartId` |

#### `adapter/platform/uart/uart_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 185 | 61 | `missing ;` | `` |
| 185 | 101 | `ERROR` | `=(` |
| 185 | 131 | `ERROR` | `)` |
| 223 | 95 | `ERROR` | `uartDevice-> uartId` |
| 268 | 85 | `ERROR` | `resource-> num` |

#### `adapter/platform/watchdog/watchdog_bes.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 127 | 65 | `missing ;` | `` |
| 127 | 105 | `ERROR` | `=(` |
| 127 | 139 | `ERROR` | `)` |

#### `adapter/platform/watchdog/watchdog_stm32f4xx.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 64 | 92 | `ERROR` | `device-> watchdogId` |
| 65 | 91 | `ERROR` | `device-> timeout` |
| 124 | 65 | `missing ;` | `` |
| 124 | 105 | `ERROR` | `=(` |
| 124 | 139 | `ERROR` | `)` |

#### `adapter/platform/watchdog/watchdog_wm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 254 | 65 | `missing ;` | `` |
| 254 | 105 | `ERROR` | `=(` |
| 254 | 139 | `ERROR` | `)` |

#### `adapter/uhdf2/hdi/src/idevmgr_client.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 98 | `ERROR` | `serviceName .data()` |
| 69 | 100 | `ERROR` | `serviceName .data()` |

#### `adapter/uhdf2/hdi/src/servstat_listener.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 134 | `ERROR` | `status .serviceName` |

#### `adapter/uhdf2/hdi/src/servstat_listener_stub.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 53 | 113 | `ERROR` | `status .serviceName .c_str()` |

#### `adapter/uhdf2/hdi/src/stub_collector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 108 | `ERROR` | `g_constructorMap .size()` |
| 49 | 106 | `ERROR` | `consruct .first .c_str()` |

#### `adapter/uhdf2/hdi/test/buffer_handle/native_buffer_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 343 | 97 | `ERROR` | `sbuffer .c_str()` |
| 358 | 98 | `ERROR` | `dbuffer .c_str()` |

#### `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 139 | 121 | `ERROR` | `status .serviceName .data()` |
| 160 | 121 | `ERROR` | `status .serviceName .data()` |
| 178 | 121 | `ERROR` | `status .serviceName .data()` |
| 199 | 121 | `ERROR` | `status .serviceName .data()` |
| 220 | 121 | `ERROR` | `status .serviceName .data()` |
| 244 | 121 | `ERROR` | `status .serviceName .data()` |

#### `adapter/uhdf2/host/devhost.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 230 | 118 | `ERROR` | `config-> hostName` |
| 249 | 119 | `ERROR` | `config-> hostName` |
| 252 | 112 | `ERROR` | `config-> hostName` |
| 265 | 96 | `ERROR` | `config-> hostName` |
| 293 | 117 | `ERROR` | `config .hostName` |

#### `adapter/uhdf2/host/src/devhost_service_full.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 122 col 183 (missing )) ` |
| 29 | 135 | `ERROR` | `msg-> messageId` |
| 77 | 133 | `ERROR` | `deviceNode-> driver-> entry-> moduleName` |
| 87 | 134 | `ERROR` | `deviceNode-> driver-> entry-> moduleName` |
| 122 | 130 | `ERROR` | `unsigned long long` |
| 122 | 227 | `missing ;` | `` |
| 122 | 246 | `ERROR` | `)` |

#### `adapter/uhdf2/host/src/device_service_stub.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 110 | `ERROR` | `service-> devId` |

#### `adapter/uhdf2/host/src/devsvc_manager_proxy.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 93 | 127 | `ERROR` | `servInfo-> servName` |
| 137 | 130 | `ERROR` | `servInfo-> servName` |

#### `adapter/uhdf2/host/src/hdf_device_thread.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 120 | `ERROR` | `msg-> messageId` |

#### `adapter/uhdf2/host/test/unittest/sample1_driver/sample1_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 141 | 64 | `missing ;` | `` |
| 141 | 104 | `ERROR` | `=(` |
| 141 | 137 | `ERROR` | `)` |

#### `adapter/uhdf2/host/test/unittest/sample_driver/sample_driver.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 72 | 63 | `missing ;` | `` |
| 72 | 103 | `ERROR` | `=(` |
| 72 | 111 | `ERROR` | `)(` |
| 72 | 135 | `ERROR` | `)` |

#### `adapter/uhdf2/ipc/src/hdf_remote_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 139 | `ERROR` | `option .GetFlags()` |
| 456 | 112 | `ERROR` | `client .c_str()` |

#### `adapter/uhdf2/manager/src/devmgr_service_full.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 82 | 100 | `ERROR` | `msg-> messageId` |

#### `adapter/uhdf2/manager/src/devmgr_uevent.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 280 | 111 | `ERROR` | `ruleCfg-> serviceName` |
| 282 | 106 | `ERROR` | `matchKey-> key` |

#### `adapter/uhdf2/manager/src/devsvc_manager_stub.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 217 | 109 | `ERROR` | `info .servName` |
| 245 | 110 | `ERROR` | `info .servName` |
| 260 | 112 | `ERROR` | `info .servName` |
| 448 | 127 | `ERROR` | `HdfRemoteGetCallingPid()` |

#### `adapter/uhdf2/osal/test/unittest/common/sample_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 128 | 63 | `missing ;` | `` |
| 128 | 103 | `ERROR` | `=(` |
| 128 | 135 | `ERROR` | `)` |

#### `adapter/uhdf2/security/src/hdf_security.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 172 | 117 | `ERROR` | `deviceRoot-> name` |
| 186 | 116 | `ERROR` | `hostRoot-> name` |

#### `adapter/uhdf2/shared/src/dev_attribute_serialize.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 96 | `ERROR` | `strlen(svcName)` |

#### `framework/core/adapter/syscall/src/hdf_syscall_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 114 | `ERROR` | `,(int` |
| 29 | 195 | `ERROR` | `newSize)` |

#### `framework/core/adapter/vnode/src/hdf_vnode_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 712 | 117 | `ERROR` | `vnodeAdapter-> vNodePath` |

#### `framework/core/common/src/hdf_attribute_macro.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 141 | `ERROR` | `hostInfo-> priority` |
| 103 | 124 | `ERROR` | `deviceNodeInfo-> policy` |
| 108 | 126 | `ERROR` | `deviceNodeInfo-> priority` |
| 113 | 125 | `ERROR` | `deviceNodeInfo-> preload` |

#### `framework/core/host/src/devhost_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 73 | 125 | `ERROR` | `deviceInfo-> moduleName` |
| 162 | 122 | `ERROR` | `deviceNode-> driver-> entry-> moduleName` |
| 171 | 123 | `ERROR` | `deviceNode-> driver-> entry-> moduleName` |
| 190 | 118 | `ERROR` | `hostService-> hostName` |

#### `framework/core/host/src/hdf_device_node.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 118 | `ERROR` | `driverEntry-> moduleName` |
| 58 | 104 | `ERROR` | `driverEntry-> moduleName` |
| 210 | 101 | `ERROR` | `devNode-> servName` |
| 264 | 130 | `ERROR` | `deviceInfo-> moduleName` |

#### `framework/core/host/src/hdf_device_object.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 165 | 131 | `ERROR` | `parentDevNode-> devStatus` |
| 215 | 106 | `ERROR` | `devNode-> driverName` |
| 221 | 109 | `ERROR` | `devNode-> driverName` |

#### `framework/core/manager/src/devhost_service_clnt.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 123 | `ERROR` | `deviceInfo-> svcName` |

#### `framework/core/manager/src/devmgr_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 126 | `ERROR` | `hostClnt-> hostName` |
| 63 | 123 | `ERROR` | `hostClnt-> hostName` |
| 101 | 126 | `ERROR` | `hostClnt-> hostName` |
| 313 | 120 | `ERROR` | `hostClnt-> hostName` |
| 328 | 125 | `ERROR` | `hostAttr-> hostId` |

#### `framework/core/manager/src/devsvc_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 119 | `ERROR` | `svcstat .serviceName` |
| 266 | 109 | `ERROR` | `record-> servName` |

#### `framework/core/manager/test/unittest/common/hdf_ioservice_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 84 | 130 | `ERROR` | `static_cast< char*>(listener-> priv)` |
| 93 | 104 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 103 | 155 | `ERROR` | `static_cast< char*>(listener-> priv)` |
| 114 | 128 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 127 | 116 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 128 | 110 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 174 | 117 | `ERROR` | `time .sec` |
| 675 | 143 | `ERROR` | `servstat-> serviceName` |
| 876 | 112 | `ERROR` | `strlen(eventData)` |
| 1035 | 118 | `ERROR` | `listener2 .eventCount` |

#### `framework/core/manager/test/unittest/common/hdf_sbuf_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 276 | 1 | `ERROR` | `ASSERT_EQ(val , static_cast< uint64_t> INT64_MAX)` |

#### `framework/model/audio/core/src/audio_host.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 443 | 62 | `missing ;` | `` |
| 443 | 102 | `ERROR` | `=(` |
| 443 | 133 | `ERROR` | `)` |

#### `framework/model/audio/dispatch/src/audio_control_dispatch.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 631 | 63 | `missing ;` | `` |
| 631 | 103 | `ERROR` | `=(` |
| 631 | 135 | `ERROR` | `)` |

#### `framework/model/audio/dispatch/src/audio_stream_dispatch.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 1678 | 62 | `missing ;` | `` |
| 1678 | 102 | `ERROR` | `=(` |
| 1678 | 133 | `ERROR` | `)` |

#### `framework/model/audio/hdmi/src/audio_hdmi_codec_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 120 | 66 | `missing ;` | `` |
| 120 | 106 | `ERROR` | `=(` |
| 120 | 141 | `ERROR` | `)` |

#### `framework/model/audio/usb/src/audio_usb_codec_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 91 | 65 | `missing ;` | `` |
| 91 | 105 | `ERROR` | `=(` |
| 91 | 139 | `ERROR` | `)` |

#### `framework/model/audio/usb/src/audio_usb_dma_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 120 | 68 | `missing ;` | `` |
| 120 | 108 | `ERROR` | `=(` |
| 120 | 145 | `ERROR` | `)` |

#### `framework/model/audio/usb/src/audio_usb_endpoints.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 289 | 59 | `ERROR` | `struct` |

#### `framework/model/audio/usb/src/audio_usb_mixer.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 104 | 27 | `ERROR` | `, 256` |
| 105 | 27 | `ERROR` | `, 256` |
| 688 | 15 | `ERROR` | `. .0` |
| 689 | 15 | `ERROR` | `. .0` |
| 690 | 15 | `ERROR` | `. .0` |

#### `framework/model/camera/parser/src/camera_config_parser.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 24 col 90 (missing )) ` |
| 26 | 41 | `ERROR` | `)` |
| 31 | 46 | `ERROR` | `const struct` |
| 32 | 1 | `ERROR` | `struct` |
| 32 | 38 | `ERROR` | `struct` |
| 37 | 180 | `ERROR` | `,` |
| 39 | 180 | `ERROR` | `,` |
| 41 | 180 | `ERROR` | `,` |
| 43 | 180 | `ERROR` | `,` |
| 45 | 180 | `ERROR` | `,` |
| 52 | 180 | `ERROR` | `,` |
| 54 | 180 | `ERROR` | `,` |
| 68 | 180 | `ERROR` | `,` |
| 70 | 180 | `ERROR` | `,` |
| 72 | 180 | `ERROR` | `,` |
| 74 | 180 | `ERROR` | `,` |
| 76 | 180 | `ERROR` | `,` |
| 78 | 180 | `ERROR` | `,` |
| 80 | 180 | `ERROR` | `,` |
| 82 | 180 | `ERROR` | `,` |
| … | … | … | *(52 more)* |

#### `framework/model/camera/utils/src/camera_utils.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 458 col 161 (missing )) ` |
| 458 | 206 | `ERROR` | `)` |

#### `framework/model/display/driver/adapter_soc/hi35xx_disp.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 311 | 60 | `missing ;` | `` |
| 311 | 100 | `ERROR` | `=(` |
| 311 | 129 | `ERROR` | `)` |

#### `framework/model/display/driver/backlight/hdf_bl.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 427 | 56 | `missing ;` | `` |
| 427 | 96 | `ERROR` | `=(` |
| 427 | 121 | `ERROR` | `)` |

#### `framework/model/display/driver/backlight/pwm_bl.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 141 | `ERROR` | `,` |
| 79 | 141 | `ERROR` | `,` |
| 81 | 141 | `ERROR` | `,` |
| 83 | 141 | `ERROR` | `,` |
| 85 | 141 | `ERROR` | `,` |
| 89 | 141 | `ERROR` | `,` |
| 162 | 59 | `missing ;` | `` |
| 162 | 99 | `ERROR` | `=(` |
| 162 | 127 | `ERROR` | `)` |

#### `framework/model/display/driver/hdf_disp.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 673 | 58 | `missing ;` | `` |
| 673 | 98 | `ERROR` | `=(` |
| 673 | 125 | `ERROR` | `)` |

#### `framework/model/display/driver/hdf_drm_panel.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 96 | 104 | `ERROR` | `hdfDrmPanel-> mode .hdisplay` |
| 133 | 20 | `ERROR` | `suspend , S_IWUSR , NULL ,` |
| 133 | 60 | `missing identifier` | `` |
| 153 | 20 | `ERROR` | `resume , S_IWUSR , NULL ,` |
| 153 | 58 | `missing identifier` | `` |
| 178 | 20 | `ERROR` | `backlight , S_IWUSR , NULL ,` |
| 178 | 64 | `missing identifier` | `` |
| 273 | 62 | `missing ;` | `` |
| 273 | 102 | `ERROR` | `=(` |
| 273 | 133 | `ERROR` | `)` |

#### `framework/model/display/driver/lcdkit/lcdkit_parse_config.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 141 col 221 (missing )) ` |
| 0 | 0 | `?` | `line 142 col 224 (missing )) ` |
| 0 | 0 | `?` | `line 143 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 144 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 145 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 146 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 147 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 148 col 215 (missing )) ` |
| 0 | 0 | `?` | `line 149 col 233 (missing )) ` |
| 0 | 0 | `?` | `line 150 col 230 (missing )) ` |
| 0 | 0 | `?` | `line 151 col 230 (missing )) ` |
| 0 | 0 | `?` | `line 153 col 230 (missing )) ` |
| 0 | 0 | `?` | `line 154 col 224 (missing )) ` |
| 0 | 0 | `?` | `line 155 col 239 (missing )) ` |
| 0 | 0 | `?` | `line 156 col 234 (missing )) ` |
| 0 | 0 | `?` | `line 158 col 229 (missing )) ` |
| 0 | 0 | `?` | `line 159 col 235 (missing )) ` |
| 0 | 0 | `?` | `line 160 col 235 (missing )) ` |
| 0 | 0 | `?` | `line 161 col 235 (missing )) ` |
| 0 | 0 | `?` | `line 164 col 226 (missing )) ` |
| … | … | … | *(24 more)* |

#### `framework/model/display/driver/lcdkit/lite_lcdkit.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 282 | 60 | `missing ;` | `` |
| 282 | 100 | `ERROR` | `=(` |
| 282 | 129 | `ERROR` | `)` |

#### `framework/model/display/driver/panel/ili9881_st_5p5.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 678 | 57 | `missing ;` | `` |
| 678 | 97 | `ERROR` | `=(` |
| 678 | 123 | `ERROR` | `)` |

#### `framework/model/display/driver/panel/ili9881c_boe.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 676 | 65 | `missing ;` | `` |
| 676 | 105 | `ERROR` | `=(` |
| 676 | 139 | `ERROR` | `)` |

#### `framework/model/display/driver/panel/mipi_icn9700.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 299 | 61 | `missing ;` | `` |
| 299 | 101 | `ERROR` | `=(` |
| 299 | 131 | `ERROR` | `)` |

#### `framework/model/display/driver/panel/ssp_st7789.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 266 | 60 | `missing ;` | `` |
| 266 | 100 | `ERROR` | `=(` |
| 266 | 129 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_encoder.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 293 | 61 | `missing ;` | `` |
| 293 | 101 | `ERROR` | `=(` |
| 293 | 131 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_encoder.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 25 | `missing ;` | `` |

#### `framework/model/input/driver/hdf_hid_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 451 | 57 | `missing ;` | `` |
| 451 | 97 | `ERROR` | `=(` |
| 451 | 123 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_infrared.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 361 | 62 | `missing ;` | `` |
| 361 | 102 | `ERROR` | `=(` |
| 361 | 133 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_input_device_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 108 | 113 | `ERROR` | `inputDev-> devId` |
| 495 | 59 | `missing ;` | `` |
| 495 | 99 | `ERROR` | `=(` |
| 495 | 127 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_key.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 220 | 57 | `missing ;` | `` |
| 220 | 97 | `ERROR` | `=(` |
| 220 | 123 | `ERROR` | `)` |

#### `framework/model/input/driver/hdf_touch.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 852 | 59 | `missing ;` | `` |
| 852 | 99 | `ERROR` | `=(` |
| 852 | 127 | `ERROR` | `)` |

#### `framework/model/input/driver/input_config_parser.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 141 | `ERROR` | `,` |
| 26 | 141 | `ERROR` | `,` |
| 28 | 141 | `ERROR` | `,` |
| 30 | 141 | `ERROR` | `,` |
| 52 | 141 | `ERROR` | `,` |
| 54 | 141 | `ERROR` | `,` |
| 56 | 141 | `ERROR` | `,` |
| 58 | 141 | `ERROR` | `,` |
| 60 | 141 | `ERROR` | `,` |
| 81 | 141 | `ERROR` | `,` |
| 83 | 141 | `ERROR` | `,` |
| 85 | 141 | `ERROR` | `,` |
| 87 | 141 | `ERROR` | `,` |
| 97 | 141 | `ERROR` | `,` |
| 99 | 141 | `ERROR` | `,` |
| 101 | 141 | `ERROR` | `,` |
| 103 | 141 | `ERROR` | `,` |
| 111 | 141 | `ERROR` | `,` |
| 115 | 141 | `ERROR` | `,` |
| 117 | 141 | `ERROR` | `,` |
| … | … | … | *(45 more)* |

#### `framework/model/input/driver/touchscreen/touch_ft5406.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 359 | 65 | `missing ;` | `` |
| 359 | 105 | `ERROR` | `=(` |
| 359 | 139 | `ERROR` | `)` |

#### `framework/model/input/driver/touchscreen/touch_ft5x06.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 396 | 68 | `missing ;` | `` |
| 396 | 108 | `ERROR` | `=(` |
| 396 | 145 | `ERROR` | `)` |

#### `framework/model/input/driver/touchscreen/touch_ft6336.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 206 | 65 | `missing ;` | `` |
| 206 | 105 | `ERROR` | `=(` |
| 206 | 139 | `ERROR` | `)` |

#### `framework/model/input/driver/touchscreen/touch_gt911.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 125 | `ERROR` | `buf[0]` |
| 45 | 128 | `ERROR` | `buf[0]` |
| 332 | 66 | `missing ;` | `` |
| 332 | 106 | `ERROR` | `=(` |
| 332 | 141 | `ERROR` | `)` |

#### `framework/model/misc/dsoftbus/src/hdf_dsoftbus_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 63 | 61 | `missing ;` | `` |
| 63 | 101 | `ERROR` | `=(` |
| 63 | 131 | `ERROR` | `)` |

#### `framework/model/misc/dsoftbus/src/module_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 105 | `ERROR` | `g_modules[i] .moduleId` |
| 37 | 95 | `ERROR` | `g_modules[i] .moduleId` |

#### `framework/model/misc/light/driver/src/light_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 393 | 218 | `ERROR` | `,` |
| 395 | 218 | `ERROR` | `,` |
| 397 | 218 | `ERROR` | `,` |
| 425 | 218 | `ERROR` | `,` |
| 429 | 218 | `ERROR` | `,` |
| 624 | 62 | `missing ;` | `` |
| 624 | 102 | `ERROR` | `=(` |
| 624 | 133 | `ERROR` | `)` |

#### `framework/model/misc/vibrator/driver/src/vibrator_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 441 | 65 | `missing ;` | `` |
| 441 | 105 | `ERROR` | `=(` |
| 441 | 139 | `ERROR` | `)` |

#### `framework/model/misc/vibrator/driver/src/vibrator_haptic.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 122 | `ERROR` | `,` |
| 76 | 116 | `ERROR` | `,` |
| 82 | 115 | `ERROR` | `,` |

#### `framework/model/misc/vibrator/driver/src/vibrator_parser.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 218 | `ERROR` | `,` |
| 15 | 218 | `ERROR` | `,` |
| 17 | 218 | `ERROR` | `,` |
| 19 | 218 | `ERROR` | `,` |
| 33 | 218 | `ERROR` | `,` |
| 35 | 218 | `ERROR` | `,` |
| 37 | 218 | `ERROR` | `,` |
| 39 | 218 | `ERROR` | `,` |
| 41 | 218 | `ERROR` | `,` |
| 43 | 218 | `ERROR` | `,` |
| 58 | 218 | `ERROR` | `,` |
| 62 | 218 | `ERROR` | `,` |
| 64 | 218 | `ERROR` | `,` |
| 66 | 218 | `ERROR` | `,` |
| 69 | 218 | `ERROR` | `,` |
| 92 | 218 | `ERROR` | `,` |
| 99 | 218 | `ERROR` | `,` |
| 106 | 218 | `ERROR` | `,` |

#### `framework/model/network/ethernet/src/hdf_eth_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 206 | 54 | `missing ;` | `` |
| 206 | 94 | `ERROR` | `=(` |
| 206 | 117 | `ERROR` | `)` |

#### `framework/model/network/wifi/core/components/p2p/p2p.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 198 | `missing identifier` | `` |
| 40 | 198 | `missing identifier` | `` |
| 44 | 198 | `missing identifier` | `` |
| 82 | 198 | `missing identifier` | `` |
| 91 | 198 | `missing identifier` | `` |
| 128 | 198 | `missing identifier` | `` |
| 173 | 198 | `missing identifier` | `` |
| 183 | 211 | `ERROR` | `,` |
| 222 | 198 | `missing identifier` | `` |
| 233 | 198 | `missing identifier` | `` |
| 271 | 198 | `missing identifier` | `` |
| 281 | 198 | `missing identifier` | `` |
| 285 | 198 | `missing identifier` | `` |
| 289 | 198 | `missing identifier` | `` |
| 327 | 198 | `missing identifier` | `` |
| 347 | 100 | `ERROR` | `params-> drvFlags` |

#### `framework/model/network/wifi/core/components/softap/ap.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 137 | 198 | `missing identifier` | `` |
| 142 | 198 | `missing identifier` | `` |
| 146 | 198 | `missing identifier` | `` |
| 150 | 198 | `missing identifier` | `` |
| 154 | 198 | `missing identifier` | `` |
| 158 | 198 | `missing identifier` | `` |
| 162 | 198 | `missing identifier` | `` |
| 166 | 198 | `missing identifier` | `` |
| 170 | 198 | `missing identifier` | `` |
| 174 | 198 | `missing identifier` | `` |
| 183 | 198 | `missing identifier` | `` |
| 187 | 198 | `missing identifier` | `` |
| 191 | 198 | `missing identifier` | `` |
| 195 | 198 | `missing identifier` | `` |
| 199 | 198 | `missing identifier` | `` |
| 203 | 198 | `missing identifier` | `` |
| 207 | 198 | `missing identifier` | `` |
| 211 | 198 | `missing identifier` | `` |
| 215 | 198 | `missing identifier` | `` |
| 236 | 198 | `missing identifier` | `` |
| … | … | … | *(13 more)* |

#### `framework/model/network/wifi/core/components/sta/sta.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 198 | `missing identifier` | `` |
| 59 | 198 | `missing identifier` | `` |
| 64 | 198 | `missing identifier` | `` |
| 69 | 198 | `missing identifier` | `` |
| 103 | 198 | `missing identifier` | `` |
| 107 | 198 | `missing identifier` | `` |
| 111 | 198 | `missing identifier` | `` |
| 115 | 198 | `missing identifier` | `` |
| 119 | 198 | `missing identifier` | `` |
| 123 | 198 | `missing identifier` | `` |
| 127 | 198 | `missing identifier` | `` |
| 131 | 198 | `missing identifier` | `` |
| 135 | 198 | `missing identifier` | `` |
| 139 | 198 | `missing identifier` | `` |
| 201 | 198 | `missing identifier` | `` |
| 251 | 198 | `missing identifier` | `` |
| 255 | 198 | `missing identifier` | `` |
| 286 | 198 | `missing identifier` | `` |
| 315 | 198 | `missing identifier` | `` |
| 347 | 198 | `missing identifier` | `` |
| … | … | … | *(10 more)* |

#### `framework/model/network/wifi/core/hdf_wifi_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 546 | 58 | `missing ;` | `` |
| 546 | 98 | `ERROR` | `=(` |
| 546 | 125 | `ERROR` | `)` |

#### `framework/model/network/wifi/core/module/wifi_base.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 198 | `missing identifier` | `` |
| 98 | 198 | `missing identifier` | `` |
| 102 | 198 | `missing identifier` | `` |
| 106 | 198 | `missing identifier` | `` |
| 110 | 198 | `missing identifier` | `` |
| 114 | 198 | `missing identifier` | `` |
| 118 | 198 | `missing identifier` | `` |
| 122 | 198 | `missing identifier` | `` |
| 126 | 198 | `missing identifier` | `` |
| 130 | 198 | `missing identifier` | `` |
| 153 | 198 | `missing identifier` | `` |
| 162 | 198 | `missing identifier` | `` |
| 198 | 198 | `missing identifier` | `` |
| 207 | 198 | `missing identifier` | `` |
| 260 | 198 | `missing identifier` | `` |
| 269 | 198 | `missing identifier` | `` |
| 304 | 198 | `missing identifier` | `` |
| 309 | 211 | `ERROR` | `,` |
| 336 | 198 | `missing identifier` | `` |
| 379 | 198 | `missing identifier` | `` |
| … | … | … | *(33 more)* |

#### `framework/model/network/wifi/platform/src/message/message_dispatcher.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 93 | 128 | `ERROR` | `context-> requestType` |
| 125 | 125 | `ERROR` | `context-> commandId` |
| 203 | 102 | `ERROR` | `context-> requestType` |

#### `framework/model/network/wifi/platform/src/message/sidecar.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 200 | 97 | `ERROR` | `data-> serviceId` |
| 254 | 118 | `ERROR` | `def-> serviceId` |

#### `framework/model/sensor/driver/accel/sensor_accel_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 395 | 65 | `missing ;` | `` |
| 395 | 105 | `ERROR` | `=(` |
| 395 | 139 | `ERROR` | `)` |

#### `framework/model/sensor/driver/accel/sensor_gravity_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 322 | 67 | `missing ;` | `` |
| 322 | 107 | `ERROR` | `=(` |
| 322 | 143 | `ERROR` | `)` |

#### `framework/model/sensor/driver/als/sensor_als_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 516 | 63 | `missing ;` | `` |
| 516 | 103 | `ERROR` | `=(` |
| 516 | 135 | `ERROR` | `)` |

#### `framework/model/sensor/driver/barometer/sensor_barometer_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 362 | 69 | `missing ;` | `` |
| 362 | 109 | `ERROR` | `=(` |
| 362 | 147 | `ERROR` | `)` |

#### `framework/model/sensor/driver/common/src/sensor_config_parser.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 453 | 119 | `ERROR` | `,` |

#### `framework/model/sensor/driver/common/src/sensor_device_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 406 | 67 | `missing ;` | `` |
| 406 | 107 | `ERROR` | `=(` |
| 406 | 143 | `ERROR` | `)` |

#### `framework/model/sensor/driver/gas/sensor_gas_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 382 | 63 | `missing ;` | `` |
| 382 | 103 | `ERROR` | `=(` |
| 382 | 135 | `ERROR` | `)` |

#### `framework/model/sensor/driver/gyro/sensor_gyro_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 372 | 64 | `missing ;` | `` |
| 372 | 104 | `ERROR` | `=(` |
| 372 | 137 | `ERROR` | `)` |

#### `framework/model/sensor/driver/hall/sensor_hall_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 478 | 64 | `missing ;` | `` |
| 478 | 104 | `ERROR` | `=(` |
| 478 | 137 | `ERROR` | `)` |

#### `framework/model/sensor/driver/humidity/sensor_humidity_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 373 | 68 | `missing ;` | `` |
| 373 | 108 | `ERROR` | `=(` |
| 373 | 145 | `ERROR` | `)` |

#### `framework/model/sensor/driver/magnetic/sensor_magnetic_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 370 | 68 | `missing ;` | `` |
| 370 | 108 | `ERROR` | `=(` |
| 370 | 145 | `ERROR` | `)` |

#### `framework/model/sensor/driver/pedometer/sensor_pedometer_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 363 | 69 | `missing ;` | `` |
| 363 | 109 | `ERROR` | `=(` |
| 363 | 147 | `ERROR` | `)` |

#### `framework/model/sensor/driver/ppg/sensor_ppg_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 36 col 164 (missing )) ` |
| 36 | 209 | `ERROR` | `[0])` |
| 321 | 63 | `missing ;` | `` |
| 321 | 103 | `ERROR` | `=(` |
| 321 | 135 | `ERROR` | `)` |

#### `framework/model/sensor/driver/proximity/sensor_proximity_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 362 | 69 | `missing ;` | `` |
| 362 | 109 | `ERROR` | `=(` |
| 362 | 147 | `ERROR` | `)` |

#### `framework/model/sensor/driver/temperature/sensor_temperature_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 375 | 71 | `missing ;` | `` |
| 375 | 111 | `ERROR` | `=(` |
| 375 | 151 | `ERROR` | `)` |

#### `framework/model/storage/src/mmc/mmc_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 553 col 157 (missing )) ` |
| 99 | 116 | `ERROR` | `cntlr-> index` |
| 222 | 111 | `ERROR` | `cntlr-> index` |
| 553 | 114 | `ERROR` | `uint32_t` |
| 553 | 203 | `ERROR` | `devType)` |
| 713 | 126 | `ERROR` | `cntlr-> index` |
| 732 | 130 | `ERROR` | `cntlr-> index` |
| 762 | 100 | `ERROR` | `cntlr-> index` |
| 766 | 102 | `ERROR` | `cntlr-> index` |
| 813 | 129 | `ERROR` | `1` |
| 817 | 141 | `ERROR` | `1` |
| 1012 | 101 | `ERROR` | `cntlr-> index` |
| 1019 | 102 | `ERROR` | `cntlr-> devType` |
| 1039 | 111 | `ERROR` | `mmc-> secSize` |
| 1043 | 108 | `ERROR` | `mmc-> capacity` |
| 1047 | 110 | `ERROR` | `mmc-> eraseSize` |

#### `framework/model/storage/src/mmc/mmc_protocol.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 1610 | 1 | `ERROR` | `(cntlr-> curDev-> state .bits .highSpeed? "High speed": "")` |
| 2059 | 116 | `ERROR` | `csd-> structure` |
| 2613 | 1 | `ERROR` | `(cntlr-> curDev-> state .bits .highSpeed? "High speed": "")` |
| 3955 | 107 | `ERROR` | `cntlr-> index` |
| 3959 | 107 | `ERROR` | `cntlr-> index` |

#### `framework/model/storage/src/mmc/mmc_sdio.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 426 | 124 | `ERROR` | `dev-> functions` |

#### `framework/model/storage/src/mtd/mtd_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 53 col 173 (missing )) ` |
| 0 | 0 | `?` | `line 53 col 662 (missing )) ` |
| 0 | 0 | `?` | `line 53 col 867 (missing )) ` |
| 0 | 0 | `?` | `line 53 col 1073 (missing )) ` |
| 0 | 0 | `?` | `line 53 col 1278 (missing )) ` |
| 0 | 0 | `?` | `line 53 col 1481 (missing )) ` |
| 8 | 114 | `ERROR` | `mtdDevice-> index` |
| 18 | 116 | `ERROR` | `mtdDevice-> type` |
| 23 | 113 | `ERROR` | `mtdDevice-> idLen` |
| 28 | 117 | `ERROR` | `mtdDevice-> capacity` |
| 33 | 119 | `ERROR` | `mtdDevice-> eraseSize` |
| 38 | 119 | `ERROR` | `mtdDevice-> writeSize` |
| 43 | 118 | `ERROR` | `mtdDevice-> readSize` |
| 53 | 268 | `ERROR` | `)` |
| 53 | 706 | `ERROR` | `-> capacity)` |
| 53 | 911 | `ERROR` | `-> eraseSize)` |
| 53 | 1117 | `ERROR` | `-> writeSize)` |
| 53 | 1322 | `ERROR` | `-> readSize)` |
| 53 | 1525 | `ERROR` | `-> oobSize)` |
| 53 | 1539 | `ERROR` | `}` |

#### `framework/model/storage/src/mtd/mtd_spi_common.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 13 col 173 (missing )) ` |
| 0 | 0 | `?` | `line 13 col 662 (missing )) ` |
| 0 | 0 | `?` | `line 13 col 867 (missing )) ` |
| 0 | 0 | `?` | `line 13 col 1073 (missing )) ` |
| 0 | 0 | `?` | `line 13 col 1278 (missing )) ` |
| 0 | 0 | `?` | `line 13 col 1481 (missing )) ` |
| 13 | 268 | `ERROR` | `)` |
| 13 | 706 | `ERROR` | `-> capacity)` |
| 13 | 911 | `ERROR` | `-> eraseSize)` |
| 13 | 1117 | `ERROR` | `-> writeSize)` |
| 13 | 1322 | `ERROR` | `-> readSize)` |
| 13 | 1525 | `ERROR` | `-> oobSize)` |
| 13 | 1539 | `ERROR` | `}` |
| 16 | 120 | `ERROR` | `spi-> cs` |
| 19 | 157 | `ERROR` | `cfg-> ifType` |
| 21 | 158 | `ERROR` | `cfg-> ifType` |
| 23 | 158 | `ERROR` | `cfg-> ifType` |
| 119 | 2 | `missing }` | `` |

#### `framework/sample/platform/uart/src/uart_sample.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 67 | `missing ;` | `` |
| 22 | 107 | `ERROR` | `=(` |
| 22 | 143 | `ERROR` | `)` |

#### `framework/support/platform/include/fwk/platform_device.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 79 | `ERROR` | `, . . .` |

#### `framework/support/platform/include/fwk/platform_errno.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 1 | `ERROR` | `enum PlatformErrno{ HDF_PLT_ERR_OS_API= HDF_ERR_BSP_PLT_API_ERR , HDF_PLT_ERR_OPEN_DEV= HDF_PAL_ERR_DEV_CREATE , HDF_PLT…` |

#### `framework/support/platform/include/fwk/platform_trace.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 17 | `ERROR` | `, . . .` |

#### `framework/support/platform/src/adc/adc_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 125 | 113 | `ERROR` | `device-> devNum` |
| 140 | 128 | `ERROR` | `device-> devNum` |
| 156 | 117 | `ERROR` | `device-> devNum` |
| 171 | 129 | `ERROR` | `device-> devNum` |
| 494 | 61 | `missing ;` | `` |
| 494 | 101 | `ERROR` | `=(` |
| 494 | 131 | `ERROR` | `)` |

#### `framework/support/platform/src/can/can_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 105 | 116 | `ERROR` | `cntlr-> number` |
| 202 | 114 | `ERROR` | `cntlr-> number` |
| 219 | 114 | `ERROR` | `cntlr-> number` |

#### `framework/support/platform/src/can/can_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 120 | `ERROR` | `cntlr-> number` |

#### `framework/support/platform/src/clock/clock_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 120 | `ERROR` | `device-> deviceIndex` |
| 74 | 132 | `ERROR` | `device-> deviceIndex` |
| 78 | 131 | `ERROR` | `device-> deviceIndex` |
| 91 | 119 | `ERROR` | `device-> deviceIndex` |
| 106 | 133 | `ERROR` | `device-> deviceIndex` |
| 849 | 63 | `missing ;` | `` |
| 849 | 103 | `ERROR` | `=(` |
| 849 | 135 | `ERROR` | `)` |

#### `framework/support/platform/src/dac/dac_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 113 | `ERROR` | `device-> devNum` |
| 86 | 129 | `ERROR` | `device-> devNum` |
| 106 | 117 | `ERROR` | `device-> devNum` |
| 121 | 129 | `ERROR` | `device-> devNum` |
| 505 | 61 | `missing ;` | `` |
| 505 | 101 | `ERROR` | `=(` |
| 505 | 131 | `ERROR` | `)` |

#### `framework/support/platform/src/dma/dmac_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 118 | `ERROR` | `cntlr-> channelNum` |
| 295 | 129 | `ERROR` | `cntlr-> maxTransSize` |
| 480 | 114 | `ERROR` | `msg-> transType` |
| 507 | 119 | `ERROR` | `cntlr-> irq` |
| 535 | 119 | `ERROR` | `cntlr-> irq` |

#### `framework/support/platform/src/fwk/platform_device.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 143 | `ERROR` | `device-> number` |
| 39 | 79 | `ERROR` | `, . . .` |
| 190 | 131 | `ERROR` | `device-> number` |
| 192 | 134 | `ERROR` | `device-> number` |
| 218 | 151 | `ERROR` | `manager-> device .name` |
| 225 | 134 | `ERROR` | `device-> number` |
| 227 | 140 | `ERROR` | `device-> number` |

#### `framework/support/platform/src/fwk/platform_dumper.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 153 | 117 | `ERROR` | `data-> data .type` |
| 200 | 128 | `ERROR` | `node-> data .name` |
| 245 | 112 | `ERROR` | `data-> name` |
| 252 | 118 | `ERROR` | `data-> name` |
| 260 | 117 | `ERROR` | `data-> name` |
| 295 | 122 | `ERROR` | `dumper-> name` |
| 362 | 138 | `ERROR` | `dumper-> name` |

#### `framework/support/platform/src/fwk/platform_listener_u.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 171 | 132 | `ERROR` | `manager-> moudle` |
| 224 | 133 | `ERROR` | `manager-> moudle` |

#### `framework/support/platform/src/fwk/platform_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 124 | 165 | `ERROR` | `device-> number` |
| 129 | 166 | `ERROR` | `device-> number` |
| 164 | 153 | `ERROR` | `device-> number` |
| 177 | 146 | `ERROR` | `device-> number` |
| 180 | 151 | `ERROR` | `device-> number` |
| 195 | 132 | `ERROR` | `device-> name` |
| 219 | 149 | `ERROR` | `device-> number` |
| 232 | 147 | `ERROR` | `device-> number` |
| 234 | 152 | `ERROR` | `device-> number` |

#### `framework/support/platform/src/fwk/platform_queue.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 146 | 111 | `ERROR` | `queue-> name` |

#### `framework/support/platform/src/fwk/platform_trace_unopen.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 87 | `ERROR` | `, . . .` |

#### `framework/support/platform/src/gpio/gpio_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 105 | 129 | `ERROR` | `cntlr-> start` |
| 113 | 131 | `ERROR` | `cntlr-> start` |
| 145 | 124 | `ERROR` | `irqRecord-> global` |
| 167 | 127 | `ERROR` | `cntlr-> start` |
| 222 | 121 | `ERROR` | `GpioInfoToGlobal(ginfo)` |
| 286 | 124 | `ERROR` | `cntlr-> start` |
| 292 | 122 | `ERROR` | `cntlr-> start` |

#### `framework/support/platform/src/gpio/gpio_manager.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 150 | `ERROR` | `cntlr-> start` |
| 43 | 155 | `ERROR` | `cntlr-> start` |
| 54 | 117 | `ERROR` | `cntlr-> start` |
| 59 | 140 | `ERROR` | `cntlr-> count` |
| 166 | 112 | `ERROR` | `cntlr-> count` |

#### `framework/support/platform/src/gpio/gpio_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 397 | 62 | `missing ;` | `` |
| 397 | 102 | `ERROR` | `=(` |
| 397 | 133 | `ERROR` | `)` |

#### `framework/support/platform/src/hdmi/hdmi_cec.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 389 | 105 | `ERROR` | `digital-> system` |
| 452 | 104 | `ERROR` | `src-> type` |
| 1018 | 114 | `ERROR` | `cmd-> cmdType` |
| 1467 | 208 | `ERROR` | `0xf` |

#### `framework/support/platform/src/hdmi/hdmi_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 398 | 116 | `ERROR` | `cntlr-> cap .audioIfType` |
| 405 | 118 | `ERROR` | `cntlr-> cap .audioBitDepth` |
| 412 | 120 | `ERROR` | `cntlr-> cap .audioSampleRate` |
| 419 | 118 | `ERROR` | `cntlr-> cap .audioChannels` |
| 433 | 119 | `ERROR` | `cntlr-> cap .hdrColorimetry` |
| 440 | 116 | `ERROR` | `cntlr-> cap .hdrUserMode` |
| 841 | 101 | `ERROR` | `commAttr-> colorSpace` |
| 1150 | 100 | `ERROR` | `curCommAttr-> deepColor` |
| 1257 | 120 | `ERROR` | `cntlr-> attr .hdrAttr .mode` |

#### `framework/support/platform/src/hdmi/hdmi_dfm.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 114 | `ERROR` | `param-> packetType` |

#### `framework/support/platform/src/hdmi/hdmi_edid.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 92 | `ERROR` | `data[HDMI_EDID_CHECKSUM_ADDR]` |
| 116 | 95 | `ERROR` | `sinkCap-> verInfo .version` |
| 119 | 96 | `ERROR` | `sinkCap-> verInfo .revision` |
| 131 | 102 | `ERROR` | `block-> width` |
| 297 | 111 | `ERROR` | `cap-> preTimingCnt` |
| 366 | 99 | `ERROR` | `sinkCap-> extBlockNum` |
| 368 | 109 | `ERROR` | `sinkCap-> extBlockNum` |
| 778 | 105 | `ERROR` | `data[3]` |
| 1183 | 90 | `ERROR` | `data[0]` |
| 1187 | 98 | `ERROR` | `data[1]` |

#### `framework/support/platform/src/hdmi/hdmi_frl.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 223 | 106 | `ERROR` | `videoInfo-> vic` |
| 232 | 100 | `ERROR` | `frl-> info .maxFrlRate` |
| 495 | 100 | `ERROR` | `frl-> info .curFrlRate` |
| 778 | 90 | `ERROR` | `frl-> info .curFrlRate` |

#### `framework/support/platform/src/hdmi/hdmi_infoframe.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 368 | 100 | `ERROR` | `infoFrame-> header .type` |
| 376 | 97 | `ERROR` | `infoFrame-> header .type` |
| 516 | 94 | `ERROR` | `audioAttr-> channels` |

#### `framework/support/platform/src/hdmi/hdmi_scdc.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 360 | 115 | `ERROR` | `videoAttr-> tmdsClock` |

#### `framework/support/platform/src/i2c/i2c_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 112 | `ERROR` | `cntlr-> busId` |
| 56 | 126 | `ERROR` | `cntlr-> busId` |
| 72 | 116 | `ERROR` | `cntlr-> busId` |
| 86 | 124 | `ERROR` | `cntlr-> busId` |
| 269 | 61 | `missing ;` | `` |
| 269 | 101 | `ERROR` | `=(` |
| 269 | 131 | `ERROR` | `)` |

#### `framework/support/platform/src/i2c/i2c_if.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 26 col 166 (missing )) ` |
| 26 | 127 | `ERROR` | `NULL` |
| 26 | 229 | `ERROR` | `)` |
| 31 | 2 | `missing }` | `` |

#### `framework/support/platform/src/i2c/i2c_if_u.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 213 | 127 | `ERROR` | `msgs[i] .len` |
| 240 | 111 | `ERROR` | `msg-> len` |

#### `framework/support/platform/src/i3c/i3c_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 290 | 121 | `ERROR` | `device-> pid` |
| 328 | 111 | `ERROR` | `cntlr-> busId` |
| 343 | 126 | `ERROR` | `cntlr-> busId` |
| 359 | 116 | `ERROR` | `cntlr-> busId` |
| 373 | 124 | `ERROR` | `cntlr-> busId` |
| 755 | 61 | `missing ;` | `` |
| 755 | 101 | `ERROR` | `=(` |
| 755 | 131 | `ERROR` | `)` |

#### `framework/support/platform/src/i3c/i3c_if.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 25 col 165 (missing )) ` |
| 25 | 126 | `ERROR` | `NULL` |
| 25 | 228 | `ERROR` | `)` |
| 51 | 32 | `ERROR` | `handle` |
| 51 | 41 | `ERROR` | `struct` |
| 56 | 33 | `ERROR` | `handle` |
| 56 | 51 | `ERROR` | `addr` |
| 56 | 69 | `ERROR` | `func` |
| 56 | 85 | `ERROR` | `payload` |
| 61 | 30 | `ERROR` | `handle` |
| 61 | 48 | `ERROR` | `addr` |

#### `framework/support/platform/src/pin/pin_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 108 | `ERROR` | `cntlr-> pinCount` |
| 159 | 126 | `ERROR` | `desc-> pinName` |
| 740 | 61 | `missing ;` | `` |
| 740 | 101 | `ERROR` | `=(` |
| 740 | 131 | `ERROR` | `)` |

#### `framework/support/platform/src/pwm/pwm_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 103 | `ERROR` | `pwm-> num` |

#### `framework/support/platform/src/regulator/regulator_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 64 | 146 | `ERROR` | `pos-> regulatorInfo .name` |
| 68 | 112 | `ERROR` | `pos-> regulatorInfo .parentName` |
| 98 | 139 | `ERROR` | `node-> regulatorInfo .name` |
| 105 | 141 | `ERROR` | `node-> regulatorInfo .name` |
| 111 | 133 | `ERROR` | `node-> regulatorInfo .name` |
| 131 | 140 | `ERROR` | `pos-> regulatorInfo .parentName` |
| 140 | 136 | `ERROR` | `parent-> regulatorInfo .name` |
| 149 | 112 | `ERROR` | `node-> regulatorInfo .name` |
| 159 | 117 | `ERROR` | `node-> regulatorInfo .name` |
| 164 | 123 | `ERROR` | `node-> regulatorInfo .name` |
| 170 | 108 | `ERROR` | `node-> regulatorInfo .name` |
| 177 | 110 | `ERROR` | `node-> regulatorInfo .name` |
| 189 | 154 | `ERROR` | `node-> regulatorInfo .name` |
| 193 | 136 | `ERROR` | `node-> regulatorInfo .name` |
| 205 | 125 | `ERROR` | `node-> regulatorInfo .name` |
| 220 | 119 | `ERROR` | `node-> regulatorInfo .name` |
| 231 | 115 | `ERROR` | `node-> regulatorInfo .name` |
| 243 | 131 | `ERROR` | `node-> regulatorInfo .name` |
| 247 | 124 | `ERROR` | `node-> regulatorInfo .name` |
| 272 | 126 | `ERROR` | `pos-> regulatorInfo .name` |
| … | … | … | *(38 more)* |

#### `framework/support/platform/src/regulator/regulator_if.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 92 | 120 | `ERROR` | `node-> regulatorInfo .name` |
| 133 | 120 | `ERROR` | `node-> regulatorInfo .name` |

#### `framework/support/platform/src/regulator/regulator_tree_mgr.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 124 | `ERROR` | `pRegulator-> name` |
| 179 | 115 | `ERROR` | `nodeInfo-> child-> regulatorInfo .name` |
| 184 | 130 | `ERROR` | `node-> regulatorInfo .name` |
| 193 | 137 | `ERROR` | `nodeInfo-> child-> regulatorInfo .name` |
| 317 | 130 | `ERROR` | `parent-> regulatorInfo .name` |
| 355 | 110 | `ERROR` | `pos-> name` |
| 357 | 119 | `ERROR` | `pos-> name` |
| 361 | 110 | `ERROR` | `pos-> name` |

#### `framework/support/platform/src/rtc/rtc_base.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 174 | `ERROR` | `time-> month` |
| 137 | 185 | `ERROR` | `time-> month` |

#### `framework/support/platform/src/spi/spi_if_u.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 99 | 111 | `ERROR` | `msg-> len` |

#### `framework/support/platform/src/timer/timer_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 114 | `ERROR` | `cntrl-> info .number` |
| 46 | 106 | `ERROR` | `cntrl-> info .number` |
| 60 | 112 | `ERROR` | `cntrl-> info .number` |
| 64 | 102 | `ERROR` | `cntrl-> info .number` |
| 80 | 116 | `ERROR` | `cntrl-> info .number` |
| 84 | 110 | `ERROR` | `cntrl-> info .number` |
| 100 | 112 | `ERROR` | `cntrl-> info .number` |
| 114 | 114 | `ERROR` | `cntrl-> info .number` |
| 118 | 106 | `ERROR` | `cntrl-> info .number` |
| 144 | 113 | `ERROR` | `cntrl-> info .number` |
| 148 | 104 | `ERROR` | `cntrl-> info .number` |
| 467 | 110 | `ERROR` | `pos-> info .number` |
| 490 | 108 | `ERROR` | `cntrl-> info .number` |
| 497 | 112 | `ERROR` | `cntrl-> info .number` |
| 502 | 112 | `ERROR` | `cntrl-> info .number` |
| 508 | 119 | `ERROR` | `cntrl-> info .number` |
| 528 | 116 | `ERROR` | `pos-> info .number` |
| 604 | 63 | `missing ;` | `` |
| 604 | 103 | `ERROR` | `=(` |
| 604 | 135 | `ERROR` | `)` |

#### `framework/support/platform/src/timer/timer_if_u.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 219 | 127 | `ERROR` | `param-> handle` |

#### `framework/support/platform/src/uart/uart_if_u.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 275 | 141 | `ERROR` | `(* attribute)` |

#### `framework/support/platform/src/uart/uart_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 111 | 143 | `ERROR` | `(* attribute)` |

#### `framework/test/unittest/common/hdf_main_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 108 | 59 | `missing ;` | `` |
| 108 | 99 | `ERROR` | `=(` |
| 108 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/manager/sample_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 284 | 63 | `missing ;` | `` |
| 284 | 103 | `ERROR` | `=(` |
| 284 | 135 | `ERROR` | `)` |

#### `framework/test/unittest/model/audio/src/audio_host_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 109 | `ERROR` | `g_audioServiceName[i]` |

#### `framework/test/unittest/model/audio/src/hdf_audio_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 110 col 171 (missing )) ` |
| 110 | 217 | `ERROR` | `, msg-> subCmd)` |

#### `framework/test/unittest/model/network/wifi/unittest/message/hdf_single_node_message_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 94 | `ERROR` | `context-> commandId` |
| 47 | 101 | `ERROR` | `context-> commandId` |
| 195 | 197 | `ERROR` | `#` |
| 202 | 95 | `ERROR` | `diffTime .sec` |
| 234 | 105 | `ERROR` | `context-> commandId` |
| 249 | 210 | `ERROR` | `#` |
| 298 | 214 | `ERROR` | `#` |
| 310 | 95 | `ERROR` | `diffTime .sec` |

#### `framework/test/unittest/model/network/wifi/unittest/net/hdf_netbuf_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 661 col 180 (missing )) ` |
| 0 | 0 | `?` | `line 680 col 185 (missing )) ` |
| 215 | 107 | `ERROR` | `NetBufGetRoom(nb , E_TAIL_BUF)` |
| 387 | 100 | `ERROR` | `NetBufGetDataLen(nb)` |
| 422 | 100 | `ERROR` | `NetBufGetDataLen(nb)` |
| 467 | 110 | `ERROR` | `NetBufQueueSize(& q)` |
| 476 | 100 | `ERROR` | `NetBufGetDataLen(nb1)` |
| 513 | 110 | `ERROR` | `NetBufQueueSize(& q)` |
| 522 | 110 | `ERROR` | `NetBufQueueSize(& q)` |
| 528 | 100 | `ERROR` | `NetBufGetDataLen(nb1)` |
| 535 | 100 | `ERROR` | `NetBufGetDataLen(nb2)` |
| 572 | 110 | `ERROR` | `NetBufQueueSize(& q)` |
| 616 | 110 | `ERROR` | `NetBufQueueSize(& q)` |
| 661 | 124 | `ERROR` | `1` |
| 661 | 251 | `missing "` | `` |
| 680 | 129 | `ERROR` | `1` |
| 680 | 256 | `missing "` | `` |

#### `framework/test/unittest/model/usb/device/src/hdf_usb_device_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 141 col 176 (missing )) ` |
| 141 | 222 | `ERROR` | `, msg-> subCmd)` |

#### `framework/test/unittest/model/usb/host/src/usb_raw_io.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 152 | 100 | `ERROR` | `req-> status` |

#### `framework/test/unittest/osal/osal_all_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 354 | 141 | `ERROR` | `,(uint32_t` |
| 354 | 225 | `missing ;` | `` |
| 354 | 239 | `ERROR` | `)` |
| 521 | 100 | `ERROR` | `,(uint32_t` |
| 521 | 185 | `missing ;` | `` |
| 521 | 228 | `ERROR` | `)` |
| 528 | 100 | `ERROR` | `,(uint32_t` |
| 528 | 185 | `missing ;` | `` |
| 528 | 228 | `ERROR` | `)` |
| 534 | 100 | `ERROR` | `,(uint32_t` |
| 534 | 185 | `missing ;` | `` |
| 534 | 228 | `ERROR` | `)` |
| 541 | 100 | `ERROR` | `,(uint32_t` |
| 541 | 185 | `missing ;` | `` |
| 541 | 228 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/adc_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 117 | 58 | `missing ;` | `` |
| 117 | 98 | `ERROR` | `=(` |
| 117 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/adc_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 129 | `ERROR` | `(* config)` |
| 78 | 113 | `ERROR` | `tester .config .devNum` |
| 107 | 109 | `ERROR` | `tester-> config .devNum` |

#### `framework/test/unittest/platform/common/can_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 132 | `ERROR` | `(* config)` |
| 96 | 136 | `ERROR` | `config-> busNum` |
| 110 | 223 | `ERROR` | `NULL` |
| 131 | 233 | `ERROR` | `NULL` |
| 183 | 87 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 195 | 103 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 202 | 267 | `ERROR` | `CanBusReadMsg(g_handle ,& msg , 0)` |
| 218 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 219 | 99 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 223 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 236 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 237 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 238 | 99 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 243 | 103 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/private/tmp/corpora/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "…` |
| 324 | 219 | `ERROR` | `NULL` |
| 340 | 219 | `ERROR` | `NULL` |
| 355 | 219 | `ERROR` | `NULL` |
| 369 | 219 | `ERROR` | `NULL` |
| 387 | 209 | `ERROR` | `#(` |
| 387 | 287 | `ERROR` | `)` |
| … | … | … | *(29 more)* |

#### `framework/test/unittest/platform/common/clock_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 60 | `missing ;` | `` |
| 94 | 100 | `ERROR` | `=(` |
| 94 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/clock_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 131 | `ERROR` | `(* config)` |
| 77 | 117 | `ERROR` | `tester .config .deviceIndex` |
| 99 | 115 | `ERROR` | `tester-> config .deviceIndex` |

#### `framework/test/unittest/platform/common/dac_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 118 | 58 | `missing ;` | `` |
| 118 | 98 | `ERROR` | `=(` |
| 118 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/dac_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 129 | `ERROR` | `(* config)` |
| 74 | 113 | `ERROR` | `tester .config .devNum` |
| 97 | 118 | `ERROR` | `value[i]` |

#### `framework/test/unittest/platform/common/emmc_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 102 | 116 | `ERROR` | `tester-> busNum` |
| 156 | 59 | `missing ;` | `` |
| 156 | 99 | `ERROR` | `=(` |
| 156 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/gpio_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 152 | 59 | `missing ;` | `` |
| 152 | 99 | `ERROR` | `=(` |
| 152 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/gpio_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 134 | `ERROR` | `(* config)` |
| 78 | 175 | `ERROR` | `tester .cfg .gpio` |
| 359 | 145 | `ERROR` | `tester-> cfg .testNameOne` |
| 365 | 79 | `ERROR` | `tester-> cfg .testNameOne` |
| 365 | 110 | `ERROR` | `,(uint16_t` |
| 365 | 195 | `missing ;` | `` |
| 365 | 220 | `ERROR` | `)` |
| 371 | 145 | `ERROR` | `tester-> cfg .testNameTwo` |
| 377 | 80 | `ERROR` | `tester-> cfg .testNameTwo` |
| 377 | 111 | `ERROR` | `,(uint16_t` |
| 377 | 196 | `missing ;` | `` |
| 377 | 228 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/hdmi_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 234 | 105 | `ERROR` | `tester-> busNum` |
| 287 | 59 | `missing ;` | `` |
| 287 | 99 | `ERROR` | `=(` |
| 287 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/i2c_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 116 | 58 | `missing ;` | `` |
| 116 | 98 | `ERROR` | `=(` |
| 116 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/i2c_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 80 | 109 | `ERROR` | `tester .config .busNum` |
| 134 | 154 | `ERROR` | `cfg-> busNum` |

#### `framework/test/unittest/platform/common/i2s_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 83 | `ERROR` | `test-> sampleRate` |
| 49 | 83 | `ERROR` | `test-> sampleRate` |
| 233 | 109 | `ERROR` | `test-> rbuf[0]` |
| 259 | 108 | `ERROR` | `test-> rbuf[0]` |
| 498 | 58 | `missing ;` | `` |
| 498 | 98 | `ERROR` | `=(` |
| 498 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/i3c_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 117 | 58 | `missing ;` | `` |
| 117 | 98 | `ERROR` | `=(` |
| 117 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/i3c_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 289 col 137 (missing )) ` |
| 51 | 133 | `ERROR` | `(* config)` |
| 85 | 119 | `ERROR` | `tester .config .busId` |
| 141 | 154 | `ERROR` | `cfg-> busId` |
| 236 | 123 | `ERROR` | `tester-> config .busId` |
| 271 | 123 | `ERROR` | `tester-> config .busId` |
| 289 | 97 | `ERROR` | `char*` |
| 289 | 182 | `missing ;` | `` |
| 289 | 192 | `ERROR` | `)` |
| 309 | 125 | `ERROR` | `tester-> config .busId` |
| 332 | 118 | `ERROR` | `tester-> config .busId` |

#### `framework/test/unittest/platform/common/mipi_csi_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 113 | `ERROR` | `test-> cntlrId` |
| 33 | 106 | `ERROR` | `test-> cntlrId` |
| 186 | 117 | `ERROR` | `attr .inputMode` |
| 489 | 62 | `missing ;` | `` |
| 489 | 102 | `ERROR` | `=(` |
| 489 | 133 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/mipi_dsi_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 103 | `ERROR` | `test-> devNo` |
| 200 | 62 | `missing ;` | `` |
| 200 | 102 | `ERROR` | `=(` |
| 200 | 133 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pcie_bus_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 62 | `missing ;` | `` |
| 94 | 102 | `ERROR` | `=(` |
| 94 | 133 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pcie_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 91 | 59 | `missing ;` | `` |
| 91 | 99 | `ERROR` | `=(` |
| 91 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pcie_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 108 | `ERROR` | `tester .config .busNum` |

#### `framework/test/unittest/platform/common/pin_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 114 | 58 | `missing ;` | `` |
| 114 | 98 | `ERROR` | `=(` |
| 114 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pin_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 133 | `ERROR` | `(* config)` |
| 92 | 106 | `ERROR` | `tester .config .pinName` |
| 118 | 131 | `ERROR` | `tester-> config .PullTypeNum` |
| 125 | 132 | `ERROR` | `tester-> config .PullTypeNum` |
| 149 | 139 | `ERROR` | `tester-> config .strengthNum` |
| 152 | 128 | `ERROR` | `tester-> config .pinName` |
| 156 | 144 | `ERROR` | `tester-> config .strengthNum` |
| 177 | 149 | `ERROR` | `tester-> config .pinName` |
| 183 | 148 | `ERROR` | `tester-> config .pinName` |
| 185 | 140 | `ERROR` | `tester-> config .funcNameBuf` |
| 207 | 144 | `ERROR` | `cfg-> pinName` |
| 223 | 108 | `ERROR` | `g_oldPinCfg .funcName` |

#### `framework/test/unittest/platform/common/platform_device_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 221 | `ERROR` | `#(` |
| 21 | 299 | `ERROR` | `)` |
| 23 | 211 | `ERROR` | `#(` |
| 23 | 219 | `ERROR` | `(0)` |
| 23 | 279 | `ERROR` | `)` |
| 27 | 224 | `ERROR` | `#(` |
| 27 | 242 | `ERROR` | `(NULL)` |
| 27 | 305 | `ERROR` | `)` |
| 31 | 221 | `ERROR` | `#(` |
| 31 | 299 | `ERROR` | `)` |
| 33 | 211 | `ERROR` | `#(` |
| 33 | 219 | `ERROR` | `(0)` |
| 33 | 279 | `ERROR` | `)` |
| 54 | 221 | `ERROR` | `#(` |
| 54 | 299 | `ERROR` | `)` |
| 58 | 239 | `ERROR` | `#(` |
| 58 | 258 | `ERROR` | `(refCntBeforeGet+ 1)` |
| 58 | 335 | `ERROR` | `)` |
| 62 | 221 | `ERROR` | `#(` |
| 62 | 299 | `ERROR` | `)` |
| … | … | … | *(78 more)* |

#### `framework/test/unittest/platform/common/platform_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 63 | `missing ;` | `` |
| 44 | 103 | `ERROR` | `=(` |
| 44 | 135 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/platform_event_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 222 | `ERROR` | `#(` |
| 18 | 242 | `ERROR` | `(0)` |
| 18 | 302 | `ERROR` | `)` |
| 22 | 223 | `ERROR` | `#(` |
| 22 | 301 | `ERROR` | `)` |
| 29 | 223 | `ERROR` | `#(` |
| 29 | 301 | `ERROR` | `)` |
| 46 | 224 | `ERROR` | `#(` |
| 46 | 306 | `ERROR` | `)` |
| 50 | 220 | `ERROR` | `#(` |
| 50 | 298 | `ERROR` | `)` |
| 54 | 220 | `ERROR` | `#(` |
| 54 | 298 | `ERROR` | `)` |
| 55 | 229 | `ERROR` | `#(` |
| 55 | 316 | `ERROR` | `)` |
| 59 | 224 | `ERROR` | `#(` |
| 59 | 306 | `ERROR` | `)` |
| 63 | 220 | `ERROR` | `#(` |
| 63 | 298 | `ERROR` | `)` |
| 68 | 224 | `ERROR` | `#(` |
| … | … | … | *(34 more)* |

#### `framework/test/unittest/platform/common/platform_manager_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 73 | 222 | `ERROR` | `#(` |
| 73 | 300 | `ERROR` | `)` |
| 76 | 240 | `ERROR` | `#(` |
| 76 | 259 | `ERROR` | `(refCntBeforeAdd+ 1)` |
| 76 | 336 | `ERROR` | `)` |
| 81 | 222 | `ERROR` | `#(` |
| 81 | 300 | `ERROR` | `)` |
| 85 | 225 | `ERROR` | `#(` |
| 85 | 306 | `ERROR` | `)` |
| 90 | 225 | `ERROR` | `#(` |
| 90 | 306 | `ERROR` | `)` |
| 98 | 222 | `ERROR` | `#(` |
| 98 | 300 | `ERROR` | `)` |
| 104 | 222 | `ERROR` | `#(` |
| 104 | 300 | `ERROR` | `)` |
| 110 | 240 | `ERROR` | `#(` |
| 110 | 336 | `ERROR` | `)` |
| 114 | 222 | `ERROR` | `#(` |
| 114 | 237 | `ERROR` | `(NULL)` |
| 114 | 300 | `ERROR` | `)` |
| … | … | … | *(30 more)* |

#### `framework/test/unittest/platform/common/platform_queue_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 117 | `ERROR` | `msg-> code` |
| 58 | 223 | `ERROR` | `#(` |
| 58 | 301 | `ERROR` | `)` |
| 65 | 223 | `ERROR` | `#(` |
| 65 | 301 | `ERROR` | `)` |
| 69 | 232 | `ERROR` | `#(` |
| 69 | 319 | `ERROR` | `)` |
| 86 | 220 | `ERROR` | `#(` |
| 86 | 298 | `ERROR` | `)` |
| 89 | 220 | `ERROR` | `#(` |
| 89 | 298 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pwm_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 58 | `missing ;` | `` |
| 123 | 98 | `ERROR` | `=(` |
| 123 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pwm_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 131 | `ERROR` | `(* config)` |
| 74 | 113 | `ERROR` | `tester .config .num` |
| 100 | 110 | `ERROR` | `tester-> config .cfg .number` |
| 109 | 110 | `ERROR` | `tester-> config .cfg .number` |
| 150 | 123 | `ERROR` | `cfg .period` |
| 190 | 129 | `ERROR` | `tester-> config .cfg .polarity` |
| 198 | 129 | `ERROR` | `tester-> config .cfg .polarity` |

#### `framework/test/unittest/platform/common/regulator_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 66 | 118 | `ERROR` | `test-> maxUv` |
| 104 | 119 | `ERROR` | `test-> minUa` |
| 143 | 127 | `ERROR` | `test-> status` |
| 367 | 151 | `ERROR` | `test-> name` |
| 408 | 64 | `missing ;` | `` |
| 408 | 104 | `ERROR` | `=(` |
| 408 | 137 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/rtc_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 149 | 58 | `missing ;` | `` |
| 149 | 98 | `ERROR` | `=(` |
| 149 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/rtc_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 129 | `ERROR` | `(* config)` |

#### `framework/test/unittest/platform/common/sdio_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 133 | `ERROR` | `data[0]` |
| 46 | 135 | `ERROR` | `data[0]` |
| 124 | 147 | `ERROR` | `data[0]` |
| 131 | 146 | `ERROR` | `data[0]` |
| 244 | 124 | `ERROR` | `info .funcInfo .enTimeout` |
| 258 | 130 | `ERROR` | `info .funcInfo .enTimeout` |
| 395 | 148 | `ERROR` | `tester-> busNum` |
| 448 | 59 | `missing ;` | `` |
| 448 | 99 | `ERROR` | `=(` |
| 448 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/spi_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 131 | 58 | `missing ;` | `` |
| 131 | 98 | `ERROR` | `=(` |
| 131 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/spi_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 126 | `ERROR` | `(* cfg)` |
| 60 | 123 | `ERROR` | `config-> len` |
| 109 | 115 | `ERROR` | `tester .config .bus` |
| 227 | 125 | `ERROR` | `g_spiCfg .bitsPerWord` |
| 234 | 125 | `ERROR` | `g_spiCfg .bitsPerWord` |

#### `framework/test/unittest/platform/common/timer_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 109 | `ERROR` | `config-> number` |
| 109 | 60 | `missing ;` | `` |
| 109 | 100 | `ERROR` | `=(` |
| 109 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/timer_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 328 | 131 | `ERROR` | `(* config)` |

#### `framework/test/unittest/platform/common/uart_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 126 | 59 | `missing ;` | `` |
| 126 | 99 | `ERROR` | `=(` |
| 126 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/uart_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 122 | `ERROR` | `config-> len` |
| 105 | 113 | `ERROR` | `tester .config .port` |
| 128 | 99 | `ERROR` | `tester-> config .len` |
| 213 | 108 | `ERROR` | `attribute .dataBits` |
| 214 | 106 | `ERROR` | `attribute .parity` |
| 215 | 108 | `ERROR` | `attribute .stopBits` |
| 216 | 103 | `ERROR` | `attribute .rts` |
| 217 | 103 | `ERROR` | `attribute .cts` |
| 218 | 108 | `ERROR` | `attribute .fifoRxEn` |
| 219 | 108 | `ERROR` | `attribute .fifoTxEn` |

#### `framework/test/unittest/platform/common/watchdog_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 104 | 63 | `missing ;` | `` |
| 104 | 103 | `ERROR` | `=(` |
| 104 | 135 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/watchdog_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 138 | `ERROR` | `(* config)` |
| 81 | 125 | `ERROR` | `tester .config .id` |
| 114 | 125 | `ERROR` | `tester-> config .timeoutSet` |

#### `framework/test/unittest/platform/config/can_test_config.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 112 | 58 | `missing ;` | `` |
| 112 | 98 | `ERROR` | `=(` |
| 112 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/adc_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 87 | 141 | `ERROR` | `virtual-> devNum` |
| 120 | 131 | `ERROR` | `virtual-> devNum` |
| 168 | 67 | `missing ;` | `` |
| 168 | 107 | `ERROR` | `=(` |
| 168 | 143 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/clock_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 139 | 116 | `ERROR` | `clockDevice-> deviceIndex` |
| 202 | 137 | `ERROR` | `virtual-> deviceIndex` |
| 253 | 69 | `missing ;` | `` |
| 253 | 109 | `ERROR` | `=(` |
| 253 | 147 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/dac_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 117 | `ERROR` | `virtual-> deviceNum` |
| 176 | 60 | `missing ;` | `` |
| 176 | 100 | `ERROR` | `=(` |
| 176 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/i3c_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 125 | `ERROR` | `msg-> flags` |
| 62 | 115 | `ERROR` | `virtual-> busId` |
| 68 | 117 | `ERROR` | `ccc-> dest` |
| 248 | 108 | `ERROR` | `virtual-> IrqNum` |
| 427 | 60 | `missing ;` | `` |
| 427 | 100 | `ERROR` | `=(` |
| 427 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/pcie_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 206 | 68 | `missing ;` | `` |
| 206 | 108 | `ERROR` | `=(` |
| 206 | 145 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/pin_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 238 | 129 | `ERROR` | `virtual-> cntlr .pins[index] .pinName` |
| 259 | 115 | `ERROR` | `virtual-> number` |
| 269 | 116 | `ERROR` | `virtual-> pinCount` |
| 385 | 67 | `missing ;` | `` |
| 385 | 107 | `ERROR` | `=(` |
| 385 | 143 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/pwm_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 121 | `ERROR` | `config-> polarity` |
| 40 | 136 | `ERROR` | `config-> period` |
| 44 | 151 | `ERROR` | `config-> duty` |
| 84 | 166 | `ERROR` | `virtual-> dev .cfg .number` |
| 145 | 56 | `missing ;` | `` |
| 145 | 96 | `ERROR` | `=(` |
| 145 | 121 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/regulator_linux_current_virtual_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 141 | 145 | `ERROR` | `g_virtualCurrentRegulatorDesc .name` |

#### `framework/test/unittest/platform/virtual/regulator_linux_voltage_virtual_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 143 | 145 | `ERROR` | `g_virtualVoltageRegulatorDesc .name` |

#### `framework/test/unittest/platform/virtual/regulator_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 112 | `ERROR` | `node-> regulatorInfo .name` |
| 27 | 113 | `ERROR` | `node-> regulatorInfo .name` |
| 38 | 125 | `ERROR` | `node-> regulatorInfo .name` |
| 50 | 123 | `ERROR` | `node-> regulatorInfo .name` |
| 61 | 125 | `ERROR` | `node-> regulatorInfo .name` |
| 73 | 123 | `ERROR` | `node-> regulatorInfo .name` |
| 85 | 122 | `ERROR` | `node-> regulatorInfo .name` |
| 136 | 156 | `ERROR` | `regNode-> regulatorInfo .name` |
| 162 | 107 | `ERROR` | `regNode-> regulatorInfo .name` |
| 174 | 113 | `ERROR` | `regNode-> regulatorInfo .parentName` |
| 178 | 111 | `ERROR` | `regNode-> regulatorInfo .constraints .alwaysOn` |
| 267 | 66 | `missing ;` | `` |
| 267 | 106 | `ERROR` | `=(` |
| 267 | 141 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/spi_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 209 | 107 | `ERROR` | `cntlr-> curCs` |
| 226 | 107 | `ERROR` | `cntlr-> curCs` |
| 268 | 109 | `ERROR` | `cntlr-> curCs` |
| 455 | 62 | `missing ;` | `` |
| 455 | 102 | `ERROR` | `=(` |
| 455 | 133 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/watchdog_virtual.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 132 | 124 | `ERROR` | `HdfDeviceGetServiceName(device)` |
| 169 | 72 | `missing ;` | `` |
| 169 | 112 | `ERROR` | `=(` |
| 169 | 153 | `ERROR` | `)` |

#### `framework/test/unittest/pm/hdf_pm_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 598 | 59 | `missing ;` | `` |
| 598 | 99 | `ERROR` | `=(` |
| 598 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/sensor/hdf_sensor_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 248 | 64 | `missing ;` | `` |
| 248 | 104 | `ERROR` | `=(` |
| 248 | 137 | `ERROR` | `)` |

#### `framework/test/unittest/uevent/devmgr_uevent_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 63 | `missing ;` | `` |
| 60 | 103 | `ERROR` | `=(` |
| 60 | 135 | `ERROR` | `)` |

#### `framework/test/unittest/utils/hcs_parser/unittest/hcs_macro_cases.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 164 col 238 (missing )) ` |
| 0 | 0 | `?` | `line 164 col 560 (missing )) ` |
| 0 | 0 | `?` | `line 173 col 238 (missing )) ` |
| 0 | 0 | `?` | `line 173 col 560 (missing )) ` |
| 0 | 0 | `?` | `line 182 col 232 (missing )) ` |
| 0 | 0 | `?` | `line 182 col 548 (missing )) ` |
| 51 | 140 | `ERROR` | `stringListNamesGen[i]` |
| 121 | 145 | `ERROR` | `HCS_PROP(HCS_ROOT_audio_info , match_attr)` |
| 121 | 195 | `ERROR` | `,` |
| 122 | 142 | `ERROR` | `HCS_PROP(HCS_ROOT_audio_info , pa_identifier)` |
| 122 | 195 | `ERROR` | `,` |
| 128 | 143 | `ERROR` | `HCS_PROP(HCS_ROOT_audio_info , cust_name)` |
| 128 | 192 | `ERROR` | `,` |
| 129 | 141 | `ERROR` | `HCS_PROP(HCS_ROOT_audio_info , dual_smartpa_delay)` |
| 129 | 199 | `ERROR` | `,` |
| 131 | 127 | `ERROR` | `HCS_PROP(HCS_ROOT_audio_info , status)` |
| 131 | 173 | `ERROR` | `,` |
| 139 | 141 | `ERROR` | `HCS_PROP(FP_INFO_NODE_finger_info , product)` |
| 139 | 461 | `ERROR` | `HCS_PROP(FP_INFO_NODE_finger_info , chip)` |
| 145 | 148 | `ERROR` | `HCS_PROP(FP_INFO_NODE_audio_info , match_attr)` |
| … | … | … | *(44 more)* |

#### `framework/test/unittest/wifi/hdf_wifi_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 52 col 170 (missing )) ` |
| 52 | 216 | `ERROR` | `, msg-> subCmd)` |

#### `framework/tools/hdi-gen/ast/ast.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 18 | `ERROR` | `, }` |
| 16 | 7 | `ERROR` | `AST: public` |
| 18 | 18 | `ERROR` | `std:: unordered_map< std::` |
| 18 | 61 | `ERROR` | `< AST>>` |
| 19 | 22 | `ERROR` | `std:: unordered_map< std::` |
| 19 | 58 | `ERROR` | `AutoPtr<` |
| 19 | 74 | `ERROR` | `>>` |
| 21 | 3 | `ERROR` | `AST()` |
| 28 | 1 | `ERROR` | `ASTFileType GetASTFileType() const` |
| 33 | 26 | `ERROR` | `:: string&` |
| 35 | 11 | `ERROR` | `:: string` |
| 40 | 27 | `ERROR` | `:: string&` |
| 42 | 11 | `ERROR` | `:: string` |
| 47 | 11 | `ERROR` | `:: string` |
| 52 | 33 | `ERROR` | `:: string&` |
| 57 | 11 | `ERROR` | `:: string` |
| 62 | 30 | `ERROR` | `:: string&` |
| 64 | 39 | `ERROR` | `const std::` |
| 66 | 32 | `ERROR` | `< ASTNamespace> &` |
| 68 | 38 | `ERROR` | `const std::` |
| … | … | … | *(33 more)* |

#### `framework/tools/hdi-gen/lexer/lexer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 26 col 33 (missing )) ` |
| 0 | 0 | `?` | `line 28 col 32 (missing )) ` |
| 0 | 0 | `?` | `line 48 col 47 (missing )) ` |
| 10 | 12 | `ERROR` | `ParseMode{` |
| 13 | 1 | `ERROR` | `} ;` |
| 19 | 21 | `ERROR` | `:: string&` |
| 21 | 11 | `ERROR` | `:: string` |
| 26 | 39 | `ERROR` | `)` |
| 28 | 38 | `ERROR` | `)` |
| 48 | 21 | `ERROR` | `&` |
| 48 | 53 | `ERROR` | `)` |
| 50 | 24 | `ERROR` | `&` |
| 52 | 18 | `ERROR` | `&` |
| 54 | 19 | `ERROR` | `&` |
| 56 | 25 | `ERROR` | `&` |
| 58 | 22 | `ERROR` | `&` |
| 60 | 22 | `ERROR` | `&` |
| 62 | 22 | `ERROR` | `&` |
| 64 | 25 | `ERROR` | `&` |
| 66 | 27 | `ERROR` | `&` |
| … | … | … | *(11 more)* |

#### `framework/tools/hdi-gen/lexer/token.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 13 | `ERROR` | `, }` |
| 82 | 4 | `ERROR` | `:: string` |
| 90 | 4 | `ERROR` | `:: string` |
| 92 | 4 | `ERROR` | `:: string` |
| 96 | 28 | `ERROR` | `&` |
| 96 | 47 | `ERROR` | `&` |
| 100 | 1 | `ERROR` | `}` |
| 102 | 5 | `ERROR` | `:` |
| 102 | 33 | `ERROR` | `&` |
| 104 | 2 | `missing }` | `` |

#### `framework/tools/hdi-gen/parser/parser.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 67 | 42 | `missing type_identifier` | `` |
| 107 | 48 | `missing type_identifier` | `` |
| 114 | 50 | `missing type_identifier` | `` |
| 121 | 49 | `missing type_identifier` | `` |

#### `framework/tools/hdi-gen/util/logger.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 54 | `ERROR` | `, . . .` |
| 20 | 54 | `ERROR` | `, . . .` |
| 32 | 54 | `ERROR` | `, . . .` |

#### `framework/tools/hdi-gen/util/logger.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 52 | `ERROR` | `, . . .` |
| 11 | 52 | `ERROR` | `, . . .` |
| 13 | 52 | `ERROR` | `, . . .` |

#### `framework/tools/hdi-gen/util/string_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 77 | 64 | `ERROR` | `, . . .` |

#### `framework/tools/hdi-gen/util/string_builder.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 48 | `ERROR` | `, . . .` |

#### `framework/tools/hdi-gen/util/string_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 146 | 55 | `ERROR` | `, . . .` |

#### `framework/tools/hdi-gen/util/string_helper.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 47 | `ERROR` | `, . . .` |

#### `framework/utils/src/hcs_parser/hcs_tree_if.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 39 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 52 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 65 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 78 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 117 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 138 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 159 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 180 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 205 col 226 (missing )) ` |
| 0 | 0 | `?` | `line 205 col 472 (missing )) ` |
| 0 | 0 | `?` | `line 223 col 226 (missing )) ` |
| 0 | 0 | `?` | `line 223 col 472 (missing )) ` |
| 0 | 0 | `?` | `line 241 col 226 (missing )) ` |
| 0 | 0 | `?` | `line 241 col 472 (missing )) ` |
| 0 | 0 | `?` | `line 270 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 285 col 287 (missing )) ` |
| 0 | 0 | `?` | `line 300 col 160 (missing )) ` |
| 0 | 0 | `?` | `line 375 col 160 (missing )) ` |
| 49 | 22 | `ERROR` | `const struct` |
| 49 | 68 | `ERROR` | `char` |
| … | … | … | *(46 more)* |

#### `framework/utils/src/hdf_sbuf.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 84 col 433 (missing )) ` |
| 0 | 0 | `?` | `line 90 col 424 (missing )) ` |
| 0 | 0 | `?` | `line 122 col 436 (missing )) ` |
| 0 | 0 | `?` | `line 128 col 442 (missing )) ` |
| 0 | 0 | `?` | `line 138 col 452 (missing )) ` |
| 0 | 0 | `?` | `line 144 col 449 (missing )) ` |
| 0 | 0 | `?` | `line 150 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 156 col 442 (missing )) ` |
| 0 | 0 | `?` | `line 162 col 442 (missing )) ` |
| 0 | 0 | `?` | `line 168 col 442 (missing )) ` |
| 0 | 0 | `?` | `line 174 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 180 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 186 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 192 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 198 col 438 (missing )) ` |
| 0 | 0 | `?` | `line 204 col 442 (missing )) ` |
| 0 | 0 | `?` | `line 210 col 446 (missing )) ` |
| 0 | 0 | `?` | `line 216 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 222 col 440 (missing )) ` |
| 0 | 0 | `?` | `line 228 col 440 (missing )) ` |
| … | … | … | *(63 more)* |

#### `framework/utils/src/hdf_sbuf_impl_raw.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 511 col 150 (missing )) ` |
| 511 | 107 | `ERROR` | `uint32_t` |
| 511 | 196 | `ERROR` | `capacity)` |

#### `interfaces/inner_api/hdi/iservstat_listener_hdi.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 32 | `ERROR` | `"HDI.IServiceStatusListener.V1_0"` |

#### `interfaces/inner_api/utils/hdf_trace.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 17 | `ERROR` | `const std:: string& value , const std::` |
| 12 | 1 | `ERROR` | `{` |
| 15 | 7 | `ERROR` | `~` |

---

## hiviewdfx_hiview

Generated from `trace analyze /private/tmp/corpora/hiviewdfx_hiview` (344 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 344

### Failure categories

| Category | Files |
|----------|------:|
| generic ERROR nodes (mixed C++ constructs) | 279 |
| other / mixed | 54 |
| missing type identifiers (often macro-expanded types) | 7 |
| gtest/HWTEST macros (`missing ;`) | 2 |
| extern template instantiations | 2 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `adapter/plugins/eventservice/service/idl/dfx/include/listener_status_util.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 2 | `adapter/plugins/eventservice/service/idl/dfx/src/listener_status_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 3 | `adapter/plugins/eventservice/service/idl/include/iquery_sys_event_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 4 | `adapter/plugins/eventservice/service/idl/include/parcelable_vector_rw.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 5 | `adapter/plugins/eventservice/service/idl/src/ash_mem_utils.cpp` | tree-sitter-cpp node `missing identifier` at 11 site(s) | 11 |
| 6 | `adapter/plugins/eventservice/service/idl/src/compliant_event_checker.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 7 | `adapter/plugins/eventservice/service/idl/src/data_publisher.cpp` | tree-sitter-cpp node `missing identifier` at 18 site(s) | 18 |
| 8 | `adapter/plugins/eventservice/service/idl/src/data_publisher_sys_event_callback.cpp` | tree-sitter-cpp node `missing identifier` at 4 site(s) | 4 |
| 9 | `adapter/plugins/eventservice/service/idl/src/data_share_dao.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 10 | `adapter/plugins/eventservice/service/idl/src/data_share_store.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 11 | `adapter/plugins/eventservice/service/idl/src/data_share_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 12 | `adapter/plugins/eventservice/service/idl/src/event_query_wrapper_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 13 | `adapter/plugins/eventservice/service/idl/src/query_sys_event_callback_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 14 | `adapter/plugins/eventservice/service/idl/src/sys_event_service_ohos.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 15 | `adapter/plugins/eventservice/service/sys_event_service_adapter.cpp` | tree-sitter-cpp node `missing identifier` at 4 site(s) | 4 |
| 16 | `adapter/plugins/eventservice/service/test/unittest/common/sys_event_service_ohos_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 91 |
| 17 | `adapter/service/client/src/hiview_remote_service.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 18 | `adapter/service/hiview_service_adapter.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 19 | `adapter/service/server/src/hiview_log_config_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 20 | `adapter/service/server/src/hiview_service_ability.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 44 |
| 21 | `base/dispatch_rule_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 22 | `base/domain_json_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 23 | `base/event_dispatch_queue.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 24 | `base/event_json_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 25 | `base/event_loop.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 26 | `base/event_publish/app_event_handler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 27 | `base/event_publish/app_event_publisher_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 28 | `base/event_publish/event_publish.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 29 | `base/event_publish/log_file_name_converter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 30 | `base/event_publish/test/unittest/common/event_publish_test_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 31 | `base/event_publish/user_data_size_reporter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 32 | `base/event_raw/base/raw_data.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 33 | `base/event_raw/decoded/decoded_event.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 34 | `base/event_raw/encoded/encoded_param.cpp` | tree-sitter-cpp node `missing identifier` at 4 site(s) | 4 |
| 35 | `base/event_raw/encoded/raw_data_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 36 | `base/event_raw/encoded/raw_data_encoder.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 37 | `base/event_raw/include/encoded/encoded_param.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 38 | `base/event_raw/include/encoded/raw_data_builder.h` | explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp | 10 |
| 39 | `base/event_raw/test/unittest/common/event_raw_encoded_and_decoded_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 47 |
| 40 | `base/event_report/hiview_event_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 41 | `base/event_source.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 42 | `base/event_store/dao/doc_query.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 43 | `base/event_store/dao/sys_event_dao.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 44 | `base/event_store/include/sys_event_query.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 45 | `base/event_store/sequence/sys_event_sequence_mgr.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 46 | `base/event_store/store/sys_event_backup.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 47 | `base/event_store/store/sys_event_database.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 48 | `base/event_store/store/sys_event_doc.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 49 | `base/event_store/store/sys_event_doc_lru_cache.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 50 | `base/event_store/store/sys_event_repeat_db.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 51 | `base/event_store/store/sys_event_repeat_guard.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 52 | `base/event_store/test/unittest/common/sys_event_dao_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 127 |
| 53 | `base/event_store/test/unittest/common/sys_event_store_utility_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 54 | `base/event_store/utility/base/event_db_file_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 55 | `base/event_store/utility/reader/content_reader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 56 | `base/event_store/utility/reader/content_reader_version_1.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 57 | `base/event_store/utility/reader/content_reader_version_2.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 58 | `base/event_store/utility/reader/content_reader_version_3.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 59 | `base/event_store/utility/reader/content_reader_version_4.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 60 | `base/event_store/utility/reader/sys_event_doc_reader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 61 | `base/event_store/utility/writer/sys_event_doc_writer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 62 | `base/include/event.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 63 | `base/include/plugin.h` | tree-sitter-cpp node `missing type_identifier` at 3 site(s) | 3 |
| 64 | `base/include/sys_event.h` | explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp | 8 |
| 65 | `base/logstore/log_store_ex.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 66 | `base/pipeline.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 67 | `base/plugin_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 68 | `base/plugin_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 69 | `base/running_status_logger/log_file_writer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 70 | `base/running_status_logger/period_file_operator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 71 | `base/running_status_logger/running_status_logger.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 72 | `base/test/unittest/common/sys_event_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 85 |
| 73 | `base/utility/ash_memory_utils.cpp` | tree-sitter-cpp node `missing identifier` at 4 site(s) | 4 |
| 74 | `base/utility/bundle_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 75 | `base/utility/common_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 76 | `base/utility/file_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 77 | `base/utility/hiview_config_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 78 | `base/utility/hiview_db_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 79 | `base/utility/hiview_zip_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 80 | `base/utility/restorable_db_store.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 81 | `base/utility/setting_observer_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 82 | `core/bundle_config/plugin_bundle_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 83 | `core/hiview_platform.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 84 | `core/param_update/include/param_event_manager.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 85 | `core/param_update/src/log_sign_tools.cpp` | tree-sitter-cpp node `missing identifier` at 16 site(s) | 16 |
| 86 | `core/param_update/src/param_event_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 87 | `core/param_update/src/param_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 88 | `core/param_update/src/param_reader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 89 | `core/platform_config/hiview_platform_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 90 | `core/plugin_bundle.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 91 | `core/plugin_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 92 | `framework/native/unified_collection/collector/common_util.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 93 | `framework/native/unified_collection/collector/config/perf_collect_config.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 94 | `framework/native/unified_collection/collector/impl/cpu/calculator/cpu_calculator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 95 | `framework/native/unified_collection/collector/impl/cpu/device_client/collect_device_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 96 | `framework/native/unified_collection/collector/impl/cpu/src/cpu_collector_impl.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 97 | `framework/native/unified_collection/collector/impl/cpu/src/process_state_info_collector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 98 | `framework/native/unified_collection/collector/impl/cpu/src/sys_cpu_usage_collector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 99 | `framework/native/unified_collection/collector/impl/cpu/src/thread_state_info_collector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 100 | `framework/native/unified_collection/collector/impl/cpu/utils/cpu_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 101 | `framework/native/unified_collection/collector/impl/gpu/gpu_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 102 | `framework/native/unified_collection/collector/impl/graphic/graphic_memory_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 103 | `framework/native/unified_collection/collector/impl/hilog/hilog_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 104 | `framework/native/unified_collection/collector/impl/io/calculator/io_calculator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 105 | `framework/native/unified_collection/collector/impl/io/io_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 106 | `framework/native/unified_collection/collector/impl/memory/memory_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 107 | `framework/native/unified_collection/collector/impl/memory/utils/memory_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 108 | `framework/native/unified_collection/collector/impl/perf/perf_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 109 | `framework/native/unified_collection/collector/impl/thermal/thermal_collector_impl.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 110 | `framework/native/unified_collection/collector/impl/trace/strategy/include/trace_handler.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 111 | `framework/native/unified_collection/collector/impl/trace/strategy/src/trace_handler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 112 | `framework/native/unified_collection/collector/impl/trace/strategy/src/trace_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 113 | `framework/native/unified_collection/collector/impl/trace/trace_collector_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 114 | `framework/native/unified_collection/collector/impl/trace/trace_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 115 | `framework/native/unified_collection/decorator/unified_collection_stat.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 116 | `framework/native/unified_collection/process/process_status.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 117 | `framework/native/unified_collection/trace_manager/src/trace_db_callback.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 118 | `framework/native/unified_collection/trace_manager/src/trace_flow_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 119 | `framework/native/unified_collection/trace_manager/src/trace_state_machine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 120 | `framework/native/unified_collection/trace_manager/state/trace_app_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 121 | `framework/native/unified_collection/trace_manager/state/trace_base_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 122 | `framework/native/unified_collection/trace_manager/state/trace_command_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 123 | `framework/native/unified_collection/trace_manager/state/trace_common_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 124 | `framework/native/unified_collection/trace_manager/state/trace_dynamic_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 125 | `framework/native/unified_collection/trace_manager/state/trace_telemetry_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 126 | `framework/native/unified_collection/trace_manager/storage/app_event_task_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 127 | `framework/native/unified_collection/trace_manager/storage/app_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 128 | `framework/native/unified_collection/trace_manager/storage/telemetry_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 129 | `framework/native/unified_collection/trace_manager/storage/trace_behavior_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 130 | `framework/native/unified_collection/trace_manager/storage/trace_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 131 | `framework/native/unified_collection/trace_manager/telemetry/telemetry_state_machine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 132 | `framework/native/unified_collection/trace_manager/test/trace_manager_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 133 | `hiretrieval/frameworks/include/hiretrieval_base_def.h` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 134 | `hiretrieval/interfaces/ets/ani/src/hiretrieval_ani.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 135 | `hiretrieval/interfaces/js/napi/src/hiretrieval_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 136 | `interfaces/ets/ani/loglibrary/src/loglibrary_ani.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 137 | `interfaces/ets/ani/loglibrary/src/loglibrary_ani_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 29 |
| 138 | `interfaces/inner_api/unified_collection/client/src/trace_collector_client_impl.cpp` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 139 | `interfaces/inner_api/unified_collection/client/trace_collector_client.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 140 | `interfaces/js/napi/src/hiview_napi_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 141 | `interfaces/js/napi/src/hiview_service_agent.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 142 | `interfaces/js/napi/src/napi_hiview_js.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 143 | `main.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 144 | `plugins/crash_validator/crash_validator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 145 | `plugins/event_store/event_export/config/export_config_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 146 | `plugins/event_store/event_export/config/export_config_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 147 | `plugins/event_store/event_export/config/export_event_list_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 148 | `plugins/event_store/event_export/database/adapter/export_db_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 149 | `plugins/event_store/event_export/database/export_db_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 150 | `plugins/event_store/event_export/event_export_engine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 151 | `plugins/event_store/event_export/task/expire/event_delete_handler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 152 | `plugins/event_store/event_export/task/expire/event_expire_task.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 153 | `plugins/event_store/event_export/task/expire/event_scan_handler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 154 | `plugins/event_store/event_export/task/export/event_export_task.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 155 | `plugins/event_store/event_export/task/export/event_export_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 156 | `plugins/event_store/event_export/task/export/event_read_handler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 157 | `plugins/event_store/event_export/task/export/event_write_handler.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 158 | `plugins/event_store/event_export/task/export/export_event_packager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 159 | `plugins/event_store/event_export/task/export/export_file_writer.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 160 | `plugins/event_store/event_export/task/export/export_json_file_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 161 | `plugins/event_store/event_export/task/export/write_zip_file_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 162 | `plugins/event_store/event_export/task/export_dir_creator.cpp` | tree-sitter-cpp node `missing identifier` at 3 site(s) | 3 |
| 163 | `plugins/event_store/event_export/task/trigger/trigger_export_task.cpp` | tree-sitter-cpp node `missing identifier` at 6 site(s) | 6 |
| 164 | `plugins/event_store/event_export/test/unittest/common/event_export_write_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 165 | `plugins/event_store/event_export/trigger_export_engine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 166 | `plugins/event_store/store/sys_event_db_mgr.cpp` | tree-sitter-cpp node `missing identifier` at 3 site(s) | 3 |
| 167 | `plugins/event_store/sys_event_store.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 168 | `plugins/event_validator/control/config/daily_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 169 | `plugins/event_validator/control/daily_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 170 | `plugins/event_validator/control/db/daily_db_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 171 | `plugins/event_validator/event_param_watcher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 172 | `plugins/event_validator/event_period_info_util.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 173 | `plugins/event_validator/event_validator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 174 | `plugins/event_validator/event_verify_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 175 | `plugins/eventlogger/config/event_logger_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 176 | `plugins/eventlogger/event_logger.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 49 |
| 177 | `plugins/eventlogger/event_logger_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 178 | `plugins/eventlogger/log_catcher/event_log_task.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 179 | `plugins/eventlogger/log_catcher/shell_catcher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 180 | `plugins/eventlogger/log_catcher/summary_log_info_catcher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 181 | `plugins/eventlogger/test/unittest/common/event_field_validator_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 182 | `plugins/eventlogger/test/unittest/common/event_logger_config_validate_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 183 | `plugins/faultlogger/framework/native/extension/include/ets_faultlog_extension.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 184 | `plugins/faultlogger/framework/native/extension/src/ets_faultlog_extension.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 185 | `plugins/faultlogger/framework/native/extension/src/ets_faultlog_extension_context.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 186 | `plugins/faultlogger/framework/native/extension/src/js_faultlog_extension.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 187 | `plugins/faultlogger/framework/native/extension/src/js_faultlog_extension_context.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 188 | `plugins/faultlogger/framework/native/extension/zidl/src/faultlog_ext_stub_imp.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 189 | `plugins/faultlogger/interfaces/cj/faultlogger_ffi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 190 | `plugins/faultlogger/interfaces/cj/faultlogger_ffi.h` | tree-sitter-cpp node `missing ::` at 1 site(s) | 1 |
| 191 | `plugins/faultlogger/interfaces/cpp/innerkits/impl/faultlogger_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 192 | `plugins/faultlogger/interfaces/js/napi/napi_faultlogger.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 193 | `plugins/faultlogger/interfaces/js/napi/napi_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 194 | `plugins/faultlogger/interfaces/js/test/unittest/cpp/faultlogger_test_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 195 | `plugins/faultlogger/service/bdfr_base/base/faultlog_event_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 196 | `plugins/faultlogger/service/bdfr_base/base/faultlog_event_interface.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 197 | `plugins/faultlogger/service/bdfr_base/base/faultlog_event_pipeline.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 198 | `plugins/faultlogger/service/bdfr_base/event/cpp_crash/faultlog_cppcrash.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 199 | `plugins/faultlogger/service/bdfr_base/event/freeze/faultlog_freeze.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 200 | `plugins/faultlogger/service/bdfr_base/event/js_cj_error/faultlog_error_reporter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 201 | `plugins/faultlogger/service/bdfr_base/event/sanitizer/faultlog_sanitizer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 202 | `plugins/faultlogger/service/bdfr_base/export_faultlogger_interface.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 203 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_bootscan.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 204 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_database.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 205 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_dump.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 39 |
| 206 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_formatter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 207 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 208 | `plugins/faultlogger/service/bdfr_base/faultlogger_base.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 209 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cjerror_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 210 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cppcrash_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 125 |
| 211 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_database_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 212 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_formatter_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 190 |
| 213 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_hilog_helper_test.cpp` | tree-sitter-cpp node `missing identifier` at 6 site(s) | 6 |
| 214 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_jserror_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 55 |
| 215 | `plugins/faultlogger/service/bdfr_base/utils/faultlog_hilog_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 216 | `plugins/faultlogger/service/bdfr_base/utils/faultlog_util.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 217 | `plugins/faultlogger/service/dynamic_library_management/dynamic_library_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 218 | `plugins/faultlogger/service/extension_manager/src/faultlog_ext_conn_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 219 | `plugins/faultlogger/service/extension_manager/src/faultlog_ext_connection.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 220 | `plugins/faultlogger/service/faultlog_bootscan_listener.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 221 | `plugins/faultlogger/service/faultlogger.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 222 | `plugins/faultlogger/service/idl/faultlogger_service_ohos.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 223 | `plugins/faultlogger/service/idl/include/ifaultlog_query_result.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 224 | `plugins/faultlogger/service/idl/include/ifaultlogger_service.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 225 | `plugins/faultlogger/service/idl/src/faultlog_info_ohos.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 226 | `plugins/faultlogger/service/idl/src/faultlog_query_result_stub.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 227 | `plugins/faultlogger/service/idl/src/faultlogger_service_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 228 | `plugins/faultlogger/service/idl/src/faultlogger_service_stub.cpp` | tree-sitter-cpp node `missing identifier` at 7 site(s) | 7 |
| 229 | `plugins/faultlogger/service/page_history/page_history_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 230 | `plugins/faultlogger/service/page_history/page_history_recorder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 231 | `plugins/faultlogger/service/page_history/pages_trace.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 232 | `plugins/freeze_detector/db_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 233 | `plugins/freeze_detector/event_field_validator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 234 | `plugins/freeze_detector/freeze_common.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 235 | `plugins/freeze_detector/freeze_detector_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 236 | `plugins/freeze_detector/freeze_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 237 | `plugins/freeze_detector/resolver.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 238 | `plugins/freeze_detector/rule_cluster.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 239 | `plugins/freeze_detector/test/unittest/common/freeze_detector_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 240 | `plugins/freeze_detector/vendor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 241 | `plugins/performance/XperfPlugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 242 | `plugins/performance/executor/ThrExecutor.cpp` | tree-sitter-cpp node `missing identifier` at 3 site(s) | 3 |
| 243 | `plugins/performance/executor/ThrTaskContainer.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 244 | `plugins/performance/monitor/AppLaunchMonitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 245 | `plugins/performance/monitor/JankAnimatorMonitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 246 | `plugins/performance/perfmonitor/common/event_builder/xperf_event_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 247 | `plugins/performance/perfmonitor/common/event_builder/xperf_event_reporter.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 248 | `plugins/performance/perfmonitor/common/perf_trace.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 249 | `plugins/performance/perfmonitor/common/perf_trace.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 250 | `plugins/performance/perfmonitor/interfaces/inner_api/include/perf_model.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 251 | `plugins/performance/perfmonitor/load_complete/src/collect_states.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 252 | `plugins/performance/perfmonitor/load_complete/src/collect_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 253 | `plugins/performance/perfmonitor/src/animator_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 254 | `plugins/performance/perfmonitor/src/input_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 255 | `plugins/performance/perfmonitor/src/jank_frame_monitor.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 256 | `plugins/performance/perfmonitor/src/perf_reporter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 257 | `plugins/performance/perfmonitor/src/scene_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 258 | `plugins/performance/perfmonitor/src/white_block_monitor.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 259 | `plugins/performance/reporter/adapter/AppStartReporterAdapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 260 | `plugins/performance/reporter/adapter/JankAnimatorReporterAdapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 261 | `plugins/performance/reporter/adapter/SimpleAppStartReporterAdapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 262 | `plugins/performance/reporter/event_poster/EventsPoster.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 263 | `plugins/performance/reporter/infrastructure/AppStartReporter.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 264 | `plugins/performance/reporter/infrastructure/JankAnimatorReporter.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 265 | `plugins/performance/scene_data_processor/AnimatorSceneDataProcessor.cpp` | tree-sitter-cpp node `missing identifier` at 13 site(s) | 13 |
| 266 | `plugins/performance/timer/SceneTimerOhImpl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 267 | `plugins/performance/xperf_service/common/src/perf_trace.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 268 | `plugins/performance/xperf_service/interfaces/inner_api/xperfservice_client/src/rs_frame_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 269 | `plugins/performance/xperf_service/interfaces/inner_api/xperfservice_client/src/xperf_service_client.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 270 | `plugins/performance/xperf_service/services/common/src/xperf_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 271 | `plugins/performance/xperf_service/services/core/src/xperf_register_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 272 | `plugins/performance/xperf_service/services/core/src/xperf_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 273 | `plugins/performance/xperf_service/services/framework/xperf_dispatcher/src/event_parser_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 274 | `plugins/performance/xperf_service/services/framework/xperf_dispatcher/src/xperf_dispatcher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 275 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/avcodec_perf_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 276 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/passthrough_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 277 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/user_action_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 278 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_jank_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 279 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_play_latency_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 280 | `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_xperf_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 281 | `plugins/performance/xperf_service/services/framework/xperf_storage/src/user_action_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 282 | `plugins/performance/xperf_service/services/server/src/xperf_service_server.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 283 | `plugins/performance/xperf_service/services/utils/time_util.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 284 | `plugins/performance/xperf_service/services/xperf_service_main.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 285 | `plugins/privacy_controller/privacy_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 286 | `plugins/reliability/bbox_detectors/bbox_detector_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 287 | `plugins/reliability/bbox_detectors/bdfr_base/bbox_detectors_base.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 288 | `plugins/reliability/bbox_detectors/bdfr_base/export_bbox_detectors_interface.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 289 | `plugins/reliability/bbox_detectors/bdfr_base/panic_error_info_handle.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 290 | `plugins/reliability/bbox_detectors/bdfr_base/panic_report_recovery.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 291 | `plugins/reliability/bbox_detectors/bdfr_base/test/unittest/bbox_detector_base_unit_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 292 | `plugins/reliability/leak_detectors/fault_detector_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 293 | `plugins/reliability/leak_detectors/fault_detector_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 294 | `plugins/reliability/leak_detectors/native_leak/native_leak_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 295 | `plugins/reliability/leak_detectors/native_leak/native_leak_detector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 296 | `plugins/reliability/leak_detectors/native_leak/native_leak_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 49 |
| 297 | `plugins/reliability/leak_detectors/native_leak/native_leak_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 298 | `plugins/reliability/leak_detectors/test/moduletest/leak_detector_module_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 299 | `plugins/reliability/leak_detectors/test/test_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 300 | `plugins/sys_dispatcher/sys_dispatcher.cpp` | tree-sitter-cpp node `missing identifier` at 5 site(s) | 5 |
| 301 | `plugins/sysevent_source/event_server.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 302 | `plugins/sysevent_source/monitor_config.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 303 | `plugins/sysevent_source/platform_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 304 | `plugins/sysevent_source/sysevent_source.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 305 | `plugins/unified_collector/observer/uc_app_state_observer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 306 | `plugins/unified_collector/observer/uc_observer_mgr.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 307 | `plugins/unified_collector/observer/uc_render_state_observer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 308 | `plugins/unified_collector/observer/uc_system_ability_listener.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 309 | `plugins/unified_collector/observer/uc_telemetry_callback.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 310 | `plugins/unified_collector/observer/uc_telemetry_listener.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 311 | `plugins/unified_collector/power/power_status_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 312 | `plugins/unified_collector/storage/cpu_storage.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 313 | `plugins/unified_collector/task/cpu_collection_task.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 314 | `plugins/unified_collector/task/dump_trace_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 315 | `plugins/unified_collector/task/trace_cache_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 316 | `plugins/unified_collector/unified_collector.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 317 | `plugins/usage_event_report/cache/event_db_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 318 | `plugins/usage_event_report/cache/usage_event_cacher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 319 | `plugins/usage_event_report/event/app_usage_event.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 320 | `plugins/usage_event_report/event/sys_usage_event.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 321 | `plugins/usage_event_report/fold/cache/fold_app_usage_db_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 322 | `plugins/usage_event_report/fold/cache/fold_app_usage_event_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 323 | `plugins/usage_event_report/fold/cache/fold_event_cacher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 324 | `plugins/usage_event_report/fold/cache/include/fold_app_usage_db_helper.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 325 | `plugins/usage_event_report/fold/usage_fold_event_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 326 | `plugins/usage_event_report/idl/hiview_shutdown_callback.cpp` | tree-sitter-cpp node `missing identifier` at 2 site(s) | 2 |
| 327 | `plugins/usage_event_report/service/factory/app_usage_event_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 328 | `plugins/usage_event_report/service/main.cpp` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 329 | `plugins/usage_event_report/service/usage_event_report_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 330 | `plugins/usage_event_report/usage_event_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 331 | `service/hiview_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 332 | `test/plugins/test_plugin/test_content.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 333 | `test/plugins/test_plugin/test_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 334 | `utility/analysis_faultlog/analysis_faultlog.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 335 | `utility/common_utils/calc_fingerprint.cpp` | tree-sitter-cpp node `missing identifier` at 6 site(s) | 6 |
| 336 | `utility/common_utils/tbox.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 337 | `utility/common_utils/test/unittest/common/utility_common_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 338 | `utility/smart_parser/feature_analysis/feature_analysis.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 339 | `utility/smart_parser/feature_analysis/log_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 340 | `utility/smart_parser/rule/compose_rule.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 341 | `utility/smart_parser/rule/extract_rule.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 342 | `utility/smart_parser/smart_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 343 | `utility/test/unittest/cpp_crash_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 344 | `utility/test/unittest/syswarning_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |

### Per-file details

#### `adapter/plugins/eventservice/service/idl/dfx/include/listener_status_util.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 91 | `missing type_identifier` | `` |
| 12 | 43 | `missing type_identifier` | `` |

#### `adapter/plugins/eventservice/service/idl/dfx/src/listener_status_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 137 | `ERROR` | `,` |
| 27 | 115 | `missing identifier` | `` |
| 52 | 202 | `ERROR` | `,` |
| 56 | 168 | `ERROR` | `,` |
| 62 | 158 | `ERROR` | `,` |
| 77 | 72 | `ERROR` | `,` |
| 91 | 71 | `ERROR` | `,` |
| 108 | 189 | `ERROR` | `,` |
| 117 | 71 | `ERROR` | `,` |
| 134 | 189 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/include/iquery_sys_event_callback.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 32 | `ERROR` | `"ohos.hiviewdfx.IQuerySysEventCallback"` |

#### `adapter/plugins/eventservice/service/idl/include/parcelable_vector_rw.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 20 | `ERROR` | `T` |
| 15 | 1 | `ERROR` | `bool` |
| 15 | 44 | `ERROR` | `std::` |
| 15 | 69 | `missing ;` | `` |
| 28 | 40 | `missing identifier` | `` |
| 40 | 20 | `ERROR` | `T` |
| 41 | 1 | `ERROR` | `bool` |
| 41 | 43 | `ERROR` | `const std::` |
| 41 | 74 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/idl/src/ash_mem_utils.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 11 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 121 | `missing identifier` | `` |
| 32 | 120 | `missing identifier` | `` |
| 35 | 122 | `missing identifier` | `` |
| 45 | 116 | `missing identifier` | `` |
| 54 | 131 | `missing identifier` | `` |
| 69 | 124 | `missing identifier` | `` |
| 76 | 124 | `missing identifier` | `` |
| 87 | 131 | `missing identifier` | `` |
| 92 | 124 | `missing identifier` | `` |
| 97 | 134 | `missing identifier` | `` |
| 105 | 120 | `missing identifier` | `` |

#### `adapter/plugins/eventservice/service/idl/src/compliant_event_checker.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 158 | `ERROR` | `,` |
| 43 | 164 | `ERROR` | `,` |
| 48 | 184 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/src/data_publisher.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 127 | `missing identifier` | `` |
| 40 | 118 | `missing identifier` | `` |
| 54 | 118 | `missing identifier` | `` |
| 66 | 129 | `missing identifier` | `` |
| 70 | 122 | `missing identifier` | `` |
| 80 | 133 | `missing identifier` | `` |
| 92 | 136 | `missing identifier` | `` |
| 113 | 132 | `missing identifier` | `` |
| 123 | 155 | `missing identifier` | `` |
| 133 | 138 | `missing identifier` | `` |
| 149 | 133 | `missing identifier` | `` |
| 152 | 125 | `missing identifier` | `` |
| 171 | 126 | `missing identifier` | `` |
| 176 | 127 | `missing identifier` | `` |
| 183 | 136 | `missing identifier` | `` |
| 198 | 132 | `missing identifier` | `` |
| 204 | 132 | `missing identifier` | `` |
| 228 | 130 | `missing identifier` | `` |

#### `adapter/plugins/eventservice/service/idl/src/data_publisher_sys_event_callback.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 132 | `missing identifier` | `` |
| 51 | 133 | `missing identifier` | `` |
| 54 | 132 | `missing identifier` | `` |
| 57 | 125 | `missing identifier` | `` |

#### `adapter/plugins/eventservice/service/idl/src/data_share_dao.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 145 | `missing identifier` | `` |
| 29 | 167 | `ERROR` | `,` |
| 39 | 152 | `missing identifier` | `` |
| 53 | 162 | `ERROR` | `,` |
| 65 | 159 | `ERROR` | `,` |
| 75 | 145 | `missing identifier` | `` |
| 84 | 145 | `missing identifier` | `` |
| 94 | 175 | `ERROR` | `,` |
| 105 | 125 | `missing identifier` | `` |
| 119 | 175 | `ERROR` | `,` |
| 130 | 125 | `missing identifier` | `` |
| 144 | 175 | `ERROR` | `,` |
| 155 | 125 | `missing identifier` | `` |

#### `adapter/plugins/eventservice/service/idl/src/data_share_store.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 154 | `ERROR` | `,` |
| 28 | 184 | `ERROR` | `,` |
| 47 | 167 | `ERROR` | `,` |
| 52 | 177 | `ERROR` | `,` |
| 61 | 142 | `missing identifier` | `` |
| 65 | 132 | `missing identifier` | `` |
| 73 | 150 | `ERROR` | `,` |
| 86 | 151 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/src/data_share_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 151 | `ERROR` | `,` |
| 37 | 156 | `ERROR` | `,` |
| 44 | 132 | `missing identifier` | `` |
| 53 | 120 | `missing identifier` | `` |
| 68 | 161 | `ERROR` | `,` |
| 70 | 149 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/src/event_query_wrapper_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 127 | `missing identifier` | `` |
| 133 | 167 | `ERROR` | `,` |
| 138 | 127 | `missing identifier` | `` |
| 143 | 121 | `missing identifier` | `` |
| 147 | 123 | `missing identifier` | `` |
| 160 | 189 | `ERROR` | `,` |
| 249 | 87 | `ERROR` | `,` |
| 280 | 157 | `ERROR` | `,` |
| 430 | 84 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/src/query_sys_event_callback_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 133 | `missing identifier` | `` |
| 17 | 126 | `missing identifier` | `` |
| 22 | 125 | `missing identifier` | `` |
| 28 | 123 | `missing identifier` | `` |
| 35 | 150 | `ERROR` | `,` |
| 43 | 133 | `missing identifier` | `` |
| 48 | 126 | `missing identifier` | `` |
| 53 | 122 | `missing identifier` | `` |
| 58 | 119 | `missing identifier` | `` |
| 65 | 150 | `ERROR` | `,` |

#### `adapter/plugins/eventservice/service/idl/src/sys_event_service_ohos.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 139 | `ERROR` | `,` |
| 139 | 138 | `missing identifier` | `` |
| 143 | 138 | `missing identifier` | `` |
| 148 | 142 | `missing identifier` | `` |
| 163 | 144 | `missing identifier` | `` |
| 169 | 94 | `ERROR` | `,` |
| 175 | 197 | `ERROR` | `,` |
| 186 | 116 | `missing identifier` | `` |
| 191 | 127 | `missing identifier` | `` |
| 198 | 172 | `ERROR` | `,` |
| 234 | 144 | `missing identifier` | `` |
| 238 | 135 | `missing identifier` | `` |
| 244 | 145 | `missing identifier` | `` |
| 252 | 52 | `ERROR` | `,` |
| 258 | 146 | `missing identifier` | `` |
| 263 | 83 | `ERROR` | `,` |
| 279 | 128 | `missing identifier` | `` |
| 283 | 119 | `missing identifier` | `` |
| 289 | 129 | `missing identifier` | `` |
| 296 | 123 | `missing identifier` | `` |
| … | … | … | *(13 more)* |

#### `adapter/plugins/eventservice/service/sys_event_service_adapter.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 134 | `missing identifier` | `` |
| 20 | 123 | `missing identifier` | `` |
| 25 | 138 | `missing identifier` | `` |
| 35 | 150 | `missing identifier` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/sys_event_service_ohos_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 178 | 102 | `ERROR` | `=` |
| 179 | 28 | `ERROR` | `}] } })~ "` |
| 179 | 39 | `ERROR` | `"` |
| 189 | 103 | `ERROR` | `=` |
| 190 | 28 | `ERROR` | `}` |
| 190 | 31 | `missing identifier` | `` |
| 190 | 32 | `ERROR` | `"param":` |
| 190 | 50 | `ERROR` | `"op":` |
| 190 | 69 | `ERROR` | `: 1201` |
| 190 | 77 | `ERROR` | `] } })~ "` |
| 190 | 87 | `ERROR` | `"` |
| 200 | 71 | `ERROR` | `":[{"` |
| 201 | 1 | `ERROR` | `{` |
| 201 | 47 | `ERROR` | `]` |
| 201 | 52 | `ERROR` | `)` |
| 201 | 58 | `missing ;` | `` |
| 204 | 1 | `ERROR` | `}` |
| 211 | 71 | `ERROR` | `":[{"` |
| 212 | 1 | `ERROR` | `{` |
| 212 | 48 | `ERROR` | `] } })` |
| … | … | … | *(71 more)* |

#### `adapter/service/client/src/hiview_remote_service.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 155 | `missing identifier` | `` |
| 33 | 128 | `missing identifier` | `` |
| 35 | 122 | `missing identifier` | `` |
| 52 | 128 | `missing identifier` | `` |
| 57 | 139 | `missing identifier` | `` |

#### `adapter/service/hiview_service_adapter.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 125 | `missing identifier` | `` |
| 15 | 116 | `missing identifier` | `` |

#### `adapter/service/server/src/hiview_log_config_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 156 | `ERROR` | `,` |
| 26 | 158 | `ERROR` | `,` |
| 54 | 161 | `ERROR` | `,` |
| 57 | 126 | `missing identifier` | `` |
| 61 | 143 | `ERROR` | `,` |
| 65 | 118 | `missing identifier` | `` |
| 70 | 115 | `missing identifier` | `` |

#### `adapter/service/server/src/hiview_service_ability.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 147 | `ERROR` | `,` |
| 52 | 123 | `missing identifier` | `` |
| 57 | 123 | `missing identifier` | `` |
| 103 | 145 | `ERROR` | `,` |
| 108 | 145 | `ERROR` | `,` |
| 130 | 137 | `ERROR` | `,` |
| 137 | 127 | `missing identifier` | `` |
| 143 | 123 | `missing identifier` | `` |
| 149 | 128 | `missing identifier` | `` |
| 153 | 119 | `missing identifier` | `` |
| 180 | 148 | `ERROR` | `,` |
| 191 | 117 | `missing identifier` | `` |
| 201 | 118 | `missing identifier` | `` |
| 236 | 148 | `ERROR` | `,` |
| 240 | 150 | `ERROR` | `,` |
| 245 | 46 | `ERROR` | `,` |
| 253 | 117 | `missing identifier` | `` |
| 257 | 116 | `missing identifier` | `` |
| 261 | 118 | `missing identifier` | `` |
| 267 | 178 | `ERROR` | `,` |
| … | … | … | *(24 more)* |

#### `base/dispatch_rule_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 160 | `ERROR` | `,` |
| 27 | 32 | `ERROR` | `,` |
| 49 | 127 | `missing identifier` | `` |
| 71 | 126 | `missing identifier` | `` |
| 90 | 128 | `missing identifier` | `` |
| 109 | 129 | `missing identifier` | `` |

#### `base/domain_json_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 156 | `ERROR` | `,` |
| 28 | 155 | `ERROR` | `,` |
| 33 | 197 | `ERROR` | `,` |
| 47 | 193 | `ERROR` | `,` |
| 67 | 197 | `ERROR` | `,` |
| 73 | 68 | `ERROR` | `,` |
| 83 | 74 | `ERROR` | `,` |

#### `base/event_dispatch_queue.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 114 | `missing identifier` | `` |
| 58 | 117 | `missing identifier` | `` |
| 62 | 116 | `missing identifier` | `` |
| 71 | 153 | `ERROR` | `,` |

#### `base/event_json_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 113 | `missing identifier` | `` |
| 52 | 142 | `ERROR` | `,` |
| 57 | 121 | `missing identifier` | `` |
| 61 | 146 | `ERROR` | `,` |
| 101 | 163 | `ERROR` | `,` |
| 180 | 177 | `ERROR` | `,` |
| 183 | 180 | `ERROR` | `,` |
| 232 | 131 | `missing identifier` | `` |
| 238 | 132 | `missing identifier` | `` |
| 244 | 31 | `ERROR` | `,` |
| 251 | 133 | `missing identifier` | `` |
| 286 | 184 | `ERROR` | `,` |
| 336 | 161 | `ERROR` | `,` |
| 354 | 129 | `missing identifier` | `` |
| 361 | 179 | `ERROR` | `,` |

#### `base/event_loop.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 130 | 163 | `missing identifier` | `` |
| 157 | 139 | `missing identifier` | `` |
| 200 | 145 | `missing identifier` | `` |
| 205 | 171 | `ERROR` | `,` |
| 229 | 185 | `ERROR` | `,` |
| 282 | 175 | `ERROR` | `,` |
| 307 | 133 | `missing identifier` | `` |
| 311 | 20 | `ERROR` | `-> operator()` |
| 313 | 136 | `missing identifier` | `` |
| 316 | 124 | `missing identifier` | `` |

#### `base/event_publish/app_event_handler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 70 | 118 | `missing identifier` | `` |
| 99 | 118 | `missing identifier` | `` |
| 204 | 118 | `missing identifier` | `` |
| 239 | 117 | `missing identifier` | `` |
| 264 | 117 | `missing identifier` | `` |
| 306 | 128 | `missing identifier` | `` |
| 314 | 182 | `ERROR` | `,` |
| 344 | 118 | `missing identifier` | `` |

#### `base/event_publish/app_event_publisher_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 131 | `missing identifier` | `` |
| 41 | 180 | `ERROR` | `,` |
| 43 | 165 | `ERROR` | `,` |
| 48 | 155 | `ERROR` | `,` |

#### `base/event_publish/event_publish.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 68 | 148 | `missing identifier` | `` |
| 75 | 168 | `ERROR` | `,` |
| 79 | 189 | `ERROR` | `,` |
| 100 | 148 | `missing identifier` | `` |
| 106 | 144 | `missing identifier` | `` |
| 127 | 137 | `missing identifier` | `` |
| 132 | 135 | `missing identifier` | `` |
| 138 | 158 | `missing identifier` | `` |
| 141 | 135 | `missing identifier` | `` |
| 152 | 129 | `missing identifier` | `` |
| 162 | 177 | `missing identifier` | `` |
| 177 | 38 | `ERROR` | `,` |
| 193 | 159 | `ERROR` | `,` |
| 210 | 165 | `ERROR` | `,` |
| 241 | 171 | `ERROR` | `,` |
| 253 | 130 | `missing identifier` | `` |
| 263 | 137 | `missing identifier` | `` |
| 276 | 131 | `missing identifier` | `` |
| 281 | 129 | `missing identifier` | `` |
| 347 | 177 | `ERROR` | `,` |
| … | … | … | *(23 more)* |

#### `base/event_publish/log_file_name_converter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 64 | 53 | `ERROR` | `,` |
| 70 | 187 | `ERROR` | `,` |
| 124 | 65 | `ERROR` | `,` |
| 137 | 180 | `ERROR` | `,` |
| 141 | 148 | `missing identifier` | `` |
| 147 | 179 | `ERROR` | `,` |

#### `base/event_publish/test/unittest/common/event_publish_test_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 139 | `missing identifier` | `` |
| 29 | 141 | `missing identifier` | `` |
| 39 | 138 | `missing identifier` | `` |
| 51 | 136 | `missing identifier` | `` |
| 62 | 138 | `missing identifier` | `` |
| 71 | 185 | `ERROR` | `,` |
| 83 | 125 | `missing identifier` | `` |
| 131 | 124 | `missing identifier` | `` |
| 137 | 130 | `missing identifier` | `` |

#### `base/event_publish/user_data_size_reporter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 73 | 163 | `ERROR` | `,` |
| 99 | 154 | `ERROR` | `,` |
| 116 | 172 | `ERROR` | `,` |
| 119 | 167 | `ERROR` | `,` |

#### `base/event_raw/base/raw_data.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 166 | `ERROR` | `,` |
| 70 | 166 | `ERROR` | `,` |
| 92 | 166 | `ERROR` | `,` |
| 153 | 135 | `missing identifier` | `` |
| 166 | 166 | `ERROR` | `,` |
| 177 | 154 | `ERROR` | `,` |

#### `base/event_raw/decoded/decoded_event.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 125 | `missing identifier` | `` |
| 35 | 165 | `ERROR` | `,` |
| 44 | 154 | `ERROR` | `,` |
| 285 | 143 | `ERROR` | `,` |
| 293 | 150 | `missing identifier` | `` |

#### `base/event_raw/encoded/encoded_param.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 51 | `missing identifier` | `` |
| 10 | 50 | `missing identifier` | `` |
| 11 | 53 | `missing identifier` | `` |
| 83 | 146 | `missing identifier` | `` |

#### `base/event_raw/encoded/raw_data_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 47 | 127 | `missing identifier` | `` |
| 53 | 125 | `missing identifier` | `` |
| 74 | 125 | `missing identifier` | `` |
| 78 | 134 | `missing identifier` | `` |
| 84 | 130 | `missing identifier` | `` |
| 88 | 148 | `missing identifier` | `` |
| 94 | 130 | `missing identifier` | `` |
| 113 | 156 | `ERROR` | `,` |
| 124 | 154 | `ERROR` | `,` |

#### `base/event_raw/encoded/raw_data_encoder.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 127 | `missing identifier` | `` |

#### `base/event_raw/include/encoded/encoded_param.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 98 | `ERROR` | `*` |
| 83 | 98 | `ERROR` | `*` |
| 132 | 48 | `ERROR` | `*` |
| 174 | 48 | `ERROR` | `*` |
| 221 | 112 | `ERROR` | `*` |
| 268 | 112 | `ERROR` | `*` |
| 396 | 17 | `ERROR` | `class` |
| 397 | 17 | `ERROR` | `class` |
| 398 | 17 | `ERROR` | `class` |

#### `base/event_raw/include/encoded/raw_data_builder.h`

**Summary:** explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 515 | 8 | `ERROR` | `template void RawDataBuilder` |
| 516 | 84 | `ERROR` | `&` |
| 517 | 8 | `ERROR` | `template void RawDataBuilder` |
| 518 | 85 | `ERROR` | `&` |
| 519 | 8 | `ERROR` | `template void RawDataBuilder` |
| 520 | 81 | `ERROR` | `&` |
| 521 | 8 | `ERROR` | `template RawDataBuilder& RawDataBuilder:: AppendValue< double>(const std:: string& , double) ;` |
| 522 | 8 | `ERROR` | `template RawDataBuilder& RawDataBuilder:: AppendValue< std:: string>(const std:: string& , std:: string) ;` |
| 523 | 8 | `ERROR` | `template RawDataBuilder& RawDataBuilder:: AppendValue< int64_t>(const std:: string& , int64_t) ;` |
| 524 | 8 | `ERROR` | `template RawDataBuilder` |

#### `base/event_raw/test/unittest/common/event_raw_encoded_and_decoded_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 311 | 8 | `ERROR` | `: 1` |
| 311 | 39 | `ERROR` | `"tz_":` |
| 311 | 62 | `ERROR` | `: 1751` |
| 311 | 77 | `ERROR` | `: 1751` |
| 311 | 92 | `ERROR` | `: 0` |
| 312 | 46 | `ERROR` | `: 3` |
| 312 | 52 | `ERROR` | `"traceid_":` |
| 313 | 1 | `ERROR` | `"spanid_":` |
| 313 | 18 | `ERROR` | `"pspanid_":` |
| 313 | 36 | `ERROR` | `"key1":` |
| 313 | 59 | `ERROR` | `:` |
| 314 | 15 | `ERROR` | `"key4":` |
| 314 | 38 | `ERROR` | `"info_":` |
| 314 | 52 | `ERROR` | `"level_":` |
| 314 | 75 | `ERROR` | `"seq_": 972 })` |
| 314 | 91 | `ERROR` | `";" auto parser= std` |
| 668 | 10 | `ERROR` | `: 3.4` |
| 668 | 53 | `ERROR` | `"DOUBLE_T":` |
| 668 | 80 | `ERROR` | `"INT64_T":` |
| 669 | 10 | `ERROR` | `:` |
| … | … | … | *(27 more)* |

#### `base/event_report/hiview_event_report.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 171 | `ERROR` | `,` |
| 25 | 173 | `ERROR` | `,` |
| 35 | 172 | `ERROR` | `,` |
| 44 | 171 | `ERROR` | `,` |
| 52 | 59 | `ERROR` | `,` |
| 60 | 135 | `missing identifier` | `` |
| 72 | 195 | `ERROR` | `,` |
| 74 | 171 | `ERROR` | `,` |

#### `base/event_source.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 134 | `missing identifier` | `` |
| 25 | 158 | `ERROR` | `,` |
| 28 | 171 | `ERROR` | `,` |

#### `base/event_store/dao/doc_query.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 162 | `ERROR` | `,` |

#### `base/event_store/dao/sys_event_dao.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 185 | `ERROR` | `,` |
| 81 | 166 | `ERROR` | `,` |
| 85 | 117 | `missing identifier` | `` |
| 99 | 189 | `ERROR` | `,` |

#### `base/event_store/include/sys_event_query.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 89 | 48 | `ERROR` | `*` |
| 271 | 59 | `missing type_identifier` | `` |

#### `base/event_store/sequence/sys_event_sequence_mgr.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 187 | `ERROR` | `,` |
| 47 | 164 | `ERROR` | `,` |
| 74 | 157 | `ERROR` | `,` |
| 82 | 145 | `ERROR` | `,` |
| 109 | 168 | `ERROR` | `,` |
| 115 | 163 | `ERROR` | `,` |
| 121 | 166 | `ERROR` | `,` |
| 137 | 135 | `missing identifier` | `` |
| 142 | 166 | `ERROR` | `,` |
| 226 | 139 | `missing identifier` | `` |

#### `base/event_store/store/sys_event_backup.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 126 | `missing identifier` | `` |
| 28 | 131 | `missing identifier` | `` |
| 33 | 115 | `missing identifier` | `` |
| 42 | 127 | `missing identifier` | `` |
| 49 | 124 | `missing identifier` | `` |
| 68 | 124 | `missing identifier` | `` |
| 73 | 127 | `missing identifier` | `` |
| 84 | 124 | `missing identifier` | `` |
| 90 | 123 | `missing identifier` | `` |
| 100 | 137 | `missing identifier` | `` |
| 111 | 128 | `missing identifier` | `` |
| 119 | 156 | `ERROR` | `,` |

#### `base/event_store/store/sys_event_database.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 166 | `ERROR` | `,` |
| 49 | 166 | `ERROR` | `,` |
| 75 | 124 | `missing identifier` | `` |
| 82 | 167 | `ERROR` | `,` |
| 85 | 175 | `ERROR` | `,` |
| 112 | 114 | `missing identifier` | `` |
| 118 | 122 | `missing identifier` | `` |
| 123 | 123 | `missing identifier` | `` |
| 129 | 148 | `ERROR` | `,` |
| 135 | 155 | `ERROR` | `,` |
| 149 | 172 | `ERROR` | `,` |
| 154 | 115 | `missing identifier` | `` |
| 161 | 115 | `missing identifier` | `` |
| 164 | 132 | `missing identifier` | `` |
| 170 | 135 | `missing identifier` | `` |
| 173 | 116 | `missing identifier` | `` |
| 212 | 153 | `ERROR` | `,` |
| 215 | 155 | `ERROR` | `,` |
| 230 | 159 | `ERROR` | `,` |
| 253 | 165 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `base/event_store/store/sys_event_doc.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 114 | `missing identifier` | `` |
| 35 | 165 | `ERROR` | `,` |
| 43 | 158 | `ERROR` | `,` |
| 47 | 159 | `ERROR` | `,` |
| 70 | 131 | `missing identifier` | `` |
| 74 | 156 | `ERROR` | `,` |
| 82 | 165 | `ERROR` | `,` |
| 117 | 156 | `ERROR` | `,` |
| 153 | 181 | `ERROR` | `,` |

#### `base/event_store/store/sys_event_doc_lru_cache.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 198 | `ERROR` | `,` |

#### `base/event_store/store/sys_event_repeat_db.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 156 | `ERROR` | `,` |
| 44 | 162 | `ERROR` | `,` |
| 51 | 172 | `ERROR` | `,` |
| 59 | 172 | `ERROR` | `,` |
| 76 | 113 | `missing identifier` | `` |
| 80 | 119 | `missing identifier` | `` |
| 104 | 174 | `ERROR` | `,` |
| 137 | 121 | `missing identifier` | `` |
| 147 | 156 | `ERROR` | `,` |
| 169 | 125 | `missing identifier` | `` |
| 185 | 166 | `ERROR` | `,` |
| 200 | 165 | `ERROR` | `,` |
| 225 | 165 | `ERROR` | `,` |

#### `base/event_store/store/sys_event_repeat_guard.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 116 | `missing identifier` | `` |
| 27 | 119 | `missing identifier` | `` |
| 66 | 120 | `missing identifier` | `` |
| 71 | 119 | `missing identifier` | `` |
| 81 | 119 | `missing identifier` | `` |
| 113 | 181 | `ERROR` | `,` |
| 131 | 137 | `missing identifier` | `` |

#### `base/event_store/test/unittest/common/sys_event_dao_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 8 | `ERROR` | `: 1620271291188` |
| 42 | 47 | `ERROR` | `: 6527` |
| 42 | 56 | `ERROR` | `"traceid_":` |
| 42 | 88 | `ERROR` | `"spanid_":` |
| 42 | 106 | `ERROR` | `"pspanid_":` |
| 43 | 14 | `ERROR` | `: 4` |
| 43 | 29 | `ERROR` | `: 1` |
| 43 | 44 | `ERROR` | `: 97` |
| 43 | 61 | `ERROR` | `:` |
| 43 | 78 | `ERROR` | `:` |
| 43 | 96 | `ERROR` | `:` |
| 43 | 118 | `ERROR` | `:` |
| 44 | 18 | `ERROR` | `: 97` |
| 44 | 43 | `ERROR` | `: 100` |
| 44 | 67 | `ERROR` | `: 200` |
| 44 | 92 | `ERROR` | `: 300` |
| 45 | 22 | `ERROR` | `: 400` |
| 45 | 40 | `ERROR` | `: 1.1` |
| 45 | 59 | `ERROR` | `: 2.2` |
| 45 | 67 | `ERROR` | `"keyString1":` |
| … | … | … | *(107 more)* |

#### `base/event_store/test/unittest/common/sys_event_store_utility_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 104 | 142 | `ERROR` | `,` |
| 138 | 61 | `ERROR` | `0800 ","` |
| 139 | 36 | `ERROR` | `12254568215815823881` |

#### `base/event_store/utility/base/event_db_file_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 142 | `ERROR` | `,` |
| 59 | 133 | `ERROR` | `,` |
| 68 | 144 | `ERROR` | `,` |

#### `base/event_store/utility/reader/content_reader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 148 | `ERROR` | `,` |
| 56 | 140 | `missing identifier` | `` |
| 74 | 136 | `missing identifier` | `` |
| 78 | 134 | `missing identifier` | `` |
| 88 | 133 | `missing identifier` | `` |
| 93 | 144 | `missing identifier` | `` |
| 98 | 142 | `missing identifier` | `` |
| 117 | 140 | `missing identifier` | `` |
| 121 | 140 | `missing identifier` | `` |
| 135 | 133 | `missing identifier` | `` |
| 146 | 135 | `missing identifier` | `` |
| 157 | 133 | `missing identifier` | `` |

#### `base/event_store/utility/reader/content_reader_version_1.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 75 | 134 | `missing identifier` | `` |

#### `base/event_store/utility/reader/content_reader_version_2.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 134 | `missing identifier` | `` |

#### `base/event_store/utility/reader/content_reader_version_3.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 125 | `missing identifier` | `` |
| 21 | 167 | `ERROR` | `,` |

#### `base/event_store/utility/reader/content_reader_version_4.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 125 | `missing identifier` | `` |
| 23 | 124 | `missing identifier` | `` |
| 27 | 80 | `ERROR` | `,` |

#### `base/event_store/utility/reader/sys_event_doc_reader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 158 | `ERROR` | `,` |
| 68 | 136 | `missing identifier` | `` |
| 74 | 160 | `ERROR` | `,` |
| 80 | 134 | `missing identifier` | `` |
| 139 | 158 | `ERROR` | `,` |
| 202 | 160 | `ERROR` | `,` |
| 208 | 185 | `ERROR` | `,` |
| 212 | 148 | `ERROR` | `,` |
| 217 | 166 | `ERROR` | `,` |
| 243 | 155 | `ERROR` | `,` |
| 247 | 162 | `ERROR` | `,` |
| 271 | 155 | `ERROR` | `,` |
| 306 | 66 | `ERROR` | `,` |

#### `base/event_store/utility/writer/sys_event_doc_writer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 115 | `missing identifier` | `` |
| 30 | 150 | `ERROR` | `,` |
| 40 | 164 | `ERROR` | `,` |
| 65 | 170 | `ERROR` | `,` |
| 71 | 189 | `ERROR` | `,` |
| 95 | 143 | `ERROR` | `,` |
| 100 | 166 | `ERROR` | `,` |
| 112 | 131 | `missing identifier` | `` |
| 117 | 158 | `ERROR` | `,` |
| 128 | 125 | `missing identifier` | `` |
| 139 | 176 | `ERROR` | `,` |
| 172 | 131 | `missing identifier` | `` |

#### `base/include/event.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 244 | 62 | `missing type_identifier` | `` |

#### `base/include/plugin.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 74 | 55 | `missing type_identifier` | `` |
| 74 | 109 | `missing type_identifier` | `` |
| 75 | 72 | `missing type_identifier` | `` |

#### `base/include/sys_event.h`

**Summary:** explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 165 col 96 (missing )) ` |
| 165 | 8 | `ERROR` | `template void SysEvent` |
| 165 | 68 | `ERROR` | `std` |
| 165 | 80 | `ERROR` | `& , std` |
| 165 | 103 | `ERROR` | `)` |
| 166 | 8 | `ERROR` | `template void SysEvent` |
| 166 | 64 | `ERROR` | `std` |
| 166 | 76 | `ERROR` | `& , uint64_t , bool` |

#### `base/logstore/log_store_ex.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 159 | `ERROR` | `,` |
| 104 | 184 | `ERROR` | `,` |
| 107 | 147 | `ERROR` | `,` |
| 149 | 136 | `ERROR` | `,` |

#### `base/pipeline.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 109 | 131 | `missing identifier` | `` |

#### `base/plugin_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 167 | `ERROR` | `,` |
| 45 | 174 | `ERROR` | `,` |
| 47 | 163 | `ERROR` | `,` |
| 52 | 152 | `ERROR` | `,` |
| 55 | 151 | `ERROR` | `,` |

#### `base/plugin_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 109 | `missing identifier` | `` |
| 12 | 174 | `ERROR` | `,` |
| 20 | 109 | `missing identifier` | `` |
| 23 | 182 | `ERROR` | `,` |
| 31 | 109 | `missing identifier` | `` |
| 34 | 187 | `ERROR` | `,` |
| 42 | 109 | `missing identifier` | `` |
| 45 | 181 | `ERROR` | `,` |
| 62 | 109 | `missing identifier` | `` |
| 65 | 191 | `ERROR` | `,` |
| 83 | 169 | `ERROR` | `,` |
| 108 | 51 | `ERROR` | `,` |
| 112 | 166 | `ERROR` | `,` |

#### `base/running_status_logger/log_file_writer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 124 | `missing identifier` | `` |
| 22 | 163 | `ERROR` | `,` |
| 52 | 164 | `ERROR` | `,` |
| 70 | 180 | `ERROR` | `,` |
| 123 | 191 | `ERROR` | `,` |
| 144 | 200 | `ERROR` | `,` |

#### `base/running_status_logger/period_file_operator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 118 | `missing identifier` | `` |
| 25 | 129 | `missing identifier` | `` |
| 31 | 165 | `ERROR` | `,` |
| 38 | 159 | `ERROR` | `,` |
| 53 | 144 | `missing identifier` | `` |
| 60 | 148 | `ERROR` | `,` |
| 69 | 116 | `missing identifier` | `` |
| 76 | 165 | `ERROR` | `,` |

#### `base/running_status_logger/running_status_logger.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 147 | `missing identifier` | `` |
| 38 | 147 | `missing identifier` | `` |

#### `base/test/unittest/common/sys_event_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 93 | `ERROR` | `0800 "," time_ ":1620271291188," "pid_":` |
| 36 | 31 | `ERROR` | `"traceid_":` |
| 36 | 63 | `ERROR` | `"spanid_":` |
| 36 | 81 | `ERROR` | `"pspanid_":` |
| 36 | 113 | `ERROR` | `: 4` |
| 36 | 118 | `ERROR` | `)` |
| 36 | 121 | `ERROR` | `";"` |
| 83 | 113 | `ERROR` | `\` |
| 83 | 116 | `ERROR` | `\ d` |
| 84 | 1 | `ERROR` | `R` |
| 84 | 65 | `ERROR` | `\` |
| 84 | 70 | `ERROR` | `","` |
| 101 | 113 | `ERROR` | `\` |
| 101 | 116 | `ERROR` | `\ d` |
| 102 | 1 | `ERROR` | `R` |
| 102 | 65 | `ERROR` | `\` |
| 102 | 70 | `ERROR` | `","` |
| 114 | 102 | `ERROR` | `0800 "," time_ ":1620271291188," "pid_":` |
| 115 | 31 | `ERROR` | `"traceid_":` |
| 115 | 63 | `ERROR` | `"spanid_":` |
| … | … | … | *(65 more)* |

#### `base/utility/ash_memory_utils.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 116 | `missing identifier` | `` |
| 21 | 121 | `missing identifier` | `` |
| 25 | 120 | `missing identifier` | `` |
| 28 | 121 | `missing identifier` | `` |

#### `base/utility/bundle_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 137 | `missing identifier` | `` |
| 23 | 135 | `missing identifier` | `` |
| 29 | 158 | `missing identifier` | `` |
| 32 | 135 | `missing identifier` | `` |
| 43 | 129 | `missing identifier` | `` |
| 49 | 127 | `missing identifier` | `` |
| 53 | 122 | `missing identifier` | `` |
| 64 | 123 | `missing identifier` | `` |
| 74 | 142 | `missing identifier` | `` |
| 86 | 131 | `missing identifier` | `` |
| 88 | 143 | `ERROR` | `,` |
| 98 | 152 | `ERROR` | `,` |
| 130 | 180 | `ERROR` | `,` |
| 133 | 195 | `ERROR` | `,` |

#### `base/utility/common_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 77 | 178 | `ERROR` | `,` |
| 114 | 138 | `ERROR` | `,` |
| 158 | 45 | `ERROR` | `,` |
| 165 | 40 | `ERROR` | `,` |
| 204 | 113 | `missing identifier` | `` |
| 209 | 132 | `ERROR` | `,` |
| 219 | 134 | `ERROR` | `,` |
| 238 | 138 | `ERROR` | `,` |
| 258 | 148 | `ERROR` | `,` |
| 286 | 150 | `ERROR` | `,` |
| 294 | 176 | `ERROR` | `,` |
| 313 | 172 | `ERROR` | `,` |
| 324 | 121 | `missing identifier` | `` |

#### `base/utility/file_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 158 | 154 | `ERROR` | `,` |

#### `base/utility/hiview_config_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 174 | `ERROR` | `,` |
| 51 | 198 | `ERROR` | `,` |
| 66 | 168 | `ERROR` | `,` |
| 71 | 149 | `ERROR` | `,` |
| 75 | 181 | `ERROR` | `,` |
| 82 | 187 | `ERROR` | `,` |
| 106 | 204 | `ERROR` | `,` |
| 115 | 169 | `ERROR` | `,` |
| 121 | 169 | `ERROR` | `,` |

#### `base/utility/hiview_db_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 157 | `ERROR` | `,` |
| 23 | 159 | `ERROR` | `,` |
| 64 | 118 | `missing identifier` | `` |
| 72 | 166 | `ERROR` | `,` |
| 76 | 154 | `ERROR` | `,` |
| 95 | 155 | `ERROR` | `,` |
| 98 | 153 | `ERROR` | `,` |
| 106 | 199 | `ERROR` | `,` |
| 108 | 153 | `ERROR` | `,` |
| 112 | 155 | `ERROR` | `,` |
| 123 | 161 | `ERROR` | `,` |

#### `base/utility/hiview_zip_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 183 | `ERROR` | `,` |
| 47 | 124 | `missing identifier` | `` |
| 53 | 129 | `missing identifier` | `` |
| 63 | 120 | `missing identifier` | `` |
| 66 | 182 | `ERROR` | `,` |
| 123 | 159 | `ERROR` | `,` |
| 134 | 181 | `ERROR` | `,` |
| 141 | 119 | `missing identifier` | `` |
| 147 | 128 | `missing identifier` | `` |
| 152 | 119 | `missing identifier` | `` |
| 157 | 126 | `missing identifier` | `` |
| 172 | 128 | `missing identifier` | `` |
| 178 | 128 | `missing identifier` | `` |
| 189 | 128 | `missing identifier` | `` |
| 195 | 128 | `missing identifier` | `` |
| 201 | 126 | `missing identifier` | `` |
| 208 | 120 | `missing identifier` | `` |

#### `base/utility/restorable_db_store.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 164 | `ERROR` | `,` |
| 27 | 139 | `missing identifier` | `` |
| 37 | 140 | `missing identifier` | `` |
| 136 | 136 | `missing identifier` | `` |
| 145 | 146 | `ERROR` | `,` |
| 155 | 138 | `missing identifier` | `` |
| 174 | 189 | `ERROR` | `,` |
| 178 | 146 | `ERROR` | `,` |
| 185 | 122 | `missing identifier` | `` |
| 189 | 163 | `ERROR` | `,` |
| 196 | 136 | `missing identifier` | `` |

#### `base/utility/setting_observer_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 118 | `missing identifier` | `` |
| 24 | 126 | `missing identifier` | `` |
| 39 | 166 | `ERROR` | `,` |
| 48 | 161 | `ERROR` | `,` |
| 55 | 170 | `ERROR` | `,` |
| 65 | 183 | `ERROR` | `,` |
| 68 | 186 | `ERROR` | `,` |
| 92 | 170 | `ERROR` | `,` |
| 97 | 166 | `ERROR` | `,` |
| 104 | 170 | `ERROR` | `,` |
| 114 | 160 | `ERROR` | `,` |
| 119 | 166 | `ERROR` | `,` |
| 125 | 172 | `ERROR` | `,` |
| 144 | 166 | `ERROR` | `,` |
| 164 | 38 | `ERROR` | `,` |

#### `core/bundle_config/plugin_bundle_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 126 | `missing identifier` | `` |
| 31 | 163 | `ERROR` | `,` |
| 38 | 53 | `ERROR` | `,` |
| 45 | 53 | `ERROR` | `,` |
| 48 | 162 | `ERROR` | `,` |

#### `core/hiview_platform.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 147 | `ERROR` | `,` |
| 85 | 176 | `ERROR` | `,` |
| 101 | 138 | `missing identifier` | `` |
| 157 | 36 | `ERROR` | `,` |
| 169 | 167 | `ERROR` | `,` |
| 175 | 161 | `ERROR` | `,` |
| 182 | 166 | `ERROR` | `,` |
| 186 | 138 | `missing identifier` | `` |
| 235 | 54 | `ERROR` | `,` |
| 242 | 161 | `ERROR` | `,` |
| 285 | 167 | `ERROR` | `,` |
| 296 | 184 | `ERROR` | `,` |
| 347 | 190 | `ERROR` | `,` |
| 355 | 64 | `ERROR` | `,` |
| 366 | 146 | `ERROR` | `,` |
| 368 | 154 | `ERROR` | `,` |
| 375 | 163 | `ERROR` | `,` |
| 395 | 146 | `ERROR` | `,` |
| 401 | 163 | `ERROR` | `,` |
| 408 | 138 | `missing identifier` | `` |
| … | … | … | *(13 more)* |

#### `core/param_update/include/param_event_manager.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 37 | `missing ;` | `` |

#### `core/param_update/src/log_sign_tools.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 16 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 123 | `missing identifier` | `` |
| 24 | 121 | `missing identifier` | `` |
| 29 | 123 | `missing identifier` | `` |
| 42 | 123 | `missing identifier` | `` |
| 50 | 142 | `missing identifier` | `` |
| 59 | 118 | `missing identifier` | `` |
| 64 | 121 | `missing identifier` | `` |
| 71 | 119 | `missing identifier` | `` |
| 83 | 118 | `missing identifier` | `` |
| 94 | 120 | `missing identifier` | `` |
| 118 | 119 | `missing identifier` | `` |
| 123 | 117 | `missing identifier` | `` |
| 131 | 117 | `missing identifier` | `` |
| 137 | 120 | `missing identifier` | `` |
| 144 | 122 | `missing identifier` | `` |
| 151 | 121 | `missing identifier` | `` |

#### `core/param_update/src/param_event_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 123 | `missing identifier` | `` |
| 22 | 138 | `missing identifier` | `` |
| 34 | 125 | `missing identifier` | `` |
| 44 | 148 | `ERROR` | `,` |
| 55 | 31 | `ERROR` | `,` |
| 58 | 120 | `missing identifier` | `` |

#### `core/param_update/src/param_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 142 | `missing identifier` | `` |
| 45 | 155 | `ERROR` | `,` |
| 57 | 146 | `ERROR` | `,` |
| 74 | 153 | `ERROR` | `,` |

#### `core/param_update/src/param_reader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 54 | `ERROR` | `,` |
| 28 | 120 | `missing identifier` | `` |
| 38 | 126 | `missing identifier` | `` |
| 46 | 128 | `missing identifier` | `` |
| 49 | 125 | `missing identifier` | `` |
| 58 | 126 | `missing identifier` | `` |
| 64 | 123 | `missing identifier` | `` |
| 77 | 126 | `missing identifier` | `` |
| 86 | 147 | `missing identifier` | `` |
| 92 | 124 | `missing identifier` | `` |
| 95 | 124 | `missing identifier` | `` |

#### `core/platform_config/hiview_platform_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 128 | `missing identifier` | `` |
| 60 | 124 | `missing identifier` | `` |
| 64 | 143 | `ERROR` | `,` |

#### `core/plugin_bundle.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 129 | `missing identifier` | `` |

#### `core/plugin_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 126 | `missing identifier` | `` |
| 28 | 120 | `missing identifier` | `` |
| 33 | 128 | `missing identifier` | `` |
| 38 | 178 | `ERROR` | `,` |
| 60 | 137 | `missing identifier` | `` |
| 72 | 122 | `missing identifier` | `` |
| 88 | 135 | `missing identifier` | `` |
| 105 | 141 | `missing identifier` | `` |
| 121 | 152 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/common_util.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 117 | `missing identifier` | `` |

#### `framework/native/unified_collection/collector/config/perf_collect_config.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 120 | `missing identifier` | `` |
| 61 | 125 | `missing identifier` | `` |
| 67 | 143 | `missing identifier` | `` |
| 90 | 120 | `missing identifier` | `` |
| 95 | 125 | `missing identifier` | `` |

#### `framework/native/unified_collection/collector/impl/cpu/calculator/cpu_calculator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 192 | `ERROR` | `,` |
| 45 | 160 | `ERROR` | `,` |
| 66 | 162 | `ERROR` | `,` |
| 79 | 171 | `ERROR` | `,` |
| 102 | 133 | `missing identifier` | `` |
| 119 | 125 | `missing identifier` | `` |

#### `framework/native/unified_collection/collector/impl/cpu/device_client/collect_device_client.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 183 | `ERROR` | `,` |
| 38 | 168 | `ERROR` | `,` |
| 54 | 132 | `missing identifier` | `` |
| 56 | 61 | `ERROR` | `int` |
| 58 | 156 | `ERROR` | `,` |
| 67 | 134 | `missing identifier` | `` |
| 68 | 104 | `ERROR` | `struct` |
| 70 | 50 | `ERROR` | `struct` |
| 72 | 157 | `ERROR` | `,` |
| 80 | 134 | `missing identifier` | `` |
| 81 | 104 | `ERROR` | `struct` |
| 83 | 50 | `ERROR` | `struct` |
| 85 | 157 | `ERROR` | `,` |
| 93 | 134 | `missing identifier` | `` |
| 95 | 54 | `ERROR` | `struct` |
| 97 | 162 | `ERROR` | `,` |
| 105 | 138 | `missing identifier` | `` |
| 107 | 58 | `ERROR` | `struct` |
| 109 | 161 | `ERROR` | `,` |
| 117 | 132 | `missing identifier` | `` |
| … | … | … | *(11 more)* |

#### `framework/native/unified_collection/collector/impl/cpu/src/cpu_collector_impl.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 130 | `missing identifier` | `` |

#### `framework/native/unified_collection/collector/impl/cpu/src/process_state_info_collector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 122 | `missing identifier` | `` |
| 75 | 192 | `ERROR` | `,` |
| 133 | 170 | `ERROR` | `,` |
| 172 | 179 | `ERROR` | `,` |
| 181 | 133 | `ERROR` | `,` |
| 226 | 146 | `ERROR` | `,` |
| 239 | 183 | `ERROR` | `,` |
| 247 | 181 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/cpu/src/sys_cpu_usage_collector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 59 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/cpu/src/thread_state_info_collector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 130 | `missing identifier` | `` |
| 48 | 118 | `missing identifier` | `` |
| 105 | 63 | `ERROR` | `,` |
| 122 | 122 | `missing identifier` | `` |
| 167 | 180 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/cpu/utils/cpu_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 160 | `ERROR` | `,` |
| 49 | 182 | `ERROR` | `,` |
| 57 | 171 | `ERROR` | `,` |
| 67 | 154 | `ERROR` | `,` |
| 74 | 159 | `ERROR` | `,` |
| 88 | 151 | `ERROR` | `,` |
| 98 | 158 | `ERROR` | `,` |
| 120 | 71 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/gpu/gpu_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 66 | `ERROR` | `,` |
| 43 | 143 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/graphic/graphic_memory_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 146 | `ERROR` | `,` |
| 37 | 145 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/hilog/hilog_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 119 | `missing identifier` | `` |
| 23 | 152 | `ERROR` | `,` |
| 36 | 120 | `missing identifier` | `` |
| 50 | 112 | `missing identifier` | `` |
| 56 | 112 | `missing identifier` | `` |
| 69 | 170 | `ERROR` | `,` |
| 71 | 139 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/io/calculator/io_calculator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 151 | `ERROR` | `,` |
| 42 | 151 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/io/io_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 119 | `missing identifier` | `` |
| 94 | 174 | `ERROR` | `,` |
| 129 | 148 | `ERROR` | `,` |
| 138 | 141 | `ERROR` | `,` |
| 143 | 120 | `missing identifier` | `` |
| 189 | 152 | `ERROR` | `,` |
| 195 | 152 | `ERROR` | `,` |
| 214 | 146 | `ERROR` | `,` |
| 252 | 153 | `ERROR` | `,` |
| 276 | 159 | `ERROR` | `,` |
| 310 | 137 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/memory/memory_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 165 | `ERROR` | `,` |
| 59 | 164 | `ERROR` | `,` |
| 78 | 160 | `ERROR` | `,` |
| 82 | 168 | `ERROR` | `,` |
| 90 | 144 | `ERROR` | `,` |
| 96 | 147 | `ERROR` | `,` |
| 129 | 161 | `ERROR` | `,` |
| 178 | 178 | `ERROR` | `,` |
| 207 | 131 | `ERROR` | `,` |
| 223 | 146 | `ERROR` | `,` |
| 230 | 146 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/memory/utils/memory_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 181 | 140 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/perf/perf_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 98 | 121 | `missing identifier` | `` |
| 103 | 155 | `ERROR` | `,` |
| 106 | 131 | `missing identifier` | `` |
| 145 | 122 | `missing identifier` | `` |
| 150 | 151 | `ERROR` | `,` |
| 162 | 124 | `missing identifier` | `` |
| 167 | 135 | `ERROR` | `,` |
| 173 | 106 | `missing identifier` | `` |
| 178 | 127 | `ERROR` | `,` |
| 184 | 106 | `missing identifier` | `` |
| 189 | 127 | `ERROR` | `,` |
| 195 | 106 | `missing identifier` | `` |
| 200 | 127 | `ERROR` | `,` |
| 206 | 106 | `missing identifier` | `` |
| 211 | 127 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/thermal/thermal_collector_impl.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 117 | `missing identifier` | `` |

#### `framework/native/unified_collection/collector/impl/trace/strategy/include/trace_handler.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 100 | `missing type_identifier` | `` |
| 35 | 92 | `missing type_identifier` | `` |

#### `framework/native/unified_collection/collector/impl/trace/strategy/src/trace_handler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 200 | `ERROR` | `,` |
| 60 | 65 | `ERROR` | `,` |
| 84 | 196 | `ERROR` | `,` |
| 90 | 148 | `ERROR` | `,` |
| 94 | 146 | `ERROR` | `,` |
| 98 | 151 | `ERROR` | `,` |
| 113 | 121 | `missing identifier` | `` |
| 121 | 167 | `ERROR` | `,` |
| 127 | 173 | `ERROR` | `,` |
| 144 | 155 | `ERROR` | `,` |
| 162 | 142 | `ERROR` | `,` |
| 170 | 167 | `ERROR` | `,` |
| 181 | 155 | `ERROR` | `,` |
| 188 | 150 | `ERROR` | `,` |
| 192 | 152 | `ERROR` | `,` |
| 197 | 30 | `ERROR` | `,` |
| 202 | 198 | `ERROR` | `,` |
| 209 | 121 | `missing identifier` | `` |
| 231 | 176 | `ERROR` | `,` |

#### `framework/native/unified_collection/collector/impl/trace/strategy/src/trace_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 66 | 150 | `ERROR` | `,` |
| 117 | 137 | `missing identifier` | `` |
| 125 | 126 | `missing identifier` | `` |
| 140 | 158 | `missing identifier` | `` |
| 152 | 122 | `missing identifier` | `` |
| 156 | 134 | `missing identifier` | `` |
| 166 | 145 | `missing identifier` | `` |
| 177 | 136 | `missing identifier` | `` |
| 189 | 121 | `missing identifier` | `` |
| 196 | 53 | `ERROR` | `,` |
| 201 | 134 | `missing identifier` | `` |
| 205 | 144 | `missing identifier` | `` |
| 229 | 139 | `missing identifier` | `` |
| 249 | 158 | `missing identifier` | `` |
| 268 | 192 | `ERROR` | `,` |
| 295 | 189 | `ERROR` | `,` |
| 301 | 165 | `ERROR` | `,` |
| 305 | 134 | `missing identifier` | `` |
| 327 | 152 | `ERROR` | `,` |
| 349 | 116 | `missing identifier` | `` |
| … | … | … | *(1 more)* |

#### `framework/native/unified_collection/collector/impl/trace/trace_collector_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 155 | `ERROR` | `,` |
| 33 | 143 | `ERROR` | `,` |
| 36 | 186 | `ERROR` | `,` |
| 57 | 175 | `ERROR` | `,` |
| 60 | 170 | `ERROR` | `,` |
| 66 | 167 | `ERROR` | `,` |
| 78 | 175 | `ERROR` | `,` |
| 90 | 206 | `ERROR` | `,` |
| 98 | 175 | `ERROR` | `,` |
| 108 | 53 | `ERROR` | `,` |
| 116 | 164 | `ERROR` | `,` |
| 120 | 160 | `ERROR` | `,` |
| 125 | 176 | `ERROR` | `,` |
| 133 | 118 | `missing identifier` | `` |
| 147 | 175 | `ERROR` | `,` |
| 150 | 151 | `ERROR` | `,` |
| 157 | 175 | `ERROR` | `,` |
| 166 | 132 | `missing identifier` | `` |
| 189 | 175 | `ERROR` | `,` |
| 196 | 187 | `ERROR` | `,` |
| … | … | … | *(13 more)* |

#### `framework/native/unified_collection/collector/impl/trace/trace_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 126 | `missing identifier` | `` |
| 59 | 126 | `missing identifier` | `` |
| 68 | 126 | `missing identifier` | `` |
| 134 | 162 | `ERROR` | `,` |
| 139 | 162 | `ERROR` | `,` |

#### `framework/native/unified_collection/decorator/unified_collection_stat.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 183 | `ERROR` | `,` |
| 51 | 157 | `ERROR` | `,` |

#### `framework/native/unified_collection/process/process_status.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 152 | `ERROR` | `,` |
| 49 | 205 | `ERROR` | `,` |
| 59 | 203 | `ERROR` | `,` |
| 104 | 152 | `ERROR` | `,` |
| 119 | 163 | `ERROR` | `,` |
| 159 | 151 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/src/trace_db_callback.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 138 | `missing identifier` | `` |
| 23 | 156 | `ERROR` | `,` |
| 46 | 152 | `ERROR` | `,` |
| 49 | 165 | `ERROR` | `,` |
| 63 | 144 | `missing identifier` | `` |
| 66 | 156 | `ERROR` | `,` |
| 82 | 157 | `ERROR` | `,` |
| 85 | 156 | `ERROR` | `,` |
| 101 | 152 | `ERROR` | `,` |
| 104 | 156 | `ERROR` | `,` |
| 113 | 115 | `missing identifier` | `` |
| 115 | 143 | `missing identifier` | `` |
| 118 | 148 | `missing identifier` | `` |
| 121 | 149 | `missing identifier` | `` |
| 124 | 147 | `missing identifier` | `` |
| 127 | 143 | `missing identifier` | `` |
| 134 | 172 | `ERROR` | `,` |
| 137 | 174 | `ERROR` | `,` |
| 141 | 174 | `ERROR` | `,` |
| 145 | 178 | `ERROR` | `,` |
| … | … | … | *(2 more)* |

#### `framework/native/unified_collection/trace_manager/src/trace_flow_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 119 | `missing identifier` | `` |
| 144 | 152 | `ERROR` | `,` |
| 147 | 178 | `ERROR` | `,` |
| 177 | 145 | `missing identifier` | `` |
| 180 | 148 | `ERROR` | `,` |
| 182 | 173 | `ERROR` | `,` |
| 194 | 133 | `missing identifier` | `` |
| 203 | 145 | `missing identifier` | `` |
| 212 | 145 | `missing identifier` | `` |
| 221 | 141 | `missing identifier` | `` |
| 230 | 136 | `missing identifier` | `` |
| 239 | 137 | `missing identifier` | `` |

#### `framework/native/unified_collection/trace_manager/src/trace_state_machine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 175 | `ERROR` | `,` |
| 119 | 30 | `ERROR` | `,` |
| 128 | 121 | `missing identifier` | `` |
| 134 | 120 | `missing identifier` | `` |
| 140 | 116 | `missing identifier` | `` |
| 146 | 121 | `missing identifier` | `` |
| 153 | 119 | `missing identifier` | `` |
| 159 | 115 | `missing identifier` | `` |
| 165 | 117 | `missing identifier` | `` |
| 172 | 122 | `missing identifier` | `` |
| 226 | 120 | `missing identifier` | `` |
| 232 | 151 | `missing identifier` | `` |
| 238 | 117 | `missing identifier` | `` |
| 269 | 136 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/state/trace_app_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 168 | `ERROR` | `,` |
| 17 | 162 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/state/trace_base_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 200 | `ERROR` | `,` |
| 20 | 179 | `ERROR` | `,` |
| 27 | 44 | `ERROR` | `,` |
| 32 | 38 | `ERROR` | `,` |
| 43 | 146 | `ERROR` | `,` |
| 63 | 120 | `missing identifier` | `` |
| 68 | 36 | `ERROR` | `,` |
| 73 | 184 | `ERROR` | `,` |
| 82 | 167 | `ERROR` | `,` |
| 88 | 167 | `ERROR` | `,` |
| 100 | 201 | `ERROR` | `,` |
| 107 | 204 | `ERROR` | `,` |
| 113 | 201 | `ERROR` | `,` |
| 119 | 201 | `ERROR` | `,` |
| 126 | 156 | `ERROR` | `,` |
| 132 | 154 | `ERROR` | `,` |
| 138 | 154 | `ERROR` | `,` |
| 164 | 154 | `ERROR` | `,` |
| 170 | 154 | `ERROR` | `,` |
| 176 | 154 | `ERROR` | `,` |
| … | … | … | *(2 more)* |

#### `framework/native/unified_collection/trace_manager/state/trace_command_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 168 | `ERROR` | `,` |
| 18 | 162 | `ERROR` | `,` |
| 25 | 168 | `ERROR` | `,` |
| 29 | 152 | `ERROR` | `,` |
| 48 | 171 | `ERROR` | `,` |
| 52 | 168 | `ERROR` | `,` |
| 63 | 36 | `ERROR` | `,` |
| 68 | 184 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/state/trace_common_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 161 | `ERROR` | `,` |
| 25 | 165 | `ERROR` | `,` |
| 33 | 171 | `ERROR` | `,` |
| 37 | 159 | `ERROR` | `,` |
| 51 | 132 | `missing identifier` | `` |
| 54 | 165 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/state/trace_dynamic_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 166 | `ERROR` | `,` |
| 17 | 160 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/state/trace_telemetry_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 132 | `missing identifier` | `` |
| 33 | 153 | `ERROR` | `,` |
| 40 | 172 | `ERROR` | `,` |
| 44 | 132 | `missing identifier` | `` |
| 53 | 172 | `ERROR` | `,` |
| 57 | 133 | `missing identifier` | `` |
| 66 | 172 | `ERROR` | `,` |
| 75 | 172 | `ERROR` | `,` |
| 84 | 172 | `ERROR` | `,` |
| 93 | 172 | `ERROR` | `,` |
| 101 | 151 | `ERROR` | `,` |
| 112 | 120 | `missing identifier` | `` |

#### `framework/native/unified_collection/trace_manager/storage/app_event_task_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 153 | `ERROR` | `,` |
| 23 | 160 | `ERROR` | `,` |
| 26 | 160 | `ERROR` | `,` |
| 29 | 154 | `ERROR` | `,` |
| 32 | 154 | `ERROR` | `,` |
| 35 | 162 | `ERROR` | `,` |
| 38 | 165 | `ERROR` | `,` |
| 41 | 161 | `ERROR` | `,` |
| 44 | 162 | `ERROR` | `,` |
| 47 | 164 | `ERROR` | `,` |
| 50 | 164 | `ERROR` | `,` |
| 53 | 159 | `ERROR` | `,` |
| 56 | 156 | `ERROR` | `,` |
| 74 | 161 | `ERROR` | `,` |
| 88 | 119 | `missing identifier` | `` |
| 99 | 119 | `missing identifier` | `` |
| 117 | 193 | `ERROR` | `,` |
| 126 | 119 | `missing identifier` | `` |
| 134 | 156 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/storage/app_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 119 | `missing identifier` | `` |
| 54 | 155 | `ERROR` | `,` |
| 61 | 173 | `ERROR` | `,` |
| 72 | 179 | `ERROR` | `,` |
| 75 | 146 | `ERROR` | `,` |
| 105 | 123 | `missing identifier` | `` |
| 110 | 138 | `missing identifier` | `` |
| 127 | 123 | `missing identifier` | `` |
| 138 | 123 | `missing identifier` | `` |
| 149 | 173 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/storage/telemetry_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 166 | `ERROR` | `,` |
| 37 | 160 | `ERROR` | `,` |
| 47 | 116 | `missing identifier` | `` |
| 65 | 161 | `ERROR` | `,` |
| 74 | 116 | `missing identifier` | `` |
| 79 | 122 | `missing identifier` | `` |
| 94 | 117 | `missing identifier` | `` |
| 101 | 156 | `ERROR` | `,` |
| 108 | 118 | `missing identifier` | `` |
| 116 | 166 | `ERROR` | `,` |
| 122 | 112 | `missing identifier` | `` |
| 127 | 112 | `missing identifier` | `` |
| 132 | 112 | `missing identifier` | `` |
| 145 | 118 | `missing identifier` | `` |
| 155 | 123 | `missing identifier` | `` |
| 165 | 116 | `missing identifier` | `` |
| 170 | 155 | `ERROR` | `,` |
| 175 | 122 | `missing identifier` | `` |

#### `framework/native/unified_collection/trace_manager/storage/trace_behavior_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 118 | `missing identifier` | `` |
| 30 | 162 | `ERROR` | `,` |
| 37 | 195 | `ERROR` | `,` |
| 49 | 118 | `missing identifier` | `` |
| 58 | 123 | `missing identifier` | `` |
| 67 | 118 | `missing identifier` | `` |
| 70 | 158 | `ERROR` | `,` |
| 77 | 123 | `missing identifier` | `` |

#### `framework/native/unified_collection/trace_manager/storage/trace_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 119 | `missing identifier` | `` |
| 71 | 123 | `missing identifier` | `` |
| 80 | 123 | `missing identifier` | `` |
| 87 | 118 | `missing identifier` | `` |
| 100 | 170 | `ERROR` | `,` |
| 191 | 123 | `missing identifier` | `` |
| 196 | 131 | `missing identifier` | `` |
| 202 | 132 | `missing identifier` | `` |
| 217 | 71 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/telemetry/telemetry_state_machine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 154 | `ERROR` | `,` |
| 21 | 117 | `missing identifier` | `` |
| 32 | 152 | `ERROR` | `,` |
| 40 | 154 | `ERROR` | `,` |
| 45 | 117 | `missing identifier` | `` |
| 57 | 145 | `ERROR` | `,` |
| 74 | 180 | `ERROR` | `,` |
| 82 | 154 | `ERROR` | `,` |
| 89 | 167 | `ERROR` | `,` |
| 93 | 119 | `missing identifier` | `` |
| 96 | 146 | `ERROR` | `,` |
| 107 | 144 | `missing identifier` | `` |
| 119 | 154 | `ERROR` | `,` |
| 124 | 162 | `ERROR` | `,` |
| 129 | 198 | `ERROR` | `,` |
| 133 | 118 | `missing identifier` | `` |
| 151 | 147 | `ERROR` | `,` |
| 154 | 113 | `missing identifier` | `` |
| 163 | 147 | `ERROR` | `,` |
| 166 | 140 | `ERROR` | `,` |

#### `framework/native/unified_collection/trace_manager/test/trace_manager_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 1 | `ERROR` | `.args` |
| 65 | 1 | `ERROR` | `.args` |
| 73 | 1 | `ERROR` | `.args` |

#### `hiretrieval/frameworks/include/hiretrieval_base_def.h`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 2 | `missing identifier` | `` |

#### `hiretrieval/interfaces/ets/ani/src/hiretrieval_ani.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 12 | `ERROR` | `ani_status` |

#### `hiretrieval/interfaces/js/napi/src/hiretrieval_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 89 | 8 | `ERROR` | `napi_value` |
| 106 | 20 | `ERROR` | `_module` |

#### `interfaces/ets/ani/loglibrary/src/loglibrary_ani.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 132 | `ERROR` | `,` |
| 61 | 132 | `ERROR` | `,` |
| 67 | 12 | `ERROR` | `ani_status` |
| 71 | 127 | `missing identifier` | `` |
| 77 | 158 | `ERROR` | `,` |
| 89 | 174 | `ERROR` | `,` |

#### `interfaces/ets/ani/loglibrary/src/loglibrary_ani_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 127 | `missing identifier` | `` |
| 23 | 123 | `missing identifier` | `` |
| 39 | 124 | `missing identifier` | `` |
| 44 | 159 | `ERROR` | `,` |
| 48 | 160 | `ERROR` | `,` |
| 52 | 159 | `ERROR` | `,` |
| 56 | 166 | `ERROR` | `,` |
| 69 | 152 | `ERROR` | `,` |
| 75 | 151 | `ERROR` | `,` |
| 80 | 157 | `ERROR` | `,` |
| 92 | 122 | `missing identifier` | `` |
| 101 | 128 | `missing identifier` | `` |
| 117 | 158 | `ERROR` | `,` |
| 122 | 146 | `missing identifier` | `` |
| 127 | 158 | `ERROR` | `,` |
| 131 | 140 | `missing identifier` | `` |
| 136 | 127 | `missing identifier` | `` |
| 140 | 143 | `missing identifier` | `` |
| 144 | 136 | `missing identifier` | `` |
| 176 | 156 | `ERROR` | `,` |
| … | … | … | *(9 more)* |

#### `interfaces/inner_api/unified_collection/client/src/trace_collector_client_impl.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 43 | `missing type_identifier` | `` |

#### `interfaces/inner_api/unified_collection/client/trace_collector_client.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 43 | `missing type_identifier` | `` |

#### `interfaces/js/napi/src/hiview_napi_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 135 | `missing identifier` | `` |
| 26 | 136 | `missing identifier` | `` |
| 33 | 136 | `missing identifier` | `` |
| 40 | 137 | `missing identifier` | `` |
| 52 | 129 | `missing identifier` | `` |
| 58 | 145 | `ERROR` | `,` |
| 86 | 157 | `ERROR` | `,` |
| 94 | 162 | `ERROR` | `,` |
| 101 | 144 | `missing identifier` | `` |
| 131 | 137 | `missing identifier` | `` |
| 166 | 179 | `ERROR` | `,` |

#### `interfaces/js/napi/src/hiview_service_agent.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 121 | `missing identifier` | `` |
| 42 | 121 | `missing identifier` | `` |
| 47 | 121 | `missing identifier` | `` |
| 58 | 121 | `missing identifier` | `` |
| 71 | 133 | `missing identifier` | `` |
| 78 | 128 | `missing identifier` | `` |
| 83 | 139 | `missing identifier` | `` |
| 95 | 155 | `missing identifier` | `` |
| 102 | 128 | `missing identifier` | `` |
| 104 | 122 | `missing identifier` | `` |
| 111 | 149 | `ERROR` | `,` |
| 117 | 118 | `missing identifier` | `` |
| 123 | 120 | `missing identifier` | `` |
| 129 | 132 | `missing identifier` | `` |
| 134 | 127 | `missing identifier` | `` |
| 138 | 125 | `missing identifier` | `` |
| 161 | 120 | `missing identifier` | `` |
| 165 | 124 | `missing identifier` | `` |

#### `interfaces/js/napi/src/napi_hiview_js.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 151 | `ERROR` | `,` |
| 38 | 145 | `ERROR` | `,` |
| 54 | 151 | `ERROR` | `,` |
| 63 | 123 | `missing identifier` | `` |
| 66 | 165 | `ERROR` | `,` |
| 68 | 155 | `ERROR` | `,` |
| 74 | 127 | `missing identifier` | `` |
| 78 | 125 | `missing identifier` | `` |
| 122 | 151 | `ERROR` | `,` |
| 133 | 132 | `ERROR` | `,` |
| 140 | 8 | `ERROR` | `napi_value` |
| 153 | 20 | `ERROR` | `_module` |

#### `main.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 145 | `missing identifier` | `` |
| 17 | 138 | `missing identifier` | `` |

#### `plugins/crash_validator/crash_validator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 124 | `missing identifier` | `` |
| 29 | 121 | `missing identifier` | `` |
| 35 | 123 | `missing identifier` | `` |
| 56 | 115 | `missing identifier` | `` |
| 60 | 131 | `missing identifier` | `` |
| 65 | 118 | `missing identifier` | `` |
| 83 | 122 | `missing identifier` | `` |
| 110 | 155 | `ERROR` | `,` |
| 136 | 155 | `ERROR` | `,` |
| 142 | 152 | `ERROR` | `,` |
| 151 | 161 | `ERROR` | `,` |
| 177 | 127 | `missing identifier` | `` |
| 223 | 82 | `ERROR` | `,` |
| 236 | 131 | `missing identifier` | `` |
| 262 | 134 | `missing identifier` | `` |

#### `plugins/event_store/event_export/config/export_config_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 122 | `missing identifier` | `` |
| 80 | 180 | `ERROR` | `,` |
| 86 | 142 | `missing identifier` | `` |
| 105 | 170 | `ERROR` | `,` |
| 111 | 177 | `ERROR` | `,` |
| 117 | 190 | `ERROR` | `,` |

#### `plugins/event_store/event_export/config/export_config_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 136 | `missing identifier` | `` |
| 64 | 167 | `ERROR` | `,` |
| 70 | 146 | `ERROR` | `,` |
| 86 | 152 | `missing identifier` | `` |
| 95 | 142 | `missing identifier` | `` |
| 100 | 142 | `missing identifier` | `` |
| 104 | 135 | `missing identifier` | `` |
| 114 | 138 | `missing identifier` | `` |
| 119 | 152 | `missing identifier` | `` |
| 124 | 161 | `missing identifier` | `` |
| 135 | 139 | `missing identifier` | `` |
| 141 | 144 | `missing identifier` | `` |
| 147 | 147 | `missing identifier` | `` |
| 152 | 143 | `missing identifier` | `` |
| 167 | 182 | `ERROR` | `,` |
| 179 | 127 | `missing identifier` | `` |
| 211 | 127 | `missing identifier` | `` |
| 232 | 131 | `missing identifier` | `` |

#### `plugins/event_store/event_export/config/export_event_list_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 167 | `ERROR` | `,` |
| 78 | 130 | `missing identifier` | `` |
| 83 | 128 | `missing identifier` | `` |
| 89 | 129 | `missing identifier` | `` |
| 105 | 141 | `missing identifier` | `` |
| 110 | 155 | `missing identifier` | `` |

#### `plugins/event_store/event_export/database/adapter/export_db_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 175 | `ERROR` | `,` |
| 37 | 53 | `ERROR` | `,` |
| 60 | 53 | `ERROR` | `,` |
| 87 | 118 | `missing identifier` | `` |
| 97 | 53 | `ERROR` | `,` |
| 115 | 118 | `missing identifier` | `` |
| 119 | 139 | `missing identifier` | `` |
| 130 | 117 | `missing identifier` | `` |
| 134 | 185 | `ERROR` | `,` |
| 140 | 169 | `ERROR` | `,` |
| 147 | 177 | `missing identifier` | `` |
| 158 | 173 | `ERROR` | `,` |
| 167 | 118 | `missing identifier` | `` |
| 178 | 53 | `ERROR` | `,` |

#### `plugins/event_store/event_export/database/export_db_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 181 | `ERROR` | `,` |
| 61 | 188 | `ERROR` | `,` |
| 66 | 177 | `ERROR` | `,` |
| 78 | 183 | `ERROR` | `,` |
| 92 | 181 | `ERROR` | `,` |
| 103 | 156 | `missing identifier` | `` |
| 118 | 181 | `ERROR` | `,` |

#### `plugins/event_store/event_export/event_export_engine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 125 | `missing identifier` | `` |
| 63 | 124 | `missing identifier` | `` |
| 78 | 165 | `ERROR` | `,` |
| 107 | 128 | `missing identifier` | `` |
| 124 | 120 | `missing identifier` | `` |
| 126 | 121 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/expire/event_delete_handler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 21 | `ERROR` | `,` |

#### `plugins/event_store/event_export/task/expire/event_expire_task.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 146 | `ERROR` | `,` |
| 26 | 126 | `missing identifier` | `` |
| 41 | 141 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/expire/event_scan_handler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 119 | `missing identifier` | `` |
| 32 | 164 | `ERROR` | `,` |
| 56 | 172 | `ERROR` | `,` |

#### `plugins/event_store/event_export/task/export/event_export_task.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 168 | `ERROR` | `,` |
| 42 | 126 | `missing identifier` | `` |
| 49 | 140 | `missing identifier` | `` |
| 53 | 132 | `missing identifier` | `` |
| 59 | 129 | `missing identifier` | `` |
| 91 | 126 | `missing identifier` | `` |
| 107 | 140 | `missing identifier` | `` |
| 110 | 155 | `missing identifier` | `` |
| 124 | 127 | `missing identifier` | `` |
| 141 | 126 | `missing identifier` | `` |
| 147 | 141 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/export/event_export_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 126 | `missing identifier` | `` |
| 54 | 122 | `missing identifier` | `` |
| 58 | 124 | `missing identifier` | `` |
| 113 | 123 | `missing identifier` | `` |
| 128 | 181 | `ERROR` | `,` |
| 147 | 191 | `ERROR` | `,` |
| 163 | 192 | `ERROR` | `,` |
| 175 | 123 | `missing identifier` | `` |
| 180 | 175 | `ERROR` | `,` |
| 188 | 174 | `ERROR` | `,` |

#### `plugins/event_store/event_export/task/export/event_read_handler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 135 | 47 | `ERROR` | `,` |
| 161 | 154 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/export/event_write_handler.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 115 | `missing identifier` | `` |
| 20 | 140 | `missing identifier` | `` |
| 33 | 130 | `missing identifier` | `` |
| 47 | 123 | `missing identifier` | `` |
| 52 | 122 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/export/export_event_packager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 126 | `missing identifier` | `` |
| 52 | 131 | `missing identifier` | `` |
| 68 | 133 | `missing identifier` | `` |
| 80 | 184 | `ERROR` | `,` |
| 92 | 94 | `ERROR` | `,` |
| 96 | 95 | `ERROR` | `,` |

#### `plugins/event_store/event_export/task/export/export_file_writer.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 129 | `missing identifier` | `` |
| 21 | 124 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/export/export_json_file_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 137 | `missing identifier` | `` |
| 45 | 143 | `missing identifier` | `` |
| 57 | 137 | `missing identifier` | `` |
| 71 | 137 | `missing identifier` | `` |
| 86 | 136 | `missing identifier` | `` |
| 91 | 134 | `missing identifier` | `` |
| 99 | 36 | `ERROR` | `,` |
| 104 | 36 | `ERROR` | `,` |
| 112 | 147 | `missing identifier` | `` |
| 126 | 129 | `missing identifier` | `` |
| 137 | 142 | `missing identifier` | `` |
| 145 | 146 | `missing identifier` | `` |
| 159 | 128 | `missing identifier` | `` |
| 172 | 134 | `missing identifier` | `` |
| 177 | 140 | `missing identifier` | `` |
| 182 | 141 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/export/write_zip_file_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 159 | `missing identifier` | `` |
| 54 | 158 | `missing identifier` | `` |
| 64 | 128 | `missing identifier` | `` |
| 68 | 129 | `missing identifier` | `` |
| 85 | 139 | `ERROR` | `,` |
| 101 | 92 | `ERROR` | `,` |
| 125 | 128 | `missing identifier` | `` |
| 134 | 181 | `ERROR` | `,` |
| 138 | 178 | `ERROR` | `,` |

#### `plugins/event_store/event_export/task/export_dir_creator.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 128 | `missing identifier` | `` |
| 20 | 129 | `missing identifier` | `` |
| 41 | 133 | `missing identifier` | `` |

#### `plugins/event_store/event_export/task/trigger/trigger_export_task.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 115 | `missing identifier` | `` |
| 34 | 126 | `missing identifier` | `` |
| 48 | 133 | `missing identifier` | `` |
| 62 | 126 | `missing identifier` | `` |
| 99 | 127 | `missing identifier` | `` |
| 104 | 132 | `missing identifier` | `` |

#### `plugins/event_store/event_export/test/unittest/common/event_export_write_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 113 | `ERROR` | `0000 "," "pid_":` |
| 23 | 19 | `ERROR` | `: 1` |
| 23 | 31 | `ERROR` | `: 0` |
| 23 | 43 | `ERROR` | `: 0` |
| 23 | 49 | `ERROR` | `"id_":` |
| 23 | 81 | `ERROR` | `"PARAM1":` |
| 23 | 109 | `ERROR` | `: 2317` |
| 24 | 1 | `ERROR` | `"period_seq_":` |
| 24 | 21 | `ERROR` | `"level_":` |
| 24 | 44 | `ERROR` | `"seq_": 76258 })` |

#### `plugins/event_store/event_export/trigger_export_engine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 171 | `ERROR` | `,` |
| 122 | 128 | `missing identifier` | `` |

#### `plugins/event_store/store/sys_event_db_mgr.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 146 | `missing identifier` | `` |
| 23 | 122 | `missing identifier` | `` |
| 31 | 121 | `missing identifier` | `` |

#### `plugins/event_store/sys_event_store.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 123 | `missing identifier` | `` |
| 54 | 125 | `missing identifier` | `` |
| 67 | 169 | `ERROR` | `,` |
| 76 | 115 | `missing identifier` | `` |
| 80 | 131 | `missing identifier` | `` |
| 85 | 118 | `missing identifier` | `` |
| 93 | 127 | `missing identifier` | `` |
| 136 | 171 | `ERROR` | `,` |
| 138 | 153 | `missing identifier` | `` |

#### `plugins/event_validator/control/config/daily_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 162 | `ERROR` | `,` |
| 31 | 53 | `ERROR` | `,` |
| 46 | 130 | `missing identifier` | `` |
| 57 | 162 | `ERROR` | `,` |
| 61 | 185 | `ERROR` | `,` |
| 71 | 130 | `missing identifier` | `` |
| 88 | 130 | `missing identifier` | `` |
| 94 | 127 | `missing identifier` | `` |
| 98 | 153 | `ERROR` | `,` |
| 115 | 125 | `missing identifier` | `` |
| 122 | 181 | `ERROR` | `,` |
| 126 | 166 | `ERROR` | `,` |

#### `plugins/event_validator/control/daily_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 114 | `missing identifier` | `` |
| 71 | 164 | `ERROR` | `,` |
| 86 | 57 | `ERROR` | `,` |
| 108 | 171 | `ERROR` | `,` |
| 125 | 173 | `ERROR` | `,` |
| 138 | 79 | `ERROR` | `,` |
| 175 | 68 | `ERROR` | `,` |

#### `plugins/event_validator/control/db/daily_db_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 172 | `ERROR` | `,` |
| 33 | 122 | `missing identifier` | `` |
| 40 | 172 | `ERROR` | `,` |
| 60 | 116 | `missing identifier` | `` |
| 67 | 156 | `ERROR` | `,` |
| 72 | 158 | `ERROR` | `,` |
| 85 | 143 | `ERROR` | `,` |
| 103 | 58 | `ERROR` | `,` |
| 108 | 58 | `ERROR` | `,` |
| 130 | 72 | `ERROR` | `,` |
| 149 | 58 | `ERROR` | `,` |
| 162 | 58 | `ERROR` | `,` |
| 169 | 72 | `ERROR` | `,` |
| 183 | 128 | `missing identifier` | `` |
| 220 | 146 | `ERROR` | `,` |

#### `plugins/event_validator/event_param_watcher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 117 | `missing identifier` | `` |
| 16 | 117 | `missing identifier` | `` |
| 22 | 188 | `ERROR` | `,` |
| 26 | 162 | `ERROR` | `,` |
| 34 | 171 | `ERROR` | `,` |

#### `plugins/event_validator/event_period_info_util.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 114 | `missing identifier` | `` |
| 105 | 132 | `missing identifier` | `` |

#### `plugins/event_validator/event_validator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 129 | `missing identifier` | `` |
| 22 | 116 | `missing identifier` | `` |
| 52 | 131 | `missing identifier` | `` |
| 63 | 114 | `missing identifier` | `` |
| 67 | 123 | `missing identifier` | `` |
| 110 | 118 | `missing identifier` | `` |
| 117 | 75 | `ERROR` | `,` |

#### `plugins/event_validator/event_verify_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 69 | `ERROR` | `,` |

#### `plugins/eventlogger/config/event_logger_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 118 | `missing identifier` | `` |
| 36 | 126 | `missing identifier` | `` |
| 57 | 120 | `missing identifier` | `` |
| 64 | 122 | `missing identifier` | `` |
| 72 | 122 | `missing identifier` | `` |
| 79 | 122 | `missing identifier` | `` |
| 86 | 122 | `missing identifier` | `` |
| 93 | 141 | `ERROR` | `,` |
| 160 | 116 | `missing identifier` | `` |
| 211 | 122 | `missing identifier` | `` |
| 219 | 107 | `missing identifier` | `` |
| 228 | 163 | `ERROR` | `,` |
| 236 | 163 | `ERROR` | `,` |
| 240 | 163 | `ERROR` | `,` |
| 244 | 163 | `ERROR` | `,` |
| 248 | 163 | `ERROR` | `,` |
| 262 | 107 | `missing identifier` | `` |
| 277 | 106 | `ERROR` | `,` |

#### `plugins/eventlogger/event_logger.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 179 | 200 | `ERROR` | `,` |
| 181 | 200 | `ERROR` | `,` |
| 199 | 165 | `ERROR` | `,` |
| 213 | 184 | `ERROR` | `,` |
| 218 | 192 | `ERROR` | `,` |
| 267 | 132 | `ERROR` | `,` |
| 353 | 172 | `ERROR` | `,` |
| 369 | 162 | `ERROR` | `,` |
| 372 | 162 | `ERROR` | `,` |
| 380 | 156 | `ERROR` | `,` |
| 438 | 164 | `ERROR` | `,` |
| 458 | 158 | `ERROR` | `,` |
| 511 | 158 | `ERROR` | `,` |
| 538 | 129 | `missing identifier` | `` |
| 680 | 120 | `missing identifier` | `` |
| 689 | 153 | `ERROR` | `,` |
| 708 | 117 | `missing identifier` | `` |
| 753 | 115 | `missing identifier` | `` |
| 758 | 117 | `missing identifier` | `` |
| 826 | 144 | `ERROR` | `,` |
| … | … | … | *(29 more)* |

#### `plugins/eventlogger/event_logger_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 40 | `ERROR` | `,` |
| 75 | 177 | `ERROR` | `,` |
| 125 | 176 | `ERROR` | `,` |
| 129 | 145 | `ERROR` | `,` |

#### `plugins/eventlogger/log_catcher/event_log_task.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 101 | 188 | `ERROR` | `,` |
| 121 | 137 | `missing identifier` | `` |
| 139 | 187 | `ERROR` | `,` |
| 170 | 171 | `ERROR` | `,` |

#### `plugins/eventlogger/log_catcher/shell_catcher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 119 | `missing identifier` | `` |
| 37 | 152 | `ERROR` | `,` |
| 77 | 111 | `missing identifier` | `` |
| 83 | 170 | `ERROR` | `,` |
| 86 | 139 | `ERROR` | `,` |
| 95 | 118 | `missing identifier` | `` |

#### `plugins/eventlogger/log_catcher/summary_log_info_catcher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 127 | `missing identifier` | `` |
| 58 | 45 | `ERROR` | `, int32_t` |
| 60 | 161 | `ERROR` | `,` |
| 63 | 141 | `ERROR` | `,` |
| 66 | 128 | `ERROR` | `,` |

#### `plugins/eventlogger/test/unittest/common/event_field_validator_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 21 col 64 (missing )) ` |
| 21 | 20 | `ERROR` | `R "~({" domain_ ":")` |
| 21 | 57 | `missing ;` | `` |

#### `plugins/eventlogger/test/unittest/common/event_logger_config_validate_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 23 col 64 (missing )) ` |
| 23 | 20 | `ERROR` | `R "~({" domain_ ":")` |
| 23 | 57 | `missing ;` | `` |

#### `plugins/faultlogger/framework/native/extension/include/ets_faultlog_extension.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 64 | `ERROR` | `, . . .` |

#### `plugins/faultlogger/framework/native/extension/src/ets_faultlog_extension.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 112 | `missing identifier` | `` |
| 35 | 122 | `missing identifier` | `` |
| 40 | 110 | `missing identifier` | `` |
| 49 | 134 | `missing identifier` | `` |
| 55 | 114 | `missing identifier` | `` |
| 61 | 117 | `missing identifier` | `` |
| 68 | 162 | `ERROR` | `,` |
| 73 | 161 | `ERROR` | `,` |
| 78 | 158 | `ERROR` | `,` |
| 88 | 118 | `missing identifier` | `` |
| 93 | 120 | `missing identifier` | `` |
| 113 | 120 | `missing identifier` | `` |
| 133 | 127 | `missing identifier` | `` |
| 143 | 87 | `ERROR` | `, . . .` |
| 146 | 137 | `ERROR` | `,` |
| 148 | 117 | `missing identifier` | `` |
| 154 | 122 | `missing identifier` | `` |
| 161 | 173 | `ERROR` | `,` |
| 169 | 181 | `ERROR` | `,` |

#### `plugins/faultlogger/framework/native/extension/src/ets_faultlog_extension_context.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 107 | `missing identifier` | `` |
| 32 | 130 | `ERROR` | `,` |
| 37 | 130 | `ERROR` | `,` |
| 43 | 130 | `ERROR` | `,` |
| 47 | 130 | `ERROR` | `,` |

#### `plugins/faultlogger/framework/native/extension/src/js_faultlog_extension.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 135 | `missing identifier` | `` |
| 35 | 131 | `missing identifier` | `` |
| 37 | 120 | `missing identifier` | `` |
| 42 | 118 | `missing identifier` | `` |
| 59 | 139 | `missing identifier` | `` |
| 64 | 158 | `missing identifier` | `` |
| 68 | 146 | `missing identifier` | `` |
| 82 | 112 | `missing identifier` | `` |
| 87 | 143 | `missing identifier` | `` |
| 93 | 187 | `ERROR` | `,` |
| 100 | 140 | `missing identifier` | `` |
| 105 | 160 | `missing identifier` | `` |
| 110 | 111 | `missing identifier` | `` |
| 117 | 141 | `missing identifier` | `` |
| 124 | 150 | `missing identifier` | `` |
| 131 | 139 | `missing identifier` | `` |
| 136 | 148 | `missing identifier` | `` |
| 140 | 39 | `ERROR` | `,` |
| 145 | 158 | `missing identifier` | `` |
| 149 | 146 | `missing identifier` | `` |
| … | … | … | *(11 more)* |

#### `plugins/faultlogger/framework/native/extension/src/js_faultlog_extension_context.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 154 | `missing identifier` | `` |

#### `plugins/faultlogger/framework/native/extension/zidl/src/faultlog_ext_stub_imp.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 126 | `missing identifier` | `` |
| 14 | 137 | `missing identifier` | `` |

#### `plugins/faultlogger/interfaces/cj/faultlogger_ffi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 182 | `ERROR` | `,` |
| 123 | 129 | `missing identifier` | `` |

#### `plugins/faultlogger/interfaces/cj/faultlogger_ffi.h`

**Summary:** tree-sitter-cpp node `missing ::` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 28 | `missing ::` | `` |

#### `plugins/faultlogger/interfaces/cpp/innerkits/impl/faultlogger_client.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 129 | `missing identifier` | `` |
| 31 | 143 | `missing identifier` | `` |
| 41 | 129 | `missing identifier` | `` |
| 47 | 143 | `missing identifier` | `` |
| 59 | 121 | `missing identifier` | `` |
| 84 | 121 | `missing identifier` | `` |
| 103 | 121 | `missing identifier` | `` |
| 109 | 122 | `missing identifier` | `` |
| 122 | 154 | `missing identifier` | `` |
| 129 | 160 | `missing identifier` | `` |
| 138 | 155 | `missing identifier` | `` |
| 149 | 157 | `missing identifier` | `` |
| 175 | 156 | `missing identifier` | `` |
| 182 | 178 | `ERROR` | `,` |

#### `plugins/faultlogger/interfaces/js/napi/napi_faultlogger.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 182 | `ERROR` | `,` |
| 142 | 130 | `missing identifier` | `` |
| 157 | 131 | `missing identifier` | `` |
| 162 | 130 | `missing identifier` | `` |
| 183 | 156 | `ERROR` | `,` |
| 188 | 133 | `missing identifier` | `` |
| 198 | 135 | `missing identifier` | `` |
| 225 | 133 | `missing identifier` | `` |
| 229 | 133 | `missing identifier` | `` |
| 233 | 138 | `missing identifier` | `` |
| 237 | 138 | `missing identifier` | `` |
| 266 | 156 | `ERROR` | `,` |
| 272 | 133 | `missing identifier` | `` |
| 287 | 135 | `missing identifier` | `` |
| 394 | 1 | `ERROR` | `napi_value` |
| 408 | 19 | `missing ::` | `` |

#### `plugins/faultlogger/interfaces/js/napi/napi_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 125 | `missing identifier` | `` |
| 17 | 125 | `missing identifier` | `` |
| 21 | 124 | `missing identifier` | `` |
| 38 | 124 | `missing identifier` | `` |
| 48 | 124 | `missing identifier` | `` |
| 58 | 125 | `missing identifier` | `` |
| 78 | 125 | `missing identifier` | `` |
| 92 | 179 | `ERROR` | `,` |

#### `plugins/faultlogger/interfaces/js/test/unittest/cpp/faultlogger_test_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 8 | `ERROR` | `napi_value` |
| 31 | 20 | `ERROR` | `demoModule` |

#### `plugins/faultlogger/service/bdfr_base/base/faultlog_event_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 172 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/base/faultlog_event_interface.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 75 | `ERROR` | `,` |
| 38 | 123 | `missing identifier` | `` |
| 57 | 196 | `ERROR` | `,` |
| 76 | 147 | `ERROR` | `,` |
| 79 | 157 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/base/faultlog_event_pipeline.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 145 | `missing identifier` | `` |

#### `plugins/faultlogger/service/bdfr_base/event/cpp_crash/faultlog_cppcrash.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 166 | `missing identifier` | `` |
| 76 | 147 | `missing identifier` | `` |
| 84 | 133 | `missing identifier` | `` |
| 95 | 136 | `missing identifier` | `` |
| 119 | 176 | `ERROR` | `,` |
| 127 | 141 | `missing identifier` | `` |
| 145 | 140 | `missing identifier` | `` |
| 148 | 125 | `missing identifier` | `` |
| 152 | 155 | `ERROR` | `,` |
| 157 | 185 | `ERROR` | `,` |
| 168 | 183 | `ERROR` | `,` |
| 179 | 189 | `ERROR` | `,` |
| 183 | 154 | `ERROR` | `,` |
| 189 | 117 | `missing identifier` | `` |
| 216 | 153 | `ERROR` | `,` |
| 224 | 172 | `ERROR` | `,` |
| 235 | 160 | `ERROR` | `,` |
| 242 | 70 | `ERROR` | `,` |
| 284 | 117 | `missing identifier` | `` |
| 289 | 120 | `missing identifier` | `` |
| … | … | … | *(12 more)* |

#### `plugins/faultlogger/service/bdfr_base/event/freeze/faultlog_freeze.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 58 | 48 | `ERROR` | `,` |
| 131 | 160 | `ERROR` | `,` |
| 137 | 131 | `missing identifier` | `` |
| 141 | 132 | `missing identifier` | `` |
| 184 | 164 | `ERROR` | `,` |
| 188 | 121 | `missing identifier` | `` |
| 284 | 165 | `ERROR` | `,` |
| 289 | 157 | `ERROR` | `,` |
| 294 | 178 | `ERROR` | `,` |
| 299 | 170 | `ERROR` | `,` |
| 307 | 161 | `ERROR` | `,` |
| 310 | 165 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/event/js_cj_error/faultlog_error_reporter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 83 | 156 | `ERROR` | `,` |
| 111 | 55 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/event/sanitizer/faultlog_sanitizer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 58 | 180 | `ERROR` | `,` |
| 65 | 180 | `ERROR` | `,` |
| 89 | 115 | `missing identifier` | `` |
| 98 | 147 | `ERROR` | `,` |
| 103 | 152 | `ERROR` | `,` |
| 151 | 157 | `ERROR` | `,` |
| 155 | 167 | `ERROR` | `,` |
| 179 | 166 | `ERROR` | `,` |
| 197 | 144 | `ERROR` | `,` |
| 263 | 159 | `ERROR` | `,` |
| 268 | 195 | `ERROR` | `,` |
| 274 | 185 | `ERROR` | `,` |
| 308 | 194 | `ERROR` | `,` |
| 326 | 154 | `ERROR` | `,` |
| 351 | 160 | `ERROR` | `,` |
| 357 | 182 | `ERROR` | `,` |
| 377 | 160 | `ERROR` | `,` |
| 392 | 165 | `ERROR` | `,` |
| 400 | 170 | `ERROR` | `,` |
| 403 | 177 | `ERROR` | `,` |
| … | … | … | *(2 more)* |

#### `plugins/faultlogger/service/bdfr_base/export_faultlogger_interface.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 127 | `missing identifier` | `` |
| 19 | 156 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_bootscan.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 175 | `ERROR` | `,` |
| 27 | 180 | `ERROR` | `,` |
| 31 | 176 | `ERROR` | `,` |
| 54 | 166 | `ERROR` | `,` |
| 65 | 173 | `ERROR` | `,` |
| 79 | 166 | `ERROR` | `,` |
| 84 | 170 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_database.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 76 | 132 | `missing identifier` | `` |
| 175 | 142 | `missing identifier` | `` |
| 191 | 147 | `missing identifier` | `` |
| 213 | 142 | `missing identifier` | `` |
| 250 | 47 | `ERROR` | `,` |
| 271 | 175 | `ERROR` | `,` |
| 320 | 195 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_dump.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 11 | `ERROR` | `s` |
| 20 | 9 | `ERROR` | `: hidumper` |
| 21 | 11 | `ERROR` | `s` |
| 22 | 11 | `ERROR` | `s` |
| 22 | 81 | `ERROR` | `2025` |
| 22 | 88 | `ERROR` | `8` |
| 22 | 91 | `ERROR` | `21 10: 00: 00` |
| 23 | 11 | `ERROR` | `Options:` |
| 24 | 3 | `ERROR` | `h Display this help` |
| 25 | 3 | `ERROR` | `l List all fault file names in the faultlogger` |
| 26 | 3 | `ERROR` | `f fileName[-- ext] View the content of a specified fault` |
| 27 | 4 | `missing ;` | `` |
| 27 | 11 | `ERROR` | `parameter only can be used for appfreeze log file` |
| 28 | 5 | `missing ;` | `` |
| 28 | 12 | `ERROR` | `can be obtained using the- l parameter - t time Query fault file names generated after the specified time in the faultlo…` |
| 31 | 23 | `missing ;` | `` |
| 31 | 64 | `missing field_identifier` | `` |
| 32 | 3 | `ERROR` | `m moduleName Query fault file names related to the specified moduleName in the faultlogger` |
| 33 | 3 | `ERROR` | `d Display detailed content of the` |
| 34 | 3 | `ERROR` | `LogSuffixWithMs List all fault file names in the faultlogger directory with millisecond timestamps in their suffixes Add…` |
| … | … | … | *(19 more)* |

#### `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_formatter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 219 | 139 | `missing identifier` | `` |
| 224 | 155 | `missing identifier` | `` |
| 255 | 147 | `missing identifier` | `` |
| 334 | 164 | `ERROR` | `,` |
| 484 | 53 | `ERROR` | `,` |
| 556 | 157 | `ERROR` | `,` |
| 561 | 155 | `ERROR` | `,` |
| 566 | 149 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 93 | 155 | `ERROR` | `,` |
| 100 | 161 | `ERROR` | `,` |
| 103 | 162 | `ERROR` | `,` |
| 121 | 120 | `missing identifier` | `` |
| 127 | 143 | `ERROR` | `,` |
| 148 | 154 | `ERROR` | `,` |
| 150 | 147 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/faultlogger_base.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 129 | `missing identifier` | `` |
| 33 | 128 | `missing identifier` | `` |
| 95 | 161 | `missing identifier` | `` |
| 112 | 156 | `ERROR` | `,` |
| 116 | 165 | `missing identifier` | `` |
| 139 | 162 | `missing identifier` | `` |
| 153 | 164 | `missing identifier` | `` |
| 160 | 162 | `ERROR` | `,` |
| 191 | 163 | `missing identifier` | `` |
| 212 | 188 | `ERROR` | `,` |
| 218 | 188 | `ERROR` | `,` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cjerror_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 69 | 53 | `ERROR` | `: 20` |
| 70 | 54 | `ERROR` | `: 33` |
| 71 | 54 | `ERROR` | `: 77` |
| 72 | 4 | `ERROR` | `";"` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cppcrash_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 61 | `ERROR` | `R "~({" "JSON_VERSION":` |
| 10 | 1 | `ERROR` | `"` |
| 10 | 12 | `missing ;` | `` |
| 10 | 12 | `ERROR` | `": "OpenHarmony 7.0.0.22` |
| 11 | 11 | `missing ;` | `` |
| 11 | 15 | `missing ;` | `` |
| 11 | 23 | `ERROR` | `06 02:43:39.2286486502\n" , "PID": 1422 , "UID": 10007 , "PNAME": "com.ohos.systemui" , "PROCESS_LIFETIME": "28406s" , "…` |
| 17 | 46 | `missing ;` | `` |
| 17 | 52 | `ERROR` | `7212:0\n" , "SIGNAL":{ "signo": 11 , "code` |
| 21 | 9 | `missing ;` | `` |
| 21 | 13 | `ERROR` | `0x00001c2c" }` |
| 23 | 1 | `ERROR` | `"KEY_THREAD_REGISTERS":` |
| 24 | 1 | `ERROR` | `"MEMORY_NEAR_REGISTERS":` |
| 25 | 1 | `ERROR` | `"FAULT_STACK":` |
| 26 | 1 | `ERROR` | `"OPEN_FILES":` |
| 27 | 1 | `ERROR` | `"KEY_THREAD_INFO":{ "thread_name":` |
| 29 | 1 | `ERROR` | `"tid":` |
| 30 | 9 | `ERROR` | `:` |
| 32 | 1 | `ERROR` | `"pc":` |
| 33 | 1 | `ERROR` | `"symbol":` |
| … | … | … | *(105 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_database_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 1 | `ERROR` | `std:: string jsonStr= R "~({" domain_ ":" RELIABILITY ", " name_ ":" CPP_CRASH ", " type_ ":1, " time_ ":1501973701070, …` |
| 11 | 32 | `ERROR` | `: 1854` |
| 11 | 47 | `ERROR` | `: 0` |
| 11 | 73 | `ERROR` | `"PID":` |
| 11 | 92 | `ERROR` | `: 0` |
| 11 | 98 | `ERROR` | `"MODULE":` |
| 12 | 1 | `ERROR` | `"REASON":` |
| 12 | 45 | `ERROR` | `"SUMMARY":` |
| 12 | 106 | `ERROR` | `"VERSION":` |
| 13 | 1 | `ERROR` | `"HAPPEN_TIME":` |
| 13 | 31 | `ERROR` | `"PNAME":` |
| 13 | 46 | `ERROR` | `"FIRST_FRAME":` |
| 13 | 67 | `ERROR` | `"SECOND_FRAME":` |
| 13 | 109 | `ERROR` | `"FINGERPRINT":` |
| 14 | 70 | `ERROR` | `"level_":` |
| 14 | 93 | `ERROR` | `"tag_":` |
| 14 | 115 | `ERROR` | `"id_":` |
| 15 | 26 | `ERROR` | `"info_": "` |
| 15 | 44 | `ERROR` | `"` |
| 141 | 23 | `ERROR` | `R "~({" domain_ ":" RELIABILITY ", " name_ ":" CPP_CRASH ", " type_ ":1, " time_ ":1501973701070," "tz_":` |
| … | … | … | *(37 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_formatter_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 6 | `ERROR` | `: 1234` |
| 51 | 1 | `ERROR` | `"PNAME":` |
| 52 | 1 | `ERROR` | `"REASON":` |
| 55 | 1 | `ERROR` | `"tid":` |
| 56 | 9 | `ERROR` | `:` |
| 57 | 2 | `ERROR` | `"pc":` |
| 57 | 21 | `ERROR` | `"symbol":` |
| 57 | 53 | `ERROR` | `: 100` |
| 57 | 61 | `ERROR` | `"file":` |
| 57 | 86 | `ERROR` | `"buildId":` |
| 60 | 20 | `ERROR` | `:` |
| 61 | 2 | `ERROR` | `"thread_name":` |
| 61 | 34 | `ERROR` | `: 1235` |
| 61 | 51 | `ERROR` | `:[]` |
| 63 | 2 | `ERROR` | `)~ "` |
| 63 | 7 | `ERROR` | `"` |
| 82 | 23 | `ERROR` | `R "~({" "KEY_THREAD_INFO":` |
| 84 | 1 | `ERROR` | `"thread_name":` |
| 85 | 6 | `ERROR` | `: 1234` |
| 86 | 9 | `ERROR` | `:` |
| … | … | … | *(170 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_hilog_helper_test.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 119 | `missing identifier` | `` |
| 36 | 117 | `missing identifier` | `` |
| 48 | 119 | `missing identifier` | `` |
| 69 | 117 | `missing identifier` | `` |
| 81 | 119 | `missing identifier` | `` |
| 99 | 117 | `missing identifier` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_jserror_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 95 | 29 | `ERROR` | `R "~(Error name:summaryHasAll TypeError" Error message: Obj is` |
| 96 | 29 | `ERROR` | `Valid object Error code: get BLO SourceCode: CKSSvalue()` |
| 98 | 47 | `missing }` | `` |
| 100 | 54 | `ERROR` | `: 76: 10` |
| 100 | 63 | `missing ;` | `` |
| 101 | 55 | `ERROR` | `: 76: 10` |
| 101 | 64 | `missing ;` | `` |
| 102 | 55 | `ERROR` | `: 76: 10` |
| 103 | 1 | `ERROR` | `)~ "` |
| 103 | 6 | `ERROR` | `"` |
| 122 | 14 | `ERROR` | `: BussinessError 2501000: Operation failed` |
| 123 | 6 | `missing ;` | `` |
| 123 | 13 | `ERROR` | `2501000` |
| 125 | 8 | `ERROR` | `get SourceMap` |
| 125 | 29 | `ERROR` | `dump raw stack: at` |
| 126 | 54 | `ERROR` | `: 76: 10` |
| 126 | 63 | `missing ;` | `` |
| 127 | 55 | `ERROR` | `: 76: 10` |
| 127 | 64 | `missing ;` | `` |
| 128 | 55 | `ERROR` | `: 76: 10` |
| … | … | … | *(35 more)* |

#### `plugins/faultlogger/service/bdfr_base/utils/faultlog_hilog_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 123 | `missing identifier` | `` |
| 57 | 120 | `missing identifier` | `` |
| 67 | 113 | `missing identifier` | `` |
| 86 | 148 | `missing identifier` | `` |
| 91 | 136 | `missing identifier` | `` |
| 100 | 111 | `missing identifier` | `` |
| 109 | 117 | `missing identifier` | `` |
| 114 | 170 | `ERROR` | `,` |
| 117 | 149 | `ERROR` | `,` |
| 125 | 151 | `ERROR` | `,` |
| 128 | 119 | `missing identifier` | `` |
| 135 | 152 | `ERROR` | `,` |
| 145 | 120 | `missing identifier` | `` |

#### `plugins/faultlogger/service/bdfr_base/utils/faultlog_util.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 484 | 128 | `missing identifier` | `` |

#### `plugins/faultlogger/service/dynamic_library_management/dynamic_library_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 163 | `ERROR` | `,` |

#### `plugins/faultlogger/service/extension_manager/src/faultlog_ext_conn_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 147 | `missing identifier` | `` |
| 47 | 137 | `missing identifier` | `` |
| 55 | 143 | `ERROR` | `,` |
| 65 | 131 | `missing identifier` | `` |
| 122 | 138 | `missing identifier` | `` |
| 132 | 128 | `missing identifier` | `` |
| 142 | 167 | `ERROR` | `,` |
| 151 | 164 | `ERROR` | `,` |
| 157 | 163 | `ERROR` | `,` |
| 166 | 127 | `missing identifier` | `` |
| 173 | 132 | `missing identifier` | `` |
| 180 | 140 | `ERROR` | `,` |

#### `plugins/faultlogger/service/extension_manager/src/faultlog_ext_connection.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 115 | `missing identifier` | `` |
| 14 | 118 | `missing identifier` | `` |
| 23 | 120 | `missing identifier` | `` |
| 26 | 118 | `missing identifier` | `` |
| 35 | 123 | `missing identifier` | `` |

#### `plugins/faultlogger/service/faultlog_bootscan_listener.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 123 | `missing identifier` | `` |
| 48 | 186 | `ERROR` | `,` |

#### `plugins/faultlogger/service/faultlogger.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 118 | `missing identifier` | `` |
| 66 | 126 | `missing identifier` | `` |

#### `plugins/faultlogger/service/idl/faultlogger_service_ohos.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 185 | `ERROR` | `,` |
| 25 | 181 | `ERROR` | `,` |
| 72 | 129 | `missing identifier` | `` |
| 78 | 129 | `missing identifier` | `` |
| 85 | 208 | `ERROR` | `,` |
| 88 | 24 | `ERROR` | `,` |
| 135 | 147 | `ERROR` | `,` |
| 140 | 123 | `missing identifier` | `` |
| 152 | 142 | `ERROR` | `,` |
| 240 | 133 | `ERROR` | `,` |

#### `plugins/faultlogger/service/idl/include/ifaultlog_query_result.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 32 | `ERROR` | `"ohos.hiviewdfx.IFaultLogQueryResult"` |

#### `plugins/faultlogger/service/idl/include/ifaultlogger_service.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 32 | `ERROR` | `"ohos.hiviewdfx.IFaultLoggerService"` |

#### `plugins/faultlogger/service/idl/src/faultlog_info_ohos.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 136 | `missing identifier` | `` |
| 15 | 168 | `ERROR` | `,` |
| 19 | 163 | `ERROR` | `,` |
| 23 | 165 | `ERROR` | `,` |
| 27 | 166 | `ERROR` | `,` |
| 31 | 169 | `ERROR` | `,` |
| 40 | 172 | `ERROR` | `,` |
| 44 | 176 | `ERROR` | `,` |
| 54 | 187 | `ERROR` | `,` |
| 67 | 135 | `missing identifier` | `` |
| 87 | 171 | `ERROR` | `,` |
| 91 | 175 | `ERROR` | `,` |

#### `plugins/faultlogger/service/idl/src/faultlog_query_result_stub.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 125 | `missing identifier` | `` |
| 23 | 126 | `missing identifier` | `` |
| 31 | 120 | `missing identifier` | `` |
| 36 | 131 | `missing identifier` | `` |
| 44 | 134 | `missing identifier` | `` |

#### `plugins/faultlogger/service/idl/src/faultlogger_service_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 134 | `missing identifier` | `` |
| 40 | 53 | `ERROR` | `,` |
| 44 | 53 | `ERROR` | `,` |
| 80 | 128 | `missing identifier` | `` |
| 225 | 153 | `missing identifier` | `` |

#### `plugins/faultlogger/service/idl/src/faultlogger_service_stub.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 138 | `missing identifier` | `` |
| 64 | 141 | `missing identifier` | `` |
| 88 | 134 | `missing identifier` | `` |
| 98 | 125 | `missing identifier` | `` |
| 107 | 131 | `missing identifier` | `` |
| 123 | 127 | `missing identifier` | `` |
| 128 | 131 | `missing identifier` | `` |

#### `plugins/faultlogger/service/page_history/page_history_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 163 | `ERROR` | `,` |

#### `plugins/faultlogger/service/page_history/page_history_recorder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 154 | `ERROR` | `,` |

#### `plugins/faultlogger/service/page_history/pages_trace.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 140 | `ERROR` | `,` |

#### `plugins/freeze_detector/db_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 98 | 132 | `missing identifier` | `` |
| 103 | 172 | `ERROR` | `,` |
| 113 | 157 | `ERROR` | `,` |
| 137 | 155 | `ERROR` | `,` |
| 165 | 42 | `ERROR` | `,` |

#### `plugins/freeze_detector/event_field_validator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 201 | 150 | `ERROR` | `,` |
| 207 | 42 | `ERROR` | `,` |
| 215 | 187 | `ERROR` | `,` |
| 228 | 42 | `ERROR` | `,` |
| 262 | 175 | `ERROR` | `,` |
| 272 | 194 | `ERROR` | `,` |
| 278 | 194 | `ERROR` | `,` |
| 286 | 194 | `ERROR` | `,` |
| 292 | 194 | `ERROR` | `,` |
| 299 | 176 | `ERROR` | `,` |
| 315 | 59 | `ERROR` | `,` |
| 332 | 59 | `ERROR` | `,` |
| 350 | 59 | `ERROR` | `,` |
| 361 | 179 | `ERROR` | `,` |
| 368 | 59 | `ERROR` | `,` |
| 377 | 59 | `ERROR` | `,` |
| 385 | 42 | `ERROR` | `,` |
| 397 | 42 | `ERROR` | `,` |
| 419 | 88 | `ERROR` | `,` |

#### `plugins/freeze_detector/freeze_common.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 67 | 131 | `missing identifier` | `` |
| 100 | 131 | `missing identifier` | `` |

#### `plugins/freeze_detector/freeze_detector_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 108 | `missing identifier` | `` |
| 52 | 127 | `missing identifier` | `` |
| 59 | 129 | `missing identifier` | `` |
| 67 | 110 | `missing identifier` | `` |
| 225 | 128 | `missing identifier` | `` |
| 230 | 141 | `missing identifier` | `` |
| 234 | 122 | `ERROR` | `,` |
| 241 | 119 | `missing identifier` | `` |
| 251 | 67 | `ERROR` | `,` |
| 257 | 67 | `ERROR` | `,` |
| 263 | 119 | `missing identifier` | `` |
| 271 | 117 | `missing identifier` | `` |
| 293 | 123 | `missing identifier` | `` |
| 300 | 137 | `missing identifier` | `` |
| 316 | 84 | `ERROR` | `,` |
| 325 | 137 | `missing identifier` | `` |

#### `plugins/freeze_detector/freeze_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 167 | 131 | `missing identifier` | `` |
| 171 | 185 | `ERROR` | `,` |
| 177 | 139 | `missing identifier` | `` |
| 186 | 130 | `ERROR` | `,` |
| 192 | 145 | `ERROR` | `,` |
| 202 | 147 | `missing identifier` | `` |
| 209 | 157 | `ERROR` | `,` |
| 214 | 183 | `ERROR` | `,` |
| 221 | 162 | `ERROR` | `,` |
| 223 | 139 | `missing identifier` | `` |
| 226 | 171 | `ERROR` | `,` |
| 239 | 148 | `ERROR` | `,` |
| 256 | 146 | `ERROR` | `,` |
| 293 | 157 | `ERROR` | `,` |
| 295 | 171 | `ERROR` | `,` |
| 309 | 40 | `ERROR` | `,` |
| 312 | 146 | `ERROR` | `,` |
| 348 | 183 | `ERROR` | `,` |
| 354 | 171 | `ERROR` | `,` |
| 358 | 165 | `ERROR` | `,` |
| … | … | … | *(2 more)* |

#### `plugins/freeze_detector/resolver.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 164 | `ERROR` | `,` |
| 66 | 171 | `missing identifier` | `` |
| 133 | 107 | `ERROR` | `,` |
| 142 | 84 | `ERROR` | `,` |
| 149 | 84 | `ERROR` | `,` |
| 165 | 124 | `missing identifier` | `` |

#### `plugins/freeze_detector/rule_cluster.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 126 | `missing identifier` | `` |
| 46 | 121 | `missing identifier` | `` |
| 51 | 128 | `missing identifier` | `` |
| 56 | 123 | `missing identifier` | `` |
| 79 | 127 | `missing identifier` | `` |
| 85 | 142 | `missing identifier` | `` |
| 128 | 129 | `missing identifier` | `` |
| 133 | 131 | `missing identifier` | `` |
| 146 | 165 | `ERROR` | `,` |
| 159 | 130 | `missing identifier` | `` |
| 165 | 132 | `missing identifier` | `` |
| 373 | 170 | `ERROR` | `,` |
| 384 | 50 | `ERROR` | `,` |

#### `plugins/freeze_detector/test/unittest/common/freeze_detector_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 1390 | 1 | `ERROR` | `std:: string jsonStr= R "~({" domain_ ":" AAFWK ", " name_ ":" THREAD_BLOCK_3S ", " type_ ":1, " time_ ":1501973701070, …` |
| 1391 | 27 | `ERROR` | `"tid_":` |
| 1391 | 49 | `ERROR` | `: 0` |
| 1391 | 75 | `ERROR` | `"PID":` |
| 1391 | 95 | `ERROR` | `: 0` |
| 1392 | 1 | `ERROR` | `"MODULE":` |
| 1392 | 38 | `ERROR` | `"REASON":` |
| 1393 | 1 | `ERROR` | `"SUMMARY":` |
| 1393 | 74 | `ERROR` | `"VERSION":` |
| 1394 | 1 | `ERROR` | `"HAPPEN_TIME":` |
| 1394 | 31 | `ERROR` | `"PNAME":` |
| 1394 | 46 | `ERROR` | `"FIRST_FRAME":` |
| 1394 | 67 | `ERROR` | `"SECOND_FRAME":` |
| 1394 | 109 | `ERROR` | `"FINGERPRINT":` |
| 1395 | 70 | `ERROR` | `"level_":` |
| 1395 | 93 | `ERROR` | `"tag_":` |
| 1395 | 115 | `ERROR` | `"id_":` |
| 1396 | 26 | `ERROR` | `"info_": "` |
| 1396 | 44 | `ERROR` | `"` |

#### `plugins/freeze_detector/vendor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 244 | 153 | `ERROR` | `,` |
| 255 | 167 | `ERROR` | `,` |
| 259 | 187 | `ERROR` | `,` |
| 267 | 49 | `ERROR` | `,` |
| 304 | 159 | `ERROR` | `,` |
| 308 | 206 | `ERROR` | `,` |
| 318 | 167 | `ERROR` | `,` |
| 320 | 134 | `missing identifier` | `` |
| 342 | 119 | `missing identifier` | `` |
| 357 | 146 | `ERROR` | `,` |
| 360 | 156 | `ERROR` | `,` |
| 365 | 167 | `ERROR` | `,` |
| 394 | 185 | `ERROR` | `,` |
| 400 | 146 | `ERROR` | `,` |
| 409 | 195 | `ERROR` | `,` |
| 417 | 77 | `ERROR` | `,` |
| 460 | 155 | `missing identifier` | `` |
| 495 | 165 | `ERROR` | `,` |
| 507 | 98 | `ERROR` | `,` |
| 511 | 131 | `missing identifier` | `` |
| … | … | … | *(3 more)* |

#### `plugins/performance/XperfPlugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 80 | 120 | `missing identifier` | `` |
| 84 | 125 | `missing identifier` | `` |
| 89 | 123 | `missing identifier` | `` |
| 94 | 167 | `missing identifier` | `` |
| 96 | 128 | `missing identifier` | `` |
| 120 | 146 | `missing identifier` | `` |
| 132 | 160 | `ERROR` | `,` |
| 135 | 150 | `ERROR` | `,` |
| 145 | 177 | `ERROR` | `,` |
| 157 | 150 | `missing identifier` | `` |
| 159 | 148 | `missing identifier` | `` |
| 161 | 157 | `missing identifier` | `` |
| 169 | 152 | `missing identifier` | `` |
| 171 | 150 | `missing identifier` | `` |
| 173 | 159 | `missing identifier` | `` |

#### `plugins/performance/executor/ThrExecutor.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 68 | 173 | `missing identifier` | `` |
| 106 | 175 | `missing identifier` | `` |
| 145 | 173 | `missing identifier` | `` |

#### `plugins/performance/executor/ThrTaskContainer.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 132 | `missing identifier` | `` |
| 56 | 120 | `missing identifier` | `` |

#### `plugins/performance/monitor/AppLaunchMonitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 128 | `missing identifier` | `` |
| 27 | 144 | `missing identifier` | `` |
| 33 | 152 | `missing identifier` | `` |
| 39 | 169 | `missing identifier` | `` |
| 42 | 165 | `ERROR` | `,` |
| 48 | 132 | `missing identifier` | `` |
| 52 | 170 | `ERROR` | `,` |
| 58 | 131 | `missing identifier` | `` |
| 60 | 119 | `missing identifier` | `` |

#### `plugins/performance/monitor/JankAnimatorMonitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 134 | `missing identifier` | `` |
| 30 | 147 | `missing identifier` | `` |
| 36 | 141 | `missing identifier` | `` |
| 41 | 118 | `missing identifier` | `` |
| 47 | 137 | `missing identifier` | `` |
| 51 | 151 | `missing identifier` | `` |
| 57 | 138 | `missing identifier` | `` |
| 67 | 185 | `ERROR` | `,` |
| 78 | 152 | `missing identifier` | `` |
| 90 | 154 | `missing identifier` | `` |

#### `plugins/performance/perfmonitor/common/event_builder/xperf_event_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 166 | `ERROR` | `,` |
| 190 | 172 | `ERROR` | `,` |
| 224 | 145 | `missing identifier` | `` |
| 229 | 147 | `missing identifier` | `` |
| 237 | 130 | `missing identifier` | `` |
| 240 | 10 | `ERROR` | `*` |

#### `plugins/performance/perfmonitor/common/event_builder/xperf_event_reporter.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 171 | `missing identifier` | `` |

#### `plugins/performance/perfmonitor/common/perf_trace.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 38 | `ERROR` | `, . . .` |
| 19 | 51 | `ERROR` | `, . . .` |

#### `plugins/performance/perfmonitor/common/perf_trace.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 38 | `ERROR` | `, . . .` |
| 9 | 51 | `ERROR` | `, . . .` |

#### `plugins/performance/perfmonitor/interfaces/inner_api/include/perf_model.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 244 | 32 | `ERROR` | `"OHOS.HiviewDFX.IFrameCallback"` |
| 253 | 32 | `ERROR` | `"OHOS.HiviewDFX.IAnimatorCallback"` |
| 262 | 32 | `ERROR` | `"OHOS.HiviewDFX.ISceneCallback"` |

#### `plugins/performance/perfmonitor/load_complete/src/collect_states.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 50 | `ERROR` | `,` |
| 21 | 50 | `ERROR` | `,` |

#### `plugins/performance/perfmonitor/load_complete/src/collect_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 133 | `ERROR` | `,` |
| 34 | 136 | `ERROR` | `,` |
| 46 | 138 | `ERROR` | `,` |
| 52 | 150 | `ERROR` | `,` |
| 83 | 37 | `ERROR` | `,` |
| 131 | 136 | `ERROR` | `,` |
| 204 | 18 | `ERROR` | `,` |

#### `plugins/performance/perfmonitor/src/animator_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 63 | 148 | `ERROR` | `,` |
| 67 | 127 | `ERROR` | `,` |
| 68 | 169 | `ERROR` | `,` |
| 72 | 172 | `ERROR` | `,` |
| 85 | 125 | `ERROR` | `,` |
| 86 | 168 | `ERROR` | `,` |
| 95 | 168 | `ERROR` | `,` |

#### `plugins/performance/perfmonitor/src/input_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 47 | `ERROR` | `,` |
| 42 | 47 | `ERROR` | `,` |
| 51 | 47 | `ERROR` | `,` |
| 80 | 91 | `missing identifier` | `` |
| 104 | 91 | `missing identifier` | `` |

#### `plugins/performance/perfmonitor/src/jank_frame_monitor.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 66 | 149 | `missing identifier` | `` |

#### `plugins/performance/perfmonitor/src/perf_reporter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 116 | 72 | `ERROR` | `,` |
| 125 | 72 | `ERROR` | `,` |
| 134 | 72 | `ERROR` | `,` |
| 206 | 119 | `missing identifier` | `` |
| 298 | 55 | `ERROR` | `,` |
| 465 | 77 | `ERROR` | `,` |

#### `plugins/performance/perfmonitor/src/scene_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 200 | `ERROR` | `,` |
| 61 | 57 | `ERROR` | `,` |
| 191 | 142 | `missing identifier` | `` |
| 192 | 110 | `missing identifier` | `` |
| 224 | 92 | `missing identifier` | `` |
| 245 | 90 | `missing identifier` | `` |
| 307 | 138 | `missing identifier` | `` |
| 373 | 92 | `missing identifier` | `` |
| 390 | 90 | `missing identifier` | `` |
| 397 | 91 | `missing identifier` | `` |
| 405 | 89 | `missing identifier` | `` |
| 493 | 137 | `ERROR` | `,` |

#### `plugins/performance/perfmonitor/src/white_block_monitor.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 97 | `missing identifier` | `` |
| 54 | 115 | `missing identifier` | `` |
| 58 | 123 | `missing identifier` | `` |
| 63 | 128 | `missing identifier` | `` |
| 79 | 119 | `missing identifier` | `` |

#### `plugins/performance/reporter/adapter/AppStartReporterAdapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 165 | `missing identifier` | `` |
| 22 | 163 | `missing identifier` | `` |
| 24 | 176 | `ERROR` | `,` |

#### `plugins/performance/reporter/adapter/JankAnimatorReporterAdapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 145 | `missing identifier` | `` |
| 23 | 159 | `missing identifier` | `` |
| 28 | 171 | `missing identifier` | `` |
| 30 | 169 | `missing identifier` | `` |
| 34 | 180 | `ERROR` | `,` |
| 40 | 147 | `missing identifier` | `` |
| 46 | 161 | `missing identifier` | `` |
| 50 | 175 | `ERROR` | `,` |

#### `plugins/performance/reporter/adapter/SimpleAppStartReporterAdapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 182 | `ERROR` | `,` |
| 28 | 184 | `ERROR` | `,` |

#### `plugins/performance/reporter/event_poster/EventsPoster.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 132 | `missing identifier` | `` |

#### `plugins/performance/reporter/infrastructure/AppStartReporter.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 132 | `missing identifier` | `` |
| 41 | 131 | `missing identifier` | `` |

#### `plugins/performance/reporter/infrastructure/JankAnimatorReporter.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 141 | `missing identifier` | `` |
| 44 | 143 | `missing identifier` | `` |

#### `plugins/performance/scene_data_processor/AnimatorSceneDataProcessor.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 148 | `missing identifier` | `` |
| 47 | 163 | `missing identifier` | `` |
| 52 | 178 | `missing identifier` | `` |
| 56 | 185 | `missing identifier` | `` |
| 63 | 180 | `missing identifier` | `` |
| 70 | 166 | `missing identifier` | `` |
| 73 | 166 | `missing identifier` | `` |
| 85 | 144 | `missing identifier` | `` |
| 123 | 149 | `missing identifier` | `` |
| 164 | 138 | `missing identifier` | `` |
| 188 | 157 | `missing identifier` | `` |
| 191 | 160 | `missing identifier` | `` |
| 199 | 155 | `missing identifier` | `` |

#### `plugins/performance/timer/SceneTimerOhImpl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 134 | 143 | `missing identifier` | `` |
| 140 | 193 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/common/src/perf_trace.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 38 | `ERROR` | `, . . .` |
| 19 | 51 | `ERROR` | `, . . .` |

#### `plugins/performance/xperf_service/interfaces/inner_api/xperfservice_client/src/rs_frame_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 153 | `ERROR` | `,` |
| 36 | 100 | `missing identifier` | `` |
| 49 | 92 | `missing identifier` | `` |
| 63 | 99 | `missing identifier` | `` |

#### `plugins/performance/xperf_service/interfaces/inner_api/xperfservice_client/src/xperf_service_client.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 97 | `missing identifier` | `` |
| 35 | 95 | `missing identifier` | `` |
| 41 | 101 | `missing identifier` | `` |
| 47 | 93 | `missing identifier` | `` |
| 53 | 100 | `missing identifier` | `` |

#### `plugins/performance/xperf_service/services/common/src/xperf_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 141 | `ERROR` | `,` |
| 60 | 140 | `ERROR` | `,` |
| 72 | 142 | `ERROR` | `,` |
| 81 | 140 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/core/src/xperf_register_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 104 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/core/src/xperf_service.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 26 | `ERROR` | `,` |
| 39 | 94 | `missing identifier` | `` |
| 44 | 138 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_dispatcher/src/event_parser_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 113 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_dispatcher/src/xperf_dispatcher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 41 | `ERROR` | `,` |
| 37 | 100 | `missing identifier` | `` |
| 43 | 113 | `ERROR` | `,` |
| 57 | 72 | `missing identifier` | `` |
| 61 | 101 | `missing identifier` | `` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/avcodec_perf_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 166 | `ERROR` | `,` |
| 18 | 113 | `ERROR` | `,` |
| 19 | 115 | `ERROR` | `,` |
| 24 | 116 | `ERROR` | `,` |
| 25 | 118 | `ERROR` | `,` |
| 33 | 120 | `ERROR` | `,` |
| 34 | 122 | `ERROR` | `,` |
| 44 | 119 | `ERROR` | `,` |
| 45 | 121 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/passthrough_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 105 | `missing identifier` | `` |
| 26 | 120 | `ERROR` | `,` |
| 55 | 65 | `ERROR` | `,` |
| 92 | 142 | `ERROR` | `,` |
| 106 | 143 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/user_action_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 119 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_jank_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 106 | `ERROR` | `,` |
| 74 | 135 | `ERROR` | `,` |
| 110 | 109 | `missing identifier` | `` |
| 114 | 101 | `missing identifier` | `` |
| 125 | 112 | `missing identifier` | `` |
| 129 | 101 | `missing identifier` | `` |
| 133 | 108 | `missing identifier` | `` |
| 137 | 101 | `missing identifier` | `` |
| 145 | 89 | `missing identifier` | `` |
| 152 | 106 | `missing identifier` | `` |
| 153 | 86 | `missing identifier` | `` |
| 165 | 96 | `missing identifier` | `` |
| 172 | 88 | `missing identifier` | `` |
| 179 | 104 | `missing identifier` | `` |
| 180 | 85 | `missing identifier` | `` |
| 192 | 95 | `missing identifier` | `` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_play_latency_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 146 | `ERROR` | `,` |
| 70 | 143 | `ERROR` | `,` |
| 92 | 143 | `ERROR` | `,` |
| 110 | 138 | `ERROR` | `,` |
| 132 | 87 | `ERROR` | `,` |
| 176 | 143 | `ERROR` | `,` |
| 196 | 107 | `missing identifier` | `` |
| 330 | 156 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/framework/xperf_monitor/src/video_xperf_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 124 | `ERROR` | `,` |
| 55 | 140 | `ERROR` | `,` |
| 62 | 118 | `missing identifier` | `` |
| 67 | 103 | `ERROR` | `,` |
| 70 | 88 | `ERROR` | `,` |
| 73 | 101 | `missing identifier` | `` |
| 94 | 85 | `missing identifier` | `` |
| 107 | 85 | `missing identifier` | `` |
| 116 | 97 | `missing identifier` | `` |
| 129 | 92 | `missing identifier` | `` |
| 134 | 99 | `missing identifier` | `` |

#### `plugins/performance/xperf_service/services/framework/xperf_storage/src/user_action_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 127 | `ERROR` | `,` |
| 23 | 121 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/server/src/xperf_service_server.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 107 | `missing identifier` | `` |
| 27 | 31 | `ERROR` | `,` |
| 39 | 138 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/utils/time_util.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 1 | `ERROR` | `static int64_t GetCurrTimeMs()` |
| 11 | 20 | `ERROR` | `:: duration_cast< std:` |
| 11 | 51 | `ERROR` | `:` |
| 12 | 1 | `ERROR` | `std:: chrono:: system_clock::` |
| 16 | 1 | `ERROR` | `}` |

#### `plugins/performance/xperf_service/services/xperf_service_main.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 78 | `missing identifier` | `` |

#### `plugins/privacy_controller/privacy_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 125 | `missing identifier` | `` |
| 87 | 98 | `ERROR` | `,` |

#### `plugins/reliability/bbox_detectors/bbox_detector_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 126 | `missing identifier` | `` |
| 40 | 128 | `missing identifier` | `` |
| 105 | 139 | `missing identifier` | `` |
| 128 | 123 | `missing identifier` | `` |
| 151 | 186 | `ERROR` | `,` |

#### `plugins/reliability/bbox_detectors/bdfr_base/bbox_detectors_base.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 131 | `missing identifier` | `` |
| 34 | 130 | `missing identifier` | `` |
| 63 | 180 | `ERROR` | `,` |
| 131 | 188 | `ERROR` | `,` |
| 200 | 152 | `ERROR` | `,` |

#### `plugins/reliability/bbox_detectors/bdfr_base/export_bbox_detectors_interface.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 127 | `missing identifier` | `` |
| 19 | 156 | `ERROR` | `,` |

#### `plugins/reliability/bbox_detectors/bdfr_base/panic_error_info_handle.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 112 | 190 | `ERROR` | `,` |
| 132 | 190 | `ERROR` | `,` |
| 154 | 179 | `ERROR` | `,` |
| 167 | 75 | `ERROR` | `,` |
| 170 | 71 | `ERROR` | `,` |
| 173 | 75 | `ERROR` | `,` |
| 184 | 147 | `ERROR` | `,` |
| 187 | 147 | `ERROR` | `,` |
| 192 | 147 | `ERROR` | `,` |
| 195 | 150 | `ERROR` | `,` |
| 205 | 159 | `ERROR` | `,` |
| 212 | 181 | `ERROR` | `,` |
| 227 | 181 | `ERROR` | `,` |
| 233 | 124 | `missing identifier` | `` |

#### `plugins/reliability/bbox_detectors/bdfr_base/panic_report_recovery.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 48 | `missing field_identifier` | `` |
| 29 | 48 | `ERROR` | `? ")\|(\S*)))"` |
| 62 | 153 | `ERROR` | `,` |
| 83 | 163 | `ERROR` | `,` |
| 102 | 161 | `ERROR` | `,` |
| 110 | 58 | `ERROR` | `,` |
| 173 | 155 | `ERROR` | `,` |
| 182 | 155 | `ERROR` | `,` |
| 192 | 170 | `ERROR` | `,` |

#### `plugins/reliability/bbox_detectors/bdfr_base/test/unittest/bbox_detector_base_unit_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 47 | `missing ;` | `` |
| 46 | 1 | `ERROR` | `) "` |
| 46 | 5 | `ERROR` | `"` |

#### `plugins/reliability/leak_detectors/fault_detector_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 145 | `missing identifier` | `` |
| 24 | 114 | `missing identifier` | `` |
| 32 | 135 | `missing identifier` | `` |
| 43 | 165 | `ERROR` | `,` |
| 66 | 128 | `missing identifier` | `` |
| 72 | 133 | `missing identifier` | `` |
| 77 | 131 | `missing identifier` | `` |
| 90 | 130 | `missing identifier` | `` |
| 103 | 204 | `ERROR` | `,` |
| 112 | 141 | `missing identifier` | `` |

#### `plugins/reliability/leak_detectors/fault_detector_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 129 | `missing identifier` | `` |
| 59 | 124 | `missing identifier` | `` |
| 73 | 198 | `ERROR` | `,` |
| 114 | 177 | `ERROR` | `,` |
| 139 | 176 | `ERROR` | `,` |
| 194 | 177 | `ERROR` | `,` |
| 219 | 125 | `missing identifier` | `` |
| 232 | 125 | `missing identifier` | `` |
| 276 | 179 | `ERROR` | `,` |
| 293 | 172 | `ERROR` | `,` |
| 308 | 179 | `ERROR` | `,` |
| 327 | 130 | `missing identifier` | `` |
| 338 | 140 | `ERROR` | `,` |
| 350 | 161 | `ERROR` | `,` |
| 353 | 173 | `ERROR` | `,` |
| 370 | 161 | `ERROR` | `,` |
| 373 | 192 | `ERROR` | `,` |
| 387 | 144 | `ERROR` | `,` |
| 392 | 164 | `ERROR` | `,` |
| 406 | 128 | `missing identifier` | `` |

#### `plugins/reliability/leak_detectors/native_leak/native_leak_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 151 | `ERROR` | `,` |
| 36 | 145 | `ERROR` | `,` |
| 38 | 123 | `missing identifier` | `` |
| 44 | 158 | `ERROR` | `,` |
| 47 | 179 | `ERROR` | `,` |

#### `plugins/reliability/leak_detectors/native_leak/native_leak_detector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 142 | `missing identifier` | `` |
| 39 | 159 | `ERROR` | `,` |
| 119 | 192 | `ERROR` | `,` |
| 139 | 60 | `ERROR` | `,` |
| 182 | 204 | `ERROR` | `,` |
| 187 | 203 | `ERROR` | `,` |
| 199 | 136 | `missing identifier` | `` |
| 235 | 190 | `ERROR` | `,` |
| 240 | 160 | `missing identifier` | `` |
| 275 | 185 | `ERROR` | `,` |
| 288 | 100 | `ERROR` | `,` |
| 291 | 160 | `ERROR` | `,` |
| 306 | 138 | `missing identifier` | `` |
| 313 | 74 | `ERROR` | `,` |
| 325 | 61 | `ERROR` | `,` |
| 335 | 201 | `ERROR` | `,` |
| 339 | 101 | `ERROR` | `,` |
| 349 | 54 | `ERROR` | `,` |
| 354 | 144 | `ERROR` | `,` |
| 359 | 145 | `ERROR` | `,` |
| … | … | … | *(3 more)* |

#### `plugins/reliability/leak_detectors/native_leak/native_leak_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 66 | 182 | `ERROR` | `,` |
| 83 | 189 | `ERROR` | `,` |
| 91 | 156 | `ERROR` | `,` |
| 111 | 125 | `ERROR` | `,` |
| 125 | 180 | `ERROR` | `,` |
| 132 | 177 | `ERROR` | `,` |
| 135 | 164 | `ERROR` | `,` |
| 139 | 163 | `ERROR` | `,` |
| 152 | 159 | `ERROR` | `,` |
| 158 | 159 | `ERROR` | `,` |
| 162 | 183 | `ERROR` | `,` |
| 174 | 188 | `ERROR` | `,` |
| 226 | 180 | `ERROR` | `,` |
| 232 | 171 | `ERROR` | `,` |
| 235 | 171 | `ERROR` | `,` |
| 242 | 142 | `ERROR` | `,` |
| 250 | 153 | `ERROR` | `,` |
| 269 | 115 | `missing identifier` | `` |
| 283 | 157 | `ERROR` | `,` |
| 290 | 136 | `missing identifier` | `` |
| … | … | … | *(29 more)* |

#### `plugins/reliability/leak_detectors/native_leak/native_leak_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 152 | `ERROR` | `,` |

#### `plugins/reliability/leak_detectors/test/moduletest/leak_detector_module_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 145 | `ERROR` | `,` |
| 53 | 135 | `missing identifier` | `` |
| 62 | 151 | `ERROR` | `,` |
| 67 | 149 | `ERROR` | `,` |

#### `plugins/reliability/leak_detectors/test/test_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 119 | `missing identifier` | `` |
| 28 | 141 | `missing identifier` | `` |
| 38 | 120 | `missing identifier` | `` |
| 41 | 142 | `missing identifier` | `` |
| 46 | 156 | `ERROR` | `,` |
| 81 | 132 | `ERROR` | `,` |
| 87 | 137 | `ERROR` | `,` |
| 105 | 139 | `missing identifier` | `` |
| 123 | 153 | `ERROR` | `,` |
| 131 | 129 | `missing identifier` | `` |
| 137 | 137 | `missing identifier` | `` |
| 146 | 154 | `ERROR` | `,` |
| 159 | 120 | `missing identifier` | `` |
| 164 | 115 | `missing identifier` | `` |
| 167 | 119 | `missing identifier` | `` |
| 174 | 135 | `missing identifier` | `` |
| 179 | 135 | `missing identifier` | `` |
| 197 | 135 | `missing identifier` | `` |
| 251 | 119 | `missing identifier` | `` |
| 254 | 125 | `missing identifier` | `` |

#### `plugins/sys_dispatcher/sys_dispatcher.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 108 | `missing identifier` | `` |
| 17 | 110 | `missing identifier` | `` |
| 35 | 115 | `missing identifier` | `` |
| 39 | 131 | `missing identifier` | `` |
| 44 | 118 | `missing identifier` | `` |

#### `plugins/sysevent_source/event_server.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 179 | `ERROR` | `,` |
| 31 | 179 | `ERROR` | `,` |
| 37 | 183 | `ERROR` | `,` |
| 39 | 182 | `ERROR` | `,` |
| 50 | 117 | `missing identifier` | `` |
| 57 | 116 | `missing identifier` | `` |
| 62 | 114 | `missing identifier` | `` |
| 69 | 114 | `missing identifier` | `` |
| 143 | 146 | `ERROR` | `,` |
| 148 | 127 | `missing identifier` | `` |
| 152 | 192 | `ERROR` | `,` |
| 189 | 132 | `missing identifier` | `` |
| 193 | 145 | `missing identifier` | `` |
| 204 | 188 | `ERROR` | `,` |
| 231 | 179 | `ERROR` | `,` |
| 251 | 145 | `ERROR` | `,` |
| 257 | 149 | `ERROR` | `,` |
| 282 | 159 | `ERROR` | `,` |
| 295 | 125 | `missing identifier` | `` |
| 298 | 146 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `plugins/sysevent_source/monitor_config.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 134 | `missing identifier` | `` |
| 29 | 149 | `ERROR` | `,` |
| 36 | 173 | `ERROR` | `,` |

#### `plugins/sysevent_source/platform_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 130 | `missing identifier` | `` |
| 242 | 162 | `ERROR` | `,` |
| 371 | 155 | `ERROR` | `,` |
| 385 | 157 | `ERROR` | `,` |
| 430 | 126 | `missing identifier` | `` |

#### `plugins/sysevent_source/sysevent_source.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 130 | `missing identifier` | `` |
| 23 | 120 | `missing identifier` | `` |
| 34 | 122 | `missing identifier` | `` |
| 40 | 121 | `missing identifier` | `` |
| 56 | 181 | `ERROR` | `,` |
| 68 | 123 | `missing identifier` | `` |
| 84 | 153 | `ERROR` | `,` |

#### `plugins/unified_collector/observer/uc_app_state_observer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 175 | `ERROR` | `,` |
| 22 | 148 | `ERROR` | `,` |
| 28 | 145 | `ERROR` | `,` |

#### `plugins/unified_collector/observer/uc_observer_mgr.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 121 | `missing identifier` | `` |
| 28 | 122 | `missing identifier` | `` |
| 32 | 167 | `ERROR` | `,` |
| 35 | 141 | `missing identifier` | `` |
| 45 | 121 | `missing identifier` | `` |
| 49 | 169 | `ERROR` | `,` |
| 53 | 143 | `missing identifier` | `` |
| 60 | 118 | `missing identifier` | `` |
| 66 | 152 | `ERROR` | `,` |
| 69 | 126 | `missing identifier` | `` |
| 76 | 118 | `missing identifier` | `` |
| 86 | 177 | `ERROR` | `,` |
| 88 | 126 | `missing identifier` | `` |
| 104 | 154 | `ERROR` | `,` |
| 108 | 128 | `missing identifier` | `` |
| 119 | 154 | `ERROR` | `,` |
| 123 | 128 | `missing identifier` | `` |

#### `plugins/unified_collector/observer/uc_render_state_observer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 142 | `ERROR` | `,` |

#### `plugins/unified_collector/observer/uc_system_ability_listener.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 126 | `missing identifier` | `` |
| 24 | 128 | `missing identifier` | `` |

#### `plugins/unified_collector/observer/uc_telemetry_callback.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 114 | `missing identifier` | `` |
| 17 | 114 | `missing identifier` | `` |
| 20 | 130 | `ERROR` | `,` |
| 42 | 151 | `ERROR` | `,` |
| 51 | 151 | `ERROR` | `,` |
| 61 | 117 | `missing identifier` | `` |
| 85 | 162 | `ERROR` | `,` |
| 90 | 181 | `ERROR` | `,` |
| 102 | 142 | `ERROR` | `,` |
| 106 | 175 | `ERROR` | `,` |
| 115 | 124 | `missing identifier` | `` |
| 120 | 125 | `missing identifier` | `` |
| 138 | 126 | `missing identifier` | `` |
| 146 | 145 | `missing identifier` | `` |
| 155 | 109 | `missing identifier` | `` |
| 160 | 147 | `missing identifier` | `` |
| 172 | 110 | `missing identifier` | `` |
| 198 | 109 | `missing identifier` | `` |
| 201 | 130 | `missing identifier` | `` |
| 207 | 110 | `missing identifier` | `` |
| … | … | … | *(1 more)* |

#### `plugins/unified_collector/observer/uc_telemetry_listener.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 128 | `missing identifier` | `` |
| 67 | 138 | `ERROR` | `,` |
| 72 | 159 | `ERROR` | `,` |
| 79 | 157 | `ERROR` | `,` |
| 241 | 130 | `ERROR` | `,` |
| 267 | 139 | `missing identifier` | `` |
| 367 | 120 | `missing identifier` | `` |

#### `plugins/unified_collector/power/power_status_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 136 | `missing identifier` | `` |
| 42 | 150 | `ERROR` | `,` |

#### `plugins/unified_collector/storage/cpu_storage.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 63 | 93 | `ERROR` | `,` |
| 102 | 184 | `ERROR` | `,` |
| 112 | 166 | `ERROR` | `,` |
| 132 | 164 | `ERROR` | `,` |
| 150 | 171 | `ERROR` | `,` |
| 163 | 161 | `ERROR` | `,` |
| 171 | 116 | `missing identifier` | `` |
| 194 | 134 | `ERROR` | `,` |
| 205 | 145 | `ERROR` | `,` |
| 230 | 141 | `missing identifier` | `` |
| 241 | 161 | `ERROR` | `,` |
| 246 | 206 | `ERROR` | `,` |
| 254 | 173 | `ERROR` | `,` |
| 265 | 158 | `ERROR` | `,` |
| 289 | 154 | `ERROR` | `,` |
| 296 | 158 | `ERROR` | `,` |
| 313 | 153 | `ERROR` | `,` |
| 327 | 137 | `missing identifier` | `` |
| 337 | 117 | `missing identifier` | `` |
| 354 | 140 | `missing identifier` | `` |
| … | … | … | *(6 more)* |

#### `plugins/unified_collector/task/cpu_collection_task.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 65 | 176 | `ERROR` | `,` |

#### `plugins/unified_collector/task/dump_trace_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 165 | `ERROR` | `,` |

#### `plugins/unified_collector/task/trace_cache_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 142 | `missing identifier` | `` |
| 39 | 114 | `missing identifier` | `` |
| 43 | 111 | `missing identifier` | `` |
| 48 | 139 | `ERROR` | `,` |
| 53 | 149 | `ERROR` | `,` |
| 62 | 140 | `ERROR` | `,` |
| 90 | 131 | `missing identifier` | `` |
| 93 | 126 | `missing identifier` | `` |
| 103 | 136 | `missing identifier` | `` |
| 106 | 139 | `missing identifier` | `` |
| 119 | 125 | `missing identifier` | `` |
| 161 | 145 | `missing identifier` | `` |
| 166 | 141 | `missing identifier` | `` |
| 173 | 174 | `missing identifier` | `` |
| 195 | 30 | `ERROR` | `,` |
| 208 | 30 | `ERROR` | `,` |
| 221 | 30 | `ERROR` | `,` |

#### `plugins/unified_collector/unified_collector.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 138 | `missing identifier` | `` |
| 26 | 140 | `missing identifier` | `` |
| 35 | 124 | `missing identifier` | `` |
| 40 | 118 | `missing identifier` | `` |
| 53 | 154 | `ERROR` | `,` |
| 67 | 116 | `missing identifier` | `` |
| 71 | 117 | `missing identifier` | `` |
| 74 | 159 | `ERROR` | `,` |
| 77 | 129 | `missing identifier` | `` |
| 98 | 158 | `ERROR` | `,` |
| 107 | 118 | `missing identifier` | `` |
| 126 | 118 | `missing identifier` | `` |
| 145 | 118 | `missing identifier` | `` |

#### `plugins/usage_event_report/cache/event_db_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 153 | `ERROR` | `,` |
| 45 | 178 | `ERROR` | `,` |
| 50 | 165 | `ERROR` | `,` |
| 58 | 115 | `missing identifier` | `` |
| 67 | 172 | `ERROR` | `,` |
| 73 | 120 | `missing identifier` | `` |
| 101 | 114 | `missing identifier` | `` |
| 105 | 117 | `missing identifier` | `` |
| 118 | 114 | `missing identifier` | `` |
| 122 | 117 | `missing identifier` | `` |
| 135 | 117 | `missing identifier` | `` |
| 141 | 180 | `ERROR` | `,` |
| 148 | 170 | `ERROR` | `,` |
| 153 | 161 | `ERROR` | `,` |
| 160 | 117 | `missing identifier` | `` |
| 165 | 161 | `ERROR` | `,` |
| 170 | 169 | `ERROR` | `,` |
| 179 | 117 | `missing identifier` | `` |
| 188 | 117 | `missing identifier` | `` |
| 196 | 178 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `plugins/usage_event_report/cache/usage_event_cacher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 144 | `missing identifier` | `` |
| 25 | 169 | `ERROR` | `,` |
| 49 | 139 | `missing identifier` | `` |
| 52 | 128 | `missing identifier` | `` |
| 63 | 167 | `ERROR` | `,` |

#### `plugins/usage_event_report/event/app_usage_event.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 58 | 156 | `ERROR` | `,` |

#### `plugins/usage_event_report/event/sys_usage_event.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 156 | `ERROR` | `,` |

#### `plugins/usage_event_report/fold/cache/fold_app_usage_db_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 196 | 169 | `ERROR` | `,` |
| 200 | 169 | `ERROR` | `,` |
| 211 | 169 | `ERROR` | `,` |
| 215 | 166 | `ERROR` | `,` |
| 235 | 169 | `ERROR` | `,` |
| 239 | 167 | `ERROR` | `,` |
| 271 | 171 | `ERROR` | `,` |
| 277 | 158 | `ERROR` | `,` |
| 279 | 158 | `ERROR` | `,` |
| 291 | 184 | `ERROR` | `,` |
| 444 | 116 | `missing identifier` | `` |
| 450 | 176 | `ERROR` | `,` |
| 453 | 193 | `ERROR` | `,` |
| 459 | 201 | `ERROR` | `,` |
| 463 | 134 | `missing identifier` | `` |
| 476 | 123 | `missing identifier` | `` |
| 487 | 143 | `missing identifier` | `` |
| 496 | 190 | `ERROR` | `,` |
| 504 | 119 | `missing identifier` | `` |
| 517 | 120 | `missing identifier` | `` |
| … | … | … | *(18 more)* |

#### `plugins/usage_event_report/fold/cache/fold_app_usage_event_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 175 | `ERROR` | `,` |
| 146 | 192 | `ERROR` | `,` |

#### `plugins/usage_event_report/fold/cache/fold_event_cacher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 120 | `missing identifier` | `` |
| 162 | 115 | `missing identifier` | `` |
| 290 | 191 | `ERROR` | `,` |
| 302 | 144 | `ERROR` | `,` |

#### `plugins/usage_event_report/fold/cache/include/fold_app_usage_db_helper.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 92 | 101 | `missing type_identifier` | `` |

#### `plugins/usage_event_report/fold/usage_fold_event_report.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 116 | `missing identifier` | `` |
| 28 | 116 | `missing identifier` | `` |
| 39 | 120 | `missing identifier` | `` |
| 58 | 131 | `missing identifier` | `` |
| 63 | 175 | `ERROR` | `,` |

#### `plugins/usage_event_report/idl/hiview_shutdown_callback.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 131 | `missing identifier` | `` |
| 12 | 129 | `missing identifier` | `` |

#### `plugins/usage_event_report/service/factory/app_usage_event_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 133 | `missing identifier` | `` |
| 42 | 196 | `ERROR` | `,` |
| 61 | 136 | `ERROR` | `,` |
| 77 | 141 | `missing identifier` | `` |

#### `plugins/usage_event_report/service/main.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 132 | `missing identifier` | `` |

#### `plugins/usage_event_report/service/usage_event_report_service.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 131 | `missing identifier` | `` |
| 42 | 135 | `missing identifier` | `` |
| 46 | 148 | `missing identifier` | `` |
| 57 | 158 | `ERROR` | `,` |
| 65 | 132 | `missing identifier` | `` |
| 69 | 149 | `missing identifier` | `` |

#### `plugins/usage_event_report/usage_event_report.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 122 | `missing identifier` | `` |
| 33 | 117 | `missing identifier` | `` |
| 42 | 126 | `missing identifier` | `` |
| 66 | 163 | `ERROR` | `,` |
| 70 | 171 | `ERROR` | `,` |
| 78 | 172 | `ERROR` | `,` |
| 109 | 132 | `missing identifier` | `` |
| 136 | 110 | `missing identifier` | `` |
| 140 | 151 | `missing identifier` | `` |
| 164 | 127 | `missing identifier` | `` |
| 168 | 143 | `missing identifier` | `` |
| 179 | 128 | `missing identifier` | `` |
| 215 | 130 | `missing identifier` | `` |
| 245 | 130 | `missing identifier` | `` |
| 266 | 154 | `ERROR` | `,` |
| 271 | 166 | `ERROR` | `,` |
| 273 | 139 | `ERROR` | `,` |

#### `service/hiview_service.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 174 | `ERROR` | `,` |
| 69 | 112 | `missing identifier` | `` |
| 156 | 128 | `missing identifier` | `` |
| 162 | 122 | `missing identifier` | `` |
| 168 | 161 | `ERROR` | `,` |
| 180 | 102 | `ERROR` | `,` |
| 186 | 91 | `ERROR` | `,` |
| 207 | 151 | `ERROR` | `,` |
| 213 | 167 | `ERROR` | `,` |
| 222 | 122 | `missing identifier` | `` |
| 265 | 136 | `missing identifier` | `` |

#### `test/plugins/test_plugin/test_content.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 48 | `ERROR` | `,` |
| 29 | 98 | `ERROR` | `,` |
| 39 | 48 | `ERROR` | `,` |
| 44 | 98 | `ERROR` | `,` |
| 54 | 48 | `ERROR` | `,` |
| 59 | 98 | `ERROR` | `,` |
| 69 | 48 | `ERROR` | `,` |
| 74 | 98 | `ERROR` | `,` |
| 84 | 48 | `ERROR` | `,` |
| 89 | 98 | `ERROR` | `,` |
| 149 | 141 | `missing identifier` | `` |
| 163 | 141 | `missing identifier` | `` |
| 176 | 178 | `ERROR` | `,` |
| 183 | 179 | `ERROR` | `,` |
| 192 | 145 | `ERROR` | `,` |
| 204 | 179 | `ERROR` | `,` |
| 213 | 145 | `ERROR` | `,` |
| 218 | 116 | `missing identifier` | `` |
| 230 | 179 | `ERROR` | `,` |
| 237 | 115 | `missing identifier` | `` |
| … | … | … | *(2 more)* |

#### `test/plugins/test_plugin/test_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 124 | `missing identifier` | `` |
| 51 | 136 | `missing identifier` | `` |
| 57 | 148 | `ERROR` | `,` |
| 63 | 126 | `missing identifier` | `` |

#### `utility/analysis_faultlog/analysis_faultlog.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 173 | `ERROR` | `,` |
| 11 | 175 | `ERROR` | `,` |
| 17 | 158 | `ERROR` | `,` |

#### `utility/common_utils/calc_fingerprint.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 116 | `missing identifier` | `` |
| 52 | 129 | `missing identifier` | `` |
| 61 | 118 | `missing identifier` | `` |
| 66 | 121 | `missing identifier` | `` |
| 73 | 119 | `missing identifier` | `` |
| 85 | 118 | `missing identifier` | `` |

#### `utility/common_utils/tbox.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 168 | `ERROR` | `,` |
| 119 | 146 | `ERROR` | `,` |
| 138 | 148 | `ERROR` | `,` |

#### `utility/common_utils/test/unittest/common/utility_common_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 175 | 26 | `missing ;` | `` |

#### `utility/smart_parser/feature_analysis/feature_analysis.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 160 | `ERROR` | `,` |
| 38 | 67 | `ERROR` | `,` |
| 55 | 141 | `ERROR` | `,` |
| 69 | 205 | `ERROR` | `,` |

#### `utility/smart_parser/feature_analysis/log_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 165 | `ERROR` | `,` |
| 66 | 156 | `ERROR` | `,` |
| 78 | 159 | `ERROR` | `,` |
| 84 | 169 | `ERROR` | `,` |
| 103 | 116 | `missing identifier` | `` |

#### `utility/smart_parser/rule/compose_rule.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 158 | `ERROR` | `,` |
| 21 | 151 | `ERROR` | `,` |
| 26 | 127 | `missing identifier` | `` |
| 52 | 168 | `ERROR` | `,` |

#### `utility/smart_parser/rule/extract_rule.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 158 | `ERROR` | `,` |
| 20 | 151 | `ERROR` | `,` |
| 31 | 156 | `ERROR` | `,` |
| 39 | 148 | `missing identifier` | `` |
| 80 | 195 | `ERROR` | `,` |
| 83 | 116 | `missing identifier` | `` |
| 104 | 165 | `ERROR` | `,` |
| 111 | 148 | `missing identifier` | `` |
| 125 | 69 | `ERROR` | `,` |
| 130 | 165 | `ERROR` | `,` |
| 155 | 145 | `ERROR` | `,` |
| 165 | 156 | `ERROR` | `,` |
| 192 | 148 | `missing identifier` | `` |
| 203 | 132 | `missing identifier` | `` |

#### `utility/smart_parser/smart_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 163 | `ERROR` | `,` |

#### `utility/test/unittest/cpp_crash_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 1 | `ERROR` | `namespace OHOS{ namespace HiviewDFX{   static void SmartParserCppCrashTest_SmartParserCppCrashTest001() {  const std:: s…` |

#### `utility/test/unittest/syswarning_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 1 | `ERROR` | `namespace OHOS{ namespace HiviewDFX{   static void SmartParserSysWarningTest_SmartParserSysWarningTest001() {  const std…` |

---

## multimedia_camera_framework

Generated from `trace analyze /private/tmp/corpora/multimedia_camera_framework` (695 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 695

### Failure categories

| Category | Files |
|----------|------:|
| generic ERROR nodes (mixed C++ constructs) | 662 |
| missing type identifiers (often macro-expanded types) | 17 |
| gtest/HWTEST macros (`missing ;`) | 15 |
| other / mixed | 1 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `common/src/task_manager/task_group/base_task_group.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 2 | `common/src/task_manager/task_group/delayed_task_group.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 3 | `common/src/task_manager/task_group/task_group.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 4 | `common/src/task_manager/task_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 5 | `common/src/task_manager/task_registry.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 6 | `common/src/task_manager/thread_pool.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 7 | `common/src/task_manager/thread_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 8 | `common/src/timer/camera_deferred_timer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 9 | `common/src/timer/core/timer_core.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 10 | `common/src/timer/steady_clock.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 11 | `common/src/timer/time_broker.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 12 | `common/test/unittest/src/camera_common_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 40 |
| 13 | `common/utils/av_codec/src/av_codec_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 14 | `common/utils/buffer_manager/src/shared_buffer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 15 | `common/utils/camera_dynamic_loader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 16 | `common/utils/camera_extend/src/camera_extend_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 17 | `common/utils/camera_metadata.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 18 | `common/utils/camera_notification/src/camera_notification_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 19 | `common/utils/camera_server_photo_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 20 | `common/utils/camera_surface_buffer_util.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 61 |
| 21 | `common/utils/camera_timer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 22 | `common/utils/camera_xcollie.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 23 | `common/utils/camera_xml_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 24 | `common/utils/codec_info_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 25 | `common/utils/image_effect/src/image_effect_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 26 | `common/utils/media_capability_interface.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 27 | `common/utils/media_capability_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 28 | `common/utils/media_manager/src/media_manager_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 29 | `common/utils/media_stream/src/recorder_engine_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 74 |
| 30 | `common/utils/movie_file/src/movie_file_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 31 | `common/utils/photo_asset_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 32 | `common/utils/picture_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 33 | `common/utils/watch_dog.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 34 | `common/utils/watermark_exif_metadata/src/watermark_exif_metadata_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 35 | `common/utils/watermark_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 36 | `common/utils/xcomponent_controller/src/xcomponent_controller_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 37 | `dynamic_libs/av_codec/src/av_codec_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 38 | `dynamic_libs/camera_notification/src/camera_notification.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 39 | `dynamic_libs/image_effect/src/image_effect_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 40 | `dynamic_libs/image_framework/src/picture_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 110 |
| 41 | `dynamic_libs/media_library/src/photo_asset_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 42 | `dynamic_libs/media_manager/include/media_manager/mpeg_manager_factory.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 43 | `dynamic_libs/media_manager/include/media_manager/track_factory.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 44 | `dynamic_libs/media_manager/src/media_manager/demuxer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 45 | `dynamic_libs/media_manager/src/media_manager/media_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 67 |
| 46 | `dynamic_libs/media_manager/src/media_manager/mpeg_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 66 |
| 47 | `dynamic_libs/media_manager/src/media_manager/mpeg_manager_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 48 | `dynamic_libs/media_manager/src/media_manager/muxer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 49 | `dynamic_libs/media_manager/src/media_manager/reader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 34 |
| 50 | `dynamic_libs/media_manager/src/media_manager/track_factory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 51 | `dynamic_libs/media_manager/src/media_manager/writer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 22 |
| 52 | `dynamic_libs/media_manager/src/media_manager_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 53 | `dynamic_libs/moving_photo/include/common/audio_record.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 54 | `dynamic_libs/moving_photo/src/avcodec/audio_capturer_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 55 | `dynamic_libs/moving_photo/src/avcodec/audio_deferred_process.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 56 | `dynamic_libs/moving_photo/src/avcodec/audio_encoder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 57 | `dynamic_libs/moving_photo/src/avcodec/avcodec_task_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 101 |
| 58 | `dynamic_libs/moving_photo/src/avcodec/moving_photo_video_cache.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 59 | `dynamic_libs/moving_photo/src/common/moving_photo_listener.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 42 |
| 60 | `dynamic_libs/moving_photo/src/common/moving_photo_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 61 | `dynamic_libs/moving_photo/src/moving_photo_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 74 |
| 62 | `dynamic_libs/watermark_exif_metadata/src/watermark_exif_metadata_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 63 | `dynamic_libs/xcomponent_controller/src/xcomponent_controller_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 64 | `frameworks/cj/camera/include/camera_ffi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 164 |
| 65 | `frameworks/cj/camera/include/camera_input_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 66 | `frameworks/cj/camera/include/camera_manager_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 67 | `frameworks/cj/camera/include/camera_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 68 | `frameworks/cj/camera/include/camera_session_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 69 | `frameworks/cj/camera/include/listener_base.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 70 | `frameworks/cj/camera/include/metadata_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 71 | `frameworks/cj/camera/include/photo_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 72 | `frameworks/cj/camera/include/preview_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 73 | `frameworks/cj/camera/include/video_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 74 | `frameworks/cj/camera/src/camera_ffi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 45 |
| 75 | `frameworks/cj/camera/src/camera_session_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 76 | `frameworks/cj/camera/src/metadata_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 77 | `frameworks/cj/camera/src/photo_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 34 |
| 78 | `frameworks/cj/camera/src/preview_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 79 | `frameworks/cj/camera/src/video_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 80 | `frameworks/cj/camera_picker/include/camera_picker_ffi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 81 | `frameworks/cj/camera_picker/src/camera_picker_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 82 | `frameworks/js/camera_napi/src/camera_napi_security_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 83 | `frameworks/js/camera_napi/src/camera_napi_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 84 | `frameworks/js/camera_napi/src/camera_napi_worker_queue_keeper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 85 | `frameworks/js/camera_napi/src/dynamic_loader/camera_napi_ex_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 86 | `frameworks/js/camera_napi/src/dynamic_loader/camera_napi_ex_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 87 | `frameworks/js/camera_napi/src/input/camera_input_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 87 |
| 88 | `frameworks/js/camera_napi/src/input/camera_manager_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 251 |
| 89 | `frameworks/js/camera_napi/src/input/camera_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 90 | `frameworks/js/camera_napi/src/listener_base.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 91 | `frameworks/js/camera_napi/src/mode/photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 35 |
| 92 | `frameworks/js/camera_napi/src/mode/secure_camera_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 93 | `frameworks/js/camera_napi/src/mode/video_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 40 |
| 94 | `frameworks/js/camera_napi/src/napi_ref_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 95 | `frameworks/js/camera_napi/src/native_module_ohos_camera.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 96 | `frameworks/js/camera_napi/src/native_module_ohos_camerapicker.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 97 | `frameworks/js/camera_napi/src/native_module_ohos_resource_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 98 | `frameworks/js/camera_napi/src/output/capture_photo_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 99 | `frameworks/js/camera_napi/src/output/deferred_photo_proxy_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 100 | `frameworks/js/camera_napi/src/output/metadata_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 93 |
| 101 | `frameworks/js/camera_napi/src/output/photo_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 102 | `frameworks/js/camera_napi/src/output/photo_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 294 |
| 103 | `frameworks/js/camera_napi/src/output/preview_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 160 |
| 104 | `frameworks/js/camera_napi/src/output/quick_thumbnail_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 105 | `frameworks/js/camera_napi/src/output/unify_movie_file_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 167 |
| 106 | `frameworks/js/camera_napi/src/output/video_capability_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 107 | `frameworks/js/camera_napi/src/output/video_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 138 |
| 108 | `frameworks/js/camera_napi/src/picker/camera_picker_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 79 |
| 109 | `frameworks/js/camera_napi/src/resource_manager/resource_manager_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 110 | `frameworks/js/camera_napi/src/session/camera_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 620 |
| 111 | `frameworks/js/camera_napi/src/session/control_center_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 122 |
| 112 | `frameworks/js/camera_napi_for_sys/src/ability/camera_ability_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 47 |
| 113 | `frameworks/js/camera_napi_for_sys/src/mode/aperture_video_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 114 | `frameworks/js/camera_napi_for_sys/src/mode/cinematic_video_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 115 | `frameworks/js/camera_napi_for_sys/src/mode/fluorescence_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 116 | `frameworks/js/camera_napi_for_sys/src/mode/high_res_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 117 | `frameworks/js/camera_napi_for_sys/src/mode/light_painting_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 118 | `frameworks/js/camera_napi_for_sys/src/mode/macro_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 119 | `frameworks/js/camera_napi_for_sys/src/mode/macro_video_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 120 | `frameworks/js/camera_napi_for_sys/src/mode/mode_manager_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 34 |
| 121 | `frameworks/js/camera_napi_for_sys/src/mode/night_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 122 | `frameworks/js/camera_napi_for_sys/src/mode/panorama_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 123 | `frameworks/js/camera_napi_for_sys/src/mode/photo_session_for_sys_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 30 |
| 124 | `frameworks/js/camera_napi_for_sys/src/mode/portrait_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 125 | `frameworks/js/camera_napi_for_sys/src/mode/profession_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 83 |
| 126 | `frameworks/js/camera_napi_for_sys/src/mode/quick_shot_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 127 | `frameworks/js/camera_napi_for_sys/src/mode/secure_session_for_sys_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 128 | `frameworks/js/camera_napi_for_sys/src/mode/slow_motion_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 129 | `frameworks/js/camera_napi_for_sys/src/mode/stitching_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 130 | `frameworks/js/camera_napi_for_sys/src/mode/time_lapse_photo_session_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 153 |
| 131 | `frameworks/js/camera_napi_for_sys/src/mode/video_session_for_sys_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 60 |
| 132 | `frameworks/js/camera_napi_for_sys/src/output/depth_data_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 133 | `frameworks/js/camera_napi_for_sys/src/output/depth_data_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 134 | `frameworks/js/camera_napi_for_sys/src/output/movie_file_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 104 |
| 135 | `frameworks/js/camera_napi_for_sys/src/output/output_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 136 | `frameworks/js/camera_napi_for_sys/src/session/camera_session_for_sys_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 189 |
| 137 | `frameworks/native/camera/base/src/ability/camera_ability.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 138 | `frameworks/native/camera/base/src/ability/camera_ability_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 139 | `frameworks/native/camera/base/src/ability/camera_ability_parse_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 140 | `frameworks/native/camera/base/src/deferred_proc_session/deferred_photo_proc_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 70 |
| 141 | `frameworks/native/camera/base/src/deferred_proc_session/deferred_video_proc_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 71 |
| 142 | `frameworks/native/camera/base/src/input/camera_device.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 143 | `frameworks/native/camera/base/src/input/camera_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 144 | `frameworks/native/camera/base/src/input/camera_input.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 110 |
| 145 | `frameworks/native/camera/base/src/input/camera_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 317 |
| 146 | `frameworks/native/camera/base/src/input/camera_service_system_ability_listener.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 147 | `frameworks/native/camera/base/src/output/camera_output_capability.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 148 | `frameworks/native/camera/base/src/output/camera_photo_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 149 | `frameworks/native/camera/base/src/output/capture_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 150 | `frameworks/native/camera/base/src/output/deferred_photo_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 151 | `frameworks/native/camera/base/src/output/metadata_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 39 |
| 152 | `frameworks/native/camera/base/src/output/movie_file_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 82 |
| 153 | `frameworks/native/camera/base/src/output/movie_file_output_mock.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 154 | `frameworks/native/camera/base/src/output/photo_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 251 |
| 155 | `frameworks/native/camera/base/src/output/photo_output_callback.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 52 |
| 156 | `frameworks/native/camera/base/src/output/preview_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 126 |
| 157 | `frameworks/native/camera/base/src/output/sketch_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 158 | `frameworks/native/camera/base/src/output/unify_movie_file_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 66 |
| 159 | `frameworks/native/camera/base/src/output/unify_movie_file_output_mock.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 160 | `frameworks/native/camera/base/src/output/video_capability.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 161 | `frameworks/native/camera/base/src/output/video_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 90 |
| 162 | `frameworks/native/camera/base/src/session/cameraSwitch_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 163 | `frameworks/native/camera/base/src/session/capture_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 829 |
| 164 | `frameworks/native/camera/base/src/session/control_center_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 55 |
| 165 | `frameworks/native/camera/base/src/session/features/composition_feature.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 166 | `frameworks/native/camera/base/src/session/features/moon_capture_boost_feature.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 167 | `frameworks/native/camera/base/src/session/mech_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 168 | `frameworks/native/camera/base/src/session/photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 169 | `frameworks/native/camera/base/src/session/scan_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 170 | `frameworks/native/camera/base/src/session/secure_camera_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 171 | `frameworks/native/camera/base/src/session/video_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 172 | `frameworks/native/camera/base/src/utils/camera_buffer_handle_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 173 | `frameworks/native/camera/base/src/utils/camera_counting_timer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 174 | `frameworks/native/camera/base/src/utils/camera_device_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 175 | `frameworks/native/camera/base/src/utils/camera_rotation_api_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 176 | `frameworks/native/camera/base/src/utils/dps_metadata_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 177 | `frameworks/native/camera/base/src/utils/logic_camera_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 178 | `frameworks/native/camera/base/src/utils/metadata_common_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 179 | `frameworks/native/camera/extension/src/input/camera_manager_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 180 | `frameworks/native/camera/extension/src/output/depth_data_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 181 | `frameworks/native/camera/extension/src/session/aperture_video_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 182 | `frameworks/native/camera/extension/src/session/capture_session_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 100 |
| 183 | `frameworks/native/camera/extension/src/session/cinematic_video_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 184 | `frameworks/native/camera/extension/src/session/fluorescence_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 185 | `frameworks/native/camera/extension/src/session/high_res_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 186 | `frameworks/native/camera/extension/src/session/light_painting_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 187 | `frameworks/native/camera/extension/src/session/macro_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 188 | `frameworks/native/camera/extension/src/session/macro_video_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 189 | `frameworks/native/camera/extension/src/session/night_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 190 | `frameworks/native/camera/extension/src/session/panorama_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 191 | `frameworks/native/camera/extension/src/session/photo_session_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 192 | `frameworks/native/camera/extension/src/session/portrait_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 193 | `frameworks/native/camera/extension/src/session/profession_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 71 |
| 194 | `frameworks/native/camera/extension/src/session/quick_shot_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 195 | `frameworks/native/camera/extension/src/session/secure_camera_session_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 196 | `frameworks/native/camera/extension/src/session/slow_motion_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 34 |
| 197 | `frameworks/native/camera/extension/src/session/stitching_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 99 |
| 198 | `frameworks/native/camera/extension/src/session/time_lapse_photo_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 60 |
| 199 | `frameworks/native/camera/extension/src/session/video_session_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 46 |
| 200 | `frameworks/native/camera/test/moduletest/camera_base_function/src/camera_base_function_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 72 |
| 201 | `frameworks/native/camera/test/moduletest/camera_deferred_photo/src/camera_deferred_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 202 | `frameworks/native/camera/test/moduletest/camera_deferred_video/src/camera_deferred_video_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 203 | `frameworks/native/camera/test/moduletest/camera_format_YUV/include/camera_format_YUV_moduletest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 204 | `frameworks/native/camera/test/moduletest/camera_format_YUV/src/camera_format_YUV_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 205 | `frameworks/native/camera/test/moduletest/camera_moving_photo/src/camera_moving_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 206 | `frameworks/native/camera/test/moduletest/camera_photo/src/camera_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 207 | `frameworks/native/camera/test/moduletest/camera_preview/src/camera_preview_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 208 | `frameworks/native/camera/test/moduletest/camera_session/include/camera_session_moduletest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 209 | `frameworks/native/camera/test/moduletest/camera_session/src/camera_session_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 73 |
| 210 | `frameworks/native/camera/test/ndktest/camera_ndk_demo/entry/src/main/cpp/main.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 211 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_event_report_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 212 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_video_report_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 213 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_manager_test/src/camera_deferred_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 214 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_post_processor_test/src/camera_deferred_post_processor_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 35 |
| 215 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_proc_test/src/camera_deferred_proc_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 216 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/include/camera_deferred_video_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 217 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_job_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 218 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_stratety_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 219 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_unittest.cpp` | tree-sitter-cpp node `missing type_identifier` at 26 site(s) | 26 |
| 220 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_controller_unittest.cpp` | tree-sitter-cpp node `missing type_identifier` at 7 site(s) | 7 |
| 221 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_unittest.cpp` | tree-sitter-cpp node `missing type_identifier` at 7 site(s) | 7 |
| 222 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/camera_deferred_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 223 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 224 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 225 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_common/camera_common.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 226 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_manager_test/src/camera_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 227 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_capturer_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 228 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_deferred_process_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 229 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_encoder_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 230 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_video_muxer_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 231 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/avcodec_task_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 232 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/camera_server_photo_proxy_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 233 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/moving_photo_video_cache_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 234 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/video_encoder_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 235 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 236 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_fwk_metadata_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 237 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_info_dumper_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 238 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_privacy_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 239 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_util_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 240 | `frameworks/native/camera/test/unittest/camera_service/client/src/camera_service_client_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 241 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_device_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 242 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_service_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 243 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 244 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 245 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_host_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 246 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_preconfig_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 247 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_restore_param_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 248 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/include/hstream_operator_unittest.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 249 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_capture_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 250 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_operator_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 251 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 252 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_reader_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 253 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_sign_tools_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 254 | `frameworks/native/camera/test/unittest/camera_service/pipeline/src/camera_common_pipeline_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 255 | `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/cubic_bezier_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 256 | `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/smooth_zoom_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 257 | `frameworks/native/camera/test/unittest/framework_native/ability/src/camera_ability_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 258 | `frameworks/native/camera/test/unittest/framework_native/camera_utils/src/camera_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 259 | `frameworks/native/camera/test/unittest/framework_native/input/src/camera_framework_input_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 260 | `frameworks/native/camera/test/unittest/framework_native/manager/src/camera_framework_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 261 | `frameworks/native/camera/test/unittest/framework_native/output/src/metadata_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 262 | `frameworks/native/camera/test/unittest/framework_native/output/src/photo_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 263 | `frameworks/native/camera/test/unittest/framework_native/session/include/camera_switch_session_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 264 | `frameworks/native/camera/test/unittest/framework_native/session/include/capture_session_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 265 | `frameworks/native/camera/test/unittest/framework_native/session/include/composition_feature_unittest.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 266 | `frameworks/native/camera/test/unittest/framework_native/session/src/camera_switch_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 267 | `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 268 | `frameworks/native/camera/test/unittest/framework_native/session/src/cinematic_video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 269 | `frameworks/native/camera/test/unittest/framework_native/session/src/mech_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 270 | `frameworks/native/camera/test/unittest/framework_native/session/src/moon_capture_boost_feature_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 271 | `frameworks/native/camera/test/unittest/framework_native/session/src/night_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 272 | `frameworks/native/camera/test/unittest/framework_native/session/src/panorama_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 273 | `frameworks/native/camera/test/unittest/framework_native/session/src/photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 274 | `frameworks/native/camera/test/unittest/framework_native/session/src/portrait_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 275 | `frameworks/native/camera/test/unittest/framework_native/session/src/profession_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 276 | `frameworks/native/camera/test/unittest/framework_native/session/src/scan_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 277 | `frameworks/native/camera/test/unittest/framework_native/session/src/secure_camera_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 278 | `frameworks/native/camera/test/unittest/framework_native/session/src/slow_motion_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 279 | `frameworks/native/camera/test/unittest/framework_native/session/src/stitching_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 280 | `frameworks/native/camera/test/unittest/framework_native/session/src/time_lapse_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 281 | `frameworks/native/camera/test/unittest/framework_native/session/src/video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 282 | `frameworks/native/camera/test/unittest/movie_file/src/hcamera_movie_file_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 72 |
| 283 | `frameworks/native/ndk/camera_input.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 284 | `frameworks/native/ndk/camera_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 140 |
| 285 | `frameworks/native/ndk/capture_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 299 |
| 286 | `frameworks/native/ndk/impl/camera_input_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 287 | `frameworks/native/ndk/impl/camera_manager_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 115 |
| 288 | `frameworks/native/ndk/impl/camera_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 289 | `frameworks/native/ndk/impl/capture_session_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 158 |
| 290 | `frameworks/native/ndk/impl/metadata_object_ext_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 291 | `frameworks/native/ndk/impl/metadata_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 292 | `frameworks/native/ndk/impl/photo_listener_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 293 | `frameworks/native/ndk/impl/photo_native_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 294 | `frameworks/native/ndk/impl/photo_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 295 | `frameworks/native/ndk/impl/photo_output_impl.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 296 | `frameworks/native/ndk/impl/preview_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 297 | `frameworks/native/ndk/impl/video_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 298 | `frameworks/native/ndk/metadata_object_ext.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 299 | `frameworks/native/ndk/metadata_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 29 |
| 300 | `frameworks/native/ndk/photo_native.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 301 | `frameworks/native/ndk/photo_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 62 |
| 302 | `frameworks/native/ndk/preview_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 46 |
| 303 | `frameworks/native/ndk/video_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 35 |
| 304 | `frameworks/taihe/include/camera_auto_ref_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 305 | `frameworks/taihe/include/camera_event_emitter_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 306 | `frameworks/taihe/include/camera_template_utils_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 307 | `frameworks/taihe/include/listener_base_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 308 | `frameworks/taihe/include/transfer/camera_lib_manager_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 309 | `frameworks/taihe/src/camera_constructor_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 310 | `frameworks/taihe/src/camera_picker_constructor_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 311 | `frameworks/taihe/src/camera_utils_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 312 | `frameworks/taihe/src/camera_worker_queue_keeper_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 313 | `frameworks/taihe/src/capture_photo_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 314 | `frameworks/taihe/src/deferred_photo_proxy_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 315 | `frameworks/taihe/src/input/camera_input_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 316 | `frameworks/taihe/src/input/camera_manager_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 97 |
| 317 | `frameworks/taihe/src/listener_base_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 318 | `frameworks/taihe/src/mode/light_painting_photo_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 319 | `frameworks/taihe/src/mode/night_photo_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 320 | `frameworks/taihe/src/mode/photo_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 321 | `frameworks/taihe/src/mode/portrait_photo_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 322 | `frameworks/taihe/src/mode/professional_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 323 | `frameworks/taihe/src/mode/secure_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 324 | `frameworks/taihe/src/mode/secure_session_taihe_for_sys.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 325 | `frameworks/taihe/src/mode/slow_motion_video_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 326 | `frameworks/taihe/src/mode/time_lapse_photo_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 33 |
| 327 | `frameworks/taihe/src/mode/video_session_for_sys_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 328 | `frameworks/taihe/src/mode/video_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 78 |
| 329 | `frameworks/taihe/src/output/depth_data_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 29 |
| 330 | `frameworks/taihe/src/output/depth_data_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 331 | `frameworks/taihe/src/output/metadata_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 332 | `frameworks/taihe/src/output/movie_file_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 74 |
| 333 | `frameworks/taihe/src/output/photo_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 126 |
| 334 | `frameworks/taihe/src/output/preview_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 49 |
| 335 | `frameworks/taihe/src/output/unify_movie_file_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 70 |
| 336 | `frameworks/taihe/src/output/video_capability_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 337 | `frameworks/taihe/src/output/video_output_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 48 |
| 338 | `frameworks/taihe/src/photo_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 339 | `frameworks/taihe/src/picker/camera_picker_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 340 | `frameworks/taihe/src/query/camera_query_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 161 |
| 341 | `frameworks/taihe/src/session/camera_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 99 |
| 342 | `frameworks/taihe/src/session/control_center_session_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 53 |
| 343 | `frameworks/taihe/src/transfer/camera_transfer_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 196 |
| 344 | `interfaces/inner_api/native/camera/include/ability/camera_ability_builder.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 345 | `interfaces/inner_api/native/camera/include/input/camera_manager.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 346 | `interfaces/inner_api/native/camera/include/input/i_standard_camera_listener.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 347 | `interfaces/inner_api/native/camera/include/output/photo_output.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 348 | `interfaces/inner_api/native/test/camera_capture.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 349 | `interfaces/inner_api/native/test/camera_capture_mode.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 350 | `interfaces/inner_api/native/test/camera_capture_video.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 351 | `interfaces/inner_api/native/test/camera_video.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 73 |
| 352 | `interfaces/inner_api/native/test/test_common.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 353 | `interfaces/kits/js/camera_napi/include/camera_napi_event_emitter.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 354 | `interfaces/kits/js/camera_napi/include/camera_napi_object_types.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 355 | `interfaces/kits/js/camera_napi/include/camera_napi_param_parser.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 30 |
| 356 | `interfaces/kits/js/camera_napi/include/camera_napi_template_utils.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 357 | `interfaces/kits/js/camera_napi/include/napi_info_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 358 | `interfaces/kits/js/camera_napi/include/session/camera_napi_adaptor.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 359 | `mediastream/include/filter/video_encoder_filter.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 360 | `mediastream/src/buffer/audio_buffer_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 361 | `mediastream/src/buffer/meta_buffer_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 362 | `mediastream/src/buffer/video_buffer_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 363 | `mediastream/src/deferred_process.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 364 | `mediastream/src/filter/audio_cache_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 29 |
| 365 | `mediastream/src/filter/audio_capture_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 366 | `mediastream/src/filter/audio_capture_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 65 |
| 367 | `mediastream/src/filter/audio_capturer_session_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 368 | `mediastream/src/filter/audio_deferred_process_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 369 | `mediastream/src/filter/audio_encoder_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 370 | `mediastream/src/filter/audio_fork_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 51 |
| 371 | `mediastream/src/filter/audio_process_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 89 |
| 372 | `mediastream/src/filter/cfilter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 373 | `mediastream/src/filter/cinematic_video_cache_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 107 |
| 374 | `mediastream/src/filter/demuxer_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 102 |
| 375 | `mediastream/src/filter/image_effect_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 376 | `mediastream/src/filter/meta_cache_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 377 | `mediastream/src/filter/metadata_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 44 |
| 378 | `mediastream/src/filter/moving_photo_audio_encoder_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 59 |
| 379 | `mediastream/src/filter/moving_photo_muxer_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 44 |
| 380 | `mediastream/src/filter/moving_photo_video_encoder_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 56 |
| 381 | `mediastream/src/filter/muxer_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 45 |
| 382 | `mediastream/src/filter/sink_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 383 | `mediastream/src/filter/video_cache_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 384 | `mediastream/src/filter/video_encoder_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 78 |
| 385 | `mediastream/src/filter/video_encoder_filter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 386 | `mediastream/src/pipeline/pipeline.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 387 | `mediastream/src/recorder_engine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 245 |
| 388 | `mediastream/src/recorder_engine_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 69 |
| 389 | `mediastream/src/util/avbuffer_context.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 390 | `mediastream/src/util/moving_photo_avmuxer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 391 | `mediastream/src/util/moving_photo_engine_context.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 392 | `mediastream/src/util/moving_photo_recorder_task.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 393 | `mediastream/test/unittest/filter/include/audio_capture_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 8 site(s) | 8 |
| 394 | `mediastream/test/unittest/filter/include/audio_encoder_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 24 site(s) | 24 |
| 395 | `mediastream/test/unittest/filter/include/audio_fork_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 18 site(s) | 18 |
| 396 | `mediastream/test/unittest/filter/include/audio_process_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 18 site(s) | 18 |
| 397 | `mediastream/test/unittest/filter/include/cfilter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 398 | `mediastream/test/unittest/filter/include/cinematic_video_cache_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 14 site(s) | 14 |
| 399 | `mediastream/test/unittest/filter/include/metadata_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 14 site(s) | 14 |
| 400 | `mediastream/test/unittest/filter/include/muxer_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 22 site(s) | 22 |
| 401 | `mediastream/test/unittest/filter/include/video_encoder_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 402 | `mediastream/test/unittest/filter/src/audio_encoder_filter_unit_test.cpp` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 403 | `mediastream/test/unittest/pipeline/pipeline_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 404 | `moviefile/include/movie_file/plugin/movie_file_video_filter_plugin.h` | tree-sitter-cpp node `missing field_identifier` at 1 site(s) | 1 |
| 405 | `moviefile/include/pipeline/producer/unified_pipeline_audio_capture_wrap.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 406 | `moviefile/include/pipeline/thread/unified_pipeline_threadpool.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 407 | `moviefile/src/movie_file/movie_file_common_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 408 | `moviefile/src/movie_file/movie_file_consumer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 62 |
| 409 | `moviefile/src/movie_file/movie_file_controller_video.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 410 | `moviefile/src/movie_file/plugin/movie_file_audio_encoder_encode_node.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 411 | `moviefile/src/movie_file/plugin/movie_file_audio_offline_algo_node.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 412 | `moviefile/src/movie_file/plugin/movie_file_video_encoder_encode_node.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 48 |
| 413 | `moviefile/src/movie_file/producer/movie_file_video_encoded_buffer_producer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 414 | `moviefile/src/movie_file_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 415 | `moviefile/src/pipeline/producer/unified_pipeline_audio_capture_wrap.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 416 | `moviefile/src/pipeline/producer/unified_pipeline_audio_data_producer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 417 | `moviefile/src/pipeline/producer/unified_pipeline_data_producer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 418 | `moviefile/src/pipeline/producer/unified_pipeline_surface_data_producer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 419 | `moviefile/src/pipeline/unified_pipeline.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 420 | `services/camera_service/binder/base/include/icamera_broker.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 421 | `services/camera_service/binder/base/include/icamera_multi_stream_output.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 422 | `services/camera_service/binder/base/include/istream_capture_photo_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 423 | `services/camera_service/binder/base/include/istream_capture_thumbnail_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 424 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_mock_session_manager_interface.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 425 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_scene_session_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 426 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_manager_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 427 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_session_manager_service.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 428 | `services/camera_service/binder/base/src/icamera_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 429 | `services/camera_service/binder/client/src/hcamera_broker_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 430 | `services/camera_service/binder/client/src/hstream_capture_photo_callback_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 431 | `services/camera_service/binder/client/src/hstream_capture_thumbnail_callback_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 432 | `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_mock_session_manager_interface_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 433 | `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_scene_session_manager_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 434 | `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_window_session_manager_service_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 435 | `services/camera_service/binder/server/src/hcamera_broker_stub.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 436 | `services/camera_service/binder/server/src/hstream_capture_photo_callback_stub.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 437 | `services/camera_service/binder/server/src/hstream_capture_thumbnail_callback_stub.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 438 | `services/camera_service/binder/server/src/window_manager_service_callback_stub_impl/hcamera_window_manager_callback_stub.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 439 | `services/camera_service/include/camera_rotate_strategy_parser.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 440 | `services/camera_service/include/camera_util.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 441 | `services/camera_service/include/hcamera_device_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 442 | `services/camera_service/include/hcamera_host_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 443 | `services/camera_service/include/hstream_operator.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 444 | `services/camera_service/include/param_update/camera_rotate_param_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 445 | `services/camera_service/include/thread_priority_util.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 446 | `services/camera_service/src/adapter/bms_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 447 | `services/camera_service/src/app_manager_utils/camera_app_manager_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 448 | `services/camera_service/src/app_manager_utils/camera_app_manager_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 449 | `services/camera_service/src/applist_manager/camera_applist_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 450 | `services/camera_service/src/camera_beauty_notification.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 451 | `services/camera_service/src/camera_buffer_manager/photo_asset_auxiliary_consumer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 452 | `services/camera_service/src/camera_buffer_manager/photo_asset_buffer_consumer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 453 | `services/camera_service/src/camera_buffer_manager/photo_buffer_consumer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 454 | `services/camera_service/src/camera_buffer_manager/picture_assembler.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 455 | `services/camera_service/src/camera_buffer_manager/thumbnail_buffer_consumer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 456 | `services/camera_service/src/camera_common_event_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 457 | `services/camera_service/src/camera_datashare_helper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 458 | `services/camera_service/src/camera_dialog_connection.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 459 | `services/camera_service/src/camera_dialog_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 460 | `services/camera_service/src/camera_fwk_metadata_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 461 | `services/camera_service/src/camera_parameters_config_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 462 | `services/camera_service/src/camera_privacy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 463 | `services/camera_service/src/camera_rotate_strategy_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 464 | `services/camera_service/src/camera_sensor_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 465 | `services/camera_service/src/camera_server_photo_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 466 | `services/camera_service/src/camera_util.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 60 |
| 467 | `services/camera_service/src/camera_xml_parser.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 468 | `services/camera_service/src/device_protection_ability_connection.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 469 | `services/camera_service/src/dfx/camera_report_dfx_uitls.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 470 | `services/camera_service/src/dfx/camera_report_uitls.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 471 | `services/camera_service/src/display_plugin/camera_display_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 472 | `services/camera_service/src/hcamera_device.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 226 |
| 473 | `services/camera_service/src/hcamera_device_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 59 |
| 474 | `services/camera_service/src/hcamera_device_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 35 |
| 475 | `services/camera_service/src/hcamera_host_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 153 |
| 476 | `services/camera_service/src/hcamera_movie_file_output.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 24 |
| 477 | `services/camera_service/src/hcamera_restore_param.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 478 | `services/camera_service/src/hcamera_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 339 |
| 479 | `services/camera_service/src/hcamera_switch_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 42 |
| 480 | `services/camera_service/src/hcapture_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 239 |
| 481 | `services/camera_service/src/hcapture_session_wrapper.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 79 |
| 482 | `services/camera_service/src/hmech_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 483 | `services/camera_service/src/hshared_camera_device.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 56 |
| 484 | `services/camera_service/src/hshared_capture_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 71 |
| 485 | `services/camera_service/src/hstream_capture.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 151 |
| 486 | `services/camera_service/src/hstream_common.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 487 | `services/camera_service/src/hstream_depth_data.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 488 | `services/camera_service/src/hstream_metadata.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 489 | `services/camera_service/src/hstream_operator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 183 |
| 490 | `services/camera_service/src/hstream_operator_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 491 | `services/camera_service/src/hstream_repeat.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 126 |
| 492 | `services/camera_service/src/json_cache_converter/json_cache_converter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 493 | `services/camera_service/src/media_library/photo_asset_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 494 | `services/camera_service/src/media_library/photo_asset_proxy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 495 | `services/camera_service/src/param_update/camera_rotate_param_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 496 | `services/camera_service/src/param_update/camera_rotate_param_reader.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 497 | `services/camera_service/src/param_update/camera_rotate_param_sign_tools.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 498 | `services/camera_service/src/recorder/movie_file_recorder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 81 |
| 499 | `services/camera_service/src/rotate_plugin/camera_rotate_plugin.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 500 | `services/camera_service/src/rss/suspend_state_observer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 501 | `services/camera_service/src/smooth_zoom/cubic_bezier.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 502 | `services/camera_service/src/window_manager_utils/camera_window_manager_agent.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 503 | `services/camera_service/src/window_manager_utils/camera_window_manager_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 504 | `services/deferred_processing_service/include/base/blocking_queue.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 505 | `services/deferred_processing_service/include/base/dps.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 506 | `services/deferred_processing_service/include/base/enable_shared_create.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 507 | `services/deferred_processing_service/include/deferred_processing_service.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 508 | `services/deferred_processing_service/include/dfx/dps_video_report.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 509 | `services/deferred_processing_service/include/event_monitor/base/events_strategy.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 510 | `services/deferred_processing_service/include/event_monitor/events_monitor.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 511 | `services/deferred_processing_service/include/post_processor/command/video_process_command.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 512 | `services/deferred_processing_service/include/post_processor/photo_process_result.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 513 | `services/deferred_processing_service/include/post_processor/video_process_result.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 514 | `services/deferred_processing_service/include/schedule/base/istate.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 515 | `services/deferred_processing_service/include/schedule/state/state_factory.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 516 | `services/deferred_processing_service/include/schedule/video_processor/strategy/ivideo_strategy.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 517 | `services/deferred_processing_service/include/schedule/video_processor/video_job_repository/ivideo_job_repository_listener.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 518 | `services/deferred_processing_service/include/utils/dp_power_manager.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 519 | `services/deferred_processing_service/include/utils/dp_safe_map.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 520 | `services/deferred_processing_service/include/utils/dp_timer.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 521 | `services/deferred_processing_service/include/utils/dp_utils.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 522 | `services/deferred_processing_service/src/base/basic_definitions.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 523 | `services/deferred_processing_service/src/base/command_server/command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 524 | `services/deferred_processing_service/src/base/command_server/command_server.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 525 | `services/deferred_processing_service/src/base/command_server/command_server_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 526 | `services/deferred_processing_service/src/base/dps.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 527 | `services/deferred_processing_service/src/base/image_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 528 | `services/deferred_processing_service/src/base/media_progress_notifier.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 529 | `services/deferred_processing_service/src/deferred_processing_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 530 | `services/deferred_processing_service/src/dfx/dps_event_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 531 | `services/deferred_processing_service/src/dfx/dps_video_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 532 | `services/deferred_processing_service/src/event_monitor/command/event_status_change_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 533 | `services/deferred_processing_service/src/event_monitor/events_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 534 | `services/deferred_processing_service/src/event_monitor/events_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 27 |
| 535 | `services/deferred_processing_service/src/event_monitor/events_subscriber.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 536 | `services/deferred_processing_service/src/event_monitor/impl/battery_level_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 537 | `services/deferred_processing_service/src/event_monitor/impl/battery_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 538 | `services/deferred_processing_service/src/event_monitor/impl/camera_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 539 | `services/deferred_processing_service/src/event_monitor/impl/charging_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 540 | `services/deferred_processing_service/src/event_monitor/impl/screen_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 541 | `services/deferred_processing_service/src/event_monitor/impl/thermal_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 542 | `services/deferred_processing_service/src/event_monitor/impl/user_strategy.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 543 | `services/deferred_processing_service/src/post_processor/command/photo_process_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 544 | `services/deferred_processing_service/src/post_processor/command/service_died_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 545 | `services/deferred_processing_service/src/post_processor/command/video_process_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 546 | `services/deferred_processing_service/src/post_processor/photo_post_processor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 547 | `services/deferred_processing_service/src/post_processor/photo_process_result.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 77 |
| 548 | `services/deferred_processing_service/src/post_processor/video_post_processor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 55 |
| 549 | `services/deferred_processing_service/src/post_processor/video_process_result.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 550 | `services/deferred_processing_service/src/schedule/base/state_machine.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 551 | `services/deferred_processing_service/src/schedule/photo_processor/command/notify_job_changed_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 552 | `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 553 | `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_processor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 554 | `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_result.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 555 | `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/deferred_photo_job.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 556 | `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/photo_job_queue.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 557 | `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/photo_job_repository.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 48 |
| 558 | `services/deferred_processing_service/src/schedule/photo_processor/strategy/photo_strategy_center.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 559 | `services/deferred_processing_service/src/schedule/scheduler_coordinator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 560 | `services/deferred_processing_service/src/schedule/scheduler_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 561 | `services/deferred_processing_service/src/schedule/state/interrupt_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 562 | `services/deferred_processing_service/src/schedule/state/photo_cache_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 563 | `services/deferred_processing_service/src/schedule/state/photo_camera_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 564 | `services/deferred_processing_service/src/schedule/state/photo_hal_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 565 | `services/deferred_processing_service/src/schedule/state/photo_media_library_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 566 | `services/deferred_processing_service/src/schedule/state/photo_temperature_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 567 | `services/deferred_processing_service/src/schedule/state/photo_trailing_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 568 | `services/deferred_processing_service/src/schedule/state/video_battery_level_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 569 | `services/deferred_processing_service/src/schedule/state/video_battery_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 570 | `services/deferred_processing_service/src/schedule/state/video_camera_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 571 | `services/deferred_processing_service/src/schedule/state/video_charging_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 572 | `services/deferred_processing_service/src/schedule/state/video_hal_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 573 | `services/deferred_processing_service/src/schedule/state/video_media_library_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 574 | `services/deferred_processing_service/src/schedule/state/video_photo_process_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 575 | `services/deferred_processing_service/src/schedule/state/video_process_time_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 576 | `services/deferred_processing_service/src/schedule/state/video_screen_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 577 | `services/deferred_processing_service/src/schedule/state/video_temperature_state.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 578 | `services/deferred_processing_service/src/schedule/video_processor/command/notify_video_job_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 579 | `services/deferred_processing_service/src/schedule/video_processor/deferred_video_controller.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 580 | `services/deferred_processing_service/src/schedule/video_processor/deferred_video_processor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 65 |
| 581 | `services/deferred_processing_service/src/schedule/video_processor/deferred_video_result.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 582 | `services/deferred_processing_service/src/schedule/video_processor/strategy/video_strategy_center.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 29 |
| 583 | `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/deferred_video_job.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 584 | `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/video_job_queue.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 585 | `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/video_job_repository.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 53 |
| 586 | `services/deferred_processing_service/src/session/command/photo_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 587 | `services/deferred_processing_service/src/session/command/session_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 588 | `services/deferred_processing_service/src/session/command/sync_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 589 | `services/deferred_processing_service/src/session/command/video_command.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 590 | `services/deferred_processing_service/src/session/photo_session/deferred_photo_processing_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 591 | `services/deferred_processing_service/src/session/photo_session/photo_session_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 592 | `services/deferred_processing_service/src/session/session_coordinator.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 593 | `services/deferred_processing_service/src/session/session_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 594 | `services/deferred_processing_service/src/session/video_session/deferred_video_processing_session.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 595 | `services/deferred_processing_service/src/session/video_session/video_session_info.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 596 | `services/deferred_processing_service/src/utils/dp_power_manager.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 597 | `services/deferred_processing_service/src/utils/dp_timer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 598 | `services/deferred_processing_service/src/utils/dp_utils.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 599 | `test/fuzztest/audiocapturersession_fuzzer/audio_capturer_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 600 | `test/fuzztest/audiodeferredprocess_fuzzer/audio_deferred_process_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 601 | `test/fuzztest/audioencoder_fuzzer/audio_encoder_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 602 | `test/fuzztest/audiovideomuxer_fuzzer/audio_video_muxer_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 603 | `test/fuzztest/avcodecproxy_fuzzer/av_codec_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 604 | `test/fuzztest/avcodectaskmanager_fuzzer/avcodec_task_manager_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 605 | `test/fuzztest/cameraability_fuzzer/camera_ability_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 606 | `test/fuzztest/cameraabilitybuilder_fuzzer/camera_ability_builder_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 607 | `test/fuzztest/camerademuxer_fuzzer/camera_demuxer_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 608 | `test/fuzztest/cameradevice_fuzzer/camera_device_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 609 | `test/fuzztest/cameradeviceserviceproxy_fuzzer/camera_device_service_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 610 | `test/fuzztest/cameradeviceservicestub_fuzzer/camera_device_service_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 611 | `test/fuzztest/camerainput_fuzzer/camera_input_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 612 | `test/fuzztest/cameralistenerstub_fuzzer/camera_listener_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 613 | `test/fuzztest/cameramanager_fuzzer/camera_manager_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 614 | `test/fuzztest/cameraoutputcapability_fuzzer/camera_outputcapability_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 615 | `test/fuzztest/camerareportdfxutils_fuzzer/camera_report_dfx_utils_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 616 | `test/fuzztest/cameraserverphotoproxy_fuzzer/camera_server_photo_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 617 | `test/fuzztest/cameraservicejson_fuzzer/camera_service_json_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 618 | `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 87 |
| 619 | `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 620 | `test/fuzztest/cameratypes_fuzzer/camera_types_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 621 | `test/fuzztest/camerawindowmanagerclient_fuzzer/camera_window_manager_client_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 622 | `test/fuzztest/captureoutput_fuzzer/capture_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 623 | `test/fuzztest/capturesession_fuzzer/capture_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 114 |
| 624 | `test/fuzztest/capturesessionadd_fuzzer/capture_sessionadd_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 625 | `test/fuzztest/capturesessioncallback_fuzzer/capture_session_callback_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 626 | `test/fuzztest/capturesessionproxy_fuzzer/capture_session_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 627 | `test/fuzztest/cloudenhancesession_fuzzer/cloud_enhance_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 628 | `test/fuzztest/commandserver_fuzzer/command_server_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 629 | `test/fuzztest/commandserverimpl_fuzzer/command_server_impl_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 630 | `test/fuzztest/compositionfeature_fuzzer/compositionfeature_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 631 | `test/fuzztest/deferredprocessingserviceeventmonitor_fuzzer/deferred_processingservice_event_monitor_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 632 | `test/fuzztest/deferredprocessingstub_fuzzer/deferred_processing_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 633 | `test/fuzztest/deferredvideocontroller_fuzzer/deferred_video_controller_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 634 | `test/fuzztest/deferredvideoprocessor_fuzzer/deferred_video_processor_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 635 | `test/fuzztest/deferredvideoprocsession_fuzzer/deferredvideoprocsession_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 636 | `test/fuzztest/dpsvideoreport_fuzzer/dps_video_report_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 637 | `test/fuzztest/hcameradevice_fuzzer/hcamera_device_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 638 | `test/fuzztest/hcameradevicemanager_fuzzer/hcamera_device_manager_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 639 | `test/fuzztest/hcamerahostmanager_fuzzer/hcamera_host_manager_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 640 | `test/fuzztest/hcameramoviefileoutput_fuzzer/hcamera_movie_file_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 641 | `test/fuzztest/hcamerapreconfig_fuzzer/hcamera_preconfig_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 642 | `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 643 | `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 644 | `test/fuzztest/hcameraservicecallbackstub_fuzzer/hcamera_service_callback_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 645 | `test/fuzztest/hcameraswitchsession_fuzzer/hcamera_switch_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 646 | `test/fuzztest/hcapturesession_fuzzer/hcapture_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 647 | `test/fuzztest/hmechsession_fuzzer/hmech_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 648 | `test/fuzztest/hstreamcapture_fuzzer/hstream_capture_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 649 | `test/fuzztest/hstreamcapturestub_fuzzer/hstream_capture_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 650 | `test/fuzztest/hstreamdepthdatacallbackproxy_fuzzer/hstream_depth_data_callback_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 651 | `test/fuzztest/hstreamdepthdatacallbackstub_fuzzer/hstream_depth_data_callback_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 652 | `test/fuzztest/hstreamdepthdataproxy_fuzzer/hstream_depth_data_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 653 | `test/fuzztest/hstreamdepthdatastub_fuzzer/hstream_depth_data_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 654 | `test/fuzztest/hstreammetadata_fuzzer/hstream_metadata_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 655 | `test/fuzztest/hstreammetadatacallbackproxy_fuzzer/hstream_metadata_callback_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 656 | `test/fuzztest/hstreammetadatacallbackstub_fuzzer/hstream_metadata_callback_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 657 | `test/fuzztest/hstreammetadatastub_fuzzer/hstream_metadata_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 658 | `test/fuzztest/hstreamoperator_fuzzer/hstream_operator_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 659 | `test/fuzztest/hstreamrepeat_fuzzer/hstream_repeat_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 660 | `test/fuzztest/hstreamrepeatcallbackstub_fuzzer/hstream_repeat_callback_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 661 | `test/fuzztest/hstreamrepeatstub_fuzzer/hstream_repeat_stub_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 662 | `test/fuzztest/lightscansession_fuzzer/light_scan_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 663 | `test/fuzztest/mediamanager_fuzzer/media_manager_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 664 | `test/fuzztest/mediamanagerproxy_fuzzer/media_manager_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 665 | `test/fuzztest/metadataoutput_fuzzer/metadata_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 666 | `test/fuzztest/metadataoutput_fuzzer/metadata_output_fuzzer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 667 | `test/fuzztest/mooncaptureboostfeature_fuzzer/moon_capture_boost_feature_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 668 | `test/fuzztest/movingphotoproxy_fuzzer/moving_photo_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 669 | `test/fuzztest/movingphotosurfacewrapper_fuzzer/moving_photo_surface_wrapper_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 670 | `test/fuzztest/photojobrepository_fuzzer/photo_job_repository_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 671 | `test/fuzztest/photooutput_fuzzer/photo_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 672 | `test/fuzztest/photopostprocessor_fuzzer/photo_post_processor_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 673 | `test/fuzztest/photoprocessresult_fuzzer/photo_process_result_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 674 | `test/fuzztest/photosession_fuzzer/photo_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 675 | `test/fuzztest/photostrategycenter_fuzzer/photo_strategy_center_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 676 | `test/fuzztest/portraitsession_fuzzer/portrait_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 677 | `test/fuzztest/previewoutput_fuzzer/preview_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 23 |
| 678 | `test/fuzztest/professionsession_fuzzer/profession_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 679 | `test/fuzztest/sessioncoordinator_fuzzer/session_coordinator_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 680 | `test/fuzztest/sketchwrapper_fuzzer/sketch_wrapper_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 681 | `test/fuzztest/slowmotionsession_fuzzer/slow_motion_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 682 | `test/fuzztest/streamcapture_fuzzer/stream_capture_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 683 | `test/fuzztest/streamcaptureproxy_fuzzer/stream_capture_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 684 | `test/fuzztest/streamdepthdataproxy_fuzzer/stream_depth_data_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 685 | `test/fuzztest/streammetadatacallbackproxy_fuzzer/stream_metadata_callback_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 686 | `test/fuzztest/streamrepeatproxy_fuzzer/stream_repeat_proxy_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 687 | `test/fuzztest/timebroker_fuzzer/time_broker_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 688 | `test/fuzztest/timelapsephotosession_fuzzer/time_lapse_photo_session_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 689 | `test/fuzztest/timercore_fuzzer/timer_core_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 690 | `test/fuzztest/videoencoder_fuzzer/video_encoder_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 691 | `test/fuzztest/videojobqueue_fuzzer/video_job_queue_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 692 | `test/fuzztest/videojobrepository_fuzzer/video_job_repository_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 693 | `test/fuzztest/videooutput_fuzzer/video_output_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 694 | `test/fuzztest/videopostprocessor_fuzzer/video_post_processor_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 695 | `test/fuzztest/videostrategycenter_fuzzer/video_strategy_center_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |

### Per-file details

#### `common/src/task_manager/task_group/base_task_group.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 139 | `ERROR` | `,` |
| 22 | 191 | `ERROR` | `,` |
| 31 | 187 | `ERROR` | `,` |
| 49 | 30 | `ERROR` | `,` |
| 53 | 45 | `ERROR` | `,` |
| 65 | 30 | `ERROR` | `,` |
| 75 | 206 | `ERROR` | `,` |
| 82 | 147 | `ERROR` | `,` |
| 94 | 16 | `ERROR` | `,` |
| 102 | 136 | `ERROR` | `,` |
| 110 | 136 | `ERROR` | `,` |
| 112 | 184 | `ERROR` | `,` |
| 120 | 202 | `ERROR` | `,` |

#### `common/src/task_manager/task_group/delayed_task_group.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 140 | `ERROR` | `,` |
| 17 | 140 | `ERROR` | `,` |
| 27 | 151 | `ERROR` | `,` |
| 35 | 181 | `ERROR` | `,` |
| 40 | 177 | `ERROR` | `,` |
| 46 | 69 | `ERROR` | `,` |
| 55 | 174 | `ERROR` | `,` |
| 62 | 193 | `ERROR` | `,` |
| 65 | 193 | `ERROR` | `,` |

#### `common/src/task_manager/task_group/task_group.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 140 | `ERROR` | `,` |

#### `common/src/task_manager/task_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 167 | `ERROR` | `,` |
| 22 | 79 | `ERROR` | `,` |
| 33 | 127 | `ERROR` | `,` |
| 37 | 43 | `ERROR` | `,` |
| 46 | 151 | `ERROR` | `,` |
| 68 | 78 | `ERROR` | `,` |
| 77 | 131 | `ERROR` | `,` |
| 82 | 131 | `ERROR` | `,` |
| 96 | 169 | `ERROR` | `,` |
| 98 | 82 | `ERROR` | `,` |
| 116 | 167 | `ERROR` | `,` |
| 118 | 83 | `ERROR` | `,` |
| 126 | 185 | `ERROR` | `,` |
| 128 | 83 | `ERROR` | `,` |
| 136 | 174 | `ERROR` | `,` |

#### `common/src/task_manager/task_registry.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 131 | `ERROR` | `,` |
| 17 | 132 | `ERROR` | `,` |
| 25 | 195 | `ERROR` | `,` |
| 27 | 162 | `ERROR` | `,` |
| 48 | 131 | `ERROR` | `,` |
| 50 | 220 | `ERROR` | `,` |
| 57 | 217 | `ERROR` | `,` |
| 67 | 163 | `ERROR` | `,` |
| 71 | 180 | `ERROR` | `,` |
| 79 | 164 | `ERROR` | `,` |
| 83 | 180 | `ERROR` | `,` |
| 91 | 159 | `ERROR` | `,` |
| 95 | 180 | `ERROR` | `,` |
| 106 | 153 | `ERROR` | `,` |
| 110 | 161 | `ERROR` | `,` |

#### `common/src/task_manager/thread_pool.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 189 | `ERROR` | `,` |
| 31 | 132 | `ERROR` | `,` |
| 36 | 154 | `ERROR` | `,` |
| 44 | 80 | `ERROR` | `,` |
| 57 | 141 | `ERROR` | `,` |
| 59 | 159 | `ERROR` | `,` |
| 62 | 155 | `ERROR` | `,` |
| 64 | 156 | `ERROR` | `,` |
| 66 | 83 | `ERROR` | `,` |
| 69 | 140 | `ERROR` | `,` |
| 74 | 80 | `ERROR` | `,` |
| 82 | 80 | `ERROR` | `,` |
| 90 | 80 | `ERROR` | `,` |
| 108 | 242 | `ERROR` | `,` |
| 111 | 200 | `ERROR` | `,` |
| 137 | 80 | `ERROR` | `,` |
| 149 | 115 | `ERROR` | `,` |

#### `common/src/task_manager/thread_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 182 | `ERROR` | `,` |
| 19 | 255 | `ERROR` | `,` |
| 32 | 192 | `ERROR` | `,` |
| 34 | 216 | `ERROR` | `,` |

#### `common/src/timer/camera_deferred_timer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 218 | `ERROR` | `,` |
| 29 | 219 | `ERROR` | `,` |
| 35 | 221 | `ERROR` | `,` |
| 66 | 228 | `ERROR` | `,` |
| 79 | 297 | `ERROR` | `,` |
| 105 | 234 | `ERROR` | `,` |
| 115 | 260 | `ERROR` | `,` |

#### `common/src/timer/core/timer_core.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 80 | `ERROR` | `,` |
| 25 | 80 | `ERROR` | `,` |
| 35 | 79 | `ERROR` | `,` |
| 40 | 80 | `ERROR` | `,` |
| 58 | 94 | `ERROR` | `,` |
| 71 | 72 | `ERROR` | `,` |
| 82 | 94 | `ERROR` | `,` |
| 85 | 193 | `ERROR` | `,` |
| 100 | 141 | `ERROR` | `,` |
| 115 | 140 | `ERROR` | `,` |
| 124 | 154 | `ERROR` | `,` |
| 134 | 90 | `ERROR` | `,` |
| 140 | 191 | `ERROR` | `,` |
| 143 | 166 | `ERROR` | `,` |

#### `common/src/timer/steady_clock.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 31 | `ERROR` | `,` |
| 39 | 289 | `ERROR` | `,` |
| 46 | 80 | `ERROR` | `,` |
| 51 | 80 | `ERROR` | `,` |
| 58 | 80 | `ERROR` | `,` |

#### `common/src/timer/time_broker.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 135 | `ERROR` | `,` |
| 19 | 136 | `ERROR` | `,` |
| 25 | 136 | `ERROR` | `,` |
| 48 | 210 | `ERROR` | `,` |
| 67 | 184 | `ERROR` | `,` |
| 80 | 182 | `ERROR` | `,` |
| 110 | 141 | `ERROR` | `,` |
| 112 | 151 | `ERROR` | `,` |
| 127 | 166 | `ERROR` | `,` |
| 138 | 153 | `ERROR` | `,` |
| 142 | 204 | `ERROR` | `,` |

#### `common/test/unittest/src/camera_common_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 278 | 108 | `ERROR` | `,` |
| 293 | 106 | `ERROR` | `,` |
| 299 | 108 | `ERROR` | `,` |
| 314 | 106 | `ERROR` | `,` |
| 320 | 108 | `ERROR` | `,` |
| 334 | 106 | `ERROR` | `,` |
| 340 | 108 | `ERROR` | `,` |
| 367 | 106 | `ERROR` | `,` |
| 373 | 108 | `ERROR` | `,` |
| 389 | 106 | `ERROR` | `,` |
| 395 | 108 | `ERROR` | `,` |
| 423 | 106 | `ERROR` | `,` |
| 429 | 108 | `ERROR` | `,` |
| 446 | 106 | `ERROR` | `,` |
| 452 | 108 | `ERROR` | `,` |
| 468 | 106 | `ERROR` | `,` |
| 474 | 108 | `ERROR` | `,` |
| 492 | 106 | `ERROR` | `,` |
| 498 | 108 | `ERROR` | `,` |
| 514 | 106 | `ERROR` | `,` |
| … | … | … | *(20 more)* |

#### `common/utils/av_codec/src/av_codec_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 96 | `ERROR` | `,` |
| 11 | 146 | `ERROR` | `, , ,` |
| 12 | 148 | `ERROR` | `, , ,` |
| 17 | 95 | `ERROR` | `,` |
| 23 | 152 | `ERROR` | `, , ,` |
| 25 | 170 | `ERROR` | `, , ,` |
| 27 | 162 | `ERROR` | `, , ,` |
| 34 | 99 | `ERROR` | `,` |
| 35 | 149 | `ERROR` | `, , ,` |
| 42 | 109 | `ERROR` | `,` |
| 43 | 149 | `ERROR` | `, , ,` |
| 49 | 146 | `ERROR` | `,` |
| 56 | 104 | `ERROR` | `,` |
| 57 | 149 | `ERROR` | `, , ,` |

#### `common/utils/buffer_manager/src/shared_buffer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 80 | `ERROR` | `,` |
| 41 | 139 | `ERROR` | `, , ,` |
| 44 | 121 | `ERROR` | `, , ,` |
| 51 | 210 | `ERROR` | `,` |
| 51 | 236 | `ERROR` | `,` |
| 51 | 262 | `ERROR` | `,` |
| 52 | 85 | `ERROR` | `,` |
| 63 | 121 | `ERROR` | `, , ,` |

#### `common/utils/camera_dynamic_loader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 81 | 91 | `ERROR` | `,` |
| 83 | 165 | `ERROR` | `,` |
| 86 | 211 | `ERROR` | `,` |
| 86 | 233 | `ERROR` | `,` |
| 86 | 255 | `ERROR` | `,` |
| 88 | 81 | `ERROR` | `,` |
| 98 | 295 | `ERROR` | `,` |
| 111 | 96 | `ERROR` | `,` |
| 113 | 204 | `ERROR` | `,` |
| 113 | 226 | `ERROR` | `,` |
| 113 | 248 | `ERROR` | `,` |
| 117 | 211 | `ERROR` | `,` |
| 117 | 237 | `ERROR` | `,` |
| 117 | 263 | `ERROR` | `,` |
| 118 | 171 | `ERROR` | `,` |
| 133 | 174 | `ERROR` | `,` |
| 138 | 213 | `ERROR` | `,` |
| 138 | 234 | `ERROR` | `,` |
| 138 | 255 | `ERROR` | `,` |
| 140 | 171 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `common/utils/camera_extend/src/camera_extend_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 100 | `ERROR` | `,` |
| 16 | 102 | `ERROR` | `,` |
| 23 | 100 | `ERROR` | `,` |
| 25 | 161 | `ERROR` | `, , ,` |
| 28 | 163 | `ERROR` | `, , ,` |
| 30 | 155 | `ERROR` | `, , ,` |
| 38 | 131 | `ERROR` | `,` |
| 45 | 116 | `ERROR` | `,` |

#### `common/utils/camera_metadata.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 154 | `ERROR` | `, , ,` |
| 18 | 213 | `ERROR` | `,` |
| 18 | 221 | `ERROR` | `,` |
| 18 | 229 | `ERROR` | `,` |
| 27 | 206 | `ERROR` | `,` |
| 27 | 214 | `ERROR` | `,` |
| 27 | 222 | `ERROR` | `,` |

#### `common/utils/camera_notification/src/camera_notification_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 107 | `ERROR` | `,` |
| 11 | 169 | `ERROR` | `, , ,` |
| 12 | 171 | `ERROR` | `, , ,` |
| 17 | 106 | `ERROR` | `,` |
| 23 | 107 | `ERROR` | `,` |
| 25 | 143 | `ERROR` | `, , ,` |
| 28 | 179 | `ERROR` | `, , ,` |
| 30 | 167 | `ERROR` | `, , ,` |
| 39 | 103 | `ERROR` | `,` |
| 40 | 171 | `ERROR` | `, , ,` |
| 46 | 102 | `ERROR` | `,` |
| 47 | 171 | `ERROR` | `, , ,` |

#### `common/utils/camera_server_photo_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 94 | `ERROR` | `,` |
| 72 | 162 | `ERROR` | `, , ,` |
| 104 | 99 | `ERROR` | `,` |
| 109 | 162 | `ERROR` | `,` |
| 114 | 162 | `ERROR` | `,` |
| 134 | 99 | `ERROR` | `,` |
| 139 | 167 | `ERROR` | `,` |
| 144 | 167 | `ERROR` | `,` |
| 173 | 109 | `ERROR` | `,` |
| 180 | 96 | `ERROR` | `,` |
| 181 | 148 | `ERROR` | `, , ,` |
| 186 | 146 | `ERROR` | `, , ,` |
| 203 | 106 | `ERROR` | `,` |
| 208 | 149 | `ERROR` | `,` |
| 227 | 99 | `ERROR` | `,` |
| 228 | 134 | `ERROR` | `, , ,` |
| 246 | 175 | `ERROR` | `,` |
| 255 | 178 | `ERROR` | `,` |
| 264 | 178 | `ERROR` | `,` |
| 271 | 115 | `ERROR` | `,` |
| … | … | … | *(17 more)* |

#### `common/utils/camera_surface_buffer_util.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 163 | `ERROR` | `, , ,` |
| 18 | 161 | `ERROR` | `,` |
| 21 | 155 | `ERROR` | `,` |
| 35 | 159 | `ERROR` | `,` |
| 36 | 214 | `ERROR` | `,` |
| 36 | 233 | `ERROR` | `,` |
| 36 | 252 | `ERROR` | `,` |
| 39 | 102 | `ERROR` | `,` |
| 45 | 167 | `ERROR` | `, , ,` |
| 47 | 159 | `ERROR` | `,` |
| 49 | 173 | `ERROR` | `, , ,` |
| 51 | 160 | `ERROR` | `,` |
| 56 | 161 | `ERROR` | `,` |
| 63 | 172 | `ERROR` | `, , ,` |
| 70 | 170 | `ERROR` | `,` |
| 75 | 245 | `ERROR` | `,` |
| 91 | 162 | `ERROR` | `,` |
| 97 | 200 | `ERROR` | `,` |
| 102 | 32 | `ERROR` | `,` |
| 112 | 127 | `ERROR` | `,` |
| … | … | … | *(41 more)* |

#### `common/utils/camera_timer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 79 | `ERROR` | `,` |
| 20 | 79 | `ERROR` | `,` |
| 30 | 136 | `ERROR` | `, , ,` |
| 32 | 135 | `ERROR` | `,` |
| 39 | 135 | `ERROR` | `,` |
| 40 | 136 | `ERROR` | `, , ,` |

#### `common/utils/camera_xcollie.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 237 | `ERROR` | `,` |
| 25 | 178 | `ERROR` | `,` |

#### `common/utils/camera_xml_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 89 | 206 | `ERROR` | `,` |
| 89 | 219 | `ERROR` | `,` |
| 89 | 232 | `ERROR` | `,` |
| 91 | 150 | `ERROR` | `, , ,` |
| 97 | 161 | `ERROR` | `, , ,` |
| 103 | 165 | `ERROR` | `, , ,` |
| 123 | 215 | `ERROR` | `,` |
| 123 | 228 | `ERROR` | `,` |
| 123 | 241 | `ERROR` | `,` |
| 131 | 141 | `ERROR` | `, , ,` |
| 138 | 158 | `ERROR` | `, , ,` |
| 167 | 162 | `ERROR` | `, , ,` |
| 174 | 169 | `ERROR` | `, , ,` |

#### `common/utils/codec_info_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 146 | `ERROR` | `, , ,` |

#### `common/utils/image_effect/src/image_effect_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 100 | `ERROR` | `,` |
| 9 | 155 | `ERROR` | `, , ,` |
| 10 | 157 | `ERROR` | `, , ,` |
| 15 | 99 | `ERROR` | `,` |
| 22 | 100 | `ERROR` | `,` |
| 24 | 143 | `ERROR` | `, , ,` |
| 27 | 161 | `ERROR` | `, , ,` |
| 29 | 153 | `ERROR` | `, , ,` |
| 43 | 119 | `ERROR` | `,` |
| 44 | 157 | `ERROR` | `, , ,` |
| 50 | 104 | `ERROR` | `,` |
| 51 | 157 | `ERROR` | `, , ,` |
| 56 | 105 | `ERROR` | `,` |
| 57 | 157 | `ERROR` | `, , ,` |
| 62 | 104 | `ERROR` | `,` |
| 63 | 157 | `ERROR` | `, , ,` |
| 68 | 105 | `ERROR` | `,` |
| 69 | 157 | `ERROR` | `, , ,` |
| 74 | 115 | `ERROR` | `,` |
| 75 | 157 | `ERROR` | `, , ,` |
| … | … | … | *(8 more)* |

#### `common/utils/media_capability_interface.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 5 | 11 | `ERROR` | `OHOS::` |
| 6 | 26 | `ERROR` | `: public Parcelable` |
| 8 | 8 | `ERROR` | `~` |
| 9 | 9 | `ERROR` | `bool` |
| 10 | 49 | `ERROR` | `&` |

#### `common/utils/media_capability_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 80 | `ERROR` | `,` |
| 14 | 128 | `ERROR` | `, , ,` |
| 35 | 147 | `ERROR` | `,` |

#### `common/utils/media_manager/src/media_manager_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 101 | `ERROR` | `,` |
| 19 | 152 | `ERROR` | `, , ,` |
| 23 | 155 | `ERROR` | `, , ,` |
| 26 | 146 | `ERROR` | `, , ,` |
| 34 | 99 | `ERROR` | `,` |
| 40 | 160 | `ERROR` | `,` |
| 41 | 155 | `ERROR` | `, , ,` |
| 47 | 144 | `ERROR` | `,` |
| 48 | 155 | `ERROR` | `, , ,` |
| 54 | 87 | `ERROR` | `,` |
| 55 | 155 | `ERROR` | `, , ,` |
| 61 | 89 | `ERROR` | `,` |
| 62 | 155 | `ERROR` | `, , ,` |
| 68 | 87 | `ERROR` | `,` |
| 69 | 155 | `ERROR` | `, , ,` |
| 75 | 95 | `ERROR` | `,` |
| 76 | 155 | `ERROR` | `, , ,` |
| 82 | 86 | `ERROR` | `,` |
| 83 | 155 | `ERROR` | `, , ,` |
| 89 | 91 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `common/utils/media_stream/src/recorder_engine_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 106 | `ERROR` | `,` |
| 15 | 105 | `ERROR` | `,` |
| 22 | 103 | `ERROR` | `,` |
| 24 | 143 | `ERROR` | `, , ,` |
| 27 | 167 | `ERROR` | `, , ,` |
| 29 | 159 | `ERROR` | `, , ,` |
| 41 | 123 | `ERROR` | `,` |
| 42 | 163 | `ERROR` | `, , ,` |
| 48 | 110 | `ERROR` | `,` |
| 49 | 163 | `ERROR` | `, , ,` |
| 55 | 117 | `ERROR` | `,` |
| 56 | 163 | `ERROR` | `, , ,` |
| 62 | 117 | `ERROR` | `,` |
| 63 | 163 | `ERROR` | `, , ,` |
| 69 | 118 | `ERROR` | `,` |
| 70 | 163 | `ERROR` | `, , ,` |
| 76 | 116 | `ERROR` | `,` |
| 77 | 163 | `ERROR` | `, , ,` |
| 83 | 114 | `ERROR` | `,` |
| 84 | 163 | `ERROR` | `, , ,` |
| … | … | … | *(54 more)* |

#### `common/utils/movie_file/src/movie_file_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 98 | `ERROR` | `,` |
| 14 | 97 | `ERROR` | `,` |
| 21 | 98 | `ERROR` | `,` |
| 23 | 143 | `ERROR` | `, , ,` |
| 25 | 157 | `ERROR` | `, , ,` |
| 27 | 149 | `ERROR` | `, , ,` |
| 41 | 124 | `ERROR` | `,` |
| 42 | 153 | `ERROR` | `, , ,` |
| 48 | 115 | `ERROR` | `,` |
| 49 | 153 | `ERROR` | `, , ,` |
| 55 | 114 | `ERROR` | `,` |
| 56 | 153 | `ERROR` | `, , ,` |
| 58 | 171 | `ERROR` | `, , ,` |
| 65 | 113 | `ERROR` | `,` |
| 66 | 153 | `ERROR` | `, , ,` |
| 68 | 170 | `ERROR` | `, , ,` |
| 75 | 115 | `ERROR` | `,` |
| 76 | 153 | `ERROR` | `, , ,` |
| 82 | 114 | `ERROR` | `,` |
| 83 | 153 | `ERROR` | `, , ,` |
| … | … | … | *(16 more)* |

#### `common/utils/photo_asset_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 161 | `ERROR` | `,` |
| 22 | 168 | `ERROR` | `,` |
| 38 | 179 | `ERROR` | `, , ,` |
| 39 | 177 | `ERROR` | `, , ,` |
| 47 | 183 | `ERROR` | `, , ,` |
| 56 | 183 | `ERROR` | `, , ,` |
| 64 | 186 | `ERROR` | `, , ,` |
| 70 | 180 | `ERROR` | `, , ,` |
| 76 | 193 | `ERROR` | `, , ,` |
| 82 | 179 | `ERROR` | `, , ,` |
| 88 | 109 | `ERROR` | `,` |
| 89 | 179 | `ERROR` | `, , ,` |
| 95 | 114 | `ERROR` | `,` |

#### `common/utils/picture_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 166 | `ERROR` | `, , ,` |
| 12 | 168 | `ERROR` | `, , ,` |
| 17 | 89 | `ERROR` | `,` |
| 22 | 88 | `ERROR` | `,` |
| 24 | 161 | `ERROR` | `, , ,` |
| 32 | 176 | `ERROR` | `, , ,` |
| 35 | 164 | `ERROR` | `, , ,` |
| 37 | 178 | `ERROR` | `, , ,` |
| 49 | 110 | `ERROR` | `,` |
| 51 | 174 | `ERROR` | `, , ,` |
| 57 | 102 | `ERROR` | `,` |
| 59 | 174 | `ERROR` | `, , ,` |
| 65 | 104 | `ERROR` | `,` |
| 67 | 175 | `ERROR` | `, , ,` |
| 73 | 106 | `ERROR` | `,` |
| 76 | 169 | `ERROR` | `, , ,` |
| 85 | 172 | `ERROR` | `, , ,` |
| 94 | 172 | `ERROR` | `, , ,` |
| 102 | 167 | `ERROR` | `, , ,` |
| 110 | 170 | `ERROR` | `, , ,` |
| … | … | … | *(3 more)* |

#### `common/utils/watch_dog.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 143 | `ERROR` | `,` |
| 34 | 157 | `ERROR` | `,` |
| 40 | 163 | `ERROR` | `,` |
| 42 | 201 | `ERROR` | `,` |
| 51 | 163 | `ERROR` | `,` |
| 54 | 142 | `ERROR` | `,` |
| 63 | 155 | `ERROR` | `,` |
| 65 | 134 | `ERROR` | `,` |

#### `common/utils/watermark_exif_metadata/src/watermark_exif_metadata_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 110 | `ERROR` | `,` |
| 11 | 186 | `ERROR` | `, , ,` |
| 12 | 177 | `ERROR` | `, , ,` |
| 17 | 109 | `ERROR` | `,` |
| 23 | 110 | `ERROR` | `,` |
| 25 | 143 | `ERROR` | `, , ,` |
| 28 | 181 | `ERROR` | `, , ,` |
| 30 | 173 | `ERROR` | `, , ,` |
| 39 | 124 | `ERROR` | `,` |
| 40 | 177 | `ERROR` | `, , ,` |
| 41 | 141 | `ERROR` | `, , ,` |

#### `common/utils/watermark_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 106 | `ERROR` | `,` |
| 35 | 216 | `ERROR` | `,` |
| 35 | 237 | `ERROR` | `,` |
| 35 | 258 | `ERROR` | `,` |
| 66 | 109 | `ERROR` | `,` |
| 70 | 171 | `ERROR` | `, , ,` |
| 89 | 103 | `ERROR` | `,` |
| 93 | 135 | `ERROR` | `, , ,` |
| 99 | 197 | `ERROR` | `,` |
| 99 | 207 | `ERROR` | `,` |
| 99 | 217 | `ERROR` | `,` |
| 102 | 192 | `ERROR` | `,` |
| 102 | 202 | `ERROR` | `,` |
| 102 | 212 | `ERROR` | `,` |
| 104 | 194 | `ERROR` | `,` |
| 104 | 204 | `ERROR` | `,` |
| 104 | 214 | `ERROR` | `,` |
| 115 | 164 | `ERROR` | `,` |
| 117 | 177 | `ERROR` | `, , ,` |
| 126 | 186 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `common/utils/xcomponent_controller/src/xcomponent_controller_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 109 | `ERROR` | `,` |
| 10 | 174 | `ERROR` | `, , ,` |
| 11 | 175 | `ERROR` | `, , ,` |
| 16 | 108 | `ERROR` | `,` |
| 22 | 109 | `ERROR` | `,` |
| 24 | 143 | `ERROR` | `, , ,` |
| 27 | 179 | `ERROR` | `, , ,` |
| 29 | 171 | `ERROR` | `, , ,` |
| 38 | 122 | `ERROR` | `,` |
| 40 | 149 | `ERROR` | `, , ,` |
| 47 | 122 | `ERROR` | `,` |
| 49 | 149 | `ERROR` | `, , ,` |

#### `dynamic_libs/av_codec/src/av_codec_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 98 | `ERROR` | `,` |
| 18 | 97 | `ERROR` | `,` |
| 24 | 144 | `ERROR` | `, , ,` |
| 27 | 154 | `ERROR` | `, , ,` |
| 35 | 146 | `ERROR` | `, , ,` |
| 51 | 145 | `ERROR` | `, , ,` |
| 69 | 146 | `ERROR` | `, , ,` |

#### `dynamic_libs/camera_notification/src/camera_notification.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 155 | `ERROR` | `, , ,` |
| 84 | 154 | `ERROR` | `, , ,` |
| 115 | 159 | `ERROR` | `, , ,` |
| 122 | 149 | `ERROR` | `, , ,` |
| 127 | 176 | `ERROR` | `,` |
| 133 | 152 | `ERROR` | `, , ,` |
| 138 | 174 | `ERROR` | `, , ,` |
| 143 | 179 | `ERROR` | `,` |
| 144 | 149 | `ERROR` | `, , ,` |
| 149 | 176 | `ERROR` | `,` |
| 150 | 170 | `ERROR` | `, , ,` |
| 161 | 155 | `ERROR` | `,` |
| 168 | 75 | `ERROR` | `,` |
| 180 | 169 | `ERROR` | `, , ,` |
| 184 | 202 | `ERROR` | `,` |
| 184 | 213 | `ERROR` | `,` |
| 184 | 224 | `ERROR` | `,` |
| 185 | 136 | `ERROR` | `, , ,` |
| 188 | 148 | `ERROR` | `, , ,` |
| 190 | 86 | `ERROR` | `,` |

#### `dynamic_libs/image_effect/src/image_effect_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 95 | `ERROR` | `,` |
| 48 | 111 | `ERROR` | `,` |
| 62 | 173 | `ERROR` | `,` |
| 67 | 96 | `ERROR` | `,` |
| 71 | 148 | `ERROR` | `, , ,` |
| 83 | 95 | `ERROR` | `,` |
| 86 | 167 | `ERROR` | `, , ,` |
| 97 | 96 | `ERROR` | `,` |
| 103 | 149 | `ERROR` | `, , ,` |
| 110 | 87 | `ERROR` | `,` |
| 111 | 146 | `ERROR` | `, , ,` |
| 123 | 222 | `ERROR` | `,` |
| 126 | 163 | `ERROR` | `, , ,` |
| 144 | 118 | `ERROR` | `,` |
| 158 | 127 | `ERROR` | `,` |
| 167 | 123 | `ERROR` | `,` |
| 182 | 124 | `ERROR` | `,` |
| 185 | 185 | `ERROR` | `, , ,` |
| 194 | 184 | `ERROR` | `, , ,` |
| 197 | 177 | `ERROR` | `, , ,` |
| … | … | … | *(12 more)* |

#### `dynamic_libs/image_framework/src/picture_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 90 | `ERROR` | `,` |
| 75 | 180 | `ERROR` | `,` |
| 75 | 190 | `ERROR` | `,` |
| 75 | 200 | `ERROR` | `,` |
| 76 | 176 | `ERROR` | `, , ,` |
| 112 | 91 | `ERROR` | `,` |
| 115 | 140 | `ERROR` | `,` |
| 123 | 122 | `ERROR` | `, , ,` |
| 124 | 125 | `ERROR` | `, , ,` |
| 125 | 138 | `ERROR` | `, , ,` |
| 128 | 215 | `ERROR` | `,` |
| 128 | 227 | `ERROR` | `,` |
| 128 | 239 | `ERROR` | `,` |
| 142 | 137 | `ERROR` | `, , ,` |
| 170 | 133 | `ERROR` | `, , ,` |
| 179 | 166 | `ERROR` | `,` |
| 183 | 122 | `ERROR` | `, , ,` |
| 184 | 134 | `ERROR` | `,` |
| 188 | 182 | `ERROR` | `, , ,` |
| 193 | 159 | `ERROR` | `, , ,` |
| … | … | … | *(90 more)* |

#### `dynamic_libs/media_library/src/photo_asset_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 93 | `ERROR` | `,` |
| 16 | 165 | `ERROR` | `, , ,` |
| 18 | 156 | `ERROR` | `, , ,` |
| 20 | 135 | `ERROR` | `, , ,` |
| 25 | 193 | `ERROR` | `,` |
| 25 | 201 | `ERROR` | `,` |
| 25 | 209 | `ERROR` | `,` |
| 27 | 160 | `ERROR` | `,` |
| 28 | 110 | `ERROR` | `,` |
| 94 | 109 | `ERROR` | `,` |
| 102 | 197 | `ERROR` | `,` |
| 110 | 116 | `ERROR` | `,` |

#### `dynamic_libs/media_manager/include/media_manager/mpeg_manager_factory.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 38 | `missing ;` | `` |

#### `dynamic_libs/media_manager/include/media_manager/track_factory.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 32 | `missing ;` | `` |

#### `dynamic_libs/media_manager/src/media_manager/demuxer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 17 | 140 | `ERROR` | `, , ,` |
| 18 | 93 | `ERROR` | `,` |
| 20 | 144 | `ERROR` | `, , ,` |
| 22 | 169 | `ERROR` | `,` |
| 60 | 152 | `ERROR` | `,` |
| 66 | 80 | `ERROR` | `,` |
| 67 | 153 | `ERROR` | `, , ,` |
| 69 | 209 | `ERROR` | `,` |
| 69 | 217 | `ERROR` | `,` |
| 69 | 225 | `ERROR` | `,` |
| 75 | 80 | `ERROR` | `,` |

#### `dynamic_libs/media_manager/src/media_manager/media_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 80 | `ERROR` | `,` |
| 35 | 131 | `ERROR` | `, , ,` |
| 38 | 131 | `ERROR` | `, , ,` |
| 43 | 127 | `ERROR` | `, , ,` |
| 54 | 80 | `ERROR` | `,` |
| 57 | 162 | `ERROR` | `,` |
| 61 | 158 | `ERROR` | `, , ,` |
| 63 | 188 | `ERROR` | `, , ,` |
| 67 | 155 | `ERROR` | `,` |
| 69 | 165 | `ERROR` | `, , ,` |
| 71 | 171 | `ERROR` | `, , ,` |
| 77 | 148 | `ERROR` | `, , ,` |
| 80 | 96 | `ERROR` | `,` |
| 90 | 158 | `ERROR` | `, , ,` |
| 92 | 114 | `ERROR` | `,` |
| 98 | 80 | `ERROR` | `,` |
| 99 | 144 | `ERROR` | `, , ,` |
| 102 | 139 | `ERROR` | `, , ,` |
| 103 | 107 | `ERROR` | `, , ,` |
| 109 | 144 | `ERROR` | `,` |
| … | … | … | *(47 more)* |

#### `dynamic_libs/media_manager/src/media_manager/mpeg_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 80 | `ERROR` | `,` |
| 34 | 183 | `ERROR` | `,` |
| 39 | 80 | `ERROR` | `,` |
| 44 | 140 | `ERROR` | `,` |
| 49 | 140 | `ERROR` | `, , ,` |
| 51 | 144 | `ERROR` | `, , ,` |
| 63 | 80 | `ERROR` | `,` |
| 70 | 144 | `ERROR` | `, , ,` |
| 80 | 80 | `ERROR` | `,` |
| 86 | 79 | `ERROR` | `,` |
| 100 | 79 | `ERROR` | `,` |
| 105 | 147 | `ERROR` | `, , ,` |
| 108 | 145 | `ERROR` | `, , ,` |
| 110 | 146 | `ERROR` | `, , ,` |
| 122 | 79 | `ERROR` | `,` |
| 126 | 147 | `ERROR` | `, , ,` |
| 134 | 153 | `ERROR` | `, , ,` |
| 142 | 85 | `ERROR` | `,` |
| 145 | 150 | `ERROR` | `, , ,` |
| 152 | 140 | `ERROR` | `, , ,` |
| … | … | … | *(46 more)* |

#### `dynamic_libs/media_manager/src/media_manager/mpeg_manager_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 27 | 118 | `ERROR` | `,` |
| 30 | 140 | `ERROR` | `, , ,` |
| 34 | 105 | `ERROR` | `,` |
| 39 | 104 | `ERROR` | `,` |
| 48 | 164 | `ERROR` | `, , ,` |
| 49 | 173 | `ERROR` | `, , ,` |
| 52 | 95 | `ERROR` | `,` |

#### `dynamic_libs/media_manager/src/media_manager/muxer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 16 | 91 | `ERROR` | `,` |
| 17 | 204 | `ERROR` | `,` |
| 17 | 217 | `ERROR` | `,` |
| 17 | 230 | `ERROR` | `,` |
| 20 | 142 | `ERROR` | `, , ,` |
| 27 | 158 | `ERROR` | `, , ,` |
| 90 | 147 | `ERROR` | `, , ,` |
| 107 | 89 | `ERROR` | `,` |
| 117 | 88 | `ERROR` | `,` |
| 119 | 208 | `ERROR` | `,` |
| 119 | 216 | `ERROR` | `,` |
| 119 | 224 | `ERROR` | `,` |

#### `dynamic_libs/media_manager/src/media_manager/reader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 80 | `ERROR` | `,` |
| 24 | 201 | `ERROR` | `,` |
| 24 | 213 | `ERROR` | `,` |
| 24 | 225 | `ERROR` | `,` |
| 27 | 161 | `ERROR` | `, , ,` |
| 29 | 144 | `ERROR` | `, , ,` |
| 32 | 139 | `ERROR` | `, , ,` |
| 35 | 135 | `ERROR` | `, , ,` |
| 38 | 143 | `ERROR` | `, , ,` |
| 44 | 80 | `ERROR` | `,` |
| 45 | 141 | `ERROR` | `, , ,` |
| 49 | 162 | `ERROR` | `, , ,` |
| 51 | 170 | `ERROR` | `,` |
| 57 | 80 | `ERROR` | `,` |
| 58 | 141 | `ERROR` | `, , ,` |
| 62 | 158 | `ERROR` | `, , ,` |
| 69 | 80 | `ERROR` | `,` |
| 75 | 174 | `ERROR` | `,` |
| 75 | 184 | `ERROR` | `,` |
| 75 | 194 | `ERROR` | `,` |
| … | … | … | *(14 more)* |

#### `dynamic_libs/media_manager/src/media_manager/track_factory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 80 | `ERROR` | `,` |
| 24 | 80 | `ERROR` | `,` |
| 29 | 140 | `ERROR` | `, , ,` |
| 32 | 159 | `ERROR` | `, , ,` |
| 34 | 145 | `ERROR` | `, , ,` |
| 36 | 181 | `ERROR` | `, , ,` |
| 39 | 169 | `ERROR` | `,` |

#### `dynamic_libs/media_manager/src/media_manager/writer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 204 | `ERROR` | `,` |
| 13 | 217 | `ERROR` | `,` |
| 13 | 230 | `ERROR` | `,` |
| 14 | 140 | `ERROR` | `, , ,` |
| 18 | 208 | `ERROR` | `,` |
| 20 | 132 | `ERROR` | `, , ,` |
| 23 | 129 | `ERROR` | `, , ,` |
| 32 | 160 | `ERROR` | `,` |
| 46 | 80 | `ERROR` | `,` |
| 48 | 160 | `ERROR` | `, , ,` |
| 51 | 183 | `ERROR` | `,` |
| 51 | 191 | `ERROR` | `,` |
| 51 | 199 | `ERROR` | `,` |
| 59 | 80 | `ERROR` | `,` |
| 60 | 162 | `ERROR` | `, , ,` |
| 62 | 182 | `ERROR` | `,` |
| 62 | 190 | `ERROR` | `,` |
| 62 | 198 | `ERROR` | `,` |
| 70 | 80 | `ERROR` | `,` |
| 72 | 134 | `ERROR` | `, , ,` |
| … | … | … | *(2 more)* |

#### `dynamic_libs/media_manager/src/media_manager_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 103 | `ERROR` | `,` |
| 14 | 102 | `ERROR` | `,` |
| 20 | 172 | `ERROR` | `,` |
| 23 | 150 | `ERROR` | `, , ,` |
| 24 | 103 | `ERROR` | `,` |
| 30 | 150 | `ERROR` | `, , ,` |
| 32 | 166 | `ERROR` | `,` |
| 38 | 94 | `ERROR` | `,` |
| 39 | 150 | `ERROR` | `, , ,` |
| 45 | 96 | `ERROR` | `,` |
| 46 | 150 | `ERROR` | `, , ,` |
| 52 | 94 | `ERROR` | `,` |
| 53 | 150 | `ERROR` | `, , ,` |
| 54 | 147 | `ERROR` | `, , ,` |
| 60 | 102 | `ERROR` | `,` |
| 61 | 150 | `ERROR` | `, , ,` |
| 67 | 93 | `ERROR` | `,` |
| 68 | 150 | `ERROR` | `, , ,` |
| 74 | 98 | `ERROR` | `,` |
| 75 | 150 | `ERROR` | `, , ,` |
| … | … | … | *(4 more)* |

#### `dynamic_libs/moving_photo/include/common/audio_record.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 97 | `ERROR` | `,` |
| 60 | 96 | `ERROR` | `,` |
| 108 | 110 | `ERROR` | `,` |
| 111 | 160 | `ERROR` | `,` |
| 133 | 148 | `ERROR` | `,` |

#### `dynamic_libs/moving_photo/src/avcodec/audio_capturer_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 102 | `ERROR` | `,` |
| 35 | 164 | `ERROR` | `, , ,` |
| 39 | 167 | `ERROR` | `, , ,` |
| 42 | 172 | `ERROR` | `, , ,` |
| 43 | 187 | `ERROR` | `,` |
| 59 | 176 | `ERROR` | `, , ,` |
| 79 | 105 | `ERROR` | `,` |
| 80 | 154 | `ERROR` | `, , ,` |
| 84 | 98 | `ERROR` | `,` |
| 89 | 92 | `ERROR` | `,` |
| 96 | 90 | `ERROR` | `,` |
| 102 | 107 | `ERROR` | `,` |
| 109 | 152 | `ERROR` | `, , ,` |
| 129 | 153 | `ERROR` | `, , ,` |
| 134 | 161 | `ERROR` | `, , ,` |
| 136 | 145 | `ERROR` | `, , ,` |
| 139 | 95 | `ERROR` | `,` |
| 140 | 160 | `ERROR` | `, , ,` |
| 145 | 158 | `ERROR` | `,` |
| 152 | 106 | `ERROR` | `,` |
| … | … | … | *(7 more)* |

#### `dynamic_libs/moving_photo/src/avcodec/audio_deferred_process.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 99 | `ERROR` | `,` |
| 15 | 98 | `ERROR` | `,` |
| 22 | 120 | `ERROR` | `,` |
| 30 | 172 | `ERROR` | `, , ,` |
| 38 | 110 | `ERROR` | `,` |
| 47 | 128 | `ERROR` | `,` |
| 58 | 129 | `ERROR` | `,` |
| 60 | 169 | `ERROR` | `, , ,` |
| 70 | 115 | `ERROR` | `,` |
| 73 | 185 | `ERROR` | `, , ,` |
| 82 | 203 | `ERROR` | `, , ,` |
| 101 | 177 | `ERROR` | `, , ,` |
| 115 | 179 | `ERROR` | `, , ,` |
| 119 | 144 | `ERROR` | `, , ,` |
| 130 | 169 | `ERROR` | `, , ,` |
| 131 | 165 | `ERROR` | `, , ,` |
| 132 | 183 | `ERROR` | `, , ,` |
| 142 | 153 | `ERROR` | `, , ,` |
| 151 | 167 | `ERROR` | `, , ,` |
| 152 | 106 | `ERROR` | `,` |
| … | … | … | *(11 more)* |

#### `dynamic_libs/moving_photo/src/avcodec/audio_encoder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 90 | `ERROR` | `,` |
| 15 | 101 | `ERROR` | `,` |
| 23 | 135 | `ERROR` | `, , ,` |
| 30 | 137 | `ERROR` | `, , ,` |
| 33 | 127 | `ERROR` | `, , ,` |
| 35 | 152 | `ERROR` | `, , ,` |
| 37 | 130 | `ERROR` | `, , ,` |
| 40 | 192 | `ERROR` | `,` |
| 40 | 200 | `ERROR` | `,` |
| 40 | 208 | `ERROR` | `,` |
| 47 | 137 | `ERROR` | `, , ,` |
| 49 | 190 | `ERROR` | `,` |
| 49 | 198 | `ERROR` | `,` |
| 49 | 206 | `ERROR` | `,` |
| 57 | 137 | `ERROR` | `, , ,` |
| 58 | 138 | `ERROR` | `, , ,` |
| 61 | 202 | `ERROR` | `,` |
| 61 | 210 | `ERROR` | `,` |
| 61 | 218 | `ERROR` | `,` |
| 63 | 200 | `ERROR` | `,` |
| … | … | … | *(34 more)* |

#### `dynamic_libs/moving_photo/src/avcodec/avcodec_task_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 115 | `ERROR` | `,` |
| 49 | 93 | `ERROR` | `,` |
| 104 | 53 | `ERROR` | `,` |
| 108 | 171 | `ERROR` | `,` |
| 111 | 144 | `ERROR` | `, , ,` |
| 141 | 153 | `ERROR` | `,` |
| 150 | 173 | `ERROR` | `,` |
| 160 | 150 | `ERROR` | `,` |
| 169 | 190 | `ERROR` | `,` |
| 172 | 106 | `ERROR` | `,` |
| 181 | 173 | `ERROR` | `,` |
| 184 | 89 | `ERROR` | `,` |
| 219 | 165 | `ERROR` | `, , ,` |
| 228 | 148 | `ERROR` | `, , ,` |
| 231 | 146 | `ERROR` | `, , ,` |
| 255 | 165 | `ERROR` | `,` |
| 278 | 205 | `ERROR` | `,` |
| 290 | 101 | `ERROR` | `,` |
| 322 | 96 | `ERROR` | `,` |
| 327 | 204 | `ERROR` | `,` |
| … | … | … | *(81 more)* |

#### `dynamic_libs/moving_photo/src/avcodec/moving_photo_video_cache.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 100 | `ERROR` | `,` |
| 32 | 88 | `ERROR` | `,` |
| 34 | 87 | `ERROR` | `,` |
| 77 | 89 | `ERROR` | `,` |
| 99 | 256 | `ERROR` | `,` |
| 108 | 185 | `ERROR` | `,` |
| 114 | 178 | `ERROR` | `,` |
| 130 | 40 | `ERROR` | `,` |
| 137 | 97 | `ERROR` | `,` |
| 139 | 113 | `ERROR` | `,` |
| 148 | 87 | `ERROR` | `,` |
| 169 | 103 | `ERROR` | `,` |
| 178 | 98 | `ERROR` | `,` |
| 211 | 190 | `ERROR` | `,` |

#### `dynamic_libs/moving_photo/src/common/moving_photo_listener.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 103 | `ERROR` | `,` |
| 33 | 90 | `ERROR` | `,` |
| 44 | 96 | `ERROR` | `,` |
| 45 | 132 | `ERROR` | `, , ,` |
| 47 | 153 | `ERROR` | `, , ,` |
| 53 | 94 | `ERROR` | `,` |
| 55 | 105 | `ERROR` | `,` |
| 62 | 95 | `ERROR` | `,` |
| 64 | 164 | `ERROR` | `, , ,` |
| 102 | 105 | `ERROR` | `,` |
| 108 | 182 | `ERROR` | `,` |
| 114 | 87 | `ERROR` | `,` |
| 118 | 255 | `ERROR` | `,` |
| 122 | 85 | `ERROR` | `,` |
| 133 | 88 | `ERROR` | `,` |
| 139 | 171 | `ERROR` | `,` |
| 148 | 176 | `missing identifier` | `` |
| 150 | 267 | `ERROR` | `,` |
| 153 | 188 | `ERROR` | `, , ,` |
| 158 | 95 | `ERROR` | `,` |
| … | … | … | *(22 more)* |

#### `dynamic_libs/moving_photo/src/common/moving_photo_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 166 | `ERROR` | `, , ,` |
| 15 | 178 | `ERROR` | `, , ,` |
| 33 | 162 | `ERROR` | `, , ,` |
| 40 | 162 | `ERROR` | `, , ,` |
| 59 | 162 | `ERROR` | `, , ,` |
| 65 | 162 | `ERROR` | `, , ,` |
| 68 | 171 | `ERROR` | `, , ,` |
| 71 | 175 | `ERROR` | `, , ,` |
| 75 | 187 | `ERROR` | `, , ,` |
| 79 | 191 | `ERROR` | `, , ,` |
| 86 | 105 | `ERROR` | `,` |
| 91 | 105 | `ERROR` | `,` |
| 96 | 120 | `ERROR` | `,` |
| 104 | 113 | `ERROR` | `,` |
| 111 | 122 | `ERROR` | `,` |
| 121 | 131 | `ERROR` | `,` |
| 136 | 114 | `ERROR` | `,` |
| 165 | 118 | `ERROR` | `,` |
| 170 | 90 | `ERROR` | `,` |
| 175 | 124 | `ERROR` | `,` |
| … | … | … | *(13 more)* |

#### `dynamic_libs/moving_photo/src/moving_photo_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 109 | `ERROR` | `,` |
| 12 | 108 | `ERROR` | `,` |
| 18 | 222 | `ERROR` | `,` |
| 20 | 173 | `ERROR` | `, , ,` |
| 23 | 167 | `ERROR` | `, , ,` |
| 25 | 163 | `ERROR` | `, , ,` |
| 27 | 164 | `ERROR` | `, , ,` |
| 34 | 222 | `ERROR` | `,` |
| 36 | 165 | `ERROR` | `, , ,` |
| 39 | 171 | `ERROR` | `, , ,` |
| 41 | 157 | `ERROR` | `, , ,` |
| 44 | 164 | `ERROR` | `, , ,` |
| 51 | 227 | `ERROR` | `,` |
| 53 | 163 | `ERROR` | `, , ,` |
| 62 | 163 | `ERROR` | `, , ,` |
| 68 | 174 | `ERROR` | `,` |
| 69 | 163 | `ERROR` | `, , ,` |
| 76 | 240 | `ERROR` | `,` |
| 78 | 163 | `ERROR` | `, , ,` |
| 84 | 174 | `ERROR` | `,` |
| … | … | … | *(54 more)* |

#### `dynamic_libs/watermark_exif_metadata/src/watermark_exif_metadata_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 105 | `ERROR` | `,` |
| 41 | 138 | `ERROR` | `, , ,` |
| 43 | 93 | `ERROR` | `,` |
| 51 | 95 | `ERROR` | `,` |
| 57 | 148 | `ERROR` | `,` |
| 88 | 132 | `ERROR` | `,` |
| 92 | 133 | `ERROR` | `, , ,` |

#### `dynamic_libs/xcomponent_controller/src/xcomponent_controller_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 104 | `ERROR` | `,` |
| 15 | 130 | `ERROR` | `,` |
| 18 | 192 | `ERROR` | `,` |
| 18 | 200 | `ERROR` | `,` |
| 18 | 208 | `ERROR` | `,` |
| 25 | 130 | `ERROR` | `,` |
| 28 | 192 | `ERROR` | `,` |
| 28 | 200 | `ERROR` | `,` |
| 28 | 208 | `ERROR` | `,` |

#### `frameworks/cj/camera/include/camera_ffi.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 12 | `ERROR` | `int64_t` |
| 11 | 12 | `ERROR` | `CArrI32` |
| 12 | 30 | `missing ::` | `` |
| 13 | 36 | `missing ::` | `` |
| 16 | 12 | `ERROR` | `bool` |
| 17 | 12 | `ERROR` | `int64_t` |
| 19 | 12 | `ERROR` | `int64_t` |
| 21 | 12 | `ERROR` | `int64_t` |
| 22 | 12 | `ERROR` | `RetInt64` |
| 23 | 12 | `ERROR` | `int64_t` |
| 24 | 12 | `ERROR` | `RetInt64` |
| 25 | 12 | `ERROR` | `int64_t` |
| 26 | 12 | `ERROR` | `int64_t` |
| 27 | 12 | `ERROR` | `int64_t` |
| 28 | 12 | `ERROR` | `int64_t` |
| 29 | 12 | `ERROR` | `int64_t` |
| 30 | 12 | `ERROR` | `int64_t` |
| 31 | 12 | `ERROR` | `bool` |
| 32 | 12 | `ERROR` | `bool` |
| 33 | 12 | `ERROR` | `int32_t` |
| … | … | … | *(144 more)* |

#### `frameworks/cj/camera/include/camera_input_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 48 | `missing ;` | `` |

#### `frameworks/cj/camera/include/camera_manager_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 50 | `missing ;` | `` |

#### `frameworks/cj/camera/include/camera_output_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 47 | `missing ;` | `` |

#### `frameworks/cj/camera/include/camera_session_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 44 | `missing ;` | `` |

#### `frameworks/cj/camera/include/listener_base.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 20 | `ERROR` | `. . .` |
| 9 | 24 | `ERROR` | `. . .` |
| 11 | 42 | `ERROR` | `. . .` |
| 18 | 20 | `ERROR` | `. . .` |
| 20 | 61 | `ERROR` | `. . .` |
| 26 | 30 | `ERROR` | `. . .` |
| 30 | 48 | `ERROR` | `. . .` |
| 33 | 20 | `ERROR` | `. . .` |
| 33 | 49 | `ERROR` | `. . .` |
| 33 | 113 | `ERROR` | `. . .` |
| 39 | 20 | `ERROR` | `. . .` |
| 39 | 49 | `ERROR` | `. . .` |
| 50 | 20 | `ERROR` | `. . .` |
| 50 | 49 | `ERROR` | `. . .` |
| 56 | 20 | `ERROR` | `. . .` |
| 56 | 49 | `ERROR` | `. . .` |
| 56 | 82 | `ERROR` | `. . .` |
| 65 | 36 | `ERROR` | `. . .` |

#### `frameworks/cj/camera/include/metadata_output_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 51 | `missing ;` | `` |

#### `frameworks/cj/camera/include/photo_output_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 48 | `missing ;` | `` |

#### `frameworks/cj/camera/include/preview_output_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 50 | `missing ;` | `` |

#### `frameworks/cj/camera/include/video_output_impl.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 48 | `missing ;` | `` |

#### `frameworks/cj/camera/src/camera_ffi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 277 | 111 | `ERROR` | `,` |
| 631 | 12 | `ERROR` | `int32_t` |
| 1127 | 85 | `ERROR` | `,` |
| 1137 | 84 | `ERROR` | `,` |
| 1333 | 12 | `ERROR` | `bool` |
| 1348 | 12 | `ERROR` | `void` |
| 1360 | 12 | `ERROR` | `void` |
| 1377 | 12 | `ERROR` | `void` |
| 1389 | 12 | `ERROR` | `void` |
| 1401 | 12 | `ERROR` | `void` |
| 1413 | 12 | `ERROR` | `void` |
| 1425 | 12 | `ERROR` | `void` |
| 1437 | 12 | `ERROR` | `void` |
| 1449 | 12 | `ERROR` | `void` |
| 1461 | 12 | `ERROR` | `void` |
| 1473 | 12 | `ERROR` | `void` |
| 1485 | 12 | `ERROR` | `bool` |
| 1498 | 23 | `missing ::` | `` |
| 1511 | 12 | `ERROR` | `int32_t` |
| 1524 | 12 | `ERROR` | `void` |
| … | … | … | *(25 more)* |

#### `frameworks/cj/camera/src/camera_session_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 449 | 98 | `ERROR` | `,` |
| 458 | 98 | `ERROR` | `,` |
| 482 | 93 | `ERROR` | `,` |
| 491 | 93 | `ERROR` | `,` |
| 515 | 98 | `ERROR` | `,` |
| 524 | 98 | `ERROR` | `,` |
| 532 | 107 | `ERROR` | `,` |
| 538 | 89 | `ERROR` | `,` |
| 544 | 100 | `ERROR` | `,` |

#### `frameworks/cj/camera/src/metadata_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 103 | `ERROR` | `,` |

#### `frameworks/cj/camera/src/photo_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 121 | 87 | `ERROR` | `,` |
| 124 | 91 | `ERROR` | `,` |
| 140 | 87 | `ERROR` | `,` |
| 143 | 93 | `ERROR` | `,` |
| 174 | 97 | `ERROR` | `,` |
| 177 | 93 | `ERROR` | `,` |
| 182 | 37 | `ERROR` | `,` |
| 187 | 106 | `ERROR` | `,` |
| 194 | 196 | `ERROR` | `,` |
| 196 | 97 | `ERROR` | `,` |
| 199 | 93 | `ERROR` | `,` |
| 213 | 106 | `ERROR` | `,` |
| 225 | 88 | `ERROR` | `,` |
| 227 | 95 | `ERROR` | `,` |
| 251 | 88 | `ERROR` | `,` |
| 253 | 95 | `ERROR` | `,` |
| 262 | 104 | `ERROR` | `,` |
| 264 | 95 | `ERROR` | `,` |
| 270 | 91 | `ERROR` | `,` |
| 280 | 99 | `ERROR` | `,` |
| … | … | … | *(14 more)* |

#### `frameworks/cj/camera/src/preview_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 88 | 107 | `ERROR` | `,` |
| 94 | 102 | `ERROR` | `,` |
| 111 | 107 | `ERROR` | `,` |
| 118 | 102 | `ERROR` | `,` |
| 135 | 93 | `ERROR` | `,` |
| 140 | 102 | `ERROR` | `,` |
| 157 | 93 | `ERROR` | `,` |
| 163 | 102 | `ERROR` | `,` |
| 172 | 104 | `ERROR` | `,` |
| 175 | 98 | `ERROR` | `,` |
| 194 | 99 | `ERROR` | `,` |
| 201 | 94 | `ERROR` | `,` |
| 203 | 97 | `ERROR` | `,` |
| 208 | 97 | `ERROR` | `,` |
| 215 | 100 | `ERROR` | `,` |
| 218 | 97 | `ERROR` | `,` |
| 230 | 98 | `ERROR` | `,` |
| 232 | 97 | `ERROR` | `,` |
| 248 | 101 | `ERROR` | `,` |
| 265 | 101 | `ERROR` | `,` |

#### `frameworks/cj/camera/src/video_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 79 | 111 | `ERROR` | `,` |
| 85 | 100 | `ERROR` | `,` |
| 98 | 111 | `ERROR` | `,` |
| 103 | 100 | `ERROR` | `,` |
| 127 | 104 | `ERROR` | `,` |
| 130 | 95 | `ERROR` | `,` |
| 135 | 195 | `ERROR` | `,` |
| 150 | 100 | `ERROR` | `,` |
| 157 | 93 | `ERROR` | `,` |
| 159 | 95 | `ERROR` | `,` |
| 167 | 99 | `ERROR` | `,` |
| 170 | 95 | `ERROR` | `,` |
| 182 | 115 | `ERROR` | `,` |
| 185 | 95 | `ERROR` | `,` |
| 206 | 99 | `ERROR` | `,` |
| 208 | 95 | `ERROR` | `,` |

#### `frameworks/cj/camera_picker/include/camera_picker_ffi.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 12 | `ERROR` | `void` |

#### `frameworks/cj/camera_picker/src/camera_picker_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 87 | `ERROR` | `,` |
| 91 | 177 | `ERROR` | `,` |
| 101 | 40 | `ERROR` | `,` |
| 113 | 103 | `ERROR` | `,` |
| 118 | 248 | `ERROR` | `,` |
| 125 | 107 | `ERROR` | `,` |
| 130 | 103 | `ERROR` | `,` |
| 136 | 88 | `ERROR` | `,` |
| 138 | 96 | `ERROR` | `,` |
| 143 | 92 | `ERROR` | `,` |
| 151 | 97 | `ERROR` | `,` |
| 156 | 167 | `ERROR` | `,` |
| 180 | 108 | `ERROR` | `,` |
| 182 | 95 | `ERROR` | `,` |
| 209 | 180 | `ERROR` | `,` |
| 214 | 151 | `ERROR` | `,` |
| 242 | 99 | `ERROR` | `,` |
| 250 | 96 | `ERROR` | `,` |
| 257 | 256 | `ERROR` | `,` |
| 259 | 187 | `ERROR` | `,` |
| … | … | … | *(1 more)* |

#### `frameworks/js/camera_napi/src/camera_napi_security_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 195 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/camera_napi_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 139 | 171 | `ERROR` | `,` |
| 144 | 178 | `ERROR` | `,` |
| 147 | 178 | `ERROR` | `,` |
| 149 | 169 | `ERROR` | `,` |
| 151 | 165 | `ERROR` | `,` |
| 156 | 181 | `ERROR` | `,` |
| 158 | 179 | `ERROR` | `,` |
| 162 | 166 | `ERROR` | `,` |
| 171 | 181 | `ERROR` | `,` |
| 178 | 188 | `ERROR` | `,` |
| 181 | 188 | `ERROR` | `,` |
| 185 | 179 | `ERROR` | `,` |
| 187 | 175 | `ERROR` | `,` |
| 192 | 191 | `ERROR` | `,` |
| 194 | 189 | `ERROR` | `,` |
| 197 | 176 | `ERROR` | `,` |
| 391 | 101 | `ERROR` | `,` |
| 400 | 92 | `ERROR` | `,` |
| 404 | 114 | `ERROR` | `,` |
| 411 | 109 | `ERROR` | `,` |
| … | … | … | *(23 more)* |

#### `frameworks/js/camera_napi/src/camera_napi_worker_queue_keeper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 224 | `ERROR` | `,` |
| 73 | 148 | `ERROR` | `,` |
| 110 | 223 | `ERROR` | `,` |
| 131 | 147 | `ERROR` | `, , ,` |
| 134 | 164 | `ERROR` | `, , ,` |

#### `frameworks/js/camera_napi/src/dynamic_loader/camera_napi_ex_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 123 | `ERROR` | `,` |
| 18 | 160 | `ERROR` | `, , ,` |
| 29 | 124 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/dynamic_loader/camera_napi_ex_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 146 | `ERROR` | `, , ,` |
| 18 | 99 | `ERROR` | `,` |
| 19 | 141 | `ERROR` | `, , ,` |
| 24 | 98 | `ERROR` | `,` |
| 33 | 120 | `ERROR` | `,` |
| 35 | 142 | `ERROR` | `, , ,` |
| 39 | 160 | `ERROR` | `, , ,` |
| 41 | 152 | `ERROR` | `, , ,` |
| 49 | 142 | `ERROR` | `, , ,` |
| 55 | 162 | `ERROR` | `, , ,` |
| 62 | 118 | `ERROR` | `,` |
| 64 | 142 | `ERROR` | `, , ,` |
| 67 | 182 | `ERROR` | `, , ,` |
| 69 | 150 | `ERROR` | `, , ,` |
| 76 | 122 | `ERROR` | `,` |
| 78 | 142 | `ERROR` | `, , ,` |
| 82 | 164 | `ERROR` | `, , ,` |
| 84 | 154 | `ERROR` | `, , ,` |
| 92 | 122 | `ERROR` | `,` |
| 94 | 142 | `ERROR` | `, , ,` |
| … | … | … | *(5 more)* |

#### `frameworks/js/camera_napi/src/input/camera_input_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 174 | `ERROR` | `, , ,` |
| 14 | 20 | `ERROR` | `,` |
| 47 | 102 | `ERROR` | `,` |
| 66 | 94 | `ERROR` | `,` |
| 74 | 97 | `ERROR` | `,` |
| 88 | 159 | `ERROR` | `,` |
| 95 | 115 | `ERROR` | `,` |
| 136 | 120 | `ERROR` | `,` |
| 158 | 94 | `ERROR` | `,` |
| 180 | 97 | `ERROR` | `,` |
| 185 | 106 | `ERROR` | `,` |
| 194 | 86 | `ERROR` | `,` |
| 229 | 89 | `ERROR` | `,` |
| 236 | 107 | `ERROR` | `,` |
| 254 | 106 | `ERROR` | `,` |
| 257 | 111 | `ERROR` | `,` |
| 263 | 98 | `ERROR` | `,` |
| 279 | 110 | `ERROR` | `,` |
| 283 | 102 | `ERROR` | `,` |
| 298 | 190 | `ERROR` | `,` |
| … | … | … | *(67 more)* |

#### `frameworks/js/camera_napi/src/input/camera_manager_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 153 | `ERROR` | `,` |
| 31 | 160 | `ERROR` | `,` |
| 41 | 187 | `ERROR` | `,` |
| 48 | 191 | `ERROR` | `,` |
| 56 | 195 | `ERROR` | `,` |
| 66 | 199 | `ERROR` | `,` |
| 76 | 108 | `ERROR` | `,` |
| 81 | 135 | `ERROR` | `,` |
| 88 | 160 | `ERROR` | `,` |
| 96 | 112 | `ERROR` | `,` |
| 101 | 139 | `ERROR` | `,` |
| 109 | 168 | `ERROR` | `,` |
| 138 | 176 | `ERROR` | `, , ,` |
| 140 | 20 | `ERROR` | `,` |
| 204 | 109 | `ERROR` | `,` |
| 224 | 94 | `ERROR` | `,` |
| 232 | 104 | `ERROR` | `,` |
| 233 | 149 | `ERROR` | `, , ,` |
| 244 | 34 | `ERROR` | `,` |
| 254 | 229 | `ERROR` | `,` |
| … | … | … | *(231 more)* |

#### `frameworks/js/camera_napi/src/input/camera_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 105 | `ERROR` | `,` |
| 17 | 105 | `ERROR` | `,` |
| 23 | 103 | `ERROR` | `,` |
| 41 | 128 | `ERROR` | `,` |
| 45 | 106 | `ERROR` | `,` |
| 51 | 102 | `ERROR` | `,` |
| 60 | 86 | `ERROR` | `,` |
| 219 | 89 | `ERROR` | `,` |
| 240 | 109 | `ERROR` | `,` |
| 255 | 106 | `ERROR` | `,` |
| 257 | 166 | `ERROR` | `, , ,` |
| 267 | 159 | `ERROR` | `, , ,` |
| 277 | 155 | `ERROR` | `,` |
| 288 | 162 | `ERROR` | `,` |
| 300 | 158 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/listener_base.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 95 | `ERROR` | `,` |
| 17 | 107 | `ERROR` | `,` |
| 25 | 142 | `ERROR` | `,` |
| 31 | 96 | `ERROR` | `,` |
| 60 | 145 | `ERROR` | `,` |
| 71 | 132 | `ERROR` | `,` |
| 89 | 162 | `ERROR` | `,` |
| 101 | 188 | `ERROR` | `,` |
| 107 | 189 | `ERROR` | `,` |
| 114 | 173 | `ERROR` | `, , ,` |
| 118 | 33 | `ERROR` | `,` |
| 125 | 150 | `ERROR` | `,` |
| 134 | 178 | `ERROR` | `,` |
| 139 | 171 | `ERROR` | `,` |
| 151 | 111 | `ERROR` | `,` |
| 158 | 172 | `ERROR` | `,` |
| 165 | 33 | `ERROR` | `,` |
| 174 | 125 | `ERROR` | `,` |
| 179 | 179 | `ERROR` | `, , ,` |
| 181 | 186 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `frameworks/js/camera_napi/src/mode/photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 99 | `ERROR` | `,` |
| 20 | 108 | `ERROR` | `,` |
| 28 | 86 | `ERROR` | `,` |
| 51 | 89 | `ERROR` | `,` |
| 57 | 101 | `ERROR` | `,` |
| 66 | 111 | `ERROR` | `,` |
| 73 | 117 | `ERROR` | `,` |
| 76 | 116 | `ERROR` | `,` |
| 79 | 121 | `ERROR` | `,` |
| 86 | 110 | `ERROR` | `,` |
| 98 | 145 | `ERROR` | `,` |
| 101 | 145 | `ERROR` | `,` |
| 104 | 108 | `ERROR` | `,` |
| 111 | 109 | `ERROR` | `,` |
| 123 | 95 | `ERROR` | `,` |
| 129 | 93 | `ERROR` | `,` |
| 138 | 123 | `ERROR` | `,` |
| 141 | 101 | `ERROR` | `,` |
| 148 | 127 | `ERROR` | `,` |
| 160 | 129 | `ERROR` | `,` |
| … | … | … | *(15 more)* |

#### `frameworks/js/camera_napi/src/mode/secure_camera_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 106 | `ERROR` | `,` |
| 18 | 115 | `ERROR` | `,` |
| 26 | 86 | `ERROR` | `,` |
| 51 | 89 | `ERROR` | `,` |
| 57 | 101 | `ERROR` | `,` |
| 66 | 112 | `ERROR` | `,` |
| 73 | 118 | `ERROR` | `,` |
| 76 | 117 | `ERROR` | `,` |
| 79 | 122 | `ERROR` | `,` |
| 87 | 111 | `ERROR` | `,` |
| 99 | 130 | `ERROR` | `,` |
| 102 | 130 | `ERROR` | `,` |
| 105 | 109 | `ERROR` | `,` |
| 112 | 96 | `ERROR` | `,` |
| 129 | 121 | `ERROR` | `,` |
| 137 | 94 | `ERROR` | `,` |
| 144 | 116 | `ERROR` | `,` |
| 156 | 95 | `ERROR` | `,` |
| 162 | 100 | `ERROR` | `,` |
| 171 | 130 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `frameworks/js/camera_napi/src/mode/video_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 99 | `ERROR` | `,` |
| 22 | 108 | `ERROR` | `,` |
| 30 | 86 | `ERROR` | `,` |
| 54 | 89 | `ERROR` | `,` |
| 60 | 101 | `ERROR` | `,` |
| 69 | 111 | `ERROR` | `,` |
| 76 | 117 | `ERROR` | `,` |
| 79 | 116 | `ERROR` | `,` |
| 82 | 121 | `ERROR` | `,` |
| 89 | 110 | `ERROR` | `,` |
| 101 | 145 | `ERROR` | `,` |
| 104 | 145 | `ERROR` | `,` |
| 107 | 108 | `ERROR` | `,` |
| 114 | 109 | `ERROR` | `,` |
| 126 | 95 | `ERROR` | `,` |
| 132 | 93 | `ERROR` | `,` |
| 141 | 123 | `ERROR` | `,` |
| 144 | 101 | `ERROR` | `,` |
| 151 | 127 | `ERROR` | `,` |
| 163 | 129 | `ERROR` | `,` |
| … | … | … | *(20 more)* |

#### `frameworks/js/camera_napi/src/napi_ref_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 112 | `ERROR` | `,` |
| 12 | 153 | `ERROR` | `, , ,` |
| 14 | 170 | `ERROR` | `, , ,` |
| 20 | 272 | `ERROR` | `,` |
| 37 | 116 | `ERROR` | `,` |
| 46 | 108 | `ERROR` | `,` |
| 50 | 205 | `ERROR` | `,` |
| 55 | 211 | `ERROR` | `,` |
| 55 | 245 | `ERROR` | `,` |
| 55 | 279 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/native_module_ohos_camera.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 87 | `ERROR` | `,` |
| 25 | 85 | `ERROR` | `,` |
| 41 | 102 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/native_module_ohos_camerapicker.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 87 | `ERROR` | `,` |
| 10 | 85 | `ERROR` | `,` |
| 26 | 96 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/native_module_ohos_resource_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 87 | `ERROR` | `,` |
| 11 | 85 | `ERROR` | `,` |
| 27 | 96 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/output/capture_photo_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 99 | `ERROR` | `,` |
| 24 | 109 | `ERROR` | `,` |
| 43 | 106 | `ERROR` | `,` |
| 46 | 112 | `ERROR` | `,` |
| 52 | 108 | `ERROR` | `,` |
| 61 | 86 | `ERROR` | `,` |
| 81 | 89 | `ERROR` | `,` |
| 87 | 93 | `ERROR` | `,` |
| 100 | 82 | `ERROR` | `,` |
| 108 | 107 | `ERROR` | `,` |
| 112 | 95 | `ERROR` | `,` |
| 118 | 95 | `ERROR` | `,` |
| 137 | 109 | `ERROR` | `,` |
| 141 | 97 | `ERROR` | `,` |
| 147 | 88 | `ERROR` | `,` |
| 154 | 109 | `ERROR` | `,` |
| 168 | 105 | `ERROR` | `,` |
| 172 | 109 | `ERROR` | `,` |
| 178 | 88 | `ERROR` | `,` |
| 222 | 141 | `ERROR` | `,` |
| … | … | … | *(1 more)* |

#### `frameworks/js/camera_napi/src/output/deferred_photo_proxy_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 105 | `ERROR` | `,` |
| 21 | 115 | `ERROR` | `,` |
| 39 | 106 | `ERROR` | `,` |
| 42 | 118 | `ERROR` | `,` |
| 48 | 114 | `ERROR` | `,` |
| 57 | 86 | `ERROR` | `,` |
| 79 | 89 | `ERROR` | `,` |
| 85 | 106 | `ERROR` | `,` |
| 99 | 120 | `ERROR` | `,` |
| 103 | 108 | `ERROR` | `,` |
| 109 | 93 | `ERROR` | `,` |
| 139 | 152 | `ERROR` | `,` |
| 147 | 97 | `ERROR` | `,` |
| 163 | 195 | `ERROR` | `,` |
| 176 | 88 | `ERROR` | `,` |
| 213 | 147 | `ERROR` | `,` |
| 220 | 92 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/output/metadata_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 177 | `ERROR` | `, , ,` |
| 14 | 227 | `ERROR` | `,` |
| 44 | 108 | `ERROR` | `,` |
| 57 | 94 | `ERROR` | `,` |
| 69 | 122 | `ERROR` | `,` |
| 74 | 122 | `ERROR` | `,` |
| 89 | 155 | `ERROR` | `,` |
| 100 | 116 | `ERROR` | `,` |
| 104 | 120 | `ERROR` | `,` |
| 129 | 119 | `ERROR` | `,` |
| 139 | 102 | `ERROR` | `,` |
| 222 | 202 | `ERROR` | `,` |
| 224 | 166 | `ERROR` | `, , ,` |
| 237 | 201 | `ERROR` | `,` |
| 239 | 110 | `ERROR` | `,` |
| 251 | 102 | `ERROR` | `,` |
| 267 | 94 | `ERROR` | `,` |
| 273 | 97 | `ERROR` | `,` |
| 287 | 159 | `ERROR` | `,` |
| 295 | 101 | `ERROR` | `,` |
| … | … | … | *(73 more)* |

#### `frameworks/js/camera_napi/src/output/photo_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 92 | `ERROR` | `,` |
| 23 | 102 | `ERROR` | `,` |
| 42 | 106 | `ERROR` | `,` |
| 45 | 105 | `ERROR` | `,` |
| 51 | 101 | `ERROR` | `,` |
| 60 | 86 | `ERROR` | `,` |
| 83 | 89 | `ERROR` | `,` |
| 89 | 93 | `ERROR` | `,` |
| 101 | 81 | `ERROR` | `,` |
| 106 | 82 | `ERROR` | `,` |
| 114 | 107 | `ERROR` | `,` |
| 118 | 95 | `ERROR` | `,` |
| 124 | 88 | `ERROR` | `,` |
| 140 | 98 | `ERROR` | `,` |
| 144 | 102 | `ERROR` | `,` |
| 150 | 96 | `ERROR` | `,` |
| 165 | 107 | `ERROR` | `,` |
| 169 | 98 | `ERROR` | `,` |
| 175 | 87 | `ERROR` | `,` |
| 189 | 97 | `ERROR` | `,` |
| … | … | … | *(13 more)* |

#### `frameworks/js/camera_napi/src/output/photo_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 174 | `ERROR` | `, , ,` |
| 18 | 20 | `ERROR` | `,` |
| 44 | 213 | `ERROR` | `,` |
| 57 | 208 | `ERROR` | `,` |
| 62 | 190 | `ERROR` | `,` |
| 74 | 99 | `ERROR` | `,` |
| 89 | 184 | `ERROR` | `,` |
| 98 | 119 | `ERROR` | `,` |
| 106 | 142 | `ERROR` | `,` |
| 117 | 120 | `ERROR` | `,` |
| 139 | 205 | `ERROR` | `,` |
| 150 | 165 | `ERROR` | `,` |
| 157 | 69 | `ERROR` | `,` |
| 167 | 216 | `ERROR` | `,` |
| 170 | 101 | `ERROR` | `,` |
| 184 | 117 | `ERROR` | `,` |
| 209 | 172 | `ERROR` | `,` |
| 220 | 103 | `ERROR` | `,` |
| 236 | 126 | `ERROR` | `,` |
| 239 | 134 | `ERROR` | `,` |
| … | … | … | *(274 more)* |

#### `frameworks/js/camera_napi/src/output/preview_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 101 | `ERROR` | `,` |
| 23 | 176 | `ERROR` | `, , ,` |
| 25 | 20 | `ERROR` | `,` |
| 55 | 103 | `ERROR` | `,` |
| 74 | 94 | `ERROR` | `,` |
| 83 | 95 | `ERROR` | `,` |
| 90 | 165 | `ERROR` | `,` |
| 98 | 95 | `ERROR` | `,` |
| 105 | 96 | `ERROR` | `,` |
| 112 | 158 | `ERROR` | `,` |
| 118 | 108 | `ERROR` | `,` |
| 133 | 148 | `ERROR` | `,` |
| 140 | 242 | `ERROR` | `,` |
| 164 | 107 | `ERROR` | `,` |
| 170 | 98 | `ERROR` | `,` |
| 196 | 100 | `ERROR` | `,` |
| 201 | 109 | `ERROR` | `,` |
| 210 | 86 | `ERROR` | `,` |
| 254 | 89 | `ERROR` | `,` |
| 261 | 110 | `ERROR` | `,` |
| … | … | … | *(140 more)* |

#### `frameworks/js/camera_napi/src/output/quick_thumbnail_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 89 | `ERROR` | `,` |
| 32 | 90 | `ERROR` | `,` |
| 37 | 106 | `ERROR` | `,` |
| 59 | 89 | `ERROR` | `,` |
| 65 | 131 | `ERROR` | `,` |
| 84 | 106 | `ERROR` | `,` |
| 87 | 134 | `ERROR` | `,` |
| 93 | 129 | `ERROR` | `,` |
| 102 | 121 | `ERROR` | `,` |
| 125 | 143 | `ERROR` | `,` |
| 129 | 119 | `ERROR` | `,` |
| 135 | 113 | `ERROR` | `,` |
| 154 | 116 | `ERROR` | `,` |
| 160 | 118 | `ERROR` | `,` |
| 179 | 121 | `ERROR` | `,` |
| 185 | 108 | `ERROR` | `,` |
| 219 | 143 | `ERROR` | `,` |
| 226 | 112 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/output/unify_movie_file_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 103 | `ERROR` | `,` |
| 35 | 94 | `ERROR` | `,` |
| 43 | 98 | `ERROR` | `,` |
| 45 | 172 | `ERROR` | `, , ,` |
| 77 | 118 | `ERROR` | `,` |
| 85 | 118 | `ERROR` | `,` |
| 93 | 119 | `ERROR` | `,` |
| 101 | 117 | `ERROR` | `,` |
| 109 | 131 | `ERROR` | `,` |
| 119 | 118 | `ERROR` | `,` |
| 127 | 112 | `ERROR` | `,` |
| 170 | 110 | `ERROR` | `,` |
| 176 | 134 | `ERROR` | `,` |
| 181 | 138 | `ERROR` | `,` |
| 199 | 162 | `ERROR` | `,` |
| 202 | 141 | `ERROR` | `,` |
| 208 | 137 | `ERROR` | `,` |
| 217 | 108 | `ERROR` | `,` |
| 230 | 109 | `ERROR` | `,` |
| 238 | 115 | `ERROR` | `,` |
| … | … | … | *(147 more)* |

#### `frameworks/js/camera_napi/src/output/video_capability_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 86 | `ERROR` | `,` |
| 25 | 160 | `ERROR` | `, , ,` |
| 27 | 151 | `ERROR` | `, , ,` |
| 28 | 104 | `ERROR` | `,` |
| 33 | 103 | `ERROR` | `,` |
| 47 | 104 | `ERROR` | `,` |
| 55 | 114 | `ERROR` | `,` |
| 58 | 106 | `ERROR` | `,` |
| 65 | 112 | `ERROR` | `,` |
| 85 | 106 | `ERROR` | `,` |
| 89 | 115 | `ERROR` | `,` |
| 95 | 111 | `ERROR` | `,` |
| 106 | 110 | `ERROR` | `,` |
| 109 | 120 | `ERROR` | `,` |
| 114 | 138 | `ERROR` | `,` |
| 118 | 133 | `ERROR` | `,` |
| 131 | 102 | `ERROR` | `,` |

#### `frameworks/js/camera_napi/src/output/video_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 174 | `ERROR` | `, , ,` |
| 14 | 20 | `ERROR` | `,` |
| 44 | 103 | `ERROR` | `,` |
| 63 | 94 | `ERROR` | `,` |
| 72 | 96 | `ERROR` | `,` |
| 80 | 165 | `ERROR` | `,` |
| 88 | 158 | `ERROR` | `,` |
| 96 | 112 | `ERROR` | `,` |
| 100 | 252 | `ERROR` | `,` |
| 107 | 95 | `ERROR` | `,` |
| 126 | 98 | `ERROR` | `,` |
| 137 | 124 | `ERROR` | `,` |
| 167 | 98 | `ERROR` | `,` |
| 172 | 107 | `ERROR` | `,` |
| 181 | 86 | `ERROR` | `,` |
| 224 | 89 | `ERROR` | `,` |
| 231 | 108 | `ERROR` | `,` |
| 251 | 106 | `ERROR` | `,` |
| 255 | 111 | `ERROR` | `,` |
| 266 | 95 | `ERROR` | `,` |
| … | … | … | *(118 more)* |

#### `frameworks/js/camera_napi/src/picker/camera_picker_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 95 | `ERROR` | `,` |
| 38 | 97 | `ERROR` | `,` |
| 48 | 99 | `ERROR` | `,` |
| 53 | 99 | `ERROR` | `,` |
| 57 | 110 | `ERROR` | `,` |
| 62 | 116 | `ERROR` | `,` |
| 67 | 127 | `ERROR` | `,` |
| 71 | 109 | `ERROR` | `,` |
| 73 | 104 | `ERROR` | `,` |
| 77 | 114 | `ERROR` | `,` |
| 83 | 95 | `ERROR` | `,` |
| 86 | 100 | `ERROR` | `,` |
| 92 | 95 | `ERROR` | `,` |
| 98 | 105 | `ERROR` | `,` |
| 104 | 105 | `ERROR` | `,` |
| 112 | 170 | `ERROR` | `,` |
| 118 | 98 | `ERROR` | `,` |
| 123 | 102 | `ERROR` | `,` |
| 128 | 98 | `ERROR` | `,` |
| 132 | 102 | `ERROR` | `,` |
| … | … | … | *(59 more)* |

#### `frameworks/js/camera_napi/src/resource_manager/resource_manager_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 98 | `ERROR` | `,` |
| 23 | 100 | `ERROR` | `,` |
| 33 | 102 | `ERROR` | `,` |
| 52 | 93 | `ERROR` | `,` |
| 86 | 93 | `ERROR` | `,` |
| 119 | 93 | `ERROR` | `,` |
| 161 | 155 | `ERROR` | `,` |
| 172 | 162 | `ERROR` | `,` |
| 184 | 158 | `ERROR` | `,` |
| 202 | 106 | `ERROR` | `,` |
| 239 | 89 | `ERROR` | `,` |
| 246 | 112 | `ERROR` | `,` |
| 263 | 106 | `ERROR` | `,` |
| 266 | 115 | `ERROR` | `,` |
| 272 | 111 | `ERROR` | `,` |
| 380 | 109 | `ERROR` | `,` |
| 390 | 133 | `ERROR` | `,` |
| 394 | 134 | `ERROR` | `,` |
| 400 | 130 | `ERROR` | `,` |
| 417 | 94 | `ERROR` | `,` |
| … | … | … | *(23 more)* |

#### `frameworks/js/camera_napi/src/session/camera_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 176 | `ERROR` | `, , ,` |
| 14 | 20 | `ERROR` | `,` |
| 345 | 111 | `ERROR` | `,` |
| 360 | 94 | `ERROR` | `,` |
| 368 | 106 | `ERROR` | `,` |
| 387 | 112 | `ERROR` | `,` |
| 401 | 167 | `ERROR` | `,` |
| 407 | 171 | `ERROR` | `,` |
| 425 | 110 | `ERROR` | `,` |
| 445 | 94 | `ERROR` | `,` |
| 453 | 105 | `ERROR` | `,` |
| 467 | 158 | `ERROR` | `,` |
| 473 | 107 | `ERROR` | `,` |
| 491 | 94 | `ERROR` | `,` |
| 499 | 102 | `ERROR` | `,` |
| 520 | 155 | `ERROR` | `,` |
| 526 | 119 | `ERROR` | `,` |
| 545 | 94 | `ERROR` | `,` |
| 553 | 114 | `ERROR` | `,` |
| 575 | 176 | `ERROR` | `,` |
| … | … | … | *(600 more)* |

#### `frameworks/js/camera_napi/src/session/control_center_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 183 | `ERROR` | `, , ,` |
| 11 | 233 | `ERROR` | `,` |
| 40 | 106 | `ERROR` | `,` |
| 45 | 111 | `ERROR` | `,` |
| 54 | 112 | `ERROR` | `,` |
| 66 | 102 | `ERROR` | `,` |
| 76 | 137 | `ERROR` | `,` |
| 79 | 116 | `ERROR` | `,` |
| 85 | 107 | `ERROR` | `,` |
| 120 | 165 | `ERROR` | `, , ,` |
| 122 | 156 | `ERROR` | `, , ,` |
| 123 | 109 | `ERROR` | `,` |
| 128 | 133 | `ERROR` | `,` |
| 143 | 109 | `ERROR` | `,` |
| 151 | 119 | `ERROR` | `,` |
| 154 | 166 | `ERROR` | `,` |
| 162 | 121 | `ERROR` | `,` |
| 165 | 110 | `ERROR` | `,` |
| 169 | 154 | `ERROR` | `,` |
| 177 | 102 | `ERROR` | `,` |
| … | … | … | *(102 more)* |

#### `frameworks/js/camera_napi_for_sys/src/ability/camera_ability_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 127 | 86 | `ERROR` | `,` |
| 130 | 173 | `ERROR` | `, , ,` |
| 133 | 175 | `ERROR` | `, , ,` |
| 159 | 151 | `ERROR` | `, , ,` |
| 160 | 104 | `ERROR` | `,` |
| 166 | 103 | `ERROR` | `,` |
| 195 | 119 | `ERROR` | `,` |
| 207 | 114 | `ERROR` | `,` |
| 210 | 105 | `ERROR` | `,` |
| 219 | 102 | `ERROR` | `,` |
| 230 | 112 | `ERROR` | `,` |
| 248 | 106 | `ERROR` | `,` |
| 251 | 114 | `ERROR` | `,` |
| 257 | 111 | `ERROR` | `,` |
| 279 | 150 | `ERROR` | `, , ,` |
| 288 | 150 | `ERROR` | `, , ,` |
| 297 | 150 | `ERROR` | `, , ,` |
| 305 | 101 | `ERROR` | `,` |
| 308 | 99 | `ERROR` | `,` |
| 315 | 90 | `ERROR` | `,` |
| … | … | … | *(27 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/aperture_video_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 107 | `ERROR` | `,` |
| 24 | 106 | `ERROR` | `,` |
| 33 | 86 | `ERROR` | `,` |
| 43 | 165 | `ERROR` | `, , ,` |
| 45 | 156 | `ERROR` | `, , ,` |
| 46 | 109 | `ERROR` | `,` |
| 51 | 101 | `ERROR` | `,` |
| 58 | 148 | `ERROR` | `, , ,` |
| 65 | 111 | `ERROR` | `,` |
| 72 | 117 | `ERROR` | `,` |
| 75 | 116 | `ERROR` | `,` |
| 78 | 121 | `ERROR` | `,` |
| 85 | 117 | `ERROR` | `,` |
| 96 | 164 | `ERROR` | `, , ,` |
| 100 | 170 | `ERROR` | `, , ,` |
| 107 | 142 | `ERROR` | `,` |
| 110 | 120 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/cinematic_video_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 136 | `ERROR` | `,` |
| 17 | 137 | `ERROR` | `,` |
| 25 | 113 | `ERROR` | `,` |
| 42 | 166 | `ERROR` | `, , ,` |
| 45 | 157 | `ERROR` | `, , ,` |
| 46 | 110 | `ERROR` | `,` |
| 51 | 128 | `ERROR` | `,` |
| 59 | 146 | `ERROR` | `, , ,` |
| 66 | 154 | `ERROR` | `,` |
| 72 | 121 | `ERROR` | `,` |
| 75 | 154 | `ERROR` | `,` |
| 78 | 150 | `ERROR` | `,` |
| 84 | 118 | `ERROR` | `,` |
| 96 | 101 | `ERROR` | `,` |
| 102 | 172 | `ERROR` | `, , ,` |
| 109 | 142 | `ERROR` | `,` |
| 112 | 121 | `ERROR` | `,` |
| 119 | 117 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/fluorescence_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 111 | `ERROR` | `,` |
| 23 | 120 | `ERROR` | `,` |
| 32 | 86 | `ERROR` | `,` |
| 44 | 169 | `ERROR` | `, , ,` |
| 46 | 160 | `ERROR` | `, , ,` |
| 47 | 113 | `ERROR` | `,` |
| 52 | 101 | `ERROR` | `,` |
| 59 | 148 | `ERROR` | `, , ,` |
| 66 | 124 | `ERROR` | `,` |
| 73 | 130 | `ERROR` | `,` |
| 76 | 129 | `ERROR` | `,` |
| 79 | 118 | `ERROR` | `,` |
| 81 | 134 | `ERROR` | `,` |
| 88 | 121 | `ERROR` | `,` |
| 99 | 164 | `ERROR` | `, , ,` |
| 103 | 178 | `ERROR` | `, , ,` |
| 110 | 135 | `ERROR` | `,` |
| 113 | 113 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/high_res_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 106 | `ERROR` | `,` |
| 20 | 115 | `ERROR` | `,` |
| 29 | 86 | `ERROR` | `,` |
| 41 | 164 | `ERROR` | `, , ,` |
| 43 | 155 | `ERROR` | `, , ,` |
| 44 | 108 | `ERROR` | `,` |
| 49 | 101 | `ERROR` | `,` |
| 56 | 148 | `ERROR` | `, , ,` |
| 63 | 120 | `ERROR` | `,` |
| 70 | 126 | `ERROR` | `,` |
| 73 | 125 | `ERROR` | `,` |
| 76 | 118 | `ERROR` | `,` |
| 78 | 130 | `ERROR` | `,` |
| 85 | 116 | `ERROR` | `,` |
| 96 | 164 | `ERROR` | `, , ,` |
| 100 | 168 | `ERROR` | `, , ,` |
| 107 | 130 | `ERROR` | `,` |
| 110 | 108 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/light_painting_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 106 | `ERROR` | `,` |
| 26 | 115 | `ERROR` | `,` |
| 35 | 86 | `ERROR` | `,` |
| 58 | 165 | `ERROR` | `, , ,` |
| 60 | 156 | `ERROR` | `, , ,` |
| 61 | 109 | `ERROR` | `,` |
| 66 | 100 | `ERROR` | `,` |
| 73 | 148 | `ERROR` | `, , ,` |
| 80 | 120 | `ERROR` | `,` |
| 87 | 126 | `ERROR` | `,` |
| 90 | 125 | `ERROR` | `,` |
| 93 | 118 | `ERROR` | `,` |
| 95 | 130 | `ERROR` | `,` |
| 102 | 116 | `ERROR` | `,` |
| 113 | 164 | `ERROR` | `, , ,` |
| 117 | 170 | `ERROR` | `, , ,` |
| 124 | 131 | `ERROR` | `,` |
| 127 | 109 | `ERROR` | `,` |
| 133 | 107 | `ERROR` | `,` |
| 140 | 126 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/macro_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 104 | `ERROR` | `,` |
| 20 | 113 | `ERROR` | `,` |
| 29 | 86 | `ERROR` | `,` |
| 45 | 162 | `ERROR` | `, , ,` |
| 47 | 153 | `ERROR` | `, , ,` |
| 48 | 106 | `ERROR` | `,` |
| 53 | 101 | `ERROR` | `,` |
| 60 | 148 | `ERROR` | `, , ,` |
| 67 | 111 | `ERROR` | `,` |
| 74 | 117 | `ERROR` | `,` |
| 77 | 116 | `ERROR` | `,` |
| 80 | 121 | `ERROR` | `,` |
| 87 | 114 | `ERROR` | `,` |
| 98 | 164 | `ERROR` | `, , ,` |
| 102 | 164 | `ERROR` | `, , ,` |
| 109 | 139 | `ERROR` | `,` |
| 112 | 117 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/macro_video_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 104 | `ERROR` | `,` |
| 20 | 113 | `ERROR` | `,` |
| 29 | 86 | `ERROR` | `,` |
| 45 | 162 | `ERROR` | `, , ,` |
| 47 | 153 | `ERROR` | `, , ,` |
| 48 | 106 | `ERROR` | `,` |
| 53 | 101 | `ERROR` | `,` |
| 60 | 148 | `ERROR` | `, , ,` |
| 67 | 111 | `ERROR` | `,` |
| 74 | 117 | `ERROR` | `,` |
| 77 | 116 | `ERROR` | `,` |
| 80 | 121 | `ERROR` | `,` |
| 87 | 114 | `ERROR` | `,` |
| 98 | 164 | `ERROR` | `, , ,` |
| 102 | 164 | `ERROR` | `, , ,` |
| 109 | 139 | `ERROR` | `,` |
| 112 | 117 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/mode_manager_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 114 | `ERROR` | `,` |
| 30 | 98 | `ERROR` | `,` |
| 36 | 108 | `ERROR` | `,` |
| 49 | 130 | `ERROR` | `,` |
| 58 | 106 | `ERROR` | `,` |
| 61 | 111 | `ERROR` | `,` |
| 67 | 107 | `ERROR` | `,` |
| 76 | 86 | `ERROR` | `,` |
| 89 | 156 | `ERROR` | `, , ,` |
| 91 | 147 | `ERROR` | `, , ,` |
| 92 | 100 | `ERROR` | `,` |
| 97 | 98 | `ERROR` | `,` |
| 104 | 148 | `ERROR` | `, , ,` |
| 112 | 106 | `ERROR` | `,` |
| 116 | 102 | `ERROR` | `,` |
| 122 | 108 | `ERROR` | `,` |
| 136 | 92 | `ERROR` | `,` |
| 142 | 180 | `ERROR` | `,` |
| 163 | 195 | `ERROR` | `,` |
| 172 | 104 | `ERROR` | `,` |
| … | … | … | *(14 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/night_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 99 | `ERROR` | `,` |
| 20 | 108 | `ERROR` | `,` |
| 28 | 86 | `ERROR` | `,` |
| 59 | 157 | `ERROR` | `, , ,` |
| 61 | 148 | `ERROR` | `, , ,` |
| 62 | 101 | `ERROR` | `,` |
| 67 | 101 | `ERROR` | `,` |
| 74 | 148 | `ERROR` | `, , ,` |
| 81 | 112 | `ERROR` | `,` |
| 88 | 118 | `ERROR` | `,` |
| 91 | 117 | `ERROR` | `,` |
| 94 | 122 | `ERROR` | `,` |
| 101 | 107 | `ERROR` | `,` |
| 124 | 165 | `ERROR` | `,` |
| 129 | 178 | `ERROR` | `,` |
| 131 | 105 | `ERROR` | `,` |
| 138 | 93 | `ERROR` | `,` |
| 156 | 157 | `ERROR` | `,` |
| 159 | 96 | `ERROR` | `,` |
| 166 | 93 | `ERROR` | `,` |
| … | … | … | *(21 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/panorama_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 102 | `ERROR` | `,` |
| 27 | 111 | `ERROR` | `,` |
| 36 | 86 | `ERROR` | `,` |
| 50 | 160 | `ERROR` | `, , ,` |
| 52 | 151 | `ERROR` | `, , ,` |
| 53 | 104 | `ERROR` | `,` |
| 58 | 101 | `ERROR` | `,` |
| 65 | 148 | `ERROR` | `, , ,` |
| 72 | 111 | `ERROR` | `,` |
| 79 | 117 | `ERROR` | `,` |
| 82 | 116 | `ERROR` | `,` |
| 85 | 121 | `ERROR` | `,` |
| 92 | 112 | `ERROR` | `,` |
| 103 | 164 | `ERROR` | `, , ,` |
| 107 | 160 | `ERROR` | `, , ,` |
| 114 | 126 | `ERROR` | `,` |
| 117 | 104 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/photo_session_for_sys_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 105 | `ERROR` | `,` |
| 20 | 114 | `ERROR` | `,` |
| 34 | 86 | `ERROR` | `,` |
| 52 | 163 | `ERROR` | `, , ,` |
| 54 | 154 | `ERROR` | `, , ,` |
| 55 | 107 | `ERROR` | `,` |
| 60 | 101 | `ERROR` | `,` |
| 67 | 148 | `ERROR` | `, , ,` |
| 74 | 111 | `ERROR` | `,` |
| 81 | 117 | `ERROR` | `,` |
| 84 | 116 | `ERROR` | `,` |
| 87 | 121 | `ERROR` | `,` |
| 94 | 115 | `ERROR` | `,` |
| 105 | 164 | `ERROR` | `, , ,` |
| 109 | 166 | `ERROR` | `, , ,` |
| 116 | 129 | `ERROR` | `,` |
| 119 | 107 | `ERROR` | `,` |
| 126 | 133 | `ERROR` | `,` |
| 138 | 135 | `ERROR` | `,` |
| 139 | 155 | `ERROR` | `, , ,` |
| … | … | … | *(10 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/portrait_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 102 | `ERROR` | `,` |
| 19 | 111 | `ERROR` | `,` |
| 27 | 86 | `ERROR` | `,` |
| 47 | 160 | `ERROR` | `, , ,` |
| 49 | 151 | `ERROR` | `, , ,` |
| 50 | 104 | `ERROR` | `,` |
| 55 | 101 | `ERROR` | `,` |
| 62 | 148 | `ERROR` | `, , ,` |
| 69 | 114 | `ERROR` | `,` |
| 76 | 120 | `ERROR` | `,` |
| 79 | 119 | `ERROR` | `,` |
| 82 | 124 | `ERROR` | `,` |
| 89 | 112 | `ERROR` | `,` |
| 100 | 164 | `ERROR` | `, , ,` |
| 104 | 160 | `ERROR` | `, , ,` |
| 111 | 126 | `ERROR` | `,` |
| 114 | 104 | `ERROR` | `,` |
| 121 | 99 | `ERROR` | `,` |
| 133 | 102 | `ERROR` | `,` |
| 141 | 197 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/profession_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 104 | `ERROR` | `,` |
| 19 | 113 | `ERROR` | `,` |
| 72 | 86 | `ERROR` | `,` |
| 98 | 162 | `ERROR` | `, , ,` |
| 100 | 153 | `ERROR` | `, , ,` |
| 101 | 106 | `ERROR` | `,` |
| 106 | 101 | `ERROR` | `,` |
| 113 | 148 | `ERROR` | `, , ,` |
| 120 | 116 | `ERROR` | `,` |
| 127 | 122 | `ERROR` | `,` |
| 130 | 121 | `ERROR` | `,` |
| 133 | 126 | `ERROR` | `,` |
| 140 | 114 | `ERROR` | `,` |
| 151 | 164 | `ERROR` | `, , ,` |
| 155 | 164 | `ERROR` | `, , ,` |
| 162 | 128 | `ERROR` | `,` |
| 165 | 106 | `ERROR` | `,` |
| 171 | 107 | `ERROR` | `,` |
| 183 | 102 | `ERROR` | `,` |
| 194 | 196 | `ERROR` | `,` |
| … | … | … | *(63 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/quick_shot_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 108 | `ERROR` | `,` |
| 24 | 117 | `ERROR` | `,` |
| 33 | 86 | `ERROR` | `,` |
| 48 | 166 | `ERROR` | `, , ,` |
| 50 | 157 | `ERROR` | `, , ,` |
| 51 | 110 | `ERROR` | `,` |
| 56 | 101 | `ERROR` | `,` |
| 63 | 148 | `ERROR` | `, , ,` |
| 70 | 111 | `ERROR` | `,` |
| 77 | 117 | `ERROR` | `,` |
| 80 | 116 | `ERROR` | `,` |
| 83 | 121 | `ERROR` | `,` |
| 90 | 118 | `ERROR` | `,` |
| 101 | 164 | `ERROR` | `, , ,` |
| 105 | 172 | `ERROR` | `, , ,` |
| 112 | 143 | `ERROR` | `,` |
| 115 | 121 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/secure_session_for_sys_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 106 | `ERROR` | `,` |
| 18 | 115 | `ERROR` | `,` |
| 26 | 86 | `ERROR` | `,` |
| 44 | 164 | `ERROR` | `, , ,` |
| 46 | 155 | `ERROR` | `, , ,` |
| 47 | 108 | `ERROR` | `,` |
| 52 | 101 | `ERROR` | `,` |
| 59 | 148 | `ERROR` | `, , ,` |
| 66 | 112 | `ERROR` | `,` |
| 73 | 118 | `ERROR` | `,` |
| 76 | 117 | `ERROR` | `,` |
| 79 | 122 | `ERROR` | `,` |
| 86 | 116 | `ERROR` | `,` |
| 97 | 164 | `ERROR` | `, , ,` |
| 101 | 180 | `ERROR` | `, , ,` |
| 108 | 130 | `ERROR` | `,` |
| 111 | 108 | `ERROR` | `,` |
| 117 | 96 | `ERROR` | `,` |
| 132 | 121 | `ERROR` | `,` |
| 138 | 94 | `ERROR` | `,` |

#### `frameworks/js/camera_napi_for_sys/src/mode/slow_motion_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 106 | `ERROR` | `,` |
| 33 | 94 | `ERROR` | `,` |
| 41 | 162 | `ERROR` | `,` |
| 57 | 113 | `ERROR` | `,` |
| 73 | 94 | `ERROR` | `,` |
| 81 | 108 | `ERROR` | `,` |
| 107 | 104 | `ERROR` | `,` |
| 112 | 113 | `ERROR` | `,` |
| 121 | 86 | `ERROR` | `,` |
| 138 | 162 | `ERROR` | `, , ,` |
| 140 | 153 | `ERROR` | `, , ,` |
| 141 | 106 | `ERROR` | `,` |
| 146 | 101 | `ERROR` | `,` |
| 153 | 148 | `ERROR` | `, , ,` |
| 160 | 116 | `ERROR` | `,` |
| 167 | 123 | `ERROR` | `,` |
| 170 | 122 | `ERROR` | `,` |
| 173 | 127 | `ERROR` | `,` |
| 180 | 114 | `ERROR` | `,` |
| 191 | 164 | `ERROR` | `, , ,` |
| … | … | … | *(16 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/stitching_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 108 | `ERROR` | `,` |
| 27 | 117 | `ERROR` | `,` |
| 36 | 86 | `ERROR` | `,` |
| 51 | 166 | `ERROR` | `, , ,` |
| 53 | 157 | `ERROR` | `, , ,` |
| 54 | 110 | `ERROR` | `,` |
| 59 | 101 | `ERROR` | `,` |
| 66 | 148 | `ERROR` | `, , ,` |
| 73 | 111 | `ERROR` | `,` |
| 80 | 117 | `ERROR` | `,` |
| 83 | 116 | `ERROR` | `,` |
| 86 | 121 | `ERROR` | `,` |
| 93 | 118 | `ERROR` | `,` |
| 104 | 164 | `ERROR` | `, , ,` |
| 108 | 172 | `ERROR` | `, , ,` |
| 115 | 143 | `ERROR` | `,` |
| 118 | 121 | `ERROR` | `,` |
| 124 | 143 | `ERROR` | `,` |
| 129 | 166 | `ERROR` | `,` |
| 133 | 161 | `ERROR` | `,` |
| … | … | … | *(37 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/time_lapse_photo_session_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 138 | `ERROR` | `,` |
| 44 | 138 | `ERROR` | `,` |
| 59 | 154 | `ERROR` | `, , ,` |
| 61 | 145 | `ERROR` | `, , ,` |
| 62 | 98 | `ERROR` | `,` |
| 71 | 148 | `ERROR` | `, , ,` |
| 81 | 138 | `ERROR` | `,` |
| 87 | 158 | `ERROR` | `,` |
| 98 | 138 | `ERROR` | `,` |
| 104 | 158 | `ERROR` | `,` |
| 115 | 138 | `ERROR` | `,` |
| 121 | 158 | `ERROR` | `,` |
| 132 | 138 | `ERROR` | `,` |
| 138 | 158 | `ERROR` | `,` |
| 149 | 138 | `ERROR` | `,` |
| 162 | 106 | `ERROR` | `,` |
| 165 | 139 | `ERROR` | `,` |
| 171 | 138 | `ERROR` | `,` |
| 182 | 138 | `ERROR` | `,` |
| 223 | 138 | `ERROR` | `,` |
| … | … | … | *(133 more)* |

#### `frameworks/js/camera_napi_for_sys/src/mode/video_session_for_sys_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 105 | `ERROR` | `,` |
| 22 | 114 | `ERROR` | `,` |
| 36 | 86 | `ERROR` | `,` |
| 55 | 163 | `ERROR` | `, , ,` |
| 57 | 154 | `ERROR` | `, , ,` |
| 58 | 107 | `ERROR` | `,` |
| 63 | 101 | `ERROR` | `,` |
| 70 | 148 | `ERROR` | `, , ,` |
| 77 | 111 | `ERROR` | `,` |
| 84 | 117 | `ERROR` | `,` |
| 87 | 116 | `ERROR` | `,` |
| 90 | 121 | `ERROR` | `,` |
| 97 | 115 | `ERROR` | `,` |
| 108 | 164 | `ERROR` | `, , ,` |
| 112 | 166 | `ERROR` | `, , ,` |
| 119 | 129 | `ERROR` | `,` |
| 122 | 107 | `ERROR` | `,` |
| 129 | 141 | `ERROR` | `,` |
| 130 | 219 | `ERROR` | `,` |
| 130 | 236 | `ERROR` | `,` |
| … | … | … | *(40 more)* |

#### `frameworks/js/camera_napi_for_sys/src/output/depth_data_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 92 | `ERROR` | `,` |
| 40 | 106 | `ERROR` | `,` |
| 61 | 106 | `ERROR` | `,` |
| 64 | 109 | `ERROR` | `,` |
| 70 | 105 | `ERROR` | `,` |
| 79 | 86 | `ERROR` | `,` |
| 95 | 154 | `ERROR` | `, , ,` |
| 97 | 145 | `ERROR` | `, , ,` |
| 98 | 98 | `ERROR` | `,` |
| 104 | 97 | `ERROR` | `,` |
| 113 | 148 | `ERROR` | `, , ,` |
| 129 | 111 | `ERROR` | `,` |
| 133 | 99 | `ERROR` | `,` |
| 139 | 91 | `ERROR` | `,` |
| 156 | 108 | `ERROR` | `,` |
| 162 | 93 | `ERROR` | `,` |
| 179 | 110 | `ERROR` | `,` |
| 185 | 96 | `ERROR` | `,` |
| 202 | 114 | `ERROR` | `,` |
| 208 | 92 | `ERROR` | `,` |
| … | … | … | *(4 more)* |

#### `frameworks/js/camera_napi_for_sys/src/output/depth_data_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 117 | `ERROR` | `,` |
| 31 | 120 | `ERROR` | `,` |
| 39 | 87 | `ERROR` | `,` |
| 40 | 165 | `ERROR` | `, , ,` |
| 51 | 212 | `ERROR` | `,` |
| 57 | 112 | `ERROR` | `,` |
| 92 | 112 | `ERROR` | `,` |
| 100 | 122 | `ERROR` | `,` |
| 109 | 117 | `ERROR` | `,` |
| 113 | 138 | `ERROR` | `,` |
| 118 | 137 | `ERROR` | `,` |
| 127 | 149 | `ERROR` | `,` |
| 137 | 121 | `ERROR` | `,` |
| 146 | 140 | `ERROR` | `,` |
| 155 | 188 | `ERROR` | `,` |
| 161 | 190 | `ERROR` | `,` |
| 175 | 167 | `ERROR` | `,` |
| 181 | 103 | `ERROR` | `,` |
| 185 | 96 | `ERROR` | `,` |
| 190 | 95 | `ERROR` | `,` |
| … | … | … | *(37 more)* |

#### `frameworks/js/camera_napi_for_sys/src/output/movie_file_output_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 107 | `ERROR` | `,` |
| 41 | 160 | `ERROR` | `, , ,` |
| 44 | 151 | `ERROR` | `, , ,` |
| 45 | 104 | `ERROR` | `,` |
| 55 | 98 | `ERROR` | `,` |
| 63 | 98 | `ERROR` | `,` |
| 71 | 99 | `ERROR` | `,` |
| 79 | 97 | `ERROR` | `,` |
| 87 | 136 | `ERROR` | `,` |
| 96 | 89 | `ERROR` | `,` |
| 105 | 136 | `ERROR` | `,` |
| 125 | 94 | `ERROR` | `,` |
| 134 | 98 | `ERROR` | `,` |
| 155 | 129 | `ERROR` | `,` |
| 224 | 97 | `ERROR` | `,` |
| 239 | 124 | `ERROR` | `,` |
| 244 | 112 | `ERROR` | `,` |
| 264 | 136 | `ERROR` | `,` |
| 268 | 115 | `ERROR` | `,` |
| 275 | 111 | `ERROR` | `,` |
| … | … | … | *(84 more)* |

#### `frameworks/js/camera_napi_for_sys/src/output/output_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 98 | `ERROR` | `,` |
| 12 | 170 | `ERROR` | `, , ,` |
| 15 | 98 | `ERROR` | `,` |
| 18 | 170 | `ERROR` | `, , ,` |

#### `frameworks/js/camera_napi_for_sys/src/session/camera_session_for_sys_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 131 | 120 | `ERROR` | `,` |
| 136 | 90 | `ERROR` | `,` |
| 149 | 113 | `ERROR` | `,` |
| 156 | 108 | `ERROR` | `,` |
| 169 | 261 | `ERROR` | `, , ,` |
| 174 | 90 | `ERROR` | `,` |
| 181 | 136 | `ERROR` | `, , ,` |
| 185 | 147 | `ERROR` | `, , ,` |
| 202 | 91 | `ERROR` | `,` |
| 213 | 94 | `ERROR` | `,` |
| 227 | 99 | `ERROR` | `,` |
| 298 | 100 | `ERROR` | `,` |
| 313 | 176 | `ERROR` | `, , ,` |
| 322 | 175 | `ERROR` | `,` |
| 329 | 108 | `ERROR` | `,` |
| 342 | 176 | `ERROR` | `, , ,` |
| 351 | 183 | `ERROR` | `,` |
| 359 | 104 | `ERROR` | `,` |
| 364 | 148 | `ERROR` | `, , ,` |
| 367 | 144 | `ERROR` | `, , ,` |
| … | … | … | *(169 more)* |

#### `frameworks/native/camera/base/src/ability/camera_ability.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 190 | 163 | `ERROR` | `,` |
| 211 | 191 | `ERROR` | `,` |
| 216 | 170 | `ERROR` | `,` |
| 217 | 183 | `ERROR` | `,` |
| 218 | 155 | `ERROR` | `,` |
| 231 | 110 | `ERROR` | `,` |
| 306 | 183 | `ERROR` | `,` |
| 318 | 187 | `ERROR` | `,` |
| 344 | 106 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/ability/camera_ability_builder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 110 | `ERROR` | `,` |
| 18 | 149 | `ERROR` | `, , ,` |
| 35 | 117 | `ERROR` | `,` |
| 38 | 157 | `ERROR` | `, , ,` |
| 120 | 153 | `ERROR` | `, , ,` |
| 192 | 219 | `ERROR` | `,` |
| 192 | 229 | `ERROR` | `,` |
| 192 | 239 | `ERROR` | `,` |
| 193 | 8 | `missing field_identifier` | `` |

#### `frameworks/native/camera/base/src/ability/camera_ability_parse_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 165 | `ERROR` | `, , ,` |
| 42 | 188 | `ERROR` | `, , ,` |
| 45 | 165 | `ERROR` | `, , ,` |
| 72 | 187 | `ERROR` | `, , ,` |
| 75 | 165 | `ERROR` | `, , ,` |
| 101 | 165 | `ERROR` | `, , ,` |
| 129 | 226 | `ERROR` | `,` |
| 129 | 236 | `ERROR` | `,` |
| 129 | 246 | `ERROR` | `,` |
| 135 | 89 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/deferred_proc_session/deferred_photo_proc_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 144 | `ERROR` | `,` |
| 26 | 159 | `ERROR` | `,` |
| 37 | 199 | `ERROR` | `,` |
| 42 | 148 | `ERROR` | `,` |
| 51 | 199 | `ERROR` | `,` |
| 56 | 155 | `ERROR` | `,` |
| 69 | 90 | `ERROR` | `,` |
| 74 | 159 | `ERROR` | `,` |
| 86 | 143 | `ERROR` | `, , ,` |
| 104 | 143 | `ERROR` | `, , ,` |
| 124 | 135 | `ERROR` | `,` |
| 125 | 281 | `ERROR` | `, , ,` |
| 141 | 161 | `ERROR` | `,` |
| 158 | 135 | `ERROR` | `,` |
| 177 | 105 | `ERROR` | `,` |
| 180 | 135 | `ERROR` | `, , ,` |
| 182 | 145 | `ERROR` | `, , ,` |
| 183 | 125 | `ERROR` | `,` |
| 185 | 125 | `ERROR` | `,` |
| 187 | 162 | `ERROR` | `,` |
| … | … | … | *(50 more)* |

#### `frameworks/native/camera/base/src/deferred_proc_session/deferred_video_proc_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 142 | `ERROR` | `,` |
| 14 | 159 | `ERROR` | `,` |
| 22 | 199 | `ERROR` | `,` |
| 27 | 148 | `ERROR` | `,` |
| 34 | 199 | `ERROR` | `,` |
| 39 | 155 | `ERROR` | `,` |
| 46 | 211 | `ERROR` | `,` |
| 52 | 161 | `ERROR` | `,` |
| 60 | 77 | `ERROR` | `,` |
| 67 | 133 | `ERROR` | `,` |
| 75 | 171 | `ERROR` | `, , ,` |
| 76 | 121 | `ERROR` | `,` |
| 78 | 201 | `ERROR` | `,` |
| 78 | 209 | `ERROR` | `,` |
| 78 | 217 | `ERROR` | `,` |
| 83 | 169 | `ERROR` | `, , ,` |
| 84 | 120 | `ERROR` | `,` |
| 86 | 201 | `ERROR` | `,` |
| 86 | 209 | `ERROR` | `,` |
| 86 | 217 | `ERROR` | `,` |
| … | … | … | *(51 more)* |

#### `frameworks/native/camera/base/src/input/camera_device.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 81 | 38 | `ERROR` | `,` |
| 95 | 38 | `ERROR` | `,` |
| 170 | 154 | `ERROR` | `,` |
| 201 | 155 | `ERROR` | `,` |
| 227 | 130 | `ERROR` | `,` |
| 370 | 187 | `ERROR` | `, , ,` |
| 375 | 174 | `ERROR` | `, , ,` |
| 380 | 191 | `ERROR` | `, , ,` |
| 385 | 226 | `ERROR` | `,` |
| 405 | 162 | `ERROR` | `, , ,` |
| 434 | 167 | `ERROR` | `, , ,` |
| 453 | 198 | `ERROR` | `,` |
| 534 | 225 | `ERROR` | `,` |
| 537 | 148 | `ERROR` | `,` |
| 562 | 197 | `ERROR` | `,` |
| 610 | 165 | `ERROR` | `, , ,` |
| 613 | 200 | `ERROR` | `,` |
| 613 | 208 | `ERROR` | `,` |
| 613 | 216 | `ERROR` | `,` |
| 615 | 206 | `ERROR` | `,` |
| … | … | … | *(17 more)* |

#### `frameworks/native/camera/base/src/input/camera_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 69 | 244 | `ERROR` | `,` |
| 107 | 218 | `ERROR` | `,` |
| 107 | 226 | `ERROR` | `,` |
| 107 | 234 | `ERROR` | `,` |
| 109 | 216 | `ERROR` | `,` |
| 109 | 232 | `ERROR` | `,` |
| 109 | 248 | `ERROR` | `,` |
| 113 | 204 | `ERROR` | `,` |
| 119 | 224 | `ERROR` | `,` |
| 119 | 232 | `ERROR` | `,` |
| 119 | 240 | `ERROR` | `,` |
| 120 | 216 | `ERROR` | `,` |
| 120 | 232 | `ERROR` | `,` |
| 120 | 248 | `ERROR` | `,` |
| 121 | 210 | `ERROR` | `,` |
| 141 | 200 | `ERROR` | `,` |
| 141 | 208 | `ERROR` | `,` |
| 141 | 216 | `ERROR` | `,` |
| 143 | 224 | `ERROR` | `,` |
| 143 | 240 | `ERROR` | `,` |
| … | … | … | *(17 more)* |

#### `frameworks/native/camera/base/src/input/camera_input.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 224 | `ERROR` | `,` |
| 19 | 143 | `ERROR` | `,` |
| 27 | 154 | `ERROR` | `, , ,` |
| 31 | 158 | `ERROR` | `, , ,` |
| 34 | 153 | `ERROR` | `,` |
| 62 | 249 | `ERROR` | `,` |
| 75 | 107 | `ERROR` | `,` |
| 83 | 160 | `ERROR` | `, , ,` |
| 86 | 182 | `ERROR` | `, , ,` |
| 88 | 169 | `ERROR` | `, , ,` |
| 97 | 194 | `ERROR` | `,` |
| 101 | 177 | `ERROR` | `, , ,` |
| 102 | 162 | `ERROR` | `, , ,` |
| 108 | 164 | `ERROR` | `, , ,` |
| 117 | 169 | `ERROR` | `, , ,` |
| 125 | 180 | `ERROR` | `, , ,` |
| 126 | 178 | `ERROR` | `, , ,` |
| 132 | 186 | `ERROR` | `, , ,` |
| 137 | 183 | `ERROR` | `, , ,` |
| 142 | 242 | `ERROR` | `,` |
| … | … | … | *(90 more)* |

#### `frameworks/native/camera/base/src/input/camera_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 101 | 195 | `ERROR` | `,` |
| 104 | 166 | `ERROR` | `,` |
| 114 | 195 | `ERROR` | `,` |
| 117 | 162 | `ERROR` | `,` |
| 123 | 115 | `ERROR` | `,` |
| 135 | 109 | `ERROR` | `,` |
| 142 | 123 | `ERROR` | `,` |
| 144 | 165 | `ERROR` | `, , ,` |
| 146 | 148 | `ERROR` | `, , ,` |
| 148 | 146 | `ERROR` | `, , ,` |
| 155 | 196 | `ERROR` | `,` |
| 157 | 173 | `ERROR` | `, , ,` |
| 187 | 200 | `ERROR` | `,` |
| 201 | 174 | `ERROR` | `,` |
| 203 | 177 | `ERROR` | `, , ,` |
| 205 | 200 | `ERROR` | `,` |
| 218 | 171 | `ERROR` | `, , ,` |
| 228 | 176 | `ERROR` | `, , ,` |
| 229 | 166 | `ERROR` | `, , ,` |
| 232 | 261 | `ERROR` | `,` |
| … | … | … | *(297 more)* |

#### `frameworks/native/camera/base/src/input/camera_service_system_ability_listener.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 156 | `ERROR` | `,` |
| 16 | 159 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/camera_output_capability.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 76 | 281 | `ERROR` | `,` |
| 119 | 166 | `ERROR` | `, , ,` |
| 131 | 124 | `ERROR` | `,` |
| 138 | 162 | `ERROR` | `, , ,` |
| 150 | 120 | `ERROR` | `,` |
| 157 | 162 | `ERROR` | `, , ,` |
| 176 | 120 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/camera_photo_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 95 | `ERROR` | `,` |
| 31 | 87 | `ERROR` | `,` |
| 46 | 203 | `ERROR` | `,` |
| 53 | 87 | `ERROR` | `,` |
| 68 | 203 | `ERROR` | `,` |
| 75 | 88 | `ERROR` | `,` |
| 96 | 97 | `ERROR` | `,` |
| 101 | 100 | `ERROR` | `,` |
| 103 | 169 | `ERROR` | `, , ,` |
| 137 | 163 | `ERROR` | `,` |
| 139 | 162 | `ERROR` | `, , ,` |
| 141 | 119 | `ERROR` | `,` |
| 143 | 96 | `ERROR` | `,` |
| 155 | 158 | `ERROR` | `, , ,` |
| 181 | 177 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/capture_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 164 | `ERROR` | `, , ,` |
| 37 | 140 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/base/src/output/deferred_photo_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 89 | `ERROR` | `,` |
| 35 | 215 | `ERROR` | `,` |
| 42 | 89 | `ERROR` | `,` |
| 52 | 264 | `ERROR` | `,` |
| 59 | 89 | `ERROR` | `,` |
| 69 | 215 | `ERROR` | `,` |
| 76 | 90 | `ERROR` | `,` |
| 95 | 105 | `ERROR` | `,` |
| 106 | 104 | `ERROR` | `,` |
| 111 | 177 | `ERROR` | `,` |
| 118 | 110 | `ERROR` | `,` |
| 125 | 106 | `ERROR` | `,` |
| 127 | 170 | `ERROR` | `, , ,` |
| 130 | 111 | `ERROR` | `,` |
| 133 | 151 | `ERROR` | `, , ,` |
| 136 | 119 | `ERROR` | `,` |
| 153 | 102 | `ERROR` | `,` |
| 155 | 158 | `ERROR` | `, , ,` |
| 157 | 158 | `ERROR` | `, , ,` |
| 174 | 103 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/metadata_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 70 | 118 | `ERROR` | `,` |
| 89 | 120 | `ERROR` | `,` |
| 158 | 105 | `ERROR` | `,` |
| 191 | 177 | `ERROR` | `,` |
| 265 | 123 | `ERROR` | `,` |
| 268 | 199 | `ERROR` | `,` |
| 299 | 123 | `ERROR` | `,` |
| 302 | 199 | `ERROR` | `,` |
| 331 | 103 | `ERROR` | `,` |
| 337 | 147 | `ERROR` | `, , ,` |
| 339 | 221 | `ERROR` | `,` |
| 339 | 233 | `ERROR` | `,` |
| 339 | 245 | `ERROR` | `,` |
| 347 | 151 | `ERROR` | `,` |
| 350 | 95 | `ERROR` | `,` |
| 355 | 104 | `ERROR` | `,` |
| 365 | 92 | `ERROR` | `,` |
| 368 | 146 | `ERROR` | `, , ,` |
| 370 | 220 | `ERROR` | `,` |
| 370 | 232 | `ERROR` | `,` |
| … | … | … | *(19 more)* |

#### `frameworks/native/camera/base/src/output/movie_file_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 116 | `ERROR` | `,` |
| 17 | 117 | `ERROR` | `,` |
| 28 | 105 | `ERROR` | `,` |
| 35 | 159 | `ERROR` | `, , ,` |
| 37 | 224 | `ERROR` | `,` |
| 37 | 236 | `ERROR` | `,` |
| 37 | 248 | `ERROR` | `,` |
| 46 | 151 | `ERROR` | `,` |
| 49 | 95 | `ERROR` | `,` |
| 57 | 116 | `ERROR` | `,` |
| 60 | 145 | `ERROR` | `, , ,` |
| 62 | 218 | `ERROR` | `,` |
| 62 | 230 | `ERROR` | `,` |
| 62 | 242 | `ERROR` | `,` |
| 64 | 93 | `ERROR` | `,` |
| 75 | 126 | `ERROR` | `,` |
| 83 | 126 | `ERROR` | `,` |
| 87 | 145 | `ERROR` | `, , ,` |
| 94 | 104 | `ERROR` | `,` |
| 95 | 157 | `ERROR` | `, , ,` |
| … | … | … | *(62 more)* |

#### `frameworks/native/camera/base/src/output/movie_file_output_mock.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 116 | `ERROR` | `,` |
| 15 | 117 | `ERROR` | `,` |
| 25 | 105 | `ERROR` | `,` |
| 31 | 151 | `ERROR` | `,` |
| 36 | 116 | `ERROR` | `,` |
| 47 | 126 | `ERROR` | `,` |
| 53 | 126 | `ERROR` | `,` |
| 59 | 104 | `ERROR` | `,` |
| 65 | 104 | `ERROR` | `,` |
| 71 | 105 | `ERROR` | `,` |
| 77 | 103 | `ERROR` | `,` |
| 83 | 177 | `ERROR` | `,` |
| 103 | 109 | `ERROR` | `,` |
| 108 | 117 | `ERROR` | `,` |
| 118 | 175 | `ERROR` | `, , ,` |
| 123 | 116 | `ERROR` | `,` |
| 129 | 178 | `ERROR` | `,` |
| 135 | 123 | `ERROR` | `,` |
| 141 | 113 | `ERROR` | `,` |
| 152 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/photo_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 165 | `ERROR` | `, , ,` |
| 59 | 187 | `ERROR` | `, , ,` |
| 85 | 167 | `ERROR` | `, , ,` |
| 104 | 263 | `ERROR` | `,` |
| 108 | 175 | `ERROR` | `, , ,` |
| 116 | 260 | `ERROR` | `,` |
| 125 | 162 | `ERROR` | `,` |
| 127 | 191 | `ERROR` | `, , ,` |
| 153 | 91 | `ERROR` | `,` |
| 155 | 160 | `ERROR` | `, , ,` |
| 164 | 166 | `ERROR` | `, , ,` |
| 167 | 160 | `ERROR` | `, , ,` |
| 172 | 158 | `ERROR` | `, , ,` |
| 180 | 162 | `ERROR` | `, , ,` |
| 183 | 160 | `ERROR` | `, , ,` |
| 189 | 208 | `ERROR` | `,` |
| 192 | 144 | `ERROR` | `,` |
| 209 | 166 | `ERROR` | `, , ,` |
| 212 | 160 | `ERROR` | `, , ,` |
| 224 | 164 | `ERROR` | `, , ,` |
| … | … | … | *(231 more)* |

#### `frameworks/native/camera/base/src/output/photo_output_callback.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 121 | `ERROR` | `,` |
| 58 | 171 | `ERROR` | `, , ,` |
| 61 | 165 | `ERROR` | `, , ,` |
| 68 | 121 | `ERROR` | `,` |
| 77 | 129 | `ERROR` | `,` |
| 95 | 120 | `ERROR` | `,` |
| 98 | 164 | `ERROR` | `,` |
| 104 | 156 | `ERROR` | `,` |
| 106 | 108 | `ERROR` | `,` |
| 117 | 171 | `ERROR` | `,` |
| 119 | 111 | `ERROR` | `,` |
| 129 | 105 | `ERROR` | `,` |
| 132 | 156 | `ERROR` | `, , ,` |
| 134 | 176 | `ERROR` | `, , ,` |
| 160 | 165 | `ERROR` | `, , ,` |
| 165 | 166 | `ERROR` | `, , ,` |
| 184 | 109 | `ERROR` | `,` |
| 186 | 136 | `ERROR` | `, , ,` |
| 194 | 171 | `ERROR` | `, , ,` |
| 195 | 246 | `ERROR` | `,` |
| … | … | … | *(32 more)* |

#### `frameworks/native/camera/base/src/output/preview_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 110 | `ERROR` | `,` |
| 55 | 109 | `ERROR` | `,` |
| 63 | 105 | `ERROR` | `,` |
| 71 | 222 | `ERROR` | `,` |
| 71 | 234 | `ERROR` | `,` |
| 71 | 246 | `ERROR` | `,` |
| 73 | 118 | `ERROR` | `,` |
| 86 | 167 | `ERROR` | `, , ,` |
| 102 | 165 | `ERROR` | `, , ,` |
| 118 | 165 | `ERROR` | `, , ,` |
| 143 | 123 | `ERROR` | `,` |
| 167 | 166 | `ERROR` | `, , ,` |
| 183 | 167 | `ERROR` | `, , ,` |
| 207 | 111 | `ERROR` | `,` |
| 208 | 170 | `ERROR` | `, , ,` |
| 210 | 164 | `ERROR` | `, , ,` |
| 217 | 114 | `ERROR` | `,` |
| 219 | 167 | `ERROR` | `, , ,` |
| 227 | 103 | `ERROR` | `,` |
| 234 | 145 | `ERROR` | `, , ,` |
| … | … | … | *(106 more)* |

#### `frameworks/native/camera/base/src/output/sketch_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 248 | `ERROR` | `,` |
| 56 | 159 | `ERROR` | `, , ,` |
| 75 | 115 | `ERROR` | `,` |
| 79 | 149 | `ERROR` | `, , ,` |
| 110 | 130 | `ERROR` | `,` |
| 120 | 129 | `ERROR` | `,` |
| 207 | 115 | `ERROR` | `,` |
| 223 | 109 | `ERROR` | `,` |
| 253 | 189 | `ERROR` | `, , ,` |
| 256 | 195 | `ERROR` | `, , ,` |
| 258 | 188 | `ERROR` | `, , ,` |
| 280 | 179 | `ERROR` | `, , ,` |
| 281 | 166 | `ERROR` | `,` |
| 321 | 227 | `ERROR` | `,` |
| 343 | 195 | `ERROR` | `, , ,` |
| 396 | 185 | `ERROR` | `, , ,` |
| 410 | 191 | `ERROR` | `,` |
| 424 | 170 | `ERROR` | `, , ,` |
| 528 | 185 | `ERROR` | `,` |
| 557 | 144 | `ERROR` | `,` |
| … | … | … | *(4 more)* |

#### `frameworks/native/camera/base/src/output/unify_movie_file_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 126 | `ERROR` | `,` |
| 17 | 127 | `ERROR` | `,` |
| 41 | 120 | `ERROR` | `,` |
| 44 | 146 | `ERROR` | `, , ,` |
| 45 | 220 | `ERROR` | `,` |
| 66 | 126 | `ERROR` | `,` |
| 86 | 127 | `ERROR` | `,` |
| 90 | 153 | `ERROR` | `, , ,` |
| 104 | 121 | `ERROR` | `,` |
| 111 | 117 | `ERROR` | `,` |
| 113 | 169 | `ERROR` | `, , ,` |
| 117 | 111 | `ERROR` | `,` |
| 124 | 120 | `ERROR` | `,` |
| 127 | 146 | `ERROR` | `, , ,` |
| 131 | 114 | `ERROR` | `,` |
| 137 | 115 | `ERROR` | `,` |
| 139 | 174 | `ERROR` | `, , ,` |
| 142 | 163 | `ERROR` | `, , ,` |
| 145 | 159 | `ERROR` | `, , ,` |
| 147 | 183 | `ERROR` | `, , ,` |
| … | … | … | *(46 more)* |

#### `frameworks/native/camera/base/src/output/unify_movie_file_output_mock.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 126 | `ERROR` | `,` |
| 16 | 127 | `ERROR` | `,` |
| 35 | 120 | `ERROR` | `,` |
| 46 | 126 | `ERROR` | `,` |
| 52 | 127 | `ERROR` | `,` |
| 58 | 117 | `ERROR` | `,` |
| 64 | 120 | `ERROR` | `,` |
| 70 | 115 | `ERROR` | `,` |
| 76 | 182 | `ERROR` | `,` |
| 82 | 104 | `ERROR` | `,` |
| 88 | 104 | `ERROR` | `,` |
| 94 | 105 | `ERROR` | `,` |
| 100 | 103 | `ERROR` | `,` |
| 107 | 135 | `ERROR` | `,` |
| 113 | 138 | `ERROR` | `,` |
| 118 | 110 | `ERROR` | `,` |
| 124 | 129 | `ERROR` | `,` |
| 131 | 105 | `ERROR` | `,` |
| 142 | 179 | `ERROR` | `,` |
| 148 | 118 | `ERROR` | `,` |
| … | … | … | *(1 more)* |

#### `frameworks/native/camera/base/src/output/video_capability.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 80 | `ERROR` | `,` |
| 22 | 160 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/output/video_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 139 | `ERROR` | `,` |
| 42 | 137 | `ERROR` | `,` |
| 55 | 137 | `ERROR` | `,` |
| 70 | 144 | `ERROR` | `,` |
| 75 | 155 | `ERROR` | `,` |
| 100 | 127 | `ERROR` | `,` |
| 105 | 181 | `ERROR` | `, , ,` |
| 112 | 118 | `ERROR` | `,` |
| 116 | 194 | `ERROR` | `,` |
| 127 | 101 | `ERROR` | `,` |
| 135 | 98 | `ERROR` | `,` |
| 143 | 218 | `ERROR` | `,` |
| 143 | 230 | `ERROR` | `,` |
| 143 | 242 | `ERROR` | `,` |
| 146 | 114 | `ERROR` | `,` |
| 155 | 100 | `ERROR` | `,` |
| 163 | 217 | `ERROR` | `,` |
| 163 | 229 | `ERROR` | `,` |
| 163 | 241 | `ERROR` | `,` |
| 166 | 113 | `ERROR` | `,` |
| … | … | … | *(70 more)* |

#### `frameworks/native/camera/base/src/session/cameraSwitch_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 191 | `ERROR` | `,` |
| 56 | 120 | `ERROR` | `,` |
| 60 | 164 | `ERROR` | `, , ,` |
| 68 | 140 | `ERROR` | `, , ,` |
| 85 | 118 | `ERROR` | `,` |
| 91 | 109 | `ERROR` | `,` |
| 94 | 187 | `ERROR` | `, , ,` |
| 97 | 174 | `ERROR` | `, , ,` |
| 104 | 240 | `ERROR` | `,` |
| 107 | 191 | `ERROR` | `, , ,` |
| 114 | 204 | `ERROR` | `,` |
| 114 | 216 | `ERROR` | `,` |
| 114 | 228 | `ERROR` | `,` |
| 119 | 115 | `ERROR` | `,` |
| 121 | 195 | `ERROR` | `, , ,` |
| 124 | 150 | `ERROR` | `, , ,` |
| 130 | 125 | `ERROR` | `,` |
| 149 | 181 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/session/capture_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 276 | 184 | `ERROR` | `,` |
| 282 | 144 | `ERROR` | `,` |
| 285 | 144 | `ERROR` | `,` |
| 292 | 193 | `ERROR` | `,` |
| 299 | 144 | `ERROR` | `,` |
| 302 | 138 | `ERROR` | `,` |
| 310 | 206 | `ERROR` | `,` |
| 336 | 173 | `ERROR` | `,` |
| 343 | 146 | `ERROR` | `,` |
| 346 | 154 | `ERROR` | `,` |
| 359 | 138 | `ERROR` | `,` |
| 361 | 154 | `ERROR` | `, , ,` |
| 363 | 152 | `ERROR` | `, , ,` |
| 365 | 151 | `ERROR` | `,` |
| 368 | 147 | `ERROR` | `, , ,` |
| 370 | 183 | `ERROR` | `,` |
| 383 | 164 | `ERROR` | `, , ,` |
| 387 | 140 | `ERROR` | `, , ,` |
| 392 | 151 | `ERROR` | `,` |
| 396 | 95 | `ERROR` | `,` |
| … | … | … | *(809 more)* |

#### `frameworks/native/camera/base/src/session/control_center_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 126 | `ERROR` | `,` |
| 14 | 127 | `ERROR` | `,` |
| 19 | 102 | `ERROR` | `,` |
| 25 | 121 | `ERROR` | `,` |
| 27 | 156 | `ERROR` | `, , ,` |
| 33 | 173 | `ERROR` | `, , ,` |
| 39 | 111 | `ERROR` | `,` |
| 41 | 156 | `ERROR` | `, , ,` |
| 44 | 163 | `ERROR` | `, , ,` |
| 47 | 162 | `ERROR` | `, , ,` |
| 53 | 111 | `ERROR` | `,` |
| 55 | 156 | `ERROR` | `, , ,` |
| 58 | 163 | `ERROR` | `, , ,` |
| 61 | 163 | `ERROR` | `, , ,` |
| 62 | 168 | `ERROR` | `,` |
| 83 | 116 | `ERROR` | `,` |
| 84 | 182 | `ERROR` | `, , ,` |
| 88 | 168 | `ERROR` | `, , ,` |
| 97 | 117 | `ERROR` | `,` |
| 98 | 182 | `ERROR` | `, , ,` |
| … | … | … | *(35 more)* |

#### `frameworks/native/camera/base/src/session/features/composition_feature.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 116 | `ERROR` | `,` |
| 24 | 136 | `ERROR` | `,` |
| 37 | 186 | `ERROR` | `,` |
| 44 | 131 | `ERROR` | `,` |
| 55 | 173 | `ERROR` | `, , ,` |
| 62 | 136 | `ERROR` | `,` |
| 87 | 227 | `ERROR` | `,` |
| 95 | 163 | `ERROR` | `,` |
| 113 | 129 | `ERROR` | `, , ,` |
| 116 | 183 | `ERROR` | `, , ,` |
| 123 | 136 | `ERROR` | `,` |
| 125 | 192 | `ERROR` | `, , ,` |
| 132 | 106 | `ERROR` | `,` |
| 148 | 121 | `ERROR` | `,` |
| 151 | 169 | `ERROR` | `, , ,` |
| 161 | 119 | `ERROR` | `,` |
| 171 | 143 | `ERROR` | `,` |
| 201 | 118 | `ERROR` | `,` |
| 215 | 121 | `ERROR` | `,` |
| 217 | 192 | `ERROR` | `, , ,` |
| … | … | … | *(5 more)* |

#### `frameworks/native/camera/base/src/session/features/moon_capture_boost_feature.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 188 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/session/mech_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 142 | `ERROR` | `,` |
| 48 | 141 | `ERROR` | `,` |
| 61 | 141 | `ERROR` | `,` |
| 93 | 141 | `ERROR` | `,` |
| 99 | 192 | `ERROR` | `,` |
| 113 | 141 | `ERROR` | `, , ,` |
| 149 | 205 | `ERROR` | `,` |
| 182 | 202 | `ERROR` | `,` |
| 208 | 141 | `ERROR` | `,` |
| 212 | 164 | `ERROR` | `, , ,` |
| 216 | 140 | `ERROR` | `, , ,` |
| 221 | 141 | `ERROR` | `,` |
| 227 | 180 | `ERROR` | `,` |
| 233 | 210 | `ERROR` | `,` |
| 233 | 222 | `ERROR` | `,` |
| 233 | 234 | `ERROR` | `,` |
| 239 | 141 | `ERROR` | `,` |
| 242 | 171 | `ERROR` | `, , ,` |
| 244 | 191 | `ERROR` | `, , ,` |
| 258 | 141 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `frameworks/native/camera/base/src/session/photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 171 | 109 | `ERROR` | `,` |
| 190 | 138 | `ERROR` | `,` |
| 205 | 239 | `ERROR` | `,` |
| 266 | 184 | `ERROR` | `, , ,` |
| 269 | 201 | `ERROR` | `,` |
| 282 | 165 | `ERROR` | `, , ,` |
| 290 | 194 | `ERROR` | `,` |
| 297 | 185 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/session/scan_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 190 | `ERROR` | `, , ,` |
| 41 | 108 | `ERROR` | `,` |
| 43 | 169 | `ERROR` | `, , ,` |
| 51 | 163 | `ERROR` | `, , ,` |
| 54 | 179 | `ERROR` | `, , ,` |
| 56 | 178 | `ERROR` | `, , ,` |
| 72 | 139 | `ERROR` | `,` |
| 74 | 190 | `ERROR` | `, , ,` |
| 82 | 194 | `ERROR` | `, , ,` |
| 91 | 196 | `ERROR` | `, , ,` |
| 100 | 107 | `ERROR` | `,` |
| 107 | 171 | `ERROR` | `, , ,` |
| 113 | 175 | `ERROR` | `,` |
| 130 | 136 | `ERROR` | `,` |
| 133 | 179 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/base/src/session/secure_camera_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 118 | `ERROR` | `,` |
| 18 | 132 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/session/video_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 153 | 109 | `ERROR` | `,` |
| 155 | 170 | `ERROR` | `, , ,` |
| 173 | 138 | `ERROR` | `,` |
| 186 | 239 | `ERROR` | `,` |
| 279 | 165 | `ERROR` | `, , ,` |
| 287 | 194 | `ERROR` | `,` |
| 294 | 185 | `ERROR` | `,` |
| 308 | 141 | `ERROR` | `,` |
| 310 | 198 | `ERROR` | `,` |
| 310 | 215 | `ERROR` | `,` |
| 310 | 232 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/utils/camera_buffer_handle_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 153 | `ERROR` | `,` |
| 31 | 162 | `ERROR` | `, , ,` |
| 49 | 163 | `ERROR` | `, , ,` |
| 52 | 162 | `ERROR` | `, , ,` |
| 58 | 106 | `ERROR` | `,` |
| 74 | 166 | `ERROR` | `, , ,` |
| 78 | 111 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/utils/camera_counting_timer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 79 | `ERROR` | `,` |
| 21 | 79 | `ERROR` | `,` |
| 29 | 173 | `ERROR` | `,` |
| 33 | 90 | `ERROR` | `,` |
| 40 | 154 | `ERROR` | `,` |
| 43 | 90 | `ERROR` | `,` |
| 49 | 136 | `ERROR` | `, , ,` |
| 52 | 135 | `ERROR` | `,` |
| 58 | 135 | `ERROR` | `,` |
| 60 | 135 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/utils/camera_device_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 174 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/base/src/utils/camera_rotation_api_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 166 | `ERROR` | `, , ,` |
| 25 | 26 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/utils/dps_metadata_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 176 | 153 | `ERROR` | `, , ,` |
| 184 | 145 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/base/src/utils/logic_camera_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 150 | `ERROR` | `, , ,` |
| 32 | 112 | `ERROR` | `,` |
| 53 | 232 | `ERROR` | `,` |
| 71 | 137 | `ERROR` | `,` |

#### `frameworks/native/camera/base/src/utils/metadata_common_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 80 | 231 | `ERROR` | `,` |
| 99 | 220 | `ERROR` | `,` |
| 114 | 231 | `ERROR` | `,` |
| 140 | 230 | `ERROR` | `,` |
| 161 | 228 | `ERROR` | `,` |
| 170 | 218 | `ERROR` | `,` |
| 184 | 219 | `ERROR` | `,` |
| 193 | 218 | `ERROR` | `,` |
| 212 | 141 | `ERROR` | `, , ,` |
| 216 | 209 | `ERROR` | `,` |
| 216 | 226 | `ERROR` | `,` |
| 216 | 243 | `ERROR` | `,` |
| 235 | 104 | `ERROR` | `,` |
| 241 | 164 | `ERROR` | `, , ,` |
| 272 | 190 | `ERROR` | `,` |
| 326 | 18 | `ERROR` | `,` |
| 342 | 214 | `ERROR` | `,` |
| 404 | 34 | `ERROR` | `,` |
| 414 | 155 | `ERROR` | `, , ,` |
| 416 | 214 | `ERROR` | `,` |
| … | … | … | *(4 more)* |

#### `frameworks/native/camera/extension/src/input/camera_manager_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 127 | `ERROR` | `,` |
| 18 | 121 | `ERROR` | `,` |
| 25 | 114 | `ERROR` | `,` |
| 85 | 129 | `ERROR` | `,` |
| 114 | 174 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/output/depth_data_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 151 | `ERROR` | `,` |
| 20 | 143 | `ERROR` | `,` |
| 35 | 118 | `ERROR` | `,` |
| 43 | 105 | `ERROR` | `,` |
| 45 | 220 | `ERROR` | `, , ,` |
| 54 | 222 | `ERROR` | `,` |
| 54 | 234 | `ERROR` | `,` |
| 54 | 246 | `ERROR` | `,` |
| 56 | 116 | `ERROR` | `,` |
| 64 | 104 | `ERROR` | `,` |
| 72 | 221 | `ERROR` | `,` |
| 72 | 233 | `ERROR` | `,` |
| 72 | 245 | `ERROR` | `,` |
| 74 | 115 | `ERROR` | `,` |
| 83 | 115 | `ERROR` | `,` |
| 92 | 206 | `ERROR` | `,` |
| 92 | 218 | `ERROR` | `,` |
| 92 | 230 | `ERROR` | `,` |
| 94 | 126 | `ERROR` | `,` |
| 101 | 106 | `ERROR` | `,` |
| … | … | … | *(11 more)* |

#### `frameworks/native/camera/extension/src/session/aperture_video_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 117 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/capture_session_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 128 | `ERROR` | `,` |
| 49 | 252 | `ERROR` | `, , ,` |
| 73 | 159 | `ERROR` | `, , ,` |
| 95 | 240 | `ERROR` | `, , ,` |
| 106 | 187 | `ERROR` | `,` |
| 108 | 178 | `ERROR` | `, , ,` |
| 118 | 259 | `ERROR` | `, , ,` |
| 142 | 160 | `ERROR` | `, , ,` |
| 164 | 247 | `ERROR` | `, , ,` |
| 175 | 190 | `ERROR` | `,` |
| 177 | 180 | `ERROR` | `, , ,` |
| 193 | 153 | `ERROR` | `, , ,` |
| 238 | 165 | `ERROR` | `, , ,` |
| 261 | 246 | `ERROR` | `, , ,` |
| 272 | 206 | `ERROR` | `,` |
| 276 | 190 | `ERROR` | `, , ,` |
| 284 | 120 | `ERROR` | `,` |
| 295 | 163 | `ERROR` | `, , ,` |
| 303 | 167 | `ERROR` | `,` |
| 306 | 235 | `ERROR` | `,` |
| … | … | … | *(80 more)* |

#### `frameworks/native/camera/extension/src/session/cinematic_video_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 198 | `ERROR` | `,` |
| 11 | 215 | `ERROR` | `,` |
| 11 | 232 | `ERROR` | `,` |
| 21 | 126 | `ERROR` | `,` |
| 27 | 117 | `ERROR` | `,` |
| 32 | 113 | `ERROR` | `,` |
| 33 | 134 | `ERROR` | `, , ,` |
| 39 | 145 | `ERROR` | `,` |
| 41 | 142 | `ERROR` | `, , ,` |
| 56 | 158 | `ERROR` | `, , ,` |
| 58 | 135 | `ERROR` | `, , ,` |
| 69 | 116 | `ERROR` | `,` |
| 75 | 149 | `ERROR` | `,` |
| 77 | 142 | `ERROR` | `, , ,` |
| 90 | 158 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/extension/src/session/fluorescence_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 121 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/high_res_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 116 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/light_painting_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 128 | `ERROR` | `,` |
| 56 | 104 | `ERROR` | `,` |
| 66 | 163 | `ERROR` | `,` |
| 78 | 103 | `ERROR` | `,` |
| 85 | 115 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/macro_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 114 | `ERROR` | `,` |
| 13 | 175 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/extension/src/session/macro_video_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 114 | `ERROR` | `,` |
| 13 | 175 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/extension/src/session/night_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 112 | `ERROR` | `,` |
| 18 | 143 | `ERROR` | `, , ,` |
| 21 | 147 | `ERROR` | `, , ,` |
| 24 | 88 | `ERROR` | `,` |
| 27 | 140 | `ERROR` | `, , ,` |
| 30 | 155 | `ERROR` | `,` |
| 49 | 148 | `ERROR` | `, , ,` |
| 52 | 181 | `ERROR` | `,` |
| 58 | 230 | `ERROR` | `, , ,` |
| 66 | 173 | `ERROR` | `, , ,` |
| 73 | 148 | `ERROR` | `, , ,` |
| 76 | 138 | `ERROR` | `, , ,` |
| 79 | 146 | `ERROR` | `, , ,` |
| 89 | 147 | `ERROR` | `,` |
| 96 | 140 | `ERROR` | `,` |
| 99 | 183 | `ERROR` | `, , ,` |
| 120 | 150 | `ERROR` | `, , ,` |
| 122 | 154 | `ERROR` | `, , ,` |
| 135 | 138 | `ERROR` | `, , ,` |
| 161 | 128 | `ERROR` | `,` |
| … | … | … | *(6 more)* |

#### `frameworks/native/camera/extension/src/session/panorama_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 112 | `ERROR` | `,` |
| 14 | 173 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/extension/src/session/photo_session_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 172 | 115 | `ERROR` | `,` |
| 192 | 144 | `ERROR` | `,` |
| 204 | 245 | `ERROR` | `,` |
| 275 | 171 | `ERROR` | `, , ,` |
| 281 | 200 | `ERROR` | `,` |
| 292 | 191 | `ERROR` | `,` |
| 303 | 127 | `ERROR` | `,` |
| 324 | 122 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/portrait_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 157 | `ERROR` | `, , ,` |
| 33 | 155 | `ERROR` | `, , ,` |
| 56 | 146 | `ERROR` | `, , ,` |
| 59 | 154 | `ERROR` | `, , ,` |
| 77 | 179 | `ERROR` | `, , ,` |
| 78 | 188 | `ERROR` | `, , ,` |
| 89 | 168 | `ERROR` | `,` |
| 91 | 166 | `ERROR` | `, , ,` |
| 98 | 112 | `ERROR` | `,` |
| 100 | 173 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/extension/src/session/profession_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 53 | 157 | `ERROR` | `, , ,` |
| 97 | 128 | `ERROR` | `,` |
| 101 | 181 | `ERROR` | `,` |
| 103 | 171 | `ERROR` | `, , ,` |
| 115 | 147 | `ERROR` | `, , ,` |
| 118 | 155 | `ERROR` | `, , ,` |
| 141 | 143 | `ERROR` | `, , ,` |
| 144 | 151 | `ERROR` | `, , ,` |
| 149 | 150 | `ERROR` | `, , ,` |
| 162 | 250 | `ERROR` | `,` |
| 169 | 208 | `ERROR` | `,` |
| 176 | 263 | `ERROR` | `,` |
| 184 | 148 | `ERROR` | `, , ,` |
| 188 | 159 | `ERROR` | `,` |
| 194 | 215 | `ERROR` | `, , ,` |
| 201 | 173 | `ERROR` | `, , ,` |
| 210 | 148 | `ERROR` | `, , ,` |
| 213 | 138 | `ERROR` | `, , ,` |
| 216 | 146 | `ERROR` | `, , ,` |
| 224 | 127 | `ERROR` | `,` |
| … | … | … | *(51 more)* |

#### `frameworks/native/camera/extension/src/session/quick_shot_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 118 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/secure_camera_session_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 18 | 138 | `ERROR` | `,` |

#### `frameworks/native/camera/extension/src/session/slow_motion_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 140 | `ERROR` | `,` |
| 22 | 172 | `ERROR` | `, , ,` |
| 37 | 95 | `ERROR` | `,` |
| 46 | 112 | `ERROR` | `,` |
| 47 | 179 | `ERROR` | `, , ,` |
| 49 | 169 | `ERROR` | `, , ,` |
| 51 | 177 | `ERROR` | `, , ,` |
| 57 | 207 | `ERROR` | `,` |
| 57 | 215 | `ERROR` | `,` |
| 57 | 223 | `ERROR` | `,` |
| 58 | 173 | `ERROR` | `,` |
| 81 | 108 | `ERROR` | `,` |
| 82 | 175 | `ERROR` | `, , ,` |
| 84 | 180 | `ERROR` | `, , ,` |
| 86 | 172 | `ERROR` | `, , ,` |
| 87 | 250 | `ERROR` | `,` |
| 95 | 164 | `ERROR` | `, , ,` |
| 104 | 168 | `ERROR` | `, , ,` |
| 111 | 170 | `ERROR` | `,` |
| 113 | 153 | `ERROR` | `,` |
| … | … | … | *(14 more)* |

#### `frameworks/native/camera/extension/src/session/stitching_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 215 | `ERROR` | `,` |
| 13 | 239 | `ERROR` | `,` |
| 13 | 263 | `ERROR` | `,` |
| 17 | 228 | `ERROR` | `,` |
| 17 | 252 | `ERROR` | `,` |
| 17 | 276 | `ERROR` | `,` |
| 48 | 202 | `ERROR` | `,` |
| 48 | 226 | `ERROR` | `,` |
| 48 | 250 | `ERROR` | `,` |
| 65 | 212 | `ERROR` | `,` |
| 65 | 236 | `ERROR` | `,` |
| 65 | 260 | `ERROR` | `,` |
| 68 | 211 | `ERROR` | `,` |
| 68 | 235 | `ERROR` | `,` |
| 68 | 259 | `ERROR` | `,` |
| 75 | 143 | `ERROR` | `,` |
| 77 | 212 | `ERROR` | `,` |
| 77 | 236 | `ERROR` | `,` |
| 77 | 260 | `ERROR` | `,` |
| 94 | 168 | `ERROR` | `,` |
| … | … | … | *(79 more)* |

#### `frameworks/native/camera/extension/src/session/time_lapse_photo_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 191 | `ERROR` | `,` |
| 33 | 188 | `ERROR` | `,` |
| 47 | 32 | `ERROR` | `,` |
| 56 | 156 | `ERROR` | `, , ,` |
| 71 | 171 | `ERROR` | `,` |
| 85 | 160 | `ERROR` | `, , ,` |
| 91 | 159 | `ERROR` | `, , ,` |
| 94 | 162 | `ERROR` | `,` |
| 109 | 164 | `ERROR` | `, , ,` |
| 114 | 164 | `ERROR` | `,` |
| 162 | 168 | `ERROR` | `, , ,` |
| 167 | 182 | `ERROR` | `,` |
| 202 | 174 | `ERROR` | `,` |
| 207 | 159 | `ERROR` | `, , ,` |
| 222 | 174 | `ERROR` | `,` |
| 227 | 159 | `ERROR` | `, , ,` |
| 283 | 175 | `ERROR` | `,` |
| 285 | 156 | `ERROR` | `, , ,` |
| 299 | 176 | `ERROR` | `,` |
| 301 | 160 | `ERROR` | `, , ,` |
| … | … | … | *(40 more)* |

#### `frameworks/native/camera/extension/src/session/video_session_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 160 | 115 | `ERROR` | `,` |
| 162 | 176 | `ERROR` | `, , ,` |
| 179 | 144 | `ERROR` | `,` |
| 190 | 245 | `ERROR` | `,` |
| 286 | 171 | `ERROR` | `, , ,` |
| 292 | 200 | `ERROR` | `,` |
| 303 | 191 | `ERROR` | `,` |
| 330 | 226 | `ERROR` | `,` |
| 330 | 243 | `ERROR` | `,` |
| 330 | 260 | `ERROR` | `,` |
| 333 | 144 | `ERROR` | `, , ,` |
| 335 | 131 | `ERROR` | `, , ,` |
| 343 | 141 | `ERROR` | `, , ,` |
| 347 | 213 | `ERROR` | `,` |
| 347 | 230 | `ERROR` | `,` |
| 347 | 247 | `ERROR` | `,` |
| 350 | 202 | `ERROR` | `,` |
| 350 | 219 | `ERROR` | `,` |
| 350 | 236 | `ERROR` | `,` |
| 363 | 141 | `ERROR` | `, , ,` |
| … | … | … | *(26 more)* |

#### `frameworks/native/camera/test/moduletest/camera_base_function/src/camera_base_function_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 170 | `ERROR` | `,` |
| 19 | 283 | `ERROR` | `,` |
| 24 | 189 | `ERROR` | `,` |
| 28 | 193 | `ERROR` | `,` |
| 34 | 228 | `ERROR` | `,` |
| 40 | 191 | `ERROR` | `,` |
| 44 | 163 | `ERROR` | `,` |
| 48 | 168 | `ERROR` | `,` |
| 52 | 163 | `ERROR` | `,` |
| 56 | 177 | `ERROR` | `,` |
| 60 | 199 | `ERROR` | `,` |
| 65 | 221 | `ERROR` | `,` |
| 74 | 170 | `ERROR` | `,` |
| 78 | 103 | `ERROR` | `,` |
| 82 | 200 | `ERROR` | `,` |
| 86 | 129 | `ERROR` | `,` |
| 91 | 242 | `ERROR` | `,` |
| 96 | 169 | `ERROR` | `,` |
| 102 | 126 | `ERROR` | `,` |
| 112 | 91 | `ERROR` | `,` |
| … | … | … | *(52 more)* |

#### `frameworks/native/camera/test/moduletest/camera_deferred_photo/src/camera_deferred_photo_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 125 | `ERROR` | `,` |
| 29 | 128 | `ERROR` | `,` |
| 35 | 179 | `ERROR` | `,` |
| 63 | 120 | `ERROR` | `,` |
| 154 | 177 | `ERROR` | `, , ,` |
| 156 | 148 | `ERROR` | `, , ,` |
| 164 | 155 | `ERROR` | `, , ,` |
| 166 | 134 | `ERROR` | `, , ,` |
| 169 | 150 | `ERROR` | `, , ,` |
| 172 | 148 | `ERROR` | `, , ,` |
| 180 | 165 | `ERROR` | `, , ,` |
| 182 | 140 | `ERROR` | `, , ,` |
| 235 | 153 | `ERROR` | `, , ,` |
| 236 | 151 | `ERROR` | `, , ,` |
| 238 | 157 | `ERROR` | `, , ,` |
| 241 | 152 | `ERROR` | `, , ,` |
| 251 | 153 | `ERROR` | `, , ,` |
| 252 | 151 | `ERROR` | `, , ,` |
| 254 | 157 | `ERROR` | `, , ,` |
| 257 | 152 | `ERROR` | `, , ,` |
| … | … | … | *(8 more)* |

#### `frameworks/native/camera/test/moduletest/camera_deferred_video/src/camera_deferred_video_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 125 | `ERROR` | `,` |
| 19 | 128 | `ERROR` | `,` |
| 24 | 117 | `ERROR` | `,` |
| 30 | 146 | `ERROR` | `, , ,` |
| 34 | 156 | `ERROR` | `, , ,` |
| 41 | 173 | `ERROR` | `, , ,` |
| 59 | 120 | `ERROR` | `,` |
| 136 | 216 | `ERROR` | `,` |
| 156 | 95 | `ERROR` | `,` |
| 161 | 93 | `ERROR` | `,` |
| 166 | 88 | `ERROR` | `,` |
| 171 | 111 | `ERROR` | `,` |
| 192 | 192 | `ERROR` | `, , ,` |
| 221 | 192 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/test/moduletest/camera_format_YUV/include/camera_format_YUV_moduletest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 157 | `ERROR` | `,` |

#### `frameworks/native/camera/test/moduletest/camera_format_YUV/src/camera_format_YUV_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 65 | 121 | `ERROR` | `,` |
| 71 | 124 | `ERROR` | `,` |
| 95 | 116 | `ERROR` | `,` |

#### `frameworks/native/camera/test/moduletest/camera_moving_photo/src/camera_moving_photo_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 125 | `ERROR` | `,` |
| 29 | 107 | `ERROR` | `,` |
| 32 | 117 | `ERROR` | `,` |
| 33 | 148 | `ERROR` | `, , ,` |
| 40 | 165 | `ERROR` | `, , ,` |
| 45 | 121 | `ERROR` | `,` |
| 49 | 153 | `ERROR` | `, , ,` |
| 50 | 125 | `ERROR` | `,` |
| 52 | 115 | `ERROR` | `,` |
| 55 | 106 | `ERROR` | `,` |
| 68 | 130 | `ERROR` | `,` |
| 77 | 137 | `ERROR` | `,` |
| 88 | 146 | `ERROR` | `, , ,` |
| 96 | 128 | `ERROR` | `, , ,` |
| 99 | 162 | `ERROR` | `, , ,` |
| 102 | 149 | `ERROR` | `, , ,` |
| 123 | 149 | `ERROR` | `,` |
| 126 | 101 | `ERROR` | `,` |
| 135 | 149 | `ERROR` | `, , ,` |
| 142 | 148 | `ERROR` | `, , ,` |
| … | … | … | *(37 more)* |

#### `frameworks/native/camera/test/moduletest/camera_photo/src/camera_photo_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 117 | `ERROR` | `,` |
| 20 | 120 | `ERROR` | `,` |
| 25 | 106 | `ERROR` | `,` |
| 41 | 104 | `ERROR` | `,` |
| 46 | 109 | `ERROR` | `,` |
| 65 | 107 | `ERROR` | `,` |
| 87 | 107 | `ERROR` | `,` |
| 166 | 96 | `ERROR` | `,` |
| 171 | 94 | `ERROR` | `,` |
| 176 | 110 | `ERROR` | `,` |
| 179 | 136 | `ERROR` | `, , ,` |
| 185 | 165 | `ERROR` | `, , ,` |
| 187 | 110 | `ERROR` | `,` |
| 192 | 120 | `ERROR` | `,` |
| 199 | 125 | `ERROR` | `,` |
| 206 | 124 | `ERROR` | `,` |
| 280 | 175 | `ERROR` | `, , ,` |
| 315 | 175 | `ERROR` | `, , ,` |
| 346 | 175 | `ERROR` | `, , ,` |
| 379 | 175 | `ERROR` | `, , ,` |
| … | … | … | *(16 more)* |

#### `frameworks/native/camera/test/moduletest/camera_preview/src/camera_preview_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 119 | `ERROR` | `,` |
| 20 | 122 | `ERROR` | `,` |
| 25 | 108 | `ERROR` | `,` |
| 41 | 106 | `ERROR` | `,` |
| 46 | 111 | `ERROR` | `,` |
| 64 | 109 | `ERROR` | `,` |
| 83 | 107 | `ERROR` | `,` |
| 129 | 96 | `ERROR` | `,` |
| 134 | 94 | `ERROR` | `,` |
| 139 | 110 | `ERROR` | `,` |
| 142 | 136 | `ERROR` | `, , ,` |
| 148 | 165 | `ERROR` | `, , ,` |
| 150 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/moduletest/camera_session/include/camera_session_moduletest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 251 | 98 | `ERROR` | `,` |
| 259 | 93 | `ERROR` | `,` |
| 267 | 98 | `ERROR` | `,` |
| 275 | 100 | `ERROR` | `,` |

#### `frameworks/native/camera/test/moduletest/camera_session/src/camera_session_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 180 | `ERROR` | `,` |
| 62 | 123 | `ERROR` | `,` |
| 71 | 192 | `ERROR` | `,` |
| 82 | 184 | `ERROR` | `,` |
| 90 | 280 | `ERROR` | `,` |
| 97 | 193 | `ERROR` | `,` |
| 106 | 121 | `ERROR` | `,` |
| 114 | 175 | `ERROR` | `,` |
| 116 | 115 | `ERROR` | `,` |
| 119 | 150 | `ERROR` | `,` |
| 124 | 143 | `ERROR` | `,` |
| 131 | 169 | `ERROR` | `,` |
| 137 | 169 | `ERROR` | `,` |
| 143 | 205 | `ERROR` | `,` |
| 171 | 177 | `ERROR` | `,` |
| 184 | 99 | `ERROR` | `,` |
| 189 | 167 | `ERROR` | `,` |
| 194 | 160 | `ERROR` | `,` |
| 200 | 178 | `ERROR` | `,` |
| 206 | 186 | `ERROR` | `,` |
| … | … | … | *(53 more)* |

#### `frameworks/native/camera/test/ndktest/camera_ndk_demo/entry/src/main/cpp/main.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 468 | 8 | `ERROR` | `napi_value` |
| 500 | 20 | `ERROR` | `demoModule` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_event_report_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 117 | `ERROR` | `,` |
| 24 | 120 | `ERROR` | `,` |
| 29 | 109 | `ERROR` | `,` |
| 34 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_video_report_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 117 | `ERROR` | `,` |
| 20 | 120 | `ERROR` | `,` |
| 25 | 109 | `ERROR` | `,` |
| 30 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_manager_test/src/camera_deferred_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 119 | `ERROR` | `,` |
| 23 | 122 | `ERROR` | `,` |
| 28 | 77 | `ERROR` | `,` |
| 36 | 114 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_post_processor_test/src/camera_deferred_post_processor_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 80 | `ERROR` | `,` |
| 71 | 41 | `missing type_identifier` | `` |
| 71 | 83 | `missing type_identifier` | `` |
| 72 | 32 | `missing type_identifier` | `` |
| 73 | 79 | `missing type_identifier` | `` |
| 74 | 32 | `missing type_identifier` | `` |
| 75 | 79 | `missing type_identifier` | `` |
| 76 | 38 | `missing type_identifier` | `` |
| 76 | 121 | `missing type_identifier` | `` |
| 78 | 38 | `missing type_identifier` | `` |
| 78 | 84 | `missing type_identifier` | `` |
| 79 | 39 | `missing type_identifier` | `` |
| 79 | 122 | `missing type_identifier` | `` |
| 81 | 37 | `missing type_identifier` | `` |
| 81 | 89 | `missing type_identifier` | `` |
| 82 | 36 | `missing type_identifier` | `` |
| 82 | 67 | `missing type_identifier` | `` |
| 83 | 34 | `missing type_identifier` | `` |
| 83 | 38 | `missing type_identifier` | `` |
| 84 | 30 | `missing type_identifier` | `` |
| … | … | … | *(15 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_proc_test/src/camera_deferred_proc_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 116 | `ERROR` | `,` |
| 23 | 119 | `ERROR` | `,` |
| 47 | 111 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/include/camera_deferred_video_unittest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 7 | `ERROR` | `TestDeferredVideoProcSessionCallback: public` |
| 12 | 37 | `ERROR` | `:: string&` |
| 12 | 68 | `ERROR` | `< IPCFileDescriptor> &` |
| 13 | 26 | `ERROR` | `:: string&` |
| 17 | 28 | `ERROR` | `: public testing:: Test` |
| 36 | 1 | `ERROR` | `sptr< DeferredProcessing::` |
| 36 | 69 | `missing ;` | `` |
| 36 | 78 | `ERROR` | `}` |
| 37 | 4 | `ERROR` | `:: shared_ptr< DeferredProcessing:` |
| 37 | 71 | `missing ;` | `` |
| 37 | 80 | `ERROR` | `}` |
| 38 | 4 | `ERROR` | `:: shared_ptr< DeferredProcessing:` |
| 38 | 75 | `missing ;` | `` |
| 38 | 84 | `ERROR` | `}` |
| 39 | 4 | `ERROR` | `:: shared_ptr< DeferredProcessing:` |
| 39 | 74 | `missing ;` | `` |
| 39 | 83 | `ERROR` | `}` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_job_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 80 | `ERROR` | `,` |
| 45 | 80 | `ERROR` | `,` |
| 50 | 80 | `ERROR` | `,` |
| 55 | 80 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_stratety_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 80 | `ERROR` | `,` |
| 34 | 80 | `ERROR` | `,` |
| 39 | 80 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 26 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 39 | `missing type_identifier` | `` |
| 20 | 111 | `missing type_identifier` | `` |
| 21 | 41 | `missing type_identifier` | `` |
| 21 | 83 | `missing type_identifier` | `` |
| 22 | 41 | `missing type_identifier` | `` |
| 22 | 92 | `missing type_identifier` | `` |
| 23 | 37 | `missing type_identifier` | `` |
| 23 | 68 | `missing type_identifier` | `` |
| 24 | 36 | `missing type_identifier` | `` |
| 24 | 67 | `missing type_identifier` | `` |
| 25 | 34 | `missing type_identifier` | `` |
| 25 | 38 | `missing type_identifier` | `` |
| 26 | 30 | `missing type_identifier` | `` |
| 26 | 34 | `missing type_identifier` | `` |
| 42 | 43 | `missing type_identifier` | `` |
| 43 | 31 | `missing type_identifier` | `` |
| 44 | 50 | `missing type_identifier` | `` |
| 46 | 43 | `missing type_identifier` | `` |
| 47 | 91 | `missing type_identifier` | `` |
| 48 | 32 | `missing type_identifier` | `` |
| … | … | … | *(6 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_controller_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 43 | `missing type_identifier` | `` |
| 23 | 74 | `missing type_identifier` | `` |
| 24 | 32 | `missing type_identifier` | `` |
| 25 | 39 | `missing type_identifier` | `` |
| 25 | 60 | `missing type_identifier` | `` |
| 26 | 45 | `missing type_identifier` | `` |
| 26 | 93 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 43 | `missing type_identifier` | `` |
| 24 | 74 | `missing type_identifier` | `` |
| 25 | 32 | `missing type_identifier` | `` |
| 26 | 39 | `missing type_identifier` | `` |
| 26 | 60 | `missing type_identifier` | `` |
| 27 | 45 | `missing type_identifier` | `` |
| 27 | 93 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/camera_deferred_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 134 | `ERROR` | `, , ,` |
| 54 | 158 | `ERROR` | `, , ,` |
| 61 | 134 | `ERROR` | `, , ,` |
| 65 | 158 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 43 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 50 | `missing type_identifier` | `` |
| 36 | 43 | `missing type_identifier` | `` |
| 37 | 91 | `missing type_identifier` | `` |
| 38 | 32 | `missing type_identifier` | `` |
| 39 | 39 | `missing type_identifier` | `` |
| 40 | 39 | `missing type_identifier` | `` |
| 42 | 48 | `missing type_identifier` | `` |
| 43 | 81 | `missing type_identifier` | `` |
| 44 | 43 | `missing type_identifier` | `` |
| 46 | 84 | `missing type_identifier` | `` |
| 68 | 134 | `ERROR` | `, , ,` |
| 70 | 148 | `ERROR` | `, , ,` |
| 74 | 84 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_video_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 58 | 43 | `missing type_identifier` | `` |
| 58 | 74 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 60 | 39 | `missing type_identifier` | `` |
| 60 | 60 | `missing type_identifier` | `` |
| 61 | 45 | `missing type_identifier` | `` |
| 61 | 93 | `missing type_identifier` | `` |
| 77 | 158 | `ERROR` | `, , ,` |
| 175 | 40 | `missing type_identifier` | `` |
| 175 | 71 | `missing type_identifier` | `` |
| 176 | 29 | `missing type_identifier` | `` |
| 176 | 91 | `missing type_identifier` | `` |
| 177 | 36 | `missing type_identifier` | `` |
| 177 | 66 | `missing type_identifier` | `` |
| 178 | 42 | `missing type_identifier` | `` |
| 178 | 90 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_common/camera_common.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 153 | `ERROR` | `, , ,` |
| 262 | 127 | `ERROR` | `,` |
| 268 | 127 | `ERROR` | `,` |
| 273 | 127 | `ERROR` | `,` |
| 279 | 127 | `ERROR` | `,` |
| 285 | 127 | `ERROR` | `,` |
| 291 | 127 | `ERROR` | `,` |
| 296 | 127 | `ERROR` | `,` |
| 302 | 127 | `ERROR` | `,` |
| 307 | 127 | `ERROR` | `,` |
| 312 | 127 | `ERROR` | `,` |
| 317 | 127 | `ERROR` | `,` |
| 322 | 127 | `ERROR` | `,` |
| 327 | 127 | `ERROR` | `,` |
| 332 | 127 | `ERROR` | `,` |
| 337 | 127 | `ERROR` | `,` |
| 342 | 127 | `ERROR` | `,` |
| 347 | 127 | `ERROR` | `,` |
| 352 | 127 | `ERROR` | `,` |
| 357 | 127 | `ERROR` | `,` |
| … | … | … | *(11 more)* |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_manager_test/src/camera_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 775 | 119 | `ERROR` | `,` |
| 778 | 104 | `ERROR` | `,` |
| 931 | 119 | `ERROR` | `,` |
| 934 | 104 | `ERROR` | `,` |
| 981 | 119 | `ERROR` | `,` |
| 984 | 104 | `ERROR` | `,` |
| 1027 | 119 | `ERROR` | `,` |
| 1030 | 104 | `ERROR` | `,` |
| 1236 | 109 | `ERROR` | `,` |
| 1244 | 119 | `ERROR` | `,` |
| 1250 | 120 | `ERROR` | `,` |
| 1255 | 115 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_capturer_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 18 | 127 | `ERROR` | `,` |
| 23 | 116 | `ERROR` | `,` |
| 28 | 119 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_deferred_process_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 17 | 127 | `ERROR` | `,` |
| 22 | 116 | `ERROR` | `,` |
| 27 | 119 | `ERROR` | `,` |
| 33 | 112 | `ERROR` | `,` |
| 48 | 110 | `ERROR` | `,` |
| 54 | 112 | `ERROR` | `,` |
| 69 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_encoder_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 116 | `ERROR` | `,` |
| 17 | 119 | `ERROR` | `,` |
| 22 | 108 | `ERROR` | `,` |
| 27 | 111 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_video_muxer_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 119 | `ERROR` | `,` |
| 19 | 122 | `ERROR` | `,` |
| 24 | 111 | `ERROR` | `,` |
| 29 | 114 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/avcodec_task_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 91 | `ERROR` | `,` |
| 19 | 122 | `ERROR` | `,` |
| 24 | 125 | `ERROR` | `,` |
| 29 | 114 | `ERROR` | `,` |
| 34 | 117 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/camera_server_photo_proxy_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 126 | `ERROR` | `,` |
| 19 | 129 | `ERROR` | `,` |
| 24 | 118 | `ERROR` | `,` |
| 29 | 121 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/moving_photo_video_cache_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 91 | `ERROR` | `,` |
| 19 | 125 | `ERROR` | `,` |
| 24 | 128 | `ERROR` | `,` |
| 29 | 117 | `ERROR` | `,` |
| 34 | 120 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/video_encoder_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 116 | `ERROR` | `,` |
| 18 | 119 | `ERROR` | `,` |
| 23 | 108 | `ERROR` | `,` |
| 28 | 111 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 117 | `ERROR` | `,` |
| 16 | 120 | `ERROR` | `,` |
| 21 | 109 | `ERROR` | `,` |
| 26 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_fwk_metadata_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 126 | `ERROR` | `,` |
| 17 | 129 | `ERROR` | `,` |
| 22 | 118 | `ERROR` | `,` |
| 27 | 121 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_info_dumper_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 120 | `ERROR` | `,` |
| 17 | 123 | `ERROR` | `,` |
| 22 | 112 | `ERROR` | `,` |
| 27 | 115 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_privacy_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 117 | `ERROR` | `,` |
| 17 | 120 | `ERROR` | `,` |
| 22 | 109 | `ERROR` | `,` |
| 27 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_util_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 114 | `ERROR` | `,` |
| 25 | 117 | `ERROR` | `,` |
| 30 | 106 | `ERROR` | `,` |
| 35 | 109 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/client/src/camera_service_client_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 180 | `ERROR` | `,` |
| 56 | 123 | `ERROR` | `,` |
| 65 | 192 | `ERROR` | `,` |
| 76 | 184 | `ERROR` | `,` |
| 84 | 280 | `ERROR` | `,` |
| 91 | 193 | `ERROR` | `,` |
| 100 | 121 | `ERROR` | `,` |
| 108 | 175 | `ERROR` | `,` |
| 110 | 115 | `ERROR` | `,` |
| 113 | 150 | `ERROR` | `,` |
| 118 | 143 | `ERROR` | `,` |
| 125 | 169 | `ERROR` | `,` |
| 131 | 169 | `ERROR` | `,` |
| 137 | 205 | `ERROR` | `,` |
| 165 | 177 | `ERROR` | `,` |
| 178 | 99 | `ERROR` | `,` |
| 183 | 167 | `ERROR` | `,` |
| 188 | 160 | `ERROR` | `,` |
| 193 | 110 | `ERROR` | `,` |
| 199 | 105 | `ERROR` | `,` |
| … | … | … | *(17 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_device_unittest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_service_unittest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 30 | `ERROR` | `u "` |
| 20 | 54 | `ERROR` | `"` |
| 35 | 30 | `ERROR` | `u "` |
| 35 | 53 | `ERROR` | `"` |
| 49 | 30 | `ERROR` | `u "` |
| 49 | 61 | `ERROR` | `"` |
| 64 | 30 | `ERROR` | `u "` |
| 64 | 63 | `ERROR` | `"` |
| 69 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |
| 101 | 30 | `ERROR` | `u "` |
| 101 | 59 | `ERROR` | `"` |
| 109 | 118 | `ERROR` | `,` |
| 114 | 120 | `ERROR` | `,` |
| 119 | 118 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |
| 34 | 124 | `ERROR` | `,` |
| 39 | 127 | `ERROR` | `,` |
| 44 | 116 | `ERROR` | `,` |
| 49 | 119 | `ERROR` | `,` |
| 51 | 170 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 113 | `ERROR` | `,` |
| 22 | 116 | `ERROR` | `,` |
| 27 | 77 | `ERROR` | `,` |
| 34 | 85 | `ERROR` | `,` |
| 37 | 83 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_host_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 118 | `ERROR` | `,` |
| 19 | 121 | `ERROR` | `,` |
| 24 | 77 | `ERROR` | `,` |
| 32 | 85 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_preconfig_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 122 | `ERROR` | `,` |
| 14 | 125 | `ERROR` | `,` |
| 19 | 77 | `ERROR` | `,` |
| 24 | 85 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_restore_param_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 123 | `ERROR` | `,` |
| 16 | 126 | `ERROR` | `,` |
| 21 | 115 | `ERROR` | `,` |
| 26 | 118 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/include/hstream_operator_unittest.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 79 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_capture_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 1 | `ERROR` | `class IStreamOperatorFork: public IStreamOperator{ public: DECLARE_HDI_DESCRIPTOR(u` |
| 19 | 64 | `ERROR` | `)` |
| 117 | 118 | `ERROR` | `,` |
| 122 | 121 | `ERROR` | `,` |
| 127 | 110 | `ERROR` | `,` |
| 132 | 113 | `ERROR` | `,` |
| 508 | 97 | `ERROR` | `,` |
| 1210 | 1 | `ERROR` | `}` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_operator_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 119 | `ERROR` | `,` |
| 18 | 122 | `ERROR` | `,` |
| 24 | 111 | `ERROR` | `,` |
| 29 | 114 | `ERROR` | `,` |
| 36 | 131 | `ERROR` | `, , ,` |
| 38 | 133 | `ERROR` | `, , ,` |
| 47 | 131 | `ERROR` | `, , ,` |
| 49 | 133 | `ERROR` | `, , ,` |
| 58 | 131 | `ERROR` | `, , ,` |
| 60 | 133 | `ERROR` | `, , ,` |
| 69 | 131 | `ERROR` | `, , ,` |
| 71 | 133 | `ERROR` | `, , ,` |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 128 | `ERROR` | `,` |
| 17 | 131 | `ERROR` | `,` |
| 23 | 91 | `ERROR` | `,` |
| 32 | 120 | `ERROR` | `,` |
| 37 | 123 | `ERROR` | `,` |
| 92 | 5 | `ERROR` | `=` |
| 93 | 8 | `ERROR` | `=` |
| 94 | 8 | `ERROR` | `=` |
| 95 | 9 | `missing literal_suffix` | `` |
| 95 | 11 | `ERROR` | `1` |
| 96 | 26 | `ERROR` | `.11.10.20.100) "` |
| 96 | 43 | `ERROR` | `"` |
| 104 | 101 | `ERROR` | `,` |
| 132 | 56 | `ERROR` | `1.0` |
| 132 | 78 | `ERROR` | `8` |
| 134 | 1 | `ERROR` | `< strategy` |
| 134 | 89 | `ERROR` | `/>` |
| 135 | 3 | `ERROR` | `strategy` |
| 135 | 87 | `ERROR` | `/>` |
| 136 | 3 | `ERROR` | `strategy` |
| … | … | … | *(7 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_reader_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 127 | `ERROR` | `,` |
| 18 | 130 | `ERROR` | `,` |
| 24 | 91 | `ERROR` | `,` |
| 33 | 119 | `ERROR` | `,` |
| 38 | 122 | `ERROR` | `,` |
| 66 | 5 | `ERROR` | `=` |
| 67 | 8 | `ERROR` | `=` |
| 68 | 8 | `ERROR` | `=` |
| 69 | 9 | `missing literal_suffix` | `` |
| 69 | 11 | `ERROR` | `1` |
| 70 | 26 | `ERROR` | `.11.10.20.100) "` |
| 70 | 43 | `ERROR` | `"` |
| 76 | 91 | `ERROR` | `,` |
| 85 | 101 | `ERROR` | `,` |
| 146 | 46 | `ERROR` | `:) "` |
| 146 | 51 | `ERROR` | `"` |
| 150 | 101 | `ERROR` | `,` |
| 158 | 48 | `ERROR` | `: aaaabbbb) "` |
| 158 | 62 | `ERROR` | `"` |
| 161 | 101 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_sign_tools_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 130 | `ERROR` | `,` |
| 21 | 133 | `ERROR` | `,` |
| 26 | 122 | `ERROR` | `,` |
| 31 | 125 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/pipeline/src/camera_common_pipeline_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 28 | `missing type_identifier` | `` |
| 20 | 32 | `missing type_identifier` | `` |
| 315 | 84 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/cubic_bezier_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 115 | `ERROR` | `,` |
| 17 | 118 | `ERROR` | `,` |
| 22 | 107 | `ERROR` | `,` |
| 27 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/smooth_zoom_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 114 | `ERROR` | `,` |
| 17 | 117 | `ERROR` | `,` |
| 22 | 106 | `ERROR` | `,` |
| 27 | 109 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/ability/src/camera_ability_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 117 | `ERROR` | `,` |
| 17 | 120 | `ERROR` | `,` |
| 22 | 109 | `ERROR` | `,` |
| 27 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/camera_utils/src/camera_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 115 | `ERROR` | `,` |
| 23 | 118 | `ERROR` | `,` |
| 28 | 107 | `ERROR` | `,` |
| 33 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/input/src/camera_framework_input_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 235 | `ERROR` | `,` |
| 578 | 130 | `ERROR` | `,` |
| 581 | 129 | `ERROR` | `,` |
| 586 | 125 | `ERROR` | `,` |
| 590 | 128 | `ERROR` | `,` |
| 595 | 135 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/manager/src/camera_framework_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 280 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/metadata_output_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 128 | `ERROR` | `,` |
| 42 | 157 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/photo_output_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 74 | `missing ;` | `` |
| 1427 | 119 | `ERROR` | `,` |
| 1465 | 117 | `ERROR` | `,` |
| 1471 | 119 | `ERROR` | `,` |
| 1507 | 117 | `ERROR` | `,` |
| 1513 | 111 | `ERROR` | `,` |
| 1555 | 109 | `ERROR` | `,` |
| 1757 | 103 | `ERROR` | `,` |
| 1795 | 101 | `ERROR` | `,` |
| 1801 | 103 | `ERROR` | `,` |
| 1819 | 101 | `ERROR` | `,` |
| 1825 | 103 | `ERROR` | `,` |
| 1861 | 101 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/include/camera_switch_session_unittest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 118 | `ERROR` | `,` |
| 39 | 120 | `ERROR` | `,` |
| 44 | 118 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/include/capture_session_unittest.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 157 | `ERROR` | `,` |
| 25 | 103 | `ERROR` | `,` |
| 31 | 106 | `ERROR` | `,` |
| 40 | 119 | `ERROR` | `,` |
| 44 | 123 | `ERROR` | `,` |
| 57 | 94 | `ERROR` | `,` |
| 65 | 97 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/include/composition_feature_unittest.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 35 | `missing type_identifier` | `` |
| 23 | 60 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/camera_switch_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 108 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 3415 | 106 | `ERROR` | `,` |
| 3462 | 106 | `ERROR` | `,` |
| 3666 | 25 | `ERROR` | `.operator()` |
| 5883 | 112 | `ERROR` | `,` |
| 5914 | 112 | `ERROR` | `,` |
| 5943 | 112 | `ERROR` | `,` |
| 6162 | 121 | `ERROR` | `,` |
| 6191 | 121 | `ERROR` | `,` |
| 11962 | 108 | `ERROR` | `,` |
| 12015 | 108 | `ERROR` | `,` |
| 12070 | 108 | `ERROR` | `,` |
| 12123 | 108 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/cinematic_video_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/mech_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 100 | `ERROR` | `,` |
| 106 | 167 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/moon_capture_boost_feature_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 105 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/night_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 101 | 105 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/panorama_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 106 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 107 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/portrait_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 233 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/profession_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 116 | `ERROR` | `,` |
| 38 | 119 | `ERROR` | `,` |
| 67 | 111 | `ERROR` | `,` |
| 78 | 105 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/scan_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 115 | 107 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/secure_camera_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 109 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/slow_motion_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 117 | `ERROR` | `,` |
| 37 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/stitching_photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 112 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/time_lapse_photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 110 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/video_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 108 | `ERROR` | `,` |

#### `frameworks/native/camera/test/unittest/movie_file/src/hcamera_movie_file_output_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 42 | `missing type_identifier` | `` |
| 10 | 118 | `missing type_identifier` | `` |
| 11 | 39 | `missing type_identifier` | `` |
| 11 | 60 | `missing type_identifier` | `` |
| 12 | 40 | `missing type_identifier` | `` |
| 12 | 44 | `missing type_identifier` | `` |
| 13 | 46 | `missing type_identifier` | `` |
| 13 | 67 | `missing type_identifier` | `` |
| 14 | 36 | `missing type_identifier` | `` |
| 14 | 40 | `missing type_identifier` | `` |
| 15 | 41 | `missing type_identifier` | `` |
| 15 | 127 | `missing type_identifier` | `` |
| 16 | 63 | `missing type_identifier` | `` |
| 16 | 67 | `missing type_identifier` | `` |
| 17 | 36 | `missing type_identifier` | `` |
| 17 | 104 | `missing type_identifier` | `` |
| 18 | 39 | `missing type_identifier` | `` |
| 18 | 73 | `missing type_identifier` | `` |
| 19 | 37 | `missing type_identifier` | `` |
| 19 | 41 | `missing type_identifier` | `` |
| … | … | … | *(52 more)* |

#### `frameworks/native/ndk/camera_input.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 163 | `ERROR` | `, , ,` |
| 8 | 157 | `ERROR` | `, , ,` |
| 10 | 149 | `ERROR` | `, , ,` |
| 17 | 163 | `ERROR` | `, , ,` |
| 18 | 157 | `ERROR` | `, , ,` |
| 20 | 149 | `ERROR` | `, , ,` |
| 27 | 163 | `ERROR` | `, , ,` |
| 34 | 163 | `ERROR` | `, , ,` |
| 35 | 163 | `ERROR` | `, , ,` |
| 43 | 163 | `ERROR` | `, , ,` |
| 51 | 163 | `ERROR` | `, , ,` |
| 58 | 163 | `ERROR` | `, , ,` |
| 95 | 163 | `ERROR` | `, , ,` |

#### `frameworks/native/ndk/camera_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 141 | `ERROR` | `, , ,` |
| 10 | 145 | `ERROR` | `, , ,` |
| 19 | 141 | `ERROR` | `, , ,` |
| 21 | 145 | `ERROR` | `, , ,` |
| 30 | 141 | `ERROR` | `, , ,` |
| 38 | 141 | `ERROR` | `, , ,` |
| 46 | 141 | `ERROR` | `, , ,` |
| 47 | 157 | `ERROR` | `, , ,` |
| 58 | 141 | `ERROR` | `, , ,` |
| 59 | 157 | `ERROR` | `, , ,` |
| 71 | 141 | `ERROR` | `, , ,` |
| 73 | 142 | `ERROR` | `, , ,` |
| 82 | 141 | `ERROR` | `, , ,` |
| 84 | 142 | `ERROR` | `, , ,` |
| 93 | 141 | `ERROR` | `, , ,` |
| 94 | 155 | `ERROR` | `, , ,` |
| 95 | 149 | `ERROR` | `, , ,` |
| 104 | 141 | `ERROR` | `, , ,` |
| 105 | 166 | `ERROR` | `, , ,` |
| 114 | 141 | `ERROR` | `, , ,` |
| … | … | … | *(120 more)* |

#### `frameworks/native/ndk/capture_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 155 | `ERROR` | `, , ,` |
| 10 | 151 | `ERROR` | `, , ,` |
| 19 | 155 | `ERROR` | `, , ,` |
| 21 | 151 | `ERROR` | `, , ,` |
| 30 | 155 | `ERROR` | `, , ,` |
| 32 | 142 | `ERROR` | `, , ,` |
| 40 | 155 | `ERROR` | `, , ,` |
| 42 | 142 | `ERROR` | `, , ,` |
| 49 | 127 | `ERROR` | `,` |
| 50 | 155 | `ERROR` | `, , ,` |
| 57 | 122 | `ERROR` | `,` |
| 58 | 155 | `ERROR` | `, , ,` |
| 64 | 155 | `ERROR` | `, , ,` |
| 65 | 157 | `ERROR` | `, , ,` |
| 66 | 241 | `ERROR` | `, , ,` |
| 76 | 155 | `ERROR` | `, , ,` |
| 77 | 157 | `ERROR` | `, , ,` |
| 78 | 241 | `ERROR` | `, , ,` |
| 88 | 155 | `ERROR` | `, , ,` |
| 90 | 145 | `ERROR` | `, , ,` |
| … | … | … | *(279 more)* |

#### `frameworks/native/ndk/impl/camera_input_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 159 | `ERROR` | `,` |
| 32 | 231 | `ERROR` | `,` |
| 49 | 106 | `ERROR` | `,` |
| 54 | 95 | `ERROR` | `,` |

#### `frameworks/native/ndk/impl/camera_manager_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 103 | 104 | `ERROR` | `,` |
| 109 | 159 | `ERROR` | `,` |
| 121 | 108 | `ERROR` | `,` |
| 140 | 102 | `ERROR` | `,` |
| 163 | 102 | `ERROR` | `,` |
| 167 | 131 | `ERROR` | `, , ,` |
| 200 | 108 | `ERROR` | `,` |
| 206 | 97 | `ERROR` | `,` |
| 247 | 98 | `ERROR` | `,` |
| 254 | 140 | `ERROR` | `, , ,` |
| 264 | 108 | `ERROR` | `,` |
| 307 | 159 | `ERROR` | `,` |
| 329 | 114 | `ERROR` | `,` |
| 338 | 102 | `ERROR` | `,` |
| 342 | 52 | `ERROR` | `*[previewProfiles` |
| 342 | 77 | `ERROR` | `]` |
| 343 | 183 | `ERROR` | `, , ,` |
| 344 | 131 | `ERROR` | `,` |
| 347 | 167 | `ERROR` | `, , ,` |
| 353 | 130 | `ERROR` | `,` |
| … | … | … | *(95 more)* |

#### `frameworks/native/ndk/impl/camera_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 170 | `ERROR` | `,` |

#### `frameworks/native/ndk/impl/capture_session_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 105 | 97 | `ERROR` | `,` |
| 112 | 159 | `ERROR` | `,` |
| 131 | 95 | `ERROR` | `,` |
| 152 | 113 | `ERROR` | `,` |
| 174 | 105 | `ERROR` | `,` |
| 194 | 116 | `ERROR` | `,` |
| 214 | 139 | `ERROR` | `,` |
| 225 | 219 | `ERROR` | `,` |
| 243 | 110 | `ERROR` | `,` |
| 263 | 103 | `ERROR` | `,` |
| 283 | 106 | `ERROR` | `,` |
| 307 | 108 | `ERROR` | `,` |
| 328 | 97 | `ERROR` | `,` |
| 345 | 200 | `ERROR` | `,` |
| 345 | 216 | `ERROR` | `,` |
| 345 | 232 | `ERROR` | `,` |
| 353 | 115 | `ERROR` | `,` |
| 358 | 104 | `ERROR` | `,` |
| 366 | 155 | `ERROR` | `, , ,` |
| 387 | 143 | `ERROR` | `, , ,` |
| … | … | … | *(138 more)* |

#### `frameworks/native/ndk/impl/metadata_object_ext_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 118 | `ERROR` | `,` |
| 16 | 110 | `ERROR` | `,` |

#### `frameworks/native/ndk/impl/metadata_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 159 | `ERROR` | `,` |
| 35 | 127 | `ERROR` | `,` |
| 36 | 172 | `ERROR` | `, , ,` |
| 65 | 115 | `ERROR` | `,` |
| 70 | 104 | `ERROR` | `,` |
| 129 | 117 | `ERROR` | `,` |
| 144 | 120 | `ERROR` | `,` |
| 181 | 191 | `ERROR` | `,` |
| 202 | 130 | `ERROR` | `,` |
| 205 | 172 | `ERROR` | `, , ,` |
| 209 | 75 | `ERROR` | `*[size]` |

#### `frameworks/native/ndk/impl/photo_listener_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 103 | `ERROR` | `,` |
| 27 | 148 | `ERROR` | `, , ,` |
| 34 | 165 | `ERROR` | `, , ,` |
| 39 | 117 | `ERROR` | `,` |
| 43 | 153 | `ERROR` | `, , ,` |
| 46 | 121 | `ERROR` | `,` |
| 50 | 111 | `ERROR` | `,` |
| 53 | 115 | `ERROR` | `,` |
| 56 | 102 | `ERROR` | `,` |
| 79 | 145 | `ERROR` | `,` |
| 81 | 106 | `ERROR` | `,` |
| 90 | 160 | `ERROR` | `,` |
| 108 | 160 | `ERROR` | `,` |
| 120 | 112 | `ERROR` | `,` |
| 122 | 121 | `ERROR` | `,` |
| 130 | 119 | `ERROR` | `,` |
| 132 | 128 | `ERROR` | `,` |
| 140 | 117 | `ERROR` | `,` |
| 142 | 126 | `ERROR` | `,` |
| 150 | 124 | `ERROR` | `,` |
| … | … | … | *(21 more)* |

#### `frameworks/native/ndk/impl/photo_native_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 108 | `ERROR` | `,` |
| 13 | 97 | `ERROR` | `,` |

#### `frameworks/native/ndk/impl/photo_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 112 | `ERROR` | `,` |
| 25 | 101 | `ERROR` | `,` |
| 38 | 151 | `ERROR` | `, , ,` |
| 41 | 157 | `ERROR` | `, , ,` |
| 52 | 151 | `ERROR` | `, , ,` |
| 55 | 157 | `ERROR` | `, , ,` |
| 70 | 151 | `ERROR` | `, , ,` |
| 73 | 157 | `ERROR` | `, , ,` |
| 95 | 151 | `ERROR` | `, , ,` |
| 98 | 157 | `ERROR` | `, , ,` |
| 118 | 151 | `ERROR` | `, , ,` |
| 121 | 157 | `ERROR` | `, , ,` |
| 140 | 151 | `ERROR` | `, , ,` |
| 143 | 157 | `ERROR` | `, , ,` |
| 162 | 151 | `ERROR` | `, , ,` |
| 165 | 157 | `ERROR` | `, , ,` |
| 185 | 151 | `ERROR` | `, , ,` |
| 188 | 157 | `ERROR` | `, , ,` |
| 208 | 151 | `ERROR` | `, , ,` |
| 209 | 145 | `ERROR` | `, , ,` |
| … | … | … | *(21 more)* |

#### `frameworks/native/ndk/impl/photo_output_impl.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 137 | 168 | `ERROR` | `,` |
| 148 | 168 | `ERROR` | `,` |
| 157 | 166 | `ERROR` | `,` |
| 168 | 169 | `ERROR` | `,` |
| 179 | 166 | `ERROR` | `,` |
| 187 | 176 | `ERROR` | `,` |
| 195 | 165 | `ERROR` | `,` |
| 196 | 167 | `ERROR` | `,` |
| 207 | 166 | `ERROR` | `,` |
| 215 | 115 | `ERROR` | `,` |
| 220 | 117 | `ERROR` | `,` |
| 225 | 90 | `ERROR` | `,` |
| 226 | 145 | `ERROR` | `, , ,` |
| 227 | 153 | `ERROR` | `, , ,` |
| 229 | 151 | `ERROR` | `, , ,` |
| 236 | 90 | `ERROR` | `,` |
| 241 | 90 | `ERROR` | `,` |
| 242 | 145 | `ERROR` | `, , ,` |
| 243 | 153 | `ERROR` | `, , ,` |
| 245 | 151 | `ERROR` | `, , ,` |
| … | … | … | *(7 more)* |

#### `frameworks/native/ndk/impl/preview_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 97 | `ERROR` | `,` |
| 31 | 166 | `ERROR` | `,` |
| 38 | 159 | `ERROR` | `,` |
| 45 | 96 | `ERROR` | `,` |
| 50 | 97 | `ERROR` | `,` |
| 64 | 114 | `ERROR` | `,` |
| 69 | 103 | `ERROR` | `,` |
| 149 | 95 | `ERROR` | `,` |
| 180 | 160 | `ERROR` | `, , ,` |
| 191 | 162 | `ERROR` | `, , ,` |
| 202 | 162 | `ERROR` | `, , ,` |
| 248 | 153 | `ERROR` | `, , ,` |
| 251 | 156 | `ERROR` | `, , ,` |
| 262 | 127 | `ERROR` | `,` |
| 268 | 125 | `ERROR` | `,` |

#### `frameworks/native/ndk/impl/video_output_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 97 | `ERROR` | `,` |
| 30 | 166 | `ERROR` | `,` |
| 36 | 159 | `ERROR` | `,` |
| 43 | 113 | `ERROR` | `,` |
| 53 | 112 | `ERROR` | `,` |
| 58 | 101 | `ERROR` | `,` |
| 101 | 159 | `ERROR` | `, , ,` |
| 131 | 159 | `ERROR` | `, , ,` |
| 144 | 95 | `ERROR` | `,` |
| 175 | 160 | `ERROR` | `, , ,` |
| 185 | 159 | `ERROR` | `, , ,` |
| 186 | 147 | `ERROR` | `, , ,` |
| 193 | 159 | `ERROR` | `, , ,` |
| 200 | 162 | `ERROR` | `, , ,` |
| 201 | 159 | `ERROR` | `, , ,` |
| 211 | 162 | `ERROR` | `, , ,` |
| 212 | 159 | `ERROR` | `, , ,` |

#### `frameworks/native/ndk/metadata_object_ext.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 149 | `ERROR` | `, , ,` |
| 12 | 123 | `ERROR` | `, , ,` |
| 22 | 149 | `ERROR` | `, , ,` |
| 24 | 133 | `ERROR` | `, , ,` |
| 34 | 149 | `ERROR` | `, , ,` |
| 36 | 137 | `ERROR` | `, , ,` |
| 46 | 149 | `ERROR` | `, , ,` |
| 48 | 135 | `ERROR` | `, , ,` |
| 58 | 149 | `ERROR` | `, , ,` |
| 60 | 131 | `ERROR` | `, , ,` |
| 70 | 149 | `ERROR` | `, , ,` |
| 72 | 133 | `ERROR` | `, , ,` |
| 82 | 149 | `ERROR` | `, , ,` |
| 84 | 137 | `ERROR` | `, , ,` |
| 94 | 149 | `ERROR` | `, , ,` |
| 96 | 137 | `ERROR` | `, , ,` |
| 106 | 149 | `ERROR` | `, , ,` |
| 108 | 129 | `ERROR` | `, , ,` |
| 117 | 175 | `ERROR` | `, , ,` |
| 129 | 149 | `ERROR` | `, , ,` |

#### `frameworks/native/ndk/metadata_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 143 | `ERROR` | `, , ,` |
| 11 | 157 | `ERROR` | `, , ,` |
| 15 | 149 | `ERROR` | `, , ,` |
| 26 | 142 | `ERROR` | `, , ,` |
| 27 | 157 | `ERROR` | `, , ,` |
| 31 | 149 | `ERROR` | `, , ,` |
| 41 | 143 | `ERROR` | `, , ,` |
| 50 | 143 | `ERROR` | `, , ,` |
| 59 | 143 | `ERROR` | `, , ,` |
| 73 | 143 | `ERROR` | `, , ,` |
| 75 | 125 | `ERROR` | `, , ,` |
| 77 | 114 | `ERROR` | `, , ,` |
| 87 | 112 | `ERROR` | `,` |
| 97 | 143 | `ERROR` | `, , ,` |
| 98 | 115 | `ERROR` | `,` |
| 100 | 125 | `ERROR` | `, , ,` |
| 102 | 114 | `ERROR` | `, , ,` |
| 120 | 143 | `ERROR` | `, , ,` |
| 130 | 143 | `ERROR` | `, , ,` |
| 132 | 145 | `ERROR` | `, , ,` |
| … | … | … | *(9 more)* |

#### `frameworks/native/ndk/photo_native.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 151 | `ERROR` | `, , ,` |
| 8 | 154 | `ERROR` | `, , ,` |
| 23 | 151 | `ERROR` | `, , ,` |

#### `frameworks/native/ndk/photo_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 163 | `ERROR` | `, , ,` |
| 9 | 157 | `ERROR` | `, , ,` |
| 11 | 179 | `ERROR` | `, , ,` |
| 21 | 163 | `ERROR` | `, , ,` |
| 22 | 157 | `ERROR` | `, , ,` |
| 24 | 179 | `ERROR` | `, , ,` |
| 35 | 163 | `ERROR` | `, , ,` |
| 36 | 157 | `ERROR` | `, , ,` |
| 45 | 163 | `ERROR` | `, , ,` |
| 46 | 157 | `ERROR` | `, , ,` |
| 55 | 163 | `ERROR` | `, , ,` |
| 56 | 157 | `ERROR` | `, , ,` |
| 65 | 163 | `ERROR` | `, , ,` |
| 66 | 157 | `ERROR` | `, , ,` |
| 75 | 163 | `ERROR` | `, , ,` |
| 76 | 157 | `ERROR` | `, , ,` |
| 85 | 163 | `ERROR` | `, , ,` |
| 86 | 157 | `ERROR` | `, , ,` |
| 95 | 163 | `ERROR` | `, , ,` |
| 96 | 157 | `ERROR` | `, , ,` |
| … | … | … | *(42 more)* |

#### `frameworks/native/ndk/preview_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 141 | `ERROR` | `, , ,` |
| 11 | 157 | `ERROR` | `, , ,` |
| 13 | 159 | `ERROR` | `, , ,` |
| 15 | 155 | `ERROR` | `, , ,` |
| 17 | 149 | `ERROR` | `, , ,` |
| 28 | 141 | `ERROR` | `, , ,` |
| 29 | 157 | `ERROR` | `, , ,` |
| 31 | 159 | `ERROR` | `, , ,` |
| 33 | 155 | `ERROR` | `, , ,` |
| 35 | 149 | `ERROR` | `, , ,` |
| 45 | 141 | `ERROR` | `, , ,` |
| 54 | 141 | `ERROR` | `, , ,` |
| 63 | 141 | `ERROR` | `, , ,` |
| 75 | 116 | `ERROR` | `,` |
| 77 | 141 | `ERROR` | `, , ,` |
| 78 | 155 | `ERROR` | `, , ,` |
| 86 | 113 | `ERROR` | `,` |
| 87 | 155 | `ERROR` | `, , ,` |
| 99 | 141 | `ERROR` | `, , ,` |
| 101 | 143 | `ERROR` | `, , ,` |
| … | … | … | *(26 more)* |

#### `frameworks/native/ndk/video_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 163 | `ERROR` | `, , ,` |
| 9 | 157 | `ERROR` | `, , ,` |
| 11 | 159 | `ERROR` | `, , ,` |
| 13 | 155 | `ERROR` | `, , ,` |
| 15 | 149 | `ERROR` | `, , ,` |
| 24 | 163 | `ERROR` | `, , ,` |
| 25 | 157 | `ERROR` | `, , ,` |
| 27 | 159 | `ERROR` | `, , ,` |
| 29 | 155 | `ERROR` | `, , ,` |
| 31 | 149 | `ERROR` | `, , ,` |
| 40 | 163 | `ERROR` | `, , ,` |
| 48 | 163 | `ERROR` | `, , ,` |
| 56 | 163 | `ERROR` | `, , ,` |
| 68 | 114 | `ERROR` | `,` |
| 69 | 163 | `ERROR` | `, , ,` |
| 70 | 155 | `ERROR` | `, , ,` |
| 78 | 111 | `ERROR` | `,` |
| 79 | 155 | `ERROR` | `, , ,` |
| 90 | 163 | `ERROR` | `, , ,` |
| 92 | 143 | `ERROR` | `, , ,` |
| … | … | … | *(15 more)* |

#### `frameworks/taihe/include/camera_auto_ref_taihe.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 103 | `ERROR` | `,` |

#### `frameworks/taihe/include/camera_event_emitter_taihe.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 14 | `ERROR` | `T::` |
| 12 | 14 | `ERROR` | `T::` |
| 21 | 180 | `ERROR` | `,` |
| 25 | 11 | `ERROR` | `->` |
| 27 | 119 | `ERROR` | `,` |
| 36 | 195 | `ERROR` | `,` |
| 40 | 11 | `ERROR` | `->` |
| 42 | 121 | `ERROR` | `,` |

#### `frameworks/taihe/include/camera_template_utils_taihe.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 84 | `ERROR` | `,` |
| 68 | 84 | `ERROR` | `,` |
| 79 | 84 | `ERROR` | `,` |
| 89 | 155 | `ERROR` | `,` |
| 103 | 155 | `ERROR` | `,` |
| 117 | 155 | `ERROR` | `,` |

#### `frameworks/taihe/include/listener_base_taihe.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 20 | `ERROR` | `. . .` |
| 43 | 106 | `ERROR` | `. . .` |
| 50 | 61 | `ERROR` | `. . .` |
| 51 | 75 | `ERROR` | `. . .` |
| 53 | 62 | `ERROR` | `. . .` |
| 62 | 20 | `ERROR` | `. . .` |
| 63 | 60 | `ERROR` | `. . .` |
| 70 | 49 | `ERROR` | `. . .` |
| 71 | 63 | `ERROR` | `. . .` |
| 72 | 26 | `ERROR` | `. . .` |

#### `frameworks/taihe/include/transfer/camera_lib_manager_taihe.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 34 | `ERROR` | `,` |
| 58 | 185 | `ERROR` | `,` |
| 64 | 201 | `ERROR` | `,` |

#### `frameworks/taihe/src/camera_constructor_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 2 | 12 | `ERROR` | `ani_status` |

#### `frameworks/taihe/src/camera_picker_constructor_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 3 | 12 | `ERROR` | `ani_status` |

#### `frameworks/taihe/src/camera_utils_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 134 | 120 | `ERROR` | `, , ,` |
| 197 | 120 | `ERROR` | `, , ,` |
| 253 | 156 | `ERROR` | `,` |
| 256 | 186 | `ERROR` | `,` |
| 320 | 116 | `ERROR` | `,` |
| 523 | 131 | `ERROR` | `, , ,` |
| 546 | 128 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/camera_worker_queue_keeper_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 225 | `ERROR` | `,` |
| 61 | 156 | `ERROR` | `, , ,` |
| 91 | 224 | `ERROR` | `,` |

#### `frameworks/taihe/src/capture_photo_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 93 | `ERROR` | `,` |
| 25 | 189 | `ERROR` | `,` |
| 31 | 200 | `ERROR` | `,` |
| 37 | 111 | `ERROR` | `,` |

#### `frameworks/taihe/src/deferred_photo_proxy_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 203 | `ERROR` | `, , ,` |
| 21 | 195 | `ERROR` | `, , ,` |
| 25 | 183 | `ERROR` | `, , ,` |
| 29 | 199 | `ERROR` | `, , ,` |
| 36 | 195 | `ERROR` | `,` |

#### `frameworks/taihe/src/input/camera_input_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 191 | `ERROR` | `,` |
| 20 | 166 | `ERROR` | `,` |
| 25 | 167 | `ERROR` | `, , ,` |
| 32 | 244 | `ERROR` | `,` |
| 53 | 167 | `ERROR` | `, , ,` |
| 65 | 90 | `ERROR` | `,` |
| 68 | 149 | `ERROR` | `, , ,` |
| 74 | 162 | `ERROR` | `, , ,` |
| 85 | 107 | `ERROR` | `,` |
| 95 | 162 | `ERROR` | `, , ,` |
| 96 | 199 | `ERROR` | `,` |
| 113 | 112 | `ERROR` | `,` |
| 116 | 149 | `ERROR` | `, , ,` |
| 124 | 162 | `ERROR` | `, , ,` |
| 133 | 91 | `ERROR` | `,` |
| 136 | 149 | `ERROR` | `, , ,` |
| 142 | 162 | `ERROR` | `, , ,` |
| 153 | 163 | `ERROR` | `, , ,` |
| 170 | 173 | `ERROR` | `, , ,` |
| 172 | 168 | `ERROR` | `, , ,` |
| … | … | … | *(18 more)* |

#### `frameworks/taihe/src/input/camera_manager_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 113 | `ERROR` | `,` |
| 18 | 114 | `ERROR` | `,` |
| 28 | 167 | `ERROR` | `, , ,` |
| 31 | 174 | `ERROR` | `,` |
| 36 | 172 | `ERROR` | `,` |
| 52 | 112 | `ERROR` | `,` |
| 58 | 113 | `ERROR` | `,` |
| 75 | 167 | `ERROR` | `, , ,` |
| 78 | 233 | `ERROR` | `,` |
| 85 | 231 | `ERROR` | `,` |
| 104 | 104 | `ERROR` | `,` |
| 109 | 105 | `ERROR` | `,` |
| 118 | 167 | `ERROR` | `, , ,` |
| 120 | 169 | `ERROR` | `,` |
| 125 | 161 | `ERROR` | `,` |
| 148 | 167 | `ERROR` | `, , ,` |
| 155 | 196 | `ERROR` | `,` |
| 168 | 99 | `ERROR` | `,` |
| 173 | 101 | `ERROR` | `,` |
| 189 | 167 | `ERROR` | `, , ,` |
| … | … | … | *(77 more)* |

#### `frameworks/taihe/src/listener_base_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 95 | `ERROR` | `,` |
| 16 | 96 | `ERROR` | `,` |
| 27 | 127 | `ERROR` | `,` |
| 35 | 33 | `ERROR` | `,` |
| 42 | 149 | `ERROR` | `,` |
| 50 | 178 | `ERROR` | `,` |
| 55 | 162 | `ERROR` | `,` |
| 63 | 122 | `ERROR` | `,` |

#### `frameworks/taihe/src/mode/light_painting_photo_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 213 | `ERROR` | `, , ,` |
| 34 | 104 | `ERROR` | `,` |
| 38 | 222 | `ERROR` | `,` |
| 38 | 234 | `ERROR` | `,` |
| 38 | 246 | `ERROR` | `,` |
| 43 | 207 | `ERROR` | `, , ,` |
| 52 | 222 | `ERROR` | `,` |
| 52 | 234 | `ERROR` | `,` |
| 52 | 246 | `ERROR` | `,` |

#### `frameworks/taihe/src/mode/night_photo_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 172 | `ERROR` | `, , ,` |
| 23 | 172 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/mode/photo_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 151 | `ERROR` | `, , ,` |
| 23 | 151 | `ERROR` | `, , ,` |
| 51 | 180 | `ERROR` | `,` |
| 58 | 101 | `ERROR` | `,` |
| 59 | 168 | `ERROR` | `, , ,` |
| 71 | 101 | `ERROR` | `,` |
| 73 | 150 | `ERROR` | `, , ,` |
| 84 | 195 | `ERROR` | `,` |
| 95 | 116 | `ERROR` | `,` |
| 96 | 148 | `ERROR` | `, , ,` |
| 107 | 124 | `ERROR` | `,` |
| 108 | 164 | `ERROR` | `, , ,` |
| 119 | 118 | `ERROR` | `,` |
| 127 | 151 | `ERROR` | `, , ,` |
| 140 | 182 | `ERROR` | `,` |
| 146 | 151 | `ERROR` | `, , ,` |
| 167 | 167 | `ERROR` | `, , ,` |
| 188 | 176 | `ERROR` | `, , ,` |
| 196 | 176 | `ERROR` | `, , ,` |
| 204 | 184 | `ERROR` | `, , ,` |
| … | … | … | *(17 more)* |

#### `frameworks/taihe/src/mode/portrait_photo_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 13 | 148 | `ERROR` | `, , ,` |
| 24 | 132 | `ERROR` | `,` |
| 25 | 164 | `ERROR` | `, , ,` |
| 37 | 126 | `ERROR` | `,` |
| 45 | 157 | `ERROR` | `, , ,` |
| 53 | 108 | `ERROR` | `,` |
| 56 | 157 | `ERROR` | `, , ,` |
| 64 | 214 | `ERROR` | `, , ,` |
| 74 | 99 | `ERROR` | `,` |
| 77 | 172 | `ERROR` | `, , ,` |
| 85 | 99 | `ERROR` | `,` |
| 89 | 172 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/mode/professional_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 161 | `ERROR` | `, , ,` |
| 21 | 193 | `ERROR` | `,` |
| 34 | 180 | `ERROR` | `,` |
| 42 | 177 | `ERROR` | `, , ,` |
| 96 | 161 | `ERROR` | `, , ,` |
| 120 | 159 | `ERROR` | `, , ,` |
| 132 | 153 | `ERROR` | `, , ,` |
| 139 | 167 | `ERROR` | `,` |
| 145 | 106 | `ERROR` | `,` |
| 152 | 167 | `ERROR` | `, , ,` |
| 159 | 159 | `ERROR` | `, , ,` |
| 171 | 163 | `ERROR` | `, , ,` |
| 178 | 159 | `ERROR` | `, , ,` |
| 190 | 163 | `ERROR` | `, , ,` |
| 197 | 159 | `ERROR` | `, , ,` |
| 203 | 184 | `ERROR` | `,` |
| 215 | 102 | `ERROR` | `,` |
| 221 | 184 | `ERROR` | `,` |
| 228 | 192 | `ERROR` | `,` |
| 234 | 113 | `ERROR` | `,` |
| … | … | … | *(1 more)* |

#### `frameworks/taihe/src/mode/secure_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 96 | `ERROR` | `,` |
| 12 | 165 | `ERROR` | `, , ,` |
| 15 | 158 | `ERROR` | `, , ,` |
| 17 | 172 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/mode/secure_session_taihe_for_sys.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 96 | `ERROR` | `,` |
| 12 | 177 | `ERROR` | `, , ,` |
| 15 | 158 | `ERROR` | `, , ,` |
| 17 | 172 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/mode/slow_motion_video_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 124 | `ERROR` | `,` |
| 16 | 126 | `ERROR` | `,` |
| 23 | 167 | `ERROR` | `, , ,` |
| 29 | 126 | `ERROR` | `,` |
| 35 | 128 | `ERROR` | `,` |
| 47 | 167 | `ERROR` | `, , ,` |
| 54 | 106 | `ERROR` | `,` |
| 61 | 159 | `ERROR` | `, , ,` |
| 67 | 104 | `ERROR` | `,` |
| 73 | 108 | `ERROR` | `,` |
| 74 | 154 | `ERROR` | `, , ,` |
| 76 | 106 | `ERROR` | `,` |
| 82 | 107 | `ERROR` | `,` |
| 84 | 153 | `ERROR` | `, , ,` |
| 95 | 105 | `ERROR` | `,` |
| 100 | 109 | `ERROR` | `,` |
| 101 | 156 | `ERROR` | `, , ,` |
| 109 | 158 | `ERROR` | `, , ,` |
| 117 | 158 | `ERROR` | `, , ,` |

#### `frameworks/taihe/src/mode/time_lapse_photo_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 167 | `ERROR` | `,` |
| 17 | 106 | `ERROR` | `,` |
| 24 | 167 | `ERROR` | `, , ,` |
| 30 | 192 | `ERROR` | `,` |
| 37 | 113 | `ERROR` | `,` |
| 44 | 167 | `ERROR` | `, , ,` |
| 50 | 185 | `ERROR` | `,` |
| 56 | 111 | `ERROR` | `,` |
| 63 | 167 | `ERROR` | `, , ,` |
| 69 | 100 | `ERROR` | `,` |
| 75 | 108 | `ERROR` | `,` |
| 88 | 167 | `ERROR` | `, , ,` |
| 95 | 167 | `ERROR` | `, , ,` |
| 107 | 153 | `ERROR` | `, , ,` |
| 119 | 186 | `ERROR` | `,` |
| 122 | 167 | `ERROR` | `, , ,` |
| 132 | 102 | `ERROR` | `,` |
| 138 | 186 | `ERROR` | `,` |
| 146 | 167 | `ERROR` | `, , ,` |
| 158 | 163 | `ERROR` | `, , ,` |
| … | … | … | *(13 more)* |

#### `frameworks/taihe/src/mode/video_session_for_sys_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 147 | `ERROR` | `,` |
| 15 | 163 | `ERROR` | `, , ,` |
| 57 | 167 | `ERROR` | `, , ,` |
| 65 | 137 | `ERROR` | `,` |
| 66 | 161 | `ERROR` | `, , ,` |
| 79 | 142 | `ERROR` | `,` |
| 81 | 95 | `ERROR` | `,` |
| 89 | 189 | `ERROR` | `,` |
| 96 | 167 | `ERROR` | `, , ,` |
| 102 | 178 | `ERROR` | `,` |

#### `frameworks/taihe/src/mode/video_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 187 | `ERROR` | `, , ,` |
| 20 | 141 | `ERROR` | `,` |
| 23 | 213 | `ERROR` | `,` |
| 23 | 230 | `ERROR` | `,` |
| 23 | 247 | `ERROR` | `,` |
| 33 | 141 | `ERROR` | `,` |
| 35 | 248 | `ERROR` | `,` |
| 35 | 265 | `ERROR` | `,` |
| 35 | 282 | `ERROR` | `,` |
| 47 | 141 | `ERROR` | `,` |
| 50 | 213 | `ERROR` | `,` |
| 50 | 230 | `ERROR` | `,` |
| 50 | 247 | `ERROR` | `,` |
| 60 | 141 | `ERROR` | `,` |
| 62 | 248 | `ERROR` | `,` |
| 62 | 265 | `ERROR` | `,` |
| 62 | 282 | `ERROR` | `,` |
| 64 | 213 | `ERROR` | `,` |
| 64 | 230 | `ERROR` | `,` |
| 64 | 247 | `ERROR` | `,` |
| … | … | … | *(58 more)* |

#### `frameworks/taihe/src/output/depth_data_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 91 | `ERROR` | `,` |
| 39 | 157 | `ERROR` | `, , ,` |
| 45 | 166 | `ERROR` | `, , ,` |
| 54 | 90 | `ERROR` | `,` |
| 59 | 157 | `ERROR` | `, , ,` |
| 65 | 166 | `ERROR` | `, , ,` |
| 74 | 93 | `ERROR` | `,` |
| 77 | 157 | `ERROR` | `, , ,` |
| 83 | 166 | `ERROR` | `, , ,` |
| 119 | 155 | `ERROR` | `, , ,` |
| 133 | 157 | `ERROR` | `, , ,` |
| 140 | 167 | `ERROR` | `,` |
| 146 | 106 | `ERROR` | `,` |
| 151 | 167 | `ERROR` | `, , ,` |
| 170 | 159 | `ERROR` | `, , ,` |
| 172 | 133 | `ERROR` | `,` |
| 178 | 175 | `ERROR` | `, , ,` |
| 198 | 123 | `ERROR` | `,` |
| 204 | 107 | `ERROR` | `,` |
| 205 | 148 | `ERROR` | `, , ,` |
| … | … | … | *(9 more)* |

#### `frameworks/taihe/src/output/depth_data_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 93 | `ERROR` | `,` |

#### `frameworks/taihe/src/output/metadata_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 91 | `ERROR` | `,` |
| 22 | 155 | `ERROR` | `, , ,` |
| 28 | 173 | `ERROR` | `, , ,` |
| 29 | 183 | `ERROR` | `, , ,` |
| 38 | 90 | `ERROR` | `,` |
| 41 | 155 | `ERROR` | `, , ,` |
| 47 | 165 | `ERROR` | `, , ,` |
| 56 | 93 | `ERROR` | `,` |
| 59 | 155 | `ERROR` | `, , ,` |
| 65 | 165 | `ERROR` | `, , ,` |
| 89 | 153 | `ERROR` | `, , ,` |
| 101 | 158 | `ERROR` | `, , ,` |
| 107 | 172 | `ERROR` | `,` |
| 113 | 111 | `ERROR` | `,` |
| 118 | 167 | `ERROR` | `, , ,` |
| 135 | 153 | `ERROR` | `, , ,` |
| 147 | 177 | `ERROR` | `, , ,` |
| 154 | 108 | `ERROR` | `,` |
| 161 | 116 | `ERROR` | `,` |
| 169 | 167 | `ERROR` | `, , ,` |
| … | … | … | *(12 more)* |

#### `frameworks/taihe/src/output/movie_file_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 91 | `ERROR` | `,` |
| 24 | 157 | `ERROR` | `, , ,` |
| 30 | 149 | `ERROR` | `, , ,` |
| 31 | 169 | `ERROR` | `, , ,` |
| 32 | 184 | `ERROR` | `, , ,` |
| 34 | 184 | `ERROR` | `,` |
| 42 | 90 | `ERROR` | `,` |
| 47 | 157 | `ERROR` | `, , ,` |
| 53 | 149 | `ERROR` | `, , ,` |
| 54 | 169 | `ERROR` | `, , ,` |
| 55 | 184 | `ERROR` | `, , ,` |
| 57 | 183 | `ERROR` | `,` |
| 64 | 91 | `ERROR` | `,` |
| 69 | 157 | `ERROR` | `, , ,` |
| 75 | 149 | `ERROR` | `, , ,` |
| 76 | 169 | `ERROR` | `, , ,` |
| 78 | 155 | `ERROR` | `, , ,` |
| 80 | 184 | `ERROR` | `,` |
| 87 | 92 | `ERROR` | `,` |
| 92 | 157 | `ERROR` | `, , ,` |
| … | … | … | *(54 more)* |

#### `frameworks/taihe/src/output/photo_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 225 | `ERROR` | `,` |
| 28 | 167 | `ERROR` | `, , ,` |
| 35 | 119 | `ERROR` | `,` |
| 46 | 209 | `ERROR` | `,` |
| 60 | 167 | `ERROR` | `, , ,` |
| 66 | 119 | `ERROR` | `,` |
| 79 | 167 | `ERROR` | `, , ,` |
| 86 | 131 | `ERROR` | `,` |
| 93 | 125 | `ERROR` | `,` |
| 95 | 167 | `ERROR` | `, , ,` |
| 112 | 167 | `ERROR` | `, , ,` |
| 119 | 211 | `ERROR` | `,` |
| 131 | 167 | `ERROR` | `, , ,` |
| 149 | 167 | `ERROR` | `, , ,` |
| 166 | 167 | `ERROR` | `, , ,` |
| 180 | 167 | `ERROR` | `, , ,` |
| 186 | 209 | `ERROR` | `,` |
| 192 | 167 | `ERROR` | `, , ,` |
| 198 | 175 | `ERROR` | `,` |
| 205 | 167 | `ERROR` | `, , ,` |
| … | … | … | *(106 more)* |

#### `frameworks/taihe/src/output/preview_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 107 | `ERROR` | `,` |
| 16 | 104 | `ERROR` | `,` |
| 23 | 167 | `ERROR` | `, , ,` |
| 29 | 102 | `ERROR` | `,` |
| 36 | 167 | `ERROR` | `, , ,` |
| 42 | 166 | `ERROR` | `,` |
| 47 | 167 | `ERROR` | `, , ,` |
| 54 | 115 | `ERROR` | `,` |
| 61 | 167 | `ERROR` | `, , ,` |
| 67 | 102 | `ERROR` | `,` |
| 74 | 167 | `ERROR` | `, , ,` |
| 80 | 104 | `ERROR` | `,` |
| 87 | 167 | `ERROR` | `, , ,` |
| 93 | 96 | `ERROR` | `,` |
| 98 | 165 | `ERROR` | `,` |
| 104 | 158 | `ERROR` | `,` |
| 111 | 107 | `ERROR` | `,` |
| 117 | 94 | `ERROR` | `,` |
| 123 | 96 | `ERROR` | `,` |
| 136 | 172 | `ERROR` | `, , ,` |
| … | … | … | *(29 more)* |

#### `frameworks/taihe/src/output/unify_movie_file_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 91 | `ERROR` | `,` |
| 24 | 167 | `ERROR` | `, , ,` |
| 30 | 149 | `ERROR` | `, , ,` |
| 31 | 174 | `ERROR` | `, , ,` |
| 34 | 155 | `ERROR` | `, , ,` |
| 36 | 189 | `ERROR` | `,` |
| 44 | 90 | `ERROR` | `,` |
| 49 | 167 | `ERROR` | `, , ,` |
| 55 | 149 | `ERROR` | `, , ,` |
| 56 | 174 | `ERROR` | `, , ,` |
| 59 | 155 | `ERROR` | `, , ,` |
| 61 | 188 | `ERROR` | `,` |
| 68 | 91 | `ERROR` | `,` |
| 73 | 167 | `ERROR` | `, , ,` |
| 79 | 149 | `ERROR` | `, , ,` |
| 80 | 174 | `ERROR` | `, , ,` |
| 83 | 155 | `ERROR` | `, , ,` |
| 85 | 189 | `ERROR` | `,` |
| 92 | 92 | `ERROR` | `,` |
| 97 | 167 | `ERROR` | `, , ,` |
| … | … | … | *(50 more)* |

#### `frameworks/taihe/src/output/video_capability_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 133 | `ERROR` | `,` |

#### `frameworks/taihe/src/output/video_output_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 91 | `ERROR` | `,` |
| 20 | 149 | `ERROR` | `, , ,` |
| 26 | 162 | `ERROR` | `, , ,` |
| 35 | 90 | `ERROR` | `,` |
| 38 | 149 | `ERROR` | `, , ,` |
| 44 | 162 | `ERROR` | `, , ,` |
| 46 | 194 | `ERROR` | `,` |
| 53 | 93 | `ERROR` | `,` |
| 56 | 149 | `ERROR` | `, , ,` |
| 62 | 162 | `ERROR` | `, , ,` |
| 72 | 129 | `ERROR` | `,` |
| 82 | 124 | `ERROR` | `,` |
| 96 | 136 | `ERROR` | `,` |
| 108 | 136 | `ERROR` | `,` |
| 122 | 151 | `ERROR` | `,` |
| 139 | 149 | `ERROR` | `,` |
| 165 | 176 | `ERROR` | `, , ,` |
| 187 | 174 | `ERROR` | `, , ,` |
| 189 | 164 | `ERROR` | `, , ,` |
| 203 | 124 | `ERROR` | `,` |
| … | … | … | *(28 more)* |

#### `frameworks/taihe/src/photo_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 81 | `ERROR` | `,` |

#### `frameworks/taihe/src/picker/camera_picker_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 99 | `ERROR` | `,` |
| 24 | 165 | `ERROR` | `, , ,` |
| 29 | 109 | `ERROR` | `,` |
| 31 | 104 | `ERROR` | `,` |
| 35 | 114 | `ERROR` | `,` |
| 42 | 97 | `ERROR` | `,` |
| 46 | 188 | `ERROR` | `, , ,` |
| 87 | 108 | `ERROR` | `,` |
| 89 | 95 | `ERROR` | `,` |
| 114 | 180 | `ERROR` | `,` |
| 119 | 177 | `ERROR` | `,` |
| 144 | 127 | `ERROR` | `,` |
| 148 | 198 | `ERROR` | `,` |
| 150 | 132 | `ERROR` | `,` |
| 169 | 138 | `ERROR` | `, , ,` |
| 180 | 177 | `ERROR` | `,` |
| 190 | 40 | `ERROR` | `,` |
| 199 | 103 | `ERROR` | `,` |
| 205 | 162 | `ERROR` | `,` |
| 208 | 138 | `ERROR` | `,` |
| … | … | … | *(21 more)* |

#### `frameworks/taihe/src/query/camera_query_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 90 | `ERROR` | `,` |
| 10 | 171 | `ERROR` | `, , ,` |
| 49 | 206 | `ERROR` | `, , ,` |
| 67 | 96 | `ERROR` | `,` |
| 72 | 140 | `ERROR` | `,` |
| 79 | 109 | `ERROR` | `,` |
| 86 | 94 | `ERROR` | `,` |
| 87 | 165 | `ERROR` | `, , ,` |
| 96 | 94 | `ERROR` | `,` |
| 98 | 165 | `ERROR` | `, , ,` |
| 115 | 99 | `ERROR` | `,` |
| 122 | 184 | `ERROR` | `,` |
| 143 | 170 | `ERROR` | `, , ,` |
| 153 | 213 | `ERROR` | `, , ,` |
| 166 | 175 | `ERROR` | `, , ,` |
| 174 | 94 | `ERROR` | `,` |
| 175 | 165 | `ERROR` | `, , ,` |
| 179 | 174 | `ERROR` | `, , ,` |
| 184 | 93 | `ERROR` | `,` |
| 187 | 176 | `ERROR` | `, , ,` |
| … | … | … | *(141 more)* |

#### `frameworks/taihe/src/session/camera_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 93 | `ERROR` | `,` |
| 24 | 98 | `ERROR` | `,` |
| 27 | 155 | `ERROR` | `, , ,` |
| 33 | 165 | `ERROR` | `, , ,` |
| 42 | 91 | `ERROR` | `,` |
| 45 | 155 | `ERROR` | `, , ,` |
| 50 | 165 | `ERROR` | `, , ,` |
| 59 | 90 | `ERROR` | `,` |
| 62 | 155 | `ERROR` | `, , ,` |
| 68 | 165 | `ERROR` | `, , ,` |
| 77 | 93 | `ERROR` | `,` |
| 80 | 155 | `ERROR` | `, , ,` |
| 86 | 165 | `ERROR` | `, , ,` |
| 95 | 90 | `ERROR` | `,` |
| 96 | 161 | `ERROR` | `, , ,` |
| 99 | 149 | `ERROR` | `, , ,` |
| 107 | 93 | `ERROR` | `,` |
| 108 | 164 | `ERROR` | `, , ,` |
| 111 | 152 | `ERROR` | `, , ,` |
| 119 | 94 | `ERROR` | `,` |
| … | … | … | *(79 more)* |

#### `frameworks/taihe/src/session/control_center_session_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 136 | `ERROR` | `,` |
| 18 | 166 | `ERROR` | `, , ,` |
| 22 | 127 | `ERROR` | `,` |
| 32 | 179 | `ERROR` | `, , ,` |
| 33 | 126 | `ERROR` | `,` |
| 34 | 182 | `ERROR` | `, , ,` |
| 38 | 127 | `ERROR` | `,` |
| 47 | 179 | `ERROR` | `, , ,` |
| 48 | 126 | `ERROR` | `,` |
| 49 | 182 | `ERROR` | `, , ,` |
| 52 | 127 | `ERROR` | `,` |
| 61 | 137 | `ERROR` | `,` |
| 63 | 167 | `ERROR` | `, , ,` |
| 75 | 180 | `ERROR` | `, , ,` |
| 76 | 127 | `ERROR` | `,` |
| 77 | 183 | `ERROR` | `, , ,` |
| 81 | 128 | `ERROR` | `,` |
| 90 | 180 | `ERROR` | `, , ,` |
| 91 | 127 | `ERROR` | `,` |
| 92 | 183 | `ERROR` | `, , ,` |
| … | … | … | *(33 more)* |

#### `frameworks/taihe/src/transfer/camera_transfer_taihe.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 133 | `ERROR` | `,` |
| 38 | 261 | `ERROR` | `,` |
| 38 | 274 | `ERROR` | `,` |
| 38 | 287 | `ERROR` | `,` |
| 41 | 261 | `ERROR` | `,` |
| 41 | 274 | `ERROR` | `,` |
| 41 | 287 | `ERROR` | `,` |
| 45 | 224 | `ERROR` | `,` |
| 45 | 237 | `ERROR` | `,` |
| 45 | 250 | `ERROR` | `,` |
| 47 | 130 | `ERROR` | `,` |
| 53 | 133 | `ERROR` | `,` |
| 54 | 195 | `ERROR` | `,` |
| 54 | 208 | `ERROR` | `,` |
| 54 | 221 | `ERROR` | `,` |
| 56 | 197 | `ERROR` | `,` |
| 56 | 210 | `ERROR` | `,` |
| 56 | 223 | `ERROR` | `,` |
| 62 | 164 | `ERROR` | `,` |
| 68 | 164 | `ERROR` | `,` |
| … | … | … | *(176 more)* |

#### `interfaces/inner_api/native/camera/include/ability/camera_ability_builder.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 14 | `ERROR` | `CameraAbilityBuilder::` |

#### `interfaces/inner_api/native/camera/include/input/camera_manager.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 617 | 63 | `missing type_identifier` | `` |

#### `interfaces/inner_api/native/camera/include/input/i_standard_camera_listener.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 30 | `ERROR` | `u "` |
| 11 | 56 | `ERROR` | `"` |

#### `interfaces/inner_api/native/camera/include/output/photo_output.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 13 | `missing }` | `` |
| 78 | 27 | `missing field_identifier` | `` |
| 81 | 1 | `ERROR` | `}` |

#### `interfaces/inner_api/native/test/camera_capture.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 110 | `ERROR` | `,` |
| 90 | 129 | `ERROR` | `,` |
| 97 | 158 | `ERROR` | `,` |
| 99 | 151 | `ERROR` | `,` |
| 126 | 98 | `ERROR` | `,` |
| 128 | 141 | `ERROR` | `,` |
| 133 | 143 | `ERROR` | `,` |
| 136 | 147 | `ERROR` | `,` |
| 143 | 96 | `ERROR` | `,` |
| 145 | 139 | `ERROR` | `,` |
| 150 | 100 | `ERROR` | `,` |
| 152 | 172 | `ERROR` | `,` |
| 154 | 98 | `ERROR` | `,` |
| 156 | 172 | `ERROR` | `,` |
| 168 | 232 | `ERROR` | `,` |
| 170 | 220 | `ERROR` | `,` |
| 172 | 155 | `ERROR` | `,` |
| 199 | 94 | `ERROR` | `,` |
| 226 | 96 | `ERROR` | `,` |
| 243 | 87 | `ERROR` | `,` |
| … | … | … | *(3 more)* |

#### `interfaces/inner_api/native/test/camera_capture_mode.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 122 | 175 | `ERROR` | `,` |
| 132 | 177 | `ERROR` | `,` |
| 141 | 176 | `ERROR` | `,` |
| 151 | 177 | `ERROR` | `,` |
| 160 | 184 | `ERROR` | `,` |
| 172 | 187 | `ERROR` | `,` |
| 181 | 189 | `ERROR` | `,` |
| 185 | 127 | `ERROR` | `,` |
| 357 | 128 | `ERROR` | `,` |
| 364 | 157 | `ERROR` | `,` |
| 366 | 150 | `ERROR` | `,` |
| 367 | 151 | `ERROR` | `,` |
| 384 | 149 | `ERROR` | `,` |
| 443 | 154 | `ERROR` | `,` |
| 487 | 93 | `ERROR` | `,` |
| 495 | 92 | `ERROR` | `,` |
| 542 | 95 | `ERROR` | `,` |
| 554 | 86 | `ERROR` | `,` |
| 604 | 93 | `ERROR` | `,` |
| 606 | 141 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `interfaces/inner_api/native/test/camera_capture_video.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 149 | `ERROR` | `,` |
| 42 | 149 | `ERROR` | `,` |
| 60 | 149 | `ERROR` | `,` |
| 199 | 92 | `ERROR` | `,` |
| 205 | 152 | `ERROR` | `,` |
| 217 | 92 | `ERROR` | `,` |
| 223 | 160 | `ERROR` | `,` |
| 229 | 159 | `ERROR` | `,` |
| 276 | 101 | `ERROR` | `,` |
| 296 | 99 | `ERROR` | `,` |
| 304 | 98 | `ERROR` | `,` |
| 306 | 141 | `ERROR` | `,` |
| 311 | 143 | `ERROR` | `,` |
| 314 | 147 | `ERROR` | `,` |
| 321 | 96 | `ERROR` | `,` |
| 323 | 139 | `ERROR` | `,` |
| 334 | 96 | `ERROR` | `,` |
| 336 | 139 | `ERROR` | `,` |
| 340 | 141 | `ERROR` | `,` |
| 343 | 145 | `ERROR` | `,` |
| … | … | … | *(34 more)* |

#### `interfaces/inner_api/native/test/camera_video.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 187 | `ERROR` | `,` |
| 20 | 168 | `ERROR` | `,` |
| 32 | 96 | `ERROR` | `,` |
| 37 | 93 | `ERROR` | `,` |
| 42 | 99 | `ERROR` | `,` |
| 47 | 106 | `ERROR` | `,` |
| 52 | 95 | `ERROR` | `,` |
| 66 | 96 | `ERROR` | `,` |
| 71 | 100 | `ERROR` | `,` |
| 76 | 97 | `ERROR` | `,` |
| 81 | 106 | `ERROR` | `,` |
| 95 | 94 | `ERROR` | `,` |
| 101 | 95 | `ERROR` | `,` |
| 106 | 95 | `ERROR` | `,` |
| 111 | 96 | `ERROR` | `,` |
| 116 | 110 | `ERROR` | `,` |
| 121 | 110 | `ERROR` | `,` |
| 126 | 95 | `ERROR` | `,` |
| 133 | 100 | `ERROR` | `,` |
| 167 | 112 | `ERROR` | `,` |
| … | … | … | *(53 more)* |

#### `interfaces/inner_api/native/test/test_common.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 189 | `ERROR` | `, , ,` |
| 40 | 161 | `ERROR` | `, , ,` |
| 41 | 151 | `ERROR` | `,` |
| 46 | 135 | `ERROR` | `, , ,` |
| 51 | 135 | `ERROR` | `, , ,` |
| 56 | 135 | `ERROR` | `, , ,` |
| 58 | 88 | `ERROR` | `,` |
| 62 | 158 | `ERROR` | `,` |
| 64 | 224 | `ERROR` | `,` |
| 64 | 244 | `ERROR` | `,` |
| 64 | 264 | `ERROR` | `,` |
| 68 | 180 | `ERROR` | `,` |
| 96 | 145 | `ERROR` | `, , ,` |
| 97 | 170 | `ERROR` | `,` |
| 126 | 95 | `ERROR` | `,` |
| 133 | 250 | `ERROR` | `,` |
| 143 | 239 | `ERROR` | `,` |
| 154 | 174 | `ERROR` | `,` |
| 165 | 214 | `ERROR` | `,` |
| 171 | 214 | `ERROR` | `,` |
| … | … | … | *(37 more)* |

#### `interfaces/kits/js/camera_napi/include/camera_napi_event_emitter.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 14 | `ERROR` | `T::` |
| 13 | 14 | `ERROR` | `T::` |
| 25 | 181 | `ERROR` | `,` |
| 29 | 11 | `ERROR` | `->` |
| 32 | 119 | `ERROR` | `,` |
| 42 | 196 | `ERROR` | `,` |
| 46 | 11 | `ERROR` | `->` |
| 49 | 121 | `ERROR` | `,` |

#### `interfaces/kits/js/camera_napi/include/camera_napi_object_types.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 33 | `ERROR` | `. . .` |
| 17 | 16 | `ERROR` | `. . .` |
| 19 | 76 | `ERROR` | `. . .` |

#### `interfaces/kits/js/camera_napi/include/camera_napi_param_parser.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 116 | 33 | `ERROR` | `. . .` |
| 117 | 102 | `ERROR` | `. . .` |
| 118 | 45 | `ERROR` | `. . .(` |
| 118 | 76 | `ERROR` | `, nullptr)` |
| 120 | 24 | `missing }` | `` |
| 123 | 14 | `ERROR` | `> 0` |
| 124 | 11 | `ERROR` | `. . .` |
| 128 | 9 | `ERROR` | `< typename T , typename . . .Args> explicit CameraNapiParamParser(napi_env env , napi_callback_info info , T*& nativeObj…` |
| 130 | 43 | `missing ::` | `` |
| 130 | 60 | `ERROR` | `Args& . . .args) :` |
| 131 | 45 | `ERROR` | `. . .(` |
| 133 | 1 | `ERROR` | `if(napiError!= napi_ok)` |
| 137 | 11 | `ERROR` | `. . .` |
| 141 | 20 | `ERROR` | `. . .` |
| 142 | 92 | `ERROR` | `. . .` |
| 143 | 33 | `ERROR` | `. . .(` |
| 143 | 44 | `ERROR` | `)` |
| 151 | 11 | `ERROR` | `. . .` |
| 173 | 8 | `ERROR` | `: template` |
| 175 | 1 | `ERROR` | `explicit CameraNapiParamParser(napi_env env , napi_callback_info info , size_t napiParamSize , T*& nativeObjPointer , st…` |
| … | … | … | *(10 more)* |

#### `interfaces/kits/js/camera_napi/include/camera_napi_template_utils.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 84 | `ERROR` | `,` |
| 63 | 95 | `ERROR` | `,` |
| 70 | 182 | `ERROR` | `,` |
| 71 | 157 | `ERROR` | `, , ,` |
| 77 | 88 | `ERROR` | `,` |
| 83 | 99 | `ERROR` | `,` |
| 87 | 105 | `ERROR` | `,` |
| 91 | 186 | `ERROR` | `,` |
| 92 | 161 | `ERROR` | `, , ,` |
| 98 | 84 | `ERROR` | `,` |
| 104 | 95 | `ERROR` | `,` |
| 111 | 182 | `ERROR` | `,` |
| 112 | 157 | `ERROR` | `, , ,` |
| 118 | 85 | `ERROR` | `,` |
| 123 | 95 | `ERROR` | `,` |
| 130 | 183 | `ERROR` | `,` |
| 131 | 159 | `ERROR` | `, , ,` |
| 141 | 95 | `ERROR` | `,` |
| 144 | 183 | `ERROR` | `,` |
| 145 | 158 | `ERROR` | `, , ,` |
| … | … | … | *(3 more)* |

#### `interfaces/kits/js/camera_napi/include/napi_info_callback.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 143 | `ERROR` | `,` |
| 27 | 143 | `ERROR` | `,` |
| 36 | 94 | `ERROR` | `,` |

#### `interfaces/kits/js/camera_napi/include/session/camera_napi_adaptor.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 143 | `ERROR` | `,` |
| 66 | 82 | `ERROR` | `,` |
| 69 | 213 | `ERROR` | `,` |
| 69 | 237 | `ERROR` | `,` |
| 69 | 261 | `ERROR` | `,` |
| 73 | 161 | `ERROR` | `,` |
| 77 | 30 | `ERROR` | `*` |
| 78 | 245 | `ERROR` | `,` |
| 79 | 33 | `ERROR` | `,` |
| 80 | 33 | `ERROR` | `,` |
| 89 | 143 | `ERROR` | `,` |
| 94 | 213 | `ERROR` | `,` |
| 94 | 237 | `ERROR` | `,` |
| 94 | 261 | `ERROR` | `,` |
| 98 | 161 | `ERROR` | `,` |
| 102 | 30 | `ERROR` | `*` |
| 103 | 245 | `ERROR` | `,` |
| 104 | 33 | `ERROR` | `,` |
| 105 | 33 | `ERROR` | `,` |
| 122 | 143 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `mediastream/include/filter/video_encoder_filter.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 110 | 171 | `ERROR` | `,` |
| 134 | 171 | `ERROR` | `,` |

#### `mediastream/src/buffer/audio_buffer_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 99 | `ERROR` | `,` |
| 36 | 162 | `ERROR` | `,` |

#### `mediastream/src/buffer/meta_buffer_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 134 | `ERROR` | `, , ,` |
| 44 | 163 | `ERROR` | `, , ,` |
| 47 | 195 | `ERROR` | `,` |
| 47 | 210 | `ERROR` | `,` |
| 47 | 225 | `ERROR` | `,` |
| 50 | 196 | `ERROR` | `,` |
| 50 | 211 | `ERROR` | `,` |
| 50 | 226 | `ERROR` | `,` |
| 58 | 154 | `ERROR` | `, , ,` |
| 70 | 211 | `ERROR` | `,` |
| 70 | 230 | `ERROR` | `,` |
| 70 | 249 | `ERROR` | `,` |
| 73 | 101 | `ERROR` | `,` |

#### `mediastream/src/buffer/video_buffer_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 144 | `ERROR` | `, , ,` |
| 31 | 195 | `ERROR` | `,` |
| 31 | 203 | `ERROR` | `,` |
| 31 | 211 | `ERROR` | `,` |
| 33 | 197 | `ERROR` | `,` |
| 33 | 205 | `ERROR` | `,` |
| 33 | 213 | `ERROR` | `,` |
| 39 | 151 | `ERROR` | `, , ,` |
| 41 | 154 | `ERROR` | `, , ,` |
| 55 | 200 | `ERROR` | `,` |
| 55 | 208 | `ERROR` | `,` |
| 55 | 216 | `ERROR` | `,` |
| 58 | 157 | `ERROR` | `, , ,` |
| 60 | 202 | `ERROR` | `,` |
| 60 | 210 | `ERROR` | `,` |
| 60 | 218 | `ERROR` | `,` |
| 66 | 149 | `ERROR` | `, , ,` |
| 68 | 154 | `ERROR` | `, , ,` |
| 71 | 147 | `ERROR` | `,` |
| 84 | 129 | `ERROR` | `, , ,` |

#### `mediastream/src/deferred_process.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 147 | `ERROR` | `, , ,` |
| 29 | 147 | `ERROR` | `, , ,` |
| 42 | 200 | `ERROR` | `,` |
| 82 | 145 | `ERROR` | `, , ,` |
| 89 | 83 | `ERROR` | `,` |
| 96 | 131 | `ERROR` | `, , ,` |
| 103 | 83 | `ERROR` | `,` |
| 107 | 131 | `ERROR` | `, , ,` |
| 115 | 84 | `ERROR` | `,` |
| 118 | 132 | `ERROR` | `, , ,` |
| 125 | 82 | `ERROR` | `,` |
| 127 | 139 | `ERROR` | `, , ,` |
| 132 | 139 | `ERROR` | `, , ,` |
| 155 | 132 | `ERROR` | `, , ,` |
| 157 | 147 | `ERROR` | `, , ,` |
| 159 | 131 | `ERROR` | `, , ,` |
| 161 | 204 | `ERROR` | `, , ,` |
| 163 | 149 | `ERROR` | `, , ,` |
| 170 | 154 | `ERROR` | `, , ,` |
| 189 | 165 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `mediastream/src/filter/audio_cache_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 95 | `ERROR` | `,` |
| 38 | 95 | `ERROR` | `,` |
| 47 | 95 | `ERROR` | `,` |
| 56 | 112 | `ERROR` | `,` |
| 61 | 113 | `ERROR` | `,` |
| 66 | 85 | `ERROR` | `,` |
| 73 | 95 | `ERROR` | `,` |
| 81 | 80 | `ERROR` | `,` |
| 88 | 100 | `ERROR` | `,` |
| 96 | 98 | `ERROR` | `,` |
| 104 | 76 | `ERROR` | `,` |
| 112 | 77 | `ERROR` | `,` |
| 120 | 93 | `ERROR` | `,` |
| 132 | 76 | `ERROR` | `,` |
| 138 | 78 | `ERROR` | `,` |
| 144 | 80 | `ERROR` | `,` |
| 150 | 83 | `ERROR` | `,` |
| 155 | 83 | `ERROR` | `,` |
| 160 | 194 | `missing identifier` | `` |
| 170 | 81 | `ERROR` | `,` |
| … | … | … | *(9 more)* |

#### `mediastream/src/filter/audio_capture_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 283 | `ERROR` | `,` |
| 26 | 131 | `ERROR` | `,` |
| 67 | 97 | `ERROR` | `,` |
| 85 | 152 | `ERROR` | `, , ,` |
| 87 | 158 | `ERROR` | `, , ,` |
| 96 | 77 | `ERROR` | `,` |
| 102 | 85 | `ERROR` | `,` |
| 106 | 154 | `ERROR` | `, , ,` |
| 112 | 185 | `ERROR` | `,` |
| 112 | 194 | `ERROR` | `,` |
| 112 | 203 | `ERROR` | `,` |
| 115 | 145 | `ERROR` | `,` |
| 121 | 83 | `ERROR` | `,` |
| 124 | 154 | `ERROR` | `, , ,` |
| 126 | 152 | `ERROR` | `, , ,` |
| 137 | 83 | `ERROR` | `,` |
| 139 | 154 | `ERROR` | `, , ,` |
| 142 | 98 | `ERROR` | `,` |
| 153 | 82 | `ERROR` | `,` |
| 157 | 95 | `ERROR` | `,` |
| … | … | … | *(30 more)* |

#### `mediastream/src/filter/audio_capture_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 92 | `ERROR` | `,` |
| 49 | 92 | `ERROR` | `,` |
| 58 | 92 | `ERROR` | `,` |
| 74 | 179 | `ERROR` | `,` |
| 83 | 98 | `ERROR` | `,` |
| 88 | 99 | `ERROR` | `,` |
| 95 | 75 | `ERROR` | `,` |
| 101 | 194 | `ERROR` | `, , ,` |
| 103 | 178 | `ERROR` | `, , ,` |
| 109 | 100 | `ERROR` | `,` |
| 116 | 90 | `ERROR` | `,` |
| 127 | 152 | `ERROR` | `, , ,` |
| 140 | 78 | `ERROR` | `,` |
| 142 | 142 | `ERROR` | `, , ,` |
| 149 | 76 | `ERROR` | `,` |
| 163 | 152 | `ERROR` | `, , ,` |
| 173 | 76 | `ERROR` | `,` |
| 182 | 149 | `ERROR` | `, , ,` |
| 194 | 131 | `ERROR` | `,` |
| 198 | 111 | `ERROR` | `,` |
| … | … | … | *(45 more)* |

#### `mediastream/src/filter/audio_capturer_session_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 109 | `ERROR` | `,` |
| 44 | 145 | `ERROR` | `, , ,` |
| 49 | 148 | `ERROR` | `, , ,` |
| 52 | 172 | `ERROR` | `, , ,` |
| 53 | 188 | `ERROR` | `,` |
| 64 | 154 | `ERROR` | `, , ,` |
| 72 | 105 | `ERROR` | `,` |
| 80 | 105 | `ERROR` | `,` |
| 81 | 154 | `ERROR` | `, , ,` |
| 82 | 179 | `ERROR` | `, , ,` |
| 84 | 90 | `ERROR` | `,` |
| 90 | 107 | `ERROR` | `,` |
| 97 | 152 | `ERROR` | `, , ,` |
| 115 | 154 | `ERROR` | `, , ,` |
| 118 | 161 | `ERROR` | `, , ,` |
| 120 | 145 | `ERROR` | `, , ,` |
| 123 | 85 | `ERROR` | `,` |
| 124 | 150 | `ERROR` | `, , ,` |
| 129 | 148 | `ERROR` | `,` |
| 137 | 106 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `mediastream/src/filter/audio_deferred_process_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 106 | `ERROR` | `,` |
| 16 | 105 | `ERROR` | `,` |
| 23 | 127 | `ERROR` | `,` |
| 28 | 279 | `ERROR` | `, , ,` |
| 31 | 179 | `ERROR` | `, , ,` |
| 49 | 117 | `ERROR` | `,` |
| 59 | 135 | `ERROR` | `,` |
| 68 | 136 | `ERROR` | `,` |
| 82 | 122 | `ERROR` | `,` |
| 86 | 160 | `ERROR` | `, , ,` |
| 88 | 205 | `ERROR` | `, , ,` |
| 104 | 193 | `ERROR` | `, , ,` |
| 117 | 151 | `ERROR` | `, , ,` |
| 123 | 151 | `ERROR` | `, , ,` |
| 128 | 176 | `ERROR` | `, , ,` |
| 129 | 160 | `ERROR` | `, , ,` |
| 137 | 160 | `ERROR` | `, , ,` |
| 146 | 113 | `ERROR` | `,` |
| 147 | 146 | `ERROR` | `, , ,` |
| 156 | 160 | `ERROR` | `, , ,` |
| … | … | … | *(1 more)* |

#### `mediastream/src/filter/audio_encoder_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 92 | `ERROR` | `,` |
| 38 | 92 | `ERROR` | `,` |
| 47 | 92 | `ERROR` | `,` |
| 58 | 98 | `ERROR` | `,` |
| 63 | 99 | `ERROR` | `,` |
| 68 | 85 | `ERROR` | `,` |
| 76 | 75 | `ERROR` | `,` |
| 81 | 146 | `ERROR` | `, , ,` |
| 84 | 85 | `ERROR` | `,` |
| 92 | 80 | `ERROR` | `,` |
| 106 | 86 | `ERROR` | `,` |
| 113 | 78 | `ERROR` | `,` |
| 117 | 85 | `ERROR` | `,` |
| 133 | 76 | `ERROR` | `,` |
| 144 | 76 | `ERROR` | `,` |
| 150 | 77 | `ERROR` | `,` |
| 157 | 75 | `ERROR` | `,` |
| 169 | 76 | `ERROR` | `,` |
| 181 | 78 | `ERROR` | `,` |
| 193 | 80 | `ERROR` | `,` |
| … | … | … | *(18 more)* |

#### `mediastream/src/filter/audio_fork_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 80 | `ERROR` | `,` |
| 29 | 95 | `ERROR` | `,` |
| 38 | 95 | `ERROR` | `,` |
| 47 | 95 | `ERROR` | `,` |
| 54 | 80 | `ERROR` | `,` |
| 70 | 95 | `ERROR` | `,` |
| 76 | 80 | `ERROR` | `,` |
| 85 | 95 | `ERROR` | `,` |
| 90 | 80 | `ERROR` | `,` |
| 95 | 79 | `ERROR` | `,` |
| 101 | 75 | `ERROR` | `,` |
| 110 | 96 | `ERROR` | `,` |
| 117 | 76 | `ERROR` | `,` |
| 123 | 76 | `ERROR` | `,` |
| 129 | 77 | `ERROR` | `,` |
| 135 | 75 | `ERROR` | `,` |
| 141 | 76 | `ERROR` | `,` |
| 147 | 78 | `ERROR` | `,` |
| 153 | 83 | `ERROR` | `,` |
| 159 | 83 | `ERROR` | `,` |
| … | … | … | *(31 more)* |

#### `mediastream/src/filter/audio_process_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 80 | `ERROR` | `,` |
| 40 | 98 | `ERROR` | `,` |
| 49 | 98 | `ERROR` | `,` |
| 58 | 98 | `ERROR` | `,` |
| 64 | 80 | `ERROR` | `,` |
| 73 | 98 | `ERROR` | `,` |
| 78 | 165 | `ERROR` | `,` |
| 83 | 165 | `ERROR` | `,` |
| 90 | 105 | `ERROR` | `,` |
| 106 | 99 | `ERROR` | `,` |
| 114 | 76 | `ERROR` | `,` |
| 121 | 76 | `ERROR` | `,` |
| 127 | 77 | `ERROR` | `,` |
| 133 | 75 | `ERROR` | `,` |
| 140 | 76 | `ERROR` | `,` |
| 146 | 78 | `ERROR` | `,` |
| 155 | 83 | `ERROR` | `,` |
| 161 | 83 | `ERROR` | `,` |
| 167 | 134 | `ERROR` | `, , ,` |
| 168 | 162 | `ERROR` | `,` |
| … | … | … | *(69 more)* |

#### `mediastream/src/filter/cfilter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 79 | `ERROR` | `,` |
| 33 | 224 | `ERROR` | `,` |
| 65 | 194 | `ERROR` | `,` |
| 78 | 146 | `ERROR` | `,` |
| 102 | 193 | `ERROR` | `,` |
| 126 | 141 | `ERROR` | `,` |
| 135 | 193 | `ERROR` | `,` |
| 151 | 201 | `ERROR` | `,` |
| 169 | 203 | `ERROR` | `,` |
| 187 | 141 | `ERROR` | `,` |
| 196 | 194 | `ERROR` | `,` |
| 220 | 142 | `ERROR` | `,` |
| 229 | 202 | `ERROR` | `,` |
| 248 | 204 | `ERROR` | `,` |
| 267 | 192 | `ERROR` | `,` |
| 283 | 140 | `ERROR` | `,` |
| 292 | 193 | `ERROR` | `,` |
| 307 | 195 | `ERROR` | `,` |
| 330 | 143 | `ERROR` | `,` |
| 351 | 200 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `mediastream/src/filter/cinematic_video_cache_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 95 | `ERROR` | `,` |
| 28 | 95 | `ERROR` | `,` |
| 37 | 95 | `ERROR` | `,` |
| 56 | 95 | `ERROR` | `,` |
| 67 | 144 | `ERROR` | `,` |
| 72 | 144 | `ERROR` | `,` |
| 80 | 109 | `ERROR` | `,` |
| 87 | 114 | `ERROR` | `,` |
| 94 | 120 | `ERROR` | `,` |
| 95 | 145 | `ERROR` | `, , ,` |
| 97 | 159 | `ERROR` | `, , ,` |
| 99 | 143 | `ERROR` | `, , ,` |
| 114 | 114 | `ERROR` | `,` |
| 115 | 147 | `ERROR` | `, , ,` |
| 123 | 112 | `ERROR` | `,` |
| 130 | 112 | `ERROR` | `,` |
| 136 | 113 | `ERROR` | `,` |
| 142 | 111 | `ERROR` | `,` |
| 148 | 112 | `ERROR` | `,` |
| 154 | 114 | `ERROR` | `,` |
| … | … | … | *(87 more)* |

#### `mediastream/src/filter/demuxer_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 96 | `ERROR` | `,` |
| 64 | 112 | `ERROR` | `,` |
| 86 | 91 | `ERROR` | `,` |
| 93 | 89 | `ERROR` | `,` |
| 96 | 107 | `ERROR` | `,` |
| 107 | 94 | `ERROR` | `,` |
| 108 | 134 | `ERROR` | `, , ,` |
| 131 | 164 | `ERROR` | `,` |
| 144 | 113 | `ERROR` | `,` |
| 152 | 115 | `ERROR` | `,` |
| 159 | 147 | `ERROR` | `, , ,` |
| 161 | 149 | `ERROR` | `,` |
| 163 | 105 | `ERROR` | `,` |
| 183 | 142 | `ERROR` | `, , ,` |
| 188 | 171 | `ERROR` | `,` |
| 188 | 181 | `ERROR` | `,` |
| 188 | 191 | `ERROR` | `,` |
| 190 | 229 | `ERROR` | `,` |
| 190 | 239 | `ERROR` | `,` |
| 190 | 249 | `ERROR` | `,` |
| … | … | … | *(82 more)* |

#### `mediastream/src/filter/image_effect_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 79 | `ERROR` | `,` |
| 16 | 79 | `ERROR` | `,` |
| 21 | 232 | `ERROR` | `,` |
| 31 | 75 | `ERROR` | `,` |
| 40 | 91 | `ERROR` | `,` |
| 49 | 83 | `ERROR` | `,` |
| 51 | 157 | `ERROR` | `, , ,` |
| 57 | 86 | `ERROR` | `,` |
| 69 | 78 | `ERROR` | `,` |
| 77 | 76 | `ERROR` | `,` |
| 85 | 76 | `ERROR` | `,` |
| 91 | 77 | `ERROR` | `,` |
| 97 | 81 | `ERROR` | `,` |
| 103 | 114 | `ERROR` | `,` |
| 106 | 112 | `ERROR` | `,` |
| 112 | 117 | `ERROR` | `,` |
| 115 | 126 | `ERROR` | `,` |
| 127 | 78 | `ERROR` | `,` |
| 135 | 79 | `ERROR` | `,` |
| 146 | 81 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `mediastream/src/filter/meta_cache_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 94 | `ERROR` | `,` |
| 39 | 94 | `ERROR` | `,` |
| 48 | 94 | `ERROR` | `,` |
| 67 | 94 | `ERROR` | `,` |
| 80 | 110 | `ERROR` | `,` |
| 85 | 111 | `ERROR` | `,` |
| 90 | 85 | `ERROR` | `,` |
| 97 | 94 | `ERROR` | `,` |
| 104 | 80 | `ERROR` | `,` |
| 111 | 103 | `ERROR` | `,` |
| 113 | 161 | `ERROR` | `, , ,` |
| 115 | 151 | `ERROR` | `, , ,` |
| 117 | 157 | `ERROR` | `, , ,` |
| 126 | 99 | `ERROR` | `,` |
| 135 | 93 | `ERROR` | `,` |
| 142 | 76 | `ERROR` | `,` |
| 150 | 77 | `ERROR` | `,` |
| 158 | 75 | `ERROR` | `,` |
| 169 | 76 | `ERROR` | `,` |
| 175 | 78 | `ERROR` | `,` |
| … | … | … | *(21 more)* |

#### `mediastream/src/filter/metadata_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 93 | `ERROR` | `,` |
| 41 | 93 | `ERROR` | `,` |
| 50 | 93 | `ERROR` | `,` |
| 69 | 93 | `ERROR` | `,` |
| 79 | 168 | `ERROR` | `,` |
| 84 | 169 | `ERROR` | `,` |
| 89 | 85 | `ERROR` | `,` |
| 96 | 75 | `ERROR` | `,` |
| 104 | 80 | `ERROR` | `,` |
| 111 | 90 | `ERROR` | `,` |
| 113 | 139 | `ERROR` | `, , ,` |
| 122 | 90 | `ERROR` | `,` |
| 123 | 162 | `ERROR` | `, , ,` |
| 126 | 161 | `ERROR` | `, , ,` |
| 129 | 161 | `ERROR` | `,` |
| 131 | 162 | `ERROR` | `,` |
| 134 | 151 | `ERROR` | `, , ,` |
| 136 | 157 | `ERROR` | `, , ,` |
| 146 | 78 | `ERROR` | `,` |
| 155 | 76 | `ERROR` | `,` |
| … | … | … | *(24 more)* |

#### `mediastream/src/filter/moving_photo_audio_encoder_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 92 | `ERROR` | `,` |
| 39 | 92 | `ERROR` | `,` |
| 48 | 92 | `ERROR` | `,` |
| 58 | 98 | `ERROR` | `,` |
| 63 | 99 | `ERROR` | `,` |
| 68 | 85 | `ERROR` | `,` |
| 76 | 108 | `ERROR` | `,` |
| 81 | 146 | `ERROR` | `, , ,` |
| 84 | 85 | `ERROR` | `,` |
| 93 | 80 | `ERROR` | `,` |
| 107 | 86 | `ERROR` | `,` |
| 113 | 113 | `ERROR` | `,` |
| 122 | 111 | `ERROR` | `,` |
| 134 | 76 | `ERROR` | `,` |
| 140 | 77 | `ERROR` | `,` |
| 147 | 110 | `ERROR` | `,` |
| 159 | 76 | `ERROR` | `,` |
| 171 | 109 | `ERROR` | `,` |
| 183 | 80 | `ERROR` | `,` |
| 194 | 88 | `ERROR` | `,` |
| … | … | … | *(39 more)* |

#### `mediastream/src/filter/moving_photo_muxer_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 100 | `ERROR` | `,` |
| 33 | 101 | `ERROR` | `,` |
| 38 | 279 | `ERROR` | `,` |
| 50 | 101 | `ERROR` | `,` |
| 57 | 106 | `ERROR` | `,` |
| 63 | 104 | `ERROR` | `,` |
| 69 | 100 | `ERROR` | `,` |
| 75 | 101 | `ERROR` | `,` |
| 81 | 99 | `ERROR` | `,` |
| 91 | 172 | `missing identifier` | `` |
| 105 | 78 | `ERROR` | `,` |
| 111 | 107 | `ERROR` | `,` |
| 119 | 107 | `ERROR` | `,` |
| 124 | 200 | `missing identifier` | `` |
| 130 | 81 | `ERROR` | `,` |
| 136 | 81 | `ERROR` | `,` |
| 142 | 84 | `ERROR` | `,` |
| 149 | 105 | `ERROR` | `,` |
| 153 | 168 | `ERROR` | `,` |
| 170 | 189 | `ERROR` | `,` |
| … | … | … | *(24 more)* |

#### `mediastream/src/filter/moving_photo_video_encoder_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 99 | `ERROR` | `,` |
| 41 | 99 | `ERROR` | `,` |
| 50 | 99 | `ERROR` | `,` |
| 70 | 100 | `ERROR` | `,` |
| 93 | 138 | `ERROR` | `,` |
| 95 | 136 | `ERROR` | `,` |
| 97 | 99 | `ERROR` | `,` |
| 111 | 161 | `ERROR` | `, , ,` |
| 118 | 175 | `ERROR` | `,` |
| 120 | 106 | `ERROR` | `,` |
| 129 | 161 | `ERROR` | `, , ,` |
| 131 | 160 | `ERROR` | `, , ,` |
| 137 | 155 | `ERROR` | `, , ,` |
| 145 | 171 | `ERROR` | `, , ,` |
| 162 | 166 | `ERROR` | `, , ,` |
| 170 | 179 | `ERROR` | `,` |
| 172 | 149 | `ERROR` | `, , ,` |
| 176 | 149 | `ERROR` | `, , ,` |
| 192 | 161 | `ERROR` | `, , ,` |
| 195 | 139 | `ERROR` | `, , ,` |
| … | … | … | *(36 more)* |

#### `mediastream/src/filter/muxer_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 91 | `ERROR` | `,` |
| 65 | 80 | `ERROR` | `,` |
| 70 | 146 | `ERROR` | `,` |
| 75 | 211 | `ERROR` | `,` |
| 88 | 88 | `ERROR` | `,` |
| 101 | 75 | `ERROR` | `,` |
| 109 | 78 | `ERROR` | `,` |
| 116 | 88 | `ERROR` | `,` |
| 119 | 136 | `ERROR` | `, , ,` |
| 133 | 76 | `ERROR` | `,` |
| 140 | 77 | `ERROR` | `,` |
| 146 | 87 | `ERROR` | `,` |
| 160 | 97 | `ERROR` | `,` |
| 161 | 136 | `ERROR` | `, , ,` |
| 184 | 78 | `ERROR` | `,` |
| 190 | 83 | `ERROR` | `,` |
| 192 | 136 | `ERROR` | `, , ,` |
| 198 | 88 | `ERROR` | `,` |
| 199 | 136 | `ERROR` | `, , ,` |
| 201 | 138 | `ERROR` | `, , ,` |
| … | … | … | *(25 more)* |

#### `mediastream/src/filter/sink_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 80 | `ERROR` | `,` |
| 59 | 199 | `ERROR` | `,` |
| 74 | 153 | `ERROR` | `,` |
| 84 | 208 | `ERROR` | `,` |
| 92 | 80 | `ERROR` | `,` |
| 97 | 146 | `ERROR` | `,` |
| 103 | 75 | `ERROR` | `,` |
| 111 | 78 | `ERROR` | `,` |
| 122 | 76 | `ERROR` | `,` |
| 130 | 76 | `ERROR` | `,` |
| 136 | 77 | `ERROR` | `,` |
| 142 | 77 | `ERROR` | `,` |
| 153 | 78 | `ERROR` | `,` |
| 159 | 162 | `ERROR` | `,` |
| 163 | 136 | `ERROR` | `, , ,` |
| 169 | 81 | `ERROR` | `,` |
| 175 | 81 | `ERROR` | `,` |
| 182 | 79 | `ERROR` | `,` |
| 199 | 80 | `ERROR` | `,` |
| 205 | 81 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `mediastream/src/filter/video_cache_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 95 | `ERROR` | `,` |
| 39 | 95 | `ERROR` | `,` |
| 48 | 95 | `ERROR` | `,` |
| 67 | 95 | `ERROR` | `,` |
| 78 | 112 | `ERROR` | `,` |
| 83 | 113 | `ERROR` | `,` |
| 88 | 85 | `ERROR` | `,` |
| 95 | 95 | `ERROR` | `,` |
| 102 | 80 | `ERROR` | `,` |
| 109 | 106 | `ERROR` | `,` |
| 111 | 161 | `ERROR` | `, , ,` |
| 113 | 143 | `ERROR` | `, , ,` |
| 115 | 151 | `ERROR` | `, , ,` |
| 117 | 157 | `ERROR` | `, , ,` |
| 138 | 100 | `ERROR` | `,` |
| 146 | 98 | `ERROR` | `,` |
| 153 | 76 | `ERROR` | `,` |
| 161 | 77 | `ERROR` | `,` |
| 169 | 75 | `ERROR` | `,` |
| 180 | 76 | `ERROR` | `,` |
| … | … | … | *(34 more)* |

#### `mediastream/src/filter/video_encoder_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 134 | `ERROR` | `,` |
| 25 | 144 | `ERROR` | `,` |
| 30 | 94 | `ERROR` | `,` |
| 39 | 142 | `ERROR` | `,` |
| 48 | 170 | `ERROR` | `,` |
| 50 | 97 | `ERROR` | `,` |
| 66 | 93 | `ERROR` | `,` |
| 91 | 150 | `ERROR` | `,` |
| 108 | 92 | `ERROR` | `,` |
| 118 | 80 | `ERROR` | `,` |
| 138 | 105 | `ERROR` | `,` |
| 155 | 83 | `ERROR` | `,` |
| 157 | 90 | `ERROR` | `,` |
| 162 | 132 | `ERROR` | `, , ,` |
| 176 | 91 | `ERROR` | `,` |
| 184 | 96 | `ERROR` | `,` |
| 201 | 107 | `ERROR` | `,` |
| 208 | 88 | `ERROR` | `,` |
| 215 | 149 | `ERROR` | `, , ,` |
| 221 | 76 | `ERROR` | `,` |
| … | … | … | *(58 more)* |

#### `mediastream/src/filter/video_encoder_filter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 80 | `ERROR` | `,` |
| 22 | 79 | `ERROR` | `,` |
| 31 | 222 | `ERROR` | `,` |
| 40 | 85 | `ERROR` | `,` |
| 53 | 166 | `ERROR` | `,` |
| 60 | 113 | `ERROR` | `,` |
| 72 | 91 | `ERROR` | `,` |
| 82 | 80 | `ERROR` | `,` |
| 90 | 83 | `ERROR` | `,` |
| 91 | 147 | `ERROR` | `, , ,` |
| 97 | 82 | `ERROR` | `,` |
| 98 | 147 | `ERROR` | `, , ,` |
| 104 | 88 | `ERROR` | `,` |
| 112 | 86 | `ERROR` | `,` |
| 123 | 78 | `ERROR` | `,` |
| 125 | 85 | `ERROR` | `,` |
| 136 | 76 | `ERROR` | `,` |
| 143 | 76 | `ERROR` | `,` |
| 150 | 77 | `ERROR` | `,` |
| 157 | 81 | `ERROR` | `,` |
| … | … | … | *(23 more)* |

#### `mediastream/src/pipeline/pipeline.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 80 | `ERROR` | `,` |
| 21 | 79 | `ERROR` | `,` |
| 27 | 82 | `ERROR` | `,` |
| 36 | 85 | `ERROR` | `,` |
| 47 | 140 | `ERROR` | `,` |
| 53 | 83 | `ERROR` | `,` |
| 64 | 138 | `ERROR` | `,` |
| 70 | 83 | `ERROR` | `,` |
| 81 | 138 | `ERROR` | `,` |
| 87 | 84 | `ERROR` | `,` |
| 98 | 139 | `ERROR` | `,` |
| 104 | 82 | `ERROR` | `,` |
| 109 | 205 | `ERROR` | `,` |
| 109 | 226 | `ERROR` | `,` |
| 109 | 247 | `ERROR` | `,` |
| 113 | 224 | `ERROR` | `,` |
| 113 | 245 | `ERROR` | `,` |
| 113 | 266 | `ERROR` | `,` |
| 118 | 137 | `ERROR` | `,` |
| 124 | 83 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `mediastream/src/recorder_engine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 70 | 118 | `ERROR` | `,` |
| 75 | 132 | `ERROR` | `,` |
| 107 | 161 | `ERROR` | `, , ,` |
| 110 | 151 | `ERROR` | `, , ,` |
| 113 | 164 | `ERROR` | `, , ,` |
| 116 | 159 | `ERROR` | `, , ,` |
| 117 | 153 | `ERROR` | `,` |
| 164 | 101 | `ERROR` | `,` |
| 169 | 100 | `ERROR` | `,` |
| 176 | 101 | `ERROR` | `,` |
| 180 | 148 | `ERROR` | `, , ,` |
| 186 | 111 | `ERROR` | `,` |
| 195 | 234 | `ERROR` | `,` |
| 202 | 109 | `ERROR` | `,` |
| 214 | 108 | `ERROR` | `,` |
| 231 | 109 | `ERROR` | `,` |
| 239 | 113 | `ERROR` | `,` |
| 242 | 121 | `ERROR` | `,` |
| 245 | 119 | `ERROR` | `,` |
| 250 | 117 | `ERROR` | `,` |
| … | … | … | *(225 more)* |

#### `mediastream/src/recorder_engine_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 108 | `ERROR` | `,` |
| 12 | 107 | `ERROR` | `,` |
| 17 | 125 | `ERROR` | `,` |
| 23 | 112 | `ERROR` | `,` |
| 24 | 152 | `ERROR` | `, , ,` |
| 30 | 119 | `ERROR` | `,` |
| 31 | 152 | `ERROR` | `, , ,` |
| 37 | 119 | `ERROR` | `,` |
| 38 | 152 | `ERROR` | `, , ,` |
| 44 | 120 | `ERROR` | `,` |
| 45 | 152 | `ERROR` | `, , ,` |
| 51 | 118 | `ERROR` | `,` |
| 52 | 152 | `ERROR` | `, , ,` |
| 58 | 116 | `ERROR` | `,` |
| 59 | 152 | `ERROR` | `, , ,` |
| 66 | 122 | `ERROR` | `,` |
| 67 | 152 | `ERROR` | `, , ,` |
| 73 | 109 | `ERROR` | `,` |
| 74 | 152 | `ERROR` | `, , ,` |
| 80 | 129 | `ERROR` | `,` |
| … | … | … | *(49 more)* |

#### `mediastream/src/util/avbuffer_context.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 196 | `ERROR` | `,` |
| 17 | 146 | `ERROR` | `, , ,` |
| 20 | 142 | `ERROR` | `, , ,` |
| 25 | 202 | `ERROR` | `,` |
| 25 | 213 | `ERROR` | `,` |
| 25 | 224 | `ERROR` | `,` |
| 35 | 157 | `ERROR` | `, , ,` |
| 36 | 176 | `ERROR` | `, , ,` |
| 37 | 157 | `ERROR` | `, , ,` |
| 38 | 176 | `ERROR` | `, , ,` |
| 41 | 147 | `ERROR` | `, , ,` |
| 44 | 146 | `ERROR` | `, , ,` |
| 46 | 142 | `ERROR` | `, , ,` |
| 51 | 212 | `ERROR` | `,` |
| 51 | 223 | `ERROR` | `,` |
| 51 | 234 | `ERROR` | `,` |
| 56 | 210 | `ERROR` | `,` |
| 56 | 221 | `ERROR` | `,` |
| 56 | 232 | `ERROR` | `,` |
| 59 | 197 | `ERROR` | `,` |

#### `mediastream/src/util/moving_photo_avmuxer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 96 | `ERROR` | `,` |
| 20 | 149 | `ERROR` | `,` |
| 22 | 140 | `ERROR` | `, , ,` |
| 28 | 134 | `ERROR` | `, , ,` |
| 30 | 190 | `ERROR` | `,` |
| 30 | 198 | `ERROR` | `,` |
| 30 | 206 | `ERROR` | `,` |
| 36 | 134 | `ERROR` | `, , ,` |
| 38 | 197 | `ERROR` | `,` |
| 38 | 205 | `ERROR` | `,` |
| 38 | 213 | `ERROR` | `,` |
| 44 | 134 | `ERROR` | `, , ,` |
| 46 | 159 | `ERROR` | `,` |
| 48 | 167 | `ERROR` | `,` |
| 51 | 174 | `ERROR` | `,` |
| 55 | 164 | `ERROR` | `,` |
| 62 | 196 | `ERROR` | `,` |
| 62 | 204 | `ERROR` | `,` |
| 62 | 212 | `ERROR` | `,` |
| 69 | 130 | `ERROR` | `, , ,` |
| … | … | … | *(18 more)* |

#### `mediastream/src/util/moving_photo_engine_context.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 144 | `ERROR` | `,` |
| 14 | 145 | `ERROR` | `,` |

#### `mediastream/src/util/moving_photo_recorder_task.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 181 | `ERROR` | `,` |
| 30 | 172 | `ERROR` | `,` |
| 48 | 191 | `ERROR` | `,` |
| 78 | 147 | `ERROR` | `, , ,` |
| 85 | 147 | `ERROR` | `, , ,` |
| 92 | 147 | `ERROR` | `, , ,` |
| 99 | 147 | `ERROR` | `, , ,` |
| 106 | 147 | `ERROR` | `, , ,` |
| 113 | 64 | `ERROR` | `,` |
| 121 | 144 | `ERROR` | `, , ,` |
| 128 | 157 | `ERROR` | `,` |
| 130 | 147 | `ERROR` | `, , ,` |
| 142 | 161 | `ERROR` | `, , ,` |
| 150 | 154 | `ERROR` | `, , ,` |
| 158 | 181 | `ERROR` | `,` |
| 160 | 155 | `ERROR` | `, , ,` |
| 162 | 181 | `ERROR` | `,` |
| 189 | 163 | `ERROR` | `, , ,` |
| 193 | 151 | `ERROR` | `, , ,` |
| 194 | 92 | `ERROR` | `,` |
| … | … | … | *(18 more)* |

#### `mediastream/test/unittest/filter/include/audio_capture_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 29 | `missing type_identifier` | `` |
| 23 | 51 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 97 | `missing type_identifier` | `` |
| 44 | 32 | `missing type_identifier` | `` |
| 44 | 104 | `missing type_identifier` | `` |
| 45 | 32 | `missing type_identifier` | `` |
| 47 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/audio_encoder_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 24 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 39 | `missing type_identifier` | `` |
| 26 | 100 | `missing type_identifier` | `` |
| 27 | 40 | `missing type_identifier` | `` |
| 27 | 102 | `missing type_identifier` | `` |
| 28 | 40 | `missing type_identifier` | `` |
| 28 | 91 | `missing type_identifier` | `` |
| 29 | 41 | `missing type_identifier` | `` |
| 29 | 87 | `missing type_identifier` | `` |
| 30 | 36 | `missing type_identifier` | `` |
| 30 | 81 | `missing type_identifier` | `` |
| 31 | 36 | `missing type_identifier` | `` |
| 31 | 75 | `missing type_identifier` | `` |
| 32 | 29 | `missing type_identifier` | `` |
| 32 | 33 | `missing type_identifier` | `` |
| 33 | 28 | `missing type_identifier` | `` |
| 33 | 32 | `missing type_identifier` | `` |
| 34 | 29 | `missing type_identifier` | `` |
| 34 | 33 | `missing type_identifier` | `` |
| 35 | 29 | `missing type_identifier` | `` |
| 35 | 33 | `missing type_identifier` | `` |
| … | … | … | *(4 more)* |

#### `mediastream/test/unittest/filter/include/audio_fork_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 29 | `missing type_identifier` | `` |
| 23 | 51 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 97 | `missing type_identifier` | `` |
| 44 | 36 | `missing type_identifier` | `` |
| 44 | 115 | `missing type_identifier` | `` |
| 46 | 38 | `missing type_identifier` | `` |
| 46 | 72 | `missing type_identifier` | `` |
| 47 | 37 | `missing type_identifier` | `` |
| 47 | 71 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 59 | 104 | `missing type_identifier` | `` |
| 60 | 32 | `missing type_identifier` | `` |
| 62 | 59 | `missing type_identifier` | `` |
| 73 | 32 | `missing type_identifier` | `` |
| 73 | 104 | `missing type_identifier` | `` |
| 74 | 32 | `missing type_identifier` | `` |
| 76 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/audio_process_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 29 | `missing type_identifier` | `` |
| 23 | 51 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 97 | `missing type_identifier` | `` |
| 44 | 36 | `missing type_identifier` | `` |
| 44 | 115 | `missing type_identifier` | `` |
| 46 | 38 | `missing type_identifier` | `` |
| 46 | 72 | `missing type_identifier` | `` |
| 47 | 37 | `missing type_identifier` | `` |
| 47 | 71 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 59 | 104 | `missing type_identifier` | `` |
| 60 | 32 | `missing type_identifier` | `` |
| 62 | 59 | `missing type_identifier` | `` |
| 73 | 32 | `missing type_identifier` | `` |
| 73 | 104 | `missing type_identifier` | `` |
| 74 | 32 | `missing type_identifier` | `` |
| 76 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/cfilter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 32 | `missing type_identifier` | `` |
| 25 | 104 | `missing type_identifier` | `` |
| 26 | 32 | `missing type_identifier` | `` |
| 28 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/cinematic_video_cache_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 14 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 29 | `missing type_identifier` | `` |
| 23 | 51 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 97 | `missing type_identifier` | `` |
| 44 | 36 | `missing type_identifier` | `` |
| 44 | 115 | `missing type_identifier` | `` |
| 46 | 38 | `missing type_identifier` | `` |
| 46 | 72 | `missing type_identifier` | `` |
| 47 | 37 | `missing type_identifier` | `` |
| 47 | 71 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 59 | 104 | `missing type_identifier` | `` |
| 60 | 32 | `missing type_identifier` | `` |
| 62 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/metadata_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 14 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 29 | `missing type_identifier` | `` |
| 22 | 51 | `missing type_identifier` | `` |
| 32 | 34 | `missing type_identifier` | `` |
| 33 | 97 | `missing type_identifier` | `` |
| 43 | 36 | `missing type_identifier` | `` |
| 43 | 115 | `missing type_identifier` | `` |
| 45 | 38 | `missing type_identifier` | `` |
| 45 | 72 | `missing type_identifier` | `` |
| 46 | 37 | `missing type_identifier` | `` |
| 46 | 71 | `missing type_identifier` | `` |
| 58 | 32 | `missing type_identifier` | `` |
| 58 | 104 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 61 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/include/muxer_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 22 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 29 | `missing type_identifier` | `` |
| 23 | 51 | `missing type_identifier` | `` |
| 33 | 34 | `missing type_identifier` | `` |
| 34 | 97 | `missing type_identifier` | `` |
| 44 | 36 | `missing type_identifier` | `` |
| 44 | 115 | `missing type_identifier` | `` |
| 46 | 38 | `missing type_identifier` | `` |
| 46 | 72 | `missing type_identifier` | `` |
| 47 | 37 | `missing type_identifier` | `` |
| 47 | 71 | `missing type_identifier` | `` |
| 59 | 32 | `missing type_identifier` | `` |
| 59 | 104 | `missing type_identifier` | `` |
| 60 | 32 | `missing type_identifier` | `` |
| 62 | 59 | `missing type_identifier` | `` |
| 73 | 29 | `missing type_identifier` | `` |
| 73 | 33 | `missing type_identifier` | `` |
| 74 | 28 | `missing type_identifier` | `` |
| 74 | 32 | `missing type_identifier` | `` |
| 75 | 32 | `missing type_identifier` | `` |
| 75 | 99 | `missing type_identifier` | `` |
| … | … | … | *(2 more)* |

#### `mediastream/test/unittest/filter/include/video_encoder_filter_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 32 | `missing type_identifier` | `` |
| 24 | 104 | `missing type_identifier` | `` |
| 25 | 32 | `missing type_identifier` | `` |
| 27 | 59 | `missing type_identifier` | `` |

#### `mediastream/test/unittest/filter/src/audio_encoder_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 131 | 32 | `missing type_identifier` | `` |
| 131 | 104 | `missing type_identifier` | `` |
| 185 | 113 | `missing ;` | `` |
| 451 | 117 | `missing ;` | `` |

#### `mediastream/test/unittest/pipeline/pipeline_unit_test.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 32 | `missing type_identifier` | `` |
| 11 | 104 | `missing type_identifier` | `` |
| 12 | 32 | `missing type_identifier` | `` |
| 14 | 59 | `missing type_identifier` | `` |

#### `moviefile/include/movie_file/plugin/movie_file_video_filter_plugin.h`

**Summary:** tree-sitter-cpp node `missing field_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 13 | `missing field_identifier` | `` |

#### `moviefile/include/pipeline/producer/unified_pipeline_audio_capture_wrap.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 191 | `ERROR` | `,` |
| 163 | 168 | `ERROR` | `,` |
| 167 | 32 | `ERROR` | `,` |
| 183 | 193 | `ERROR` | `,` |
| 199 | 183 | `ERROR` | `,` |

#### `moviefile/include/pipeline/thread/unified_pipeline_threadpool.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 33 | `ERROR` | `. . .` |
| 19 | 28 | `ERROR` | `. . .` |
| 19 | 96 | `ERROR` | `. . .` |
| 45 | 33 | `ERROR` | `. . .` |
| 46 | 56 | `ERROR` | `. . .` |
| 47 | 59 | `ERROR` | `. . .` |
| 49 | 35 | `ERROR` | `. . .` |
| 51 | 117 | `ERROR` | `. . .` |

#### `moviefile/src/movie_file/movie_file_common_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 119 | `ERROR` | `,` |
| 25 | 154 | `ERROR` | `,` |
| 35 | 152 | `ERROR` | `,` |

#### `moviefile/src/movie_file/movie_file_consumer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 94 | `ERROR` | `,` |
| 26 | 154 | `ERROR` | `, , ,` |
| 28 | 158 | `ERROR` | `, , ,` |
| 45 | 51 | `ERROR` | `,` |
| 48 | 217 | `ERROR` | `,` |
| 48 | 225 | `ERROR` | `,` |
| 48 | 233 | `ERROR` | `,` |
| 53 | 107 | `ERROR` | `,` |
| 56 | 101 | `ERROR` | `,` |
| 63 | 175 | `ERROR` | `,` |
| 66 | 217 | `ERROR` | `,` |
| 66 | 225 | `ERROR` | `,` |
| 66 | 233 | `ERROR` | `,` |
| 71 | 112 | `ERROR` | `,` |
| 91 | 195 | `ERROR` | `, , ,` |
| 107 | 195 | `ERROR` | `, , ,` |
| 114 | 178 | `ERROR` | `,` |
| 124 | 245 | `ERROR` | `,` |
| 133 | 167 | `ERROR` | `,` |
| 140 | 248 | `ERROR` | `,` |
| … | … | … | *(42 more)* |

#### `moviefile/src/movie_file/movie_file_controller_video.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 180 | `ERROR` | `, , ,` |
| 53 | 183 | `ERROR` | `,` |
| 56 | 243 | `ERROR` | `,` |
| 57 | 17 | `ERROR` | `,` |
| 58 | 17 | `ERROR` | `,` |
| 61 | 160 | `ERROR` | `, , ,` |
| 63 | 194 | `ERROR` | `,` |
| 63 | 202 | `ERROR` | `,` |
| 63 | 210 | `ERROR` | `,` |
| 64 | 130 | `ERROR` | `,` |
| 71 | 127 | `ERROR` | `,` |
| 73 | 160 | `ERROR` | `, , ,` |
| 81 | 202 | `ERROR` | `,` |
| 86 | 229 | `ERROR` | `,` |
| 96 | 142 | `ERROR` | `,` |
| 109 | 218 | `ERROR` | `,` |
| 125 | 116 | `ERROR` | `,` |
| 128 | 209 | `ERROR` | `,` |
| 134 | 189 | `ERROR` | `,` |
| 136 | 198 | `ERROR` | `,` |
| … | … | … | *(17 more)* |

#### `moviefile/src/movie_file/plugin/movie_file_audio_encoder_encode_node.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 162 | `ERROR` | `,` |
| 56 | 162 | `ERROR` | `,` |
| 99 | 165 | `ERROR` | `, , ,` |
| 102 | 174 | `ERROR` | `, , ,` |
| 114 | 223 | `ERROR` | `,` |
| 114 | 231 | `ERROR` | `,` |
| 114 | 239 | `ERROR` | `,` |
| 124 | 229 | `ERROR` | `,` |
| 124 | 237 | `ERROR` | `,` |
| 124 | 245 | `ERROR` | `,` |
| 128 | 224 | `ERROR` | `,` |
| 128 | 232 | `ERROR` | `,` |
| 128 | 240 | `ERROR` | `,` |
| 164 | 140 | `ERROR` | `,` |
| 195 | 124 | `ERROR` | `,` |
| 239 | 204 | `ERROR` | `,` |
| 239 | 212 | `ERROR` | `,` |
| 239 | 220 | `ERROR` | `,` |
| 241 | 202 | `ERROR` | `,` |
| 241 | 210 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `moviefile/src/movie_file/plugin/movie_file_audio_offline_algo_node.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 110 | `ERROR` | `,` |
| 24 | 172 | `ERROR` | `,` |
| 36 | 174 | `ERROR` | `, , ,` |
| 38 | 180 | `ERROR` | `, , ,` |
| 46 | 122 | `ERROR` | `,` |
| 86 | 182 | `ERROR` | `, , ,` |
| 93 | 147 | `ERROR` | `,` |
| 102 | 180 | `ERROR` | `, , ,` |
| 104 | 226 | `ERROR` | `,` |
| 108 | 205 | `ERROR` | `,` |
| 108 | 213 | `ERROR` | `,` |
| 108 | 221 | `ERROR` | `,` |
| 133 | 160 | `ERROR` | `, , ,` |
| 139 | 177 | `ERROR` | `, , ,` |
| 153 | 124 | `ERROR` | `,` |
| 173 | 109 | `ERROR` | `,` |

#### `moviefile/src/movie_file/plugin/movie_file_video_encoder_encode_node.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 165 | `ERROR` | `, , ,` |
| 79 | 228 | `ERROR` | `,` |
| 79 | 236 | `ERROR` | `,` |
| 79 | 244 | `ERROR` | `,` |
| 83 | 231 | `ERROR` | `,` |
| 83 | 239 | `ERROR` | `,` |
| 83 | 247 | `ERROR` | `,` |
| 89 | 209 | `ERROR` | `,` |
| 89 | 217 | `ERROR` | `,` |
| 89 | 225 | `ERROR` | `,` |
| 93 | 226 | `ERROR` | `,` |
| 93 | 234 | `ERROR` | `,` |
| 93 | 242 | `ERROR` | `,` |
| 96 | 224 | `ERROR` | `,` |
| 96 | 232 | `ERROR` | `,` |
| 96 | 240 | `ERROR` | `,` |
| 101 | 136 | `ERROR` | `,` |
| 118 | 116 | `ERROR` | `,` |
| 135 | 168 | `ERROR` | `, , ,` |
| 154 | 200 | `ERROR` | `,` |
| … | … | … | *(28 more)* |

#### `moviefile/src/movie_file/producer/movie_file_video_encoded_buffer_producer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 151 | `ERROR` | `,` |
| 49 | 118 | `ERROR` | `,` |
| 54 | 117 | `ERROR` | `,` |
| 60 | 145 | `ERROR` | `,` |
| 80 | 145 | `ERROR` | `, , ,` |
| 85 | 231 | `ERROR` | `,` |
| 85 | 239 | `ERROR` | `,` |
| 85 | 247 | `ERROR` | `,` |
| 132 | 130 | `ERROR` | `, , ,` |
| 134 | 208 | `ERROR` | `,` |
| 134 | 216 | `ERROR` | `,` |
| 134 | 224 | `ERROR` | `,` |
| 146 | 206 | `ERROR` | `,` |
| 146 | 214 | `ERROR` | `,` |
| 146 | 222 | `ERROR` | `,` |
| 162 | 96 | `ERROR` | `,` |
| 172 | 204 | `ERROR` | `,` |
| 172 | 212 | `ERROR` | `,` |
| 172 | 220 | `ERROR` | `,` |
| 316 | 182 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `moviefile/src/movie_file_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 93 | `ERROR` | `,` |
| 13 | 126 | `ERROR` | `,` |
| 20 | 117 | `ERROR` | `,` |
| 26 | 116 | `ERROR` | `,` |
| 28 | 160 | `ERROR` | `, , ,` |
| 34 | 115 | `ERROR` | `,` |
| 36 | 160 | `ERROR` | `, , ,` |
| 42 | 117 | `ERROR` | `,` |
| 44 | 160 | `ERROR` | `, , ,` |
| 50 | 116 | `ERROR` | `,` |
| 52 | 160 | `ERROR` | `, , ,` |
| 58 | 117 | `ERROR` | `,` |
| 60 | 160 | `ERROR` | `, , ,` |
| 66 | 118 | `ERROR` | `,` |
| 68 | 160 | `ERROR` | `, , ,` |
| 74 | 114 | `ERROR` | `,` |
| 76 | 160 | `ERROR` | `, , ,` |
| 82 | 117 | `ERROR` | `,` |
| 84 | 160 | `ERROR` | `, , ,` |
| 90 | 122 | `ERROR` | `,` |
| … | … | … | *(7 more)* |

#### `moviefile/src/pipeline/producer/unified_pipeline_audio_capture_wrap.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 192 | `ERROR` | `,` |
| 25 | 186 | `ERROR` | `, , ,` |
| 41 | 118 | `ERROR` | `,` |
| 48 | 188 | `ERROR` | `, , ,` |
| 49 | 186 | `ERROR` | `, , ,` |
| 53 | 180 | `ERROR` | `, , ,` |
| 60 | 187 | `ERROR` | `, , ,` |
| 61 | 185 | `ERROR` | `, , ,` |
| 62 | 178 | `ERROR` | `, , ,` |
| 70 | 130 | `ERROR` | `,` |
| 88 | 98 | `ERROR` | `,` |
| 133 | 174 | `ERROR` | `, , ,` |
| 136 | 165 | `ERROR` | `,` |
| 154 | 151 | `ERROR` | `, , ,` |
| 200 | 152 | `ERROR` | `, , ,` |
| 204 | 151 | `ERROR` | `, , ,` |
| 207 | 161 | `ERROR` | `, , ,` |
| 235 | 113 | `ERROR` | `,` |
| 245 | 230 | `ERROR` | `, , ,` |
| 253 | 92 | `ERROR` | `,` |
| … | … | … | *(12 more)* |

#### `moviefile/src/pipeline/producer/unified_pipeline_audio_data_producer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 113 | `ERROR` | `,` |
| 15 | 138 | `ERROR` | `,` |

#### `moviefile/src/pipeline/producer/unified_pipeline_data_producer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 125 | `ERROR` | `,` |
| 31 | 116 | `ERROR` | `,` |

#### `moviefile/src/pipeline/producer/unified_pipeline_surface_data_producer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 234 | `ERROR` | `,` |
| 12 | 259 | `ERROR` | `,` |
| 12 | 284 | `ERROR` | `,` |
| 46 | 141 | `ERROR` | `,` |
| 71 | 142 | `ERROR` | `,` |
| 76 | 147 | `ERROR` | `,` |
| 88 | 138 | `ERROR` | `,` |
| 100 | 126 | `ERROR` | `,` |

#### `moviefile/src/pipeline/unified_pipeline.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 196 | `ERROR` | `,` |

#### `services/camera_service/binder/base/include/icamera_broker.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

#### `services/camera_service/binder/base/include/icamera_multi_stream_output.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 31 | `ERROR` | `: public IRemoteBroker` |

#### `services/camera_service/binder/base/include/istream_capture_photo_callback.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 30 | `ERROR` | `u "` |
| 14 | 60 | `ERROR` | `"` |

#### `services/camera_service/binder/base/include/istream_capture_thumbnail_callback.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 30 | `ERROR` | `u "` |
| 15 | 64 | `ERROR` | `"` |

#### `services/camera_service/binder/base/include/window_manager_service_utils/icamera_mock_session_manager_interface.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 32 | `ERROR` | `"OHOS.IMockSessionManager"` |

#### `services/camera_service/binder/base/include/window_manager_service_utils/icamera_scene_session_manager.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 32 | `ERROR` | `"OHOS.IWindowManager"` |
| 28 | 32 | `ERROR` | `"OHOS.ISceneSessionManager"` |

#### `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_manager_callback.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 32 | `ERROR` | `"OHOS.IWindowManagerAgent"` |

#### `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_session_manager_service.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 32 | `ERROR` | `"OHOS.ISessionManagerService"` |

#### `services/camera_service/binder/base/src/icamera_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 170 | `ERROR` | `,` |
| 139 | 170 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/hcamera_broker_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 227 | `ERROR` | `,` |
| 20 | 237 | `ERROR` | `,` |
| 20 | 247 | `ERROR` | `,` |
| 21 | 109 | `ERROR` | `,` |
| 36 | 226 | `ERROR` | `,` |
| 36 | 236 | `ERROR` | `,` |
| 36 | 246 | `ERROR` | `,` |
| 37 | 108 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/hstream_capture_photo_callback_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 109 | `ERROR` | `,` |
| 16 | 132 | `ERROR` | `,` |
| 17 | 148 | `ERROR` | `, , ,` |
| 26 | 152 | `ERROR` | `, , ,` |
| 28 | 198 | `ERROR` | `,` |
| 28 | 206 | `ERROR` | `,` |
| 28 | 214 | `ERROR` | `,` |
| 38 | 213 | `ERROR` | `,` |
| 38 | 223 | `ERROR` | `,` |
| 38 | 233 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/hstream_capture_thumbnail_callback_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 114 | `ERROR` | `,` |
| 15 | 140 | `ERROR` | `,` |
| 16 | 148 | `ERROR` | `, , ,` |
| 25 | 152 | `ERROR` | `, , ,` |
| 27 | 198 | `ERROR` | `,` |
| 27 | 206 | `ERROR` | `,` |
| 27 | 214 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_mock_session_manager_interface_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 216 | `ERROR` | `,` |
| 16 | 226 | `ERROR` | `,` |
| 16 | 236 | `ERROR` | `,` |
| 18 | 201 | `ERROR` | `,` |
| 18 | 213 | `ERROR` | `,` |
| 18 | 225 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_scene_session_manager_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 158 | `ERROR` | `, , ,` |
| 20 | 134 | `ERROR` | `, , ,` |
| 24 | 216 | `ERROR` | `,` |
| 24 | 226 | `ERROR` | `,` |
| 24 | 236 | `ERROR` | `,` |
| 37 | 158 | `ERROR` | `, , ,` |
| 41 | 134 | `ERROR` | `, , ,` |
| 45 | 218 | `ERROR` | `,` |
| 45 | 228 | `ERROR` | `,` |
| 45 | 238 | `ERROR` | `,` |
| 56 | 98 | `ERROR` | `,` |
| 60 | 94 | `ERROR` | `,` |
| 65 | 86 | `ERROR` | `,` |
| 72 | 222 | `ERROR` | `,` |
| 72 | 232 | `ERROR` | `,` |
| 72 | 242 | `ERROR` | `,` |
| 77 | 109 | `ERROR` | `,` |

#### `services/camera_service/binder/client/src/window_manager_service_proxy_impl/hcamera_window_session_manager_service_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 205 | `ERROR` | `,` |
| 18 | 215 | `ERROR` | `,` |
| 18 | 225 | `ERROR` | `,` |

#### `services/camera_service/binder/server/src/hcamera_broker_stub.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 227 | `ERROR` | `,` |
| 20 | 237 | `ERROR` | `,` |
| 20 | 247 | `ERROR` | `,` |
| 21 | 109 | `ERROR` | `,` |
| 37 | 226 | `ERROR` | `,` |
| 37 | 236 | `ERROR` | `,` |
| 37 | 246 | `ERROR` | `,` |
| 38 | 108 | `ERROR` | `,` |

#### `services/camera_service/binder/server/src/hstream_capture_photo_callback_stub.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 175 | `ERROR` | `,` |
| 26 | 131 | `ERROR` | `,` |
| 30 | 206 | `ERROR` | `,` |
| 30 | 214 | `ERROR` | `,` |
| 30 | 222 | `ERROR` | `,` |
| 32 | 149 | `ERROR` | `, , ,` |
| 34 | 199 | `ERROR` | `,` |
| 34 | 207 | `ERROR` | `,` |
| 34 | 215 | `ERROR` | `,` |

#### `services/camera_service/binder/server/src/hstream_capture_thumbnail_callback_stub.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 184 | `ERROR` | `,` |
| 27 | 139 | `ERROR` | `,` |
| 31 | 206 | `ERROR` | `,` |
| 31 | 214 | `ERROR` | `,` |
| 31 | 222 | `ERROR` | `,` |
| 33 | 149 | `ERROR` | `, , ,` |
| 35 | 199 | `ERROR` | `,` |
| 35 | 207 | `ERROR` | `,` |
| 35 | 215 | `ERROR` | `,` |

#### `services/camera_service/binder/server/src/window_manager_service_callback_stub_impl/hcamera_window_manager_callback_stub.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 174 | `ERROR` | `,` |
| 29 | 197 | `ERROR` | `,` |

#### `services/camera_service/include/camera_rotate_strategy_parser.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 103 | `ERROR` | `,` |
| 25 | 103 | `ERROR` | `,` |

#### `services/camera_service/include/camera_util.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 154 | 82 | `ERROR` | `, . . .` |

#### `services/camera_service/include/hcamera_device_manager.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 156 | `ERROR` | `,` |
| 25 | 205 | `ERROR` | `,` |
| 30 | 215 | `ERROR` | `,` |
| 34 | 210 | `ERROR` | `,` |

#### `services/camera_service/include/hcamera_host_manager.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 131 | `ERROR` | `,` |

#### `services/camera_service/include/hstream_operator.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 163 | 150 | `ERROR` | `,` |

#### `services/camera_service/include/param_update/camera_rotate_param_manager.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 14 | `ERROR` | `CameraRoateParamManager::` |
| 59 | 79 | `ERROR` | `const` |

#### `services/camera_service/include/thread_priority_util.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 150 | `ERROR` | `,` |
| 30 | 166 | `ERROR` | `,` |
| 36 | 177 | `ERROR` | `,` |
| 41 | 194 | `ERROR` | `,` |
| 52 | 171 | `ERROR` | `,` |

#### `services/camera_service/src/adapter/bms_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 156 | `ERROR` | `,` |
| 16 | 159 | `ERROR` | `,` |
| 35 | 104 | `ERROR` | `,` |
| 47 | 147 | `ERROR` | `, , ,` |
| 49 | 149 | `ERROR` | `, , ,` |
| 51 | 144 | `ERROR` | `, , ,` |
| 69 | 161 | `ERROR` | `, , ,` |
| 71 | 157 | `ERROR` | `, , ,` |
| 73 | 164 | `ERROR` | `, , ,` |
| 75 | 137 | `ERROR` | `, , ,` |
| 76 | 151 | `ERROR` | `,` |
| 83 | 110 | `ERROR` | `,` |
| 84 | 149 | `ERROR` | `, , ,` |
| 86 | 132 | `ERROR` | `, , ,` |
| 95 | 155 | `ERROR` | `, , ,` |
| 97 | 192 | `ERROR` | `,` |
| 97 | 200 | `ERROR` | `,` |
| 97 | 208 | `ERROR` | `,` |
| 103 | 112 | `ERROR` | `,` |
| 104 | 170 | `ERROR` | `, , ,` |
| … | … | … | *(5 more)* |

#### `services/camera_service/src/app_manager_utils/camera_app_manager_client.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 97 | `ERROR` | `,` |
| 23 | 118 | `ERROR` | `,` |
| 34 | 203 | `ERROR` | `,` |

#### `services/camera_service/src/app_manager_utils/camera_app_manager_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 84 | `ERROR` | `,` |
| 30 | 159 | `ERROR` | `, , ,` |
| 33 | 205 | `ERROR` | `,` |
| 33 | 228 | `ERROR` | `,` |
| 33 | 251 | `ERROR` | `,` |
| 36 | 158 | `ERROR` | `, , ,` |
| 49 | 199 | `ERROR` | `,` |
| 63 | 185 | `ERROR` | `,` |

#### `services/camera_service/src/applist_manager/camera_applist_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 113 | `ERROR` | `,` |
| 25 | 102 | `ERROR` | `,` |
| 32 | 115 | `ERROR` | `,` |
| 39 | 159 | `ERROR` | `, , ,` |

#### `services/camera_service/src/camera_beauty_notification.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 192 | `ERROR` | `, , ,` |
| 40 | 176 | `ERROR` | `,` |
| 44 | 14 | `ERROR` | `.operator++` |
| 56 | 191 | `ERROR` | `, , ,` |
| 58 | 175 | `ERROR` | `,` |
| 93 | 102 | `ERROR` | `,` |
| 108 | 170 | `ERROR` | `, , ,` |
| 124 | 102 | `ERROR` | `,` |
| 139 | 169 | `ERROR` | `, , ,` |
| 149 | 190 | `ERROR` | `,` |

#### `services/camera_service/src/camera_buffer_manager/photo_asset_auxiliary_consumer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 182 | `ERROR` | `,` |
| 16 | 180 | `ERROR` | `,` |
| 21 | 172 | `ERROR` | `,` |
| 23 | 148 | `ERROR` | `, , ,` |
| 24 | 167 | `ERROR` | `, , ,` |
| 25 | 170 | `ERROR` | `, , ,` |
| 26 | 168 | `ERROR` | `, , ,` |
| 27 | 167 | `ERROR` | `, , ,` |
| 50 | 90 | `ERROR` | `,` |
| 55 | 181 | `ERROR` | `,` |
| 58 | 148 | `ERROR` | `, , ,` |
| 76 | 136 | `ERROR` | `, , ,` |
| 78 | 195 | `ERROR` | `,` |
| 79 | 189 | `ERROR` | `, , ,` |
| 82 | 154 | `ERROR` | `, , ,` |
| 88 | 165 | `ERROR` | `,` |
| 92 | 161 | `ERROR` | `,` |
| 106 | 184 | `ERROR` | `,` |
| 109 | 181 | `ERROR` | `,` |
| 112 | 181 | `ERROR` | `,` |
| … | … | … | *(4 more)* |

#### `services/camera_service/src/camera_buffer_manager/photo_asset_buffer_consumer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 101 | `ERROR` | `,` |
| 15 | 99 | `ERROR` | `,` |
| 20 | 90 | `ERROR` | `,` |
| 23 | 148 | `ERROR` | `, , ,` |
| 25 | 140 | `ERROR` | `, , ,` |
| 32 | 90 | `ERROR` | `,` |
| 37 | 100 | `ERROR` | `,` |
| 40 | 148 | `ERROR` | `, , ,` |
| 41 | 153 | `ERROR` | `, , ,` |
| 47 | 165 | `ERROR` | `, , ,` |
| 53 | 150 | `ERROR` | `, , ,` |
| 58 | 172 | `ERROR` | `,` |
| 62 | 198 | `ERROR` | `,` |
| 71 | 182 | `ERROR` | `,` |
| 73 | 153 | `ERROR` | `,` |
| 76 | 100 | `ERROR` | `,` |
| 82 | 269 | `ERROR` | `,` |

#### `services/camera_service/src/camera_buffer_manager/photo_buffer_consumer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 156 | `ERROR` | `,` |
| 16 | 94 | `ERROR` | `,` |
| 21 | 110 | `ERROR` | `,` |
| 23 | 148 | `ERROR` | `, , ,` |
| 25 | 140 | `ERROR` | `, , ,` |
| 31 | 110 | `ERROR` | `,` |
| 37 | 99 | `ERROR` | `,` |
| 40 | 148 | `ERROR` | `, , ,` |
| 47 | 136 | `ERROR` | `, , ,` |
| 53 | 185 | `ERROR` | `, , ,` |
| 55 | 172 | `ERROR` | `,` |
| 61 | 154 | `ERROR` | `, , ,` |
| 67 | 99 | `ERROR` | `,` |

#### `services/camera_service/src/camera_buffer_manager/picture_assembler.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 93 | `ERROR` | `,` |
| 16 | 91 | `ERROR` | `,` |
| 21 | 99 | `ERROR` | `,` |
| 23 | 148 | `ERROR` | `, , ,` |
| 30 | 185 | `ERROR` | `, , ,` |
| 39 | 179 | `ERROR` | `, , ,` |
| 47 | 179 | `ERROR` | `, , ,` |
| 55 | 181 | `ERROR` | `, , ,` |
| 60 | 229 | `ERROR` | `,` |
| 60 | 249 | `ERROR` | `,` |
| 60 | 269 | `ERROR` | `,` |
| 61 | 99 | `ERROR` | `,` |
| 66 | 106 | `ERROR` | `,` |
| 78 | 106 | `ERROR` | `,` |

#### `services/camera_service/src/camera_buffer_manager/thumbnail_buffer_consumer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 100 | `ERROR` | `,` |
| 14 | 98 | `ERROR` | `,` |
| 21 | 148 | `ERROR` | `, , ,` |
| 22 | 165 | `ERROR` | `, , ,` |
| 28 | 114 | `ERROR` | `,` |
| 33 | 99 | `ERROR` | `,` |
| 36 | 148 | `ERROR` | `, , ,` |
| 40 | 148 | `ERROR` | `, , ,` |
| 46 | 189 | `ERROR` | `, , ,` |
| 51 | 128 | `ERROR` | `,` |
| 55 | 109 | `ERROR` | `,` |
| 57 | 154 | `ERROR` | `, , ,` |
| 58 | 107 | `ERROR` | `,` |
| 68 | 99 | `ERROR` | `,` |

#### `services/camera_service/src/camera_common_event_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 167 | `ERROR` | `,` |
| 34 | 150 | `ERROR` | `, , ,` |
| 36 | 99 | `ERROR` | `,` |
| 46 | 169 | `ERROR` | `,` |
| 74 | 152 | `ERROR` | `, , ,` |

#### `services/camera_service/src/camera_datashare_helper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 172 | `ERROR` | `, , ,` |
| 13 | 177 | `ERROR` | `, , ,` |
| 20 | 156 | `ERROR` | `, , ,` |
| 29 | 103 | `ERROR` | `,` |
| 35 | 127 | `ERROR` | `,` |
| 46 | 174 | `ERROR` | `,` |
| 53 | 156 | `ERROR` | `, , ,` |
| 64 | 167 | `ERROR` | `,` |
| 71 | 167 | `ERROR` | `,` |
| 79 | 172 | `ERROR` | `, , ,` |
| 81 | 177 | `ERROR` | `, , ,` |
| 84 | 155 | `ERROR` | `,` |
| 89 | 157 | `ERROR` | `,` |

#### `services/camera_service/src/camera_dialog_connection.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 163 | `ERROR` | `,` |
| 13 | 150 | `ERROR` | `, , ,` |
| 30 | 171 | `ERROR` | `,` |
| 37 | 166 | `ERROR` | `,` |

#### `services/camera_service/src/camera_dialog_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 108 | `ERROR` | `,` |
| 31 | 150 | `ERROR` | `, , ,` |
| 37 | 134 | `ERROR` | `, , ,` |
| 48 | 145 | `ERROR` | `, , ,` |
| 55 | 159 | `ERROR` | `, , ,` |
| 56 | 106 | `ERROR` | `,` |
| 63 | 119 | `ERROR` | `,` |

#### `services/camera_service/src/camera_fwk_metadata_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 192 | `ERROR` | `,` |
| 59 | 202 | `ERROR` | `,` |
| 59 | 212 | `ERROR` | `,` |
| 62 | 155 | `ERROR` | `,` |
| 66 | 158 | `ERROR` | `,` |
| 73 | 206 | `ERROR` | `,` |
| 73 | 224 | `ERROR` | `,` |
| 73 | 242 | `ERROR` | `,` |
| 81 | 155 | `ERROR` | `, , ,` |
| 91 | 168 | `ERROR` | `, , ,` |
| 100 | 198 | `ERROR` | `,` |
| 100 | 209 | `ERROR` | `,` |
| 100 | 220 | `ERROR` | `,` |
| 106 | 161 | `ERROR` | `, , ,` |
| 113 | 164 | `ERROR` | `,` |
| 129 | 167 | `ERROR` | `,` |
| 133 | 261 | `ERROR` | `,` |
| 138 | 262 | `ERROR` | `,` |
| 143 | 263 | `ERROR` | `,` |
| 148 | 260 | `ERROR` | `,` |
| … | … | … | *(7 more)* |

#### `services/camera_service/src/camera_parameters_config_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 289 | `ERROR` | `,` |
| 12 | 313 | `ERROR` | `,` |
| 12 | 337 | `ERROR` | `,` |
| 14 | 253 | `ERROR` | `,` |
| 14 | 277 | `ERROR` | `,` |
| 14 | 301 | `ERROR` | `,` |
| 52 | 158 | `ERROR` | `, , ,` |

#### `services/camera_service/src/camera_privacy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 153 | `ERROR` | `, , ,` |
| 20 | 253 | `ERROR` | `,` |
| 24 | 160 | `ERROR` | `, , ,` |
| 64 | 191 | `ERROR` | `,` |
| 70 | 173 | `ERROR` | `, , ,` |
| 72 | 152 | `ERROR` | `, , ,` |
| 75 | 258 | `ERROR` | `,` |
| 78 | 125 | `ERROR` | `,` |
| 112 | 144 | `ERROR` | `, , ,` |
| 122 | 167 | `ERROR` | `,` |
| 123 | 153 | `ERROR` | `, , ,` |
| 129 | 146 | `ERROR` | `, , ,` |
| 131 | 167 | `ERROR` | `, , ,` |
| 132 | 111 | `ERROR` | `,` |
| 134 | 169 | `ERROR` | `,` |
| 135 | 155 | `ERROR` | `, , ,` |
| 144 | 167 | `ERROR` | `,` |
| 148 | 170 | `ERROR` | `,` |
| 149 | 156 | `ERROR` | `, , ,` |
| 155 | 183 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `services/camera_service/src/camera_rotate_strategy_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 156 | `ERROR` | `, , ,` |
| 18 | 152 | `ERROR` | `, , ,` |
| 55 | 107 | `ERROR` | `,` |
| 66 | 110 | `ERROR` | `,` |
| 79 | 101 | `ERROR` | `,` |

#### `services/camera_service/src/camera_sensor_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 153 | `ERROR` | `, , ,` |
| 20 | 177 | `ERROR` | `,` |
| 28 | 92 | `ERROR` | `,` |
| 36 | 141 | `ERROR` | `, , ,` |
| 37 | 141 | `ERROR` | `, , ,` |
| 41 | 141 | `ERROR` | `,` |
| 49 | 141 | `ERROR` | `, , ,` |
| 50 | 141 | `ERROR` | `, , ,` |
| 57 | 142 | `ERROR` | `,` |
| 58 | 150 | `ERROR` | `,` |

#### `services/camera_service/src/camera_server_photo_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 94 | `ERROR` | `,` |
| 49 | 162 | `ERROR` | `, , ,` |
| 79 | 99 | `ERROR` | `,` |
| 84 | 162 | `ERROR` | `,` |
| 89 | 162 | `ERROR` | `,` |
| 107 | 99 | `ERROR` | `,` |
| 112 | 167 | `ERROR` | `,` |
| 117 | 167 | `ERROR` | `,` |
| 144 | 109 | `ERROR` | `,` |
| 149 | 96 | `ERROR` | `,` |
| 150 | 148 | `ERROR` | `, , ,` |
| 155 | 146 | `ERROR` | `, , ,` |
| 171 | 106 | `ERROR` | `,` |
| 176 | 149 | `ERROR` | `,` |
| 192 | 175 | `ERROR` | `,` |
| 199 | 178 | `ERROR` | `,` |
| 206 | 178 | `ERROR` | `,` |
| 213 | 115 | `ERROR` | `,` |
| 224 | 110 | `ERROR` | `,` |
| 227 | 165 | `ERROR` | `, , ,` |
| … | … | … | *(12 more)* |

#### `services/camera_service/src/camera_util.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 138 | 162 | `ERROR` | `,` |
| 162 | 163 | `ERROR` | `,` |
| 195 | 167 | `ERROR` | `,` |
| 201 | 43 | `ERROR` | `, . . .` |
| 207 | 98 | `ERROR` | `,` |
| 223 | 208 | `ERROR` | `,` |
| 223 | 219 | `ERROR` | `,` |
| 223 | 230 | `ERROR` | `,` |
| 228 | 200 | `ERROR` | `,` |
| 231 | 174 | `ERROR` | `, , ,` |
| 237 | 199 | `ERROR` | `,` |
| 237 | 210 | `ERROR` | `,` |
| 237 | 221 | `ERROR` | `,` |
| 239 | 170 | `ERROR` | `,` |
| 252 | 218 | `ERROR` | `,` |
| 252 | 228 | `ERROR` | `,` |
| 252 | 238 | `ERROR` | `,` |
| 254 | 156 | `ERROR` | `, , ,` |
| 257 | 188 | `ERROR` | `,` |
| 261 | 189 | `ERROR` | `,` |
| … | … | … | *(40 more)* |

#### `services/camera_service/src/camera_xml_parser.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 89 | 206 | `ERROR` | `,` |
| 89 | 219 | `ERROR` | `,` |
| 89 | 232 | `ERROR` | `,` |
| 91 | 150 | `ERROR` | `, , ,` |
| 97 | 161 | `ERROR` | `, , ,` |
| 103 | 165 | `ERROR` | `, , ,` |
| 123 | 215 | `ERROR` | `,` |
| 123 | 228 | `ERROR` | `,` |
| 123 | 241 | `ERROR` | `,` |
| 132 | 141 | `ERROR` | `, , ,` |
| 140 | 158 | `ERROR` | `, , ,` |
| 169 | 162 | `ERROR` | `, , ,` |
| 176 | 169 | `ERROR` | `, , ,` |

#### `services/camera_service/src/device_protection_ability_connection.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 163 | `ERROR` | `,` |
| 11 | 150 | `ERROR` | `, , ,` |
| 20 | 171 | `ERROR` | `,` |
| 26 | 166 | `ERROR` | `,` |

#### `services/camera_service/src/dfx/camera_report_dfx_uitls.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 110 | `ERROR` | `,` |
| 26 | 226 | `ERROR` | `,` |
| 40 | 198 | `ERROR` | `,` |
| 48 | 183 | `ERROR` | `,` |
| 53 | 115 | `ERROR` | `,` |
| 62 | 186 | `ERROR` | `,` |
| 67 | 118 | `ERROR` | `,` |
| 76 | 184 | `ERROR` | `,` |
| 81 | 116 | `ERROR` | `,` |
| 90 | 182 | `ERROR` | `,` |
| 95 | 114 | `ERROR` | `,` |
| 104 | 180 | `ERROR` | `,` |
| 109 | 112 | `ERROR` | `,` |
| 120 | 130 | `ERROR` | `,` |
| 124 | 142 | `ERROR` | `,` |
| 178 | 132 | `ERROR` | `,` |
| 184 | 150 | `ERROR` | `,` |
| 311 | 130 | `ERROR` | `,` |
| 349 | 236 | `ERROR` | `,` |

#### `services/camera_service/src/dfx/camera_report_uitls.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 112 | `ERROR` | `,` |
| 15 | 127 | `ERROR` | `,` |
| 27 | 114 | `ERROR` | `,` |
| 31 | 129 | `ERROR` | `,` |
| 37 | 132 | `ERROR` | `,` |
| 74 | 112 | `ERROR` | `,` |
| 78 | 164 | `ERROR` | `, , ,` |
| 80 | 126 | `ERROR` | `,` |
| 114 | 117 | `ERROR` | `,` |
| 127 | 115 | `ERROR` | `,` |
| 131 | 138 | `ERROR` | `,` |
| 141 | 115 | `ERROR` | `,` |
| 144 | 178 | `ERROR` | `, , ,` |
| 147 | 132 | `ERROR` | `,` |
| 178 | 195 | `ERROR` | `,` |
| 187 | 118 | `ERROR` | `,` |
| 192 | 112 | `ERROR` | `,` |
| 208 | 120 | `ERROR` | `,` |
| 230 | 116 | `ERROR` | `,` |
| 241 | 114 | `ERROR` | `,` |
| … | … | … | *(18 more)* |

#### `services/camera_service/src/display_plugin/camera_display_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 101 | `ERROR` | `,` |
| 21 | 100 | `ERROR` | `,` |
| 27 | 149 | `ERROR` | `, , ,` |
| 28 | 186 | `ERROR` | `, , ,` |
| 30 | 218 | `ERROR` | `,` |
| 30 | 232 | `ERROR` | `,` |
| 30 | 246 | `ERROR` | `,` |
| 31 | 115 | `ERROR` | `,` |
| 38 | 166 | `ERROR` | `, , ,` |
| 40 | 177 | `ERROR` | `,` |
| 48 | 147 | `ERROR` | `, , ,` |
| 54 | 188 | `ERROR` | `,` |

#### `services/camera_service/src/hcamera_device.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 76 | 173 | `ERROR` | `,` |
| 87 | 125 | `ERROR` | `, , ,` |
| 89 | 263 | `ERROR` | `,` |
| 96 | 161 | `ERROR` | `,` |
| 113 | 174 | `ERROR` | `,` |
| 126 | 164 | `ERROR` | `,` |
| 144 | 183 | `ERROR` | `,` |
| 168 | 185 | `ERROR` | `,` |
| 226 | 183 | `ERROR` | `, , ,` |
| 236 | 168 | `ERROR` | `, , ,` |
| 284 | 111 | `ERROR` | `,` |
| 301 | 109 | `ERROR` | `,` |
| 308 | 119 | `ERROR` | `,` |
| 324 | 175 | `ERROR` | `,` |
| 341 | 186 | `ERROR` | `,` |
| 370 | 210 | `ERROR` | `,` |
| 370 | 222 | `ERROR` | `,` |
| 370 | 234 | `ERROR` | `,` |
| 379 | 175 | `ERROR` | `, , ,` |
| 381 | 159 | `ERROR` | `, , ,` |
| … | … | … | *(206 more)* |

#### `services/camera_service/src/hcamera_device_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 92 | `ERROR` | `,` |
| 34 | 114 | `ERROR` | `,` |
| 45 | 186 | `ERROR` | `,` |
| 51 | 198 | `ERROR` | `,` |
| 64 | 193 | `ERROR` | `,` |
| 66 | 196 | `ERROR` | `,` |
| 71 | 185 | `ERROR` | `,` |
| 78 | 170 | `ERROR` | `, , ,` |
| 96 | 110 | `ERROR` | `,` |
| 101 | 119 | `ERROR` | `,` |
| 106 | 187 | `ERROR` | `,` |
| 112 | 113 | `ERROR` | `,` |
| 123 | 111 | `ERROR` | `,` |
| 129 | 114 | `ERROR` | `,` |
| 137 | 112 | `ERROR` | `,` |
| 149 | 209 | `ERROR` | `,` |
| 156 | 114 | `ERROR` | `,` |
| 166 | 187 | `ERROR` | `, , ,` |
| 184 | 196 | `ERROR` | `, , ,` |
| 187 | 141 | `ERROR` | `, , ,` |
| … | … | … | *(39 more)* |

#### `services/camera_service/src/hcamera_device_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 210 | `ERROR` | `,` |
| 18 | 120 | `ERROR` | `,` |
| 28 | 124 | `ERROR` | `,` |
| 30 | 129 | `ERROR` | `,` |
| 36 | 209 | `ERROR` | `,` |
| 69 | 217 | `ERROR` | `,` |
| 73 | 133 | `ERROR` | `,` |
| 91 | 187 | `ERROR` | `,` |
| 96 | 196 | `ERROR` | `,` |
| 102 | 142 | `ERROR` | `,` |
| 105 | 190 | `ERROR` | `,` |
| 109 | 119 | `ERROR` | `,` |
| 118 | 113 | `ERROR` | `,` |
| 127 | 125 | `ERROR` | `,` |
| 136 | 129 | `ERROR` | `,` |
| 145 | 125 | `ERROR` | `,` |
| 154 | 114 | `ERROR` | `,` |
| 163 | 121 | `ERROR` | `,` |
| 172 | 116 | `ERROR` | `,` |
| 181 | 122 | `ERROR` | `,` |
| … | … | … | *(15 more)* |

#### `services/camera_service/src/hcamera_host_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 100 | `ERROR` | `,` |
| 118 | 101 | `ERROR` | `,` |
| 143 | 197 | `ERROR` | `,` |
| 145 | 117 | `ERROR` | `,` |
| 152 | 117 | `ERROR` | `,` |
| 159 | 117 | `ERROR` | `,` |
| 166 | 117 | `ERROR` | `,` |
| 173 | 117 | `ERROR` | `,` |
| 185 | 172 | `ERROR` | `, , ,` |
| 187 | 155 | `ERROR` | `, , ,` |
| 191 | 121 | `ERROR` | `,` |
| 194 | 121 | `ERROR` | `,` |
| 197 | 121 | `ERROR` | `,` |
| 201 | 121 | `ERROR` | `,` |
| 204 | 121 | `ERROR` | `,` |
| 210 | 199 | `ERROR` | `, , ,` |
| 214 | 155 | `ERROR` | `,` |
| 229 | 212 | `ERROR` | `,` |
| 229 | 231 | `ERROR` | `,` |
| 229 | 250 | `ERROR` | `,` |
| … | … | … | *(133 more)* |

#### `services/camera_service/src/hcamera_movie_file_output.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 119 | `ERROR` | `,` |
| 64 | 150 | `ERROR` | `, , ,` |
| 101 | 106 | `ERROR` | `,` |
| 103 | 150 | `ERROR` | `, , ,` |
| 109 | 169 | `ERROR` | `, , ,` |
| 113 | 170 | `ERROR` | `, , ,` |
| 119 | 104 | `ERROR` | `,` |
| 126 | 105 | `ERROR` | `,` |
| 130 | 150 | `ERROR` | `, , ,` |
| 136 | 124 | `ERROR` | `,` |
| 147 | 113 | `ERROR` | `,` |
| 150 | 103 | `ERROR` | `,` |
| 157 | 106 | `ERROR` | `,` |
| 161 | 150 | `ERROR` | `, , ,` |
| 167 | 125 | `ERROR` | `,` |
| 175 | 104 | `ERROR` | `,` |
| 183 | 150 | `ERROR` | `, , ,` |
| 212 | 150 | `ERROR` | `, , ,` |
| 233 | 127 | `ERROR` | `,` |
| 241 | 129 | `ERROR` | `,` |
| … | … | … | *(4 more)* |

#### `services/camera_service/src/hcamera_restore_param.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 123 | `ERROR` | `,` |
| 23 | 123 | `ERROR` | `,` |

#### `services/camera_service/src/hcamera_service.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 1 | `ERROR` | `REGISTER_SYSTEM_ABILITY_BY_ID(HCameraService , CAMERA_SERVICE_ID , true) constexpr` |
| 92 | 101 | `ERROR` | `,` |
| 96 | 194 | `ERROR` | `, , ,` |
| 97 | 99 | `ERROR` | `,` |
| 110 | 99 | `ERROR` | `,` |
| 116 | 175 | `ERROR` | `, , ,` |
| 124 | 108 | `ERROR` | `,` |
| 126 | 108 | `ERROR` | `,` |
| 136 | 168 | `ERROR` | `, , ,` |
| 139 | 97 | `ERROR` | `,` |
| 150 | 171 | `ERROR` | `, , ,` |
| 151 | 183 | `ERROR` | `,` |
| 158 | 110 | `ERROR` | `,` |
| 163 | 163 | `ERROR` | `, , ,` |
| 170 | 112 | `ERROR` | `,` |
| 178 | 100 | `ERROR` | `,` |
| 183 | 100 | `ERROR` | `,` |
| 194 | 171 | `ERROR` | `, , ,` |
| 197 | 201 | `ERROR` | `,` |
| 198 | 165 | `ERROR` | `, , ,` |
| … | … | … | *(319 more)* |

#### `services/camera_service/src/hcamera_switch_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 119 | `ERROR` | `,` |
| 15 | 120 | `ERROR` | `,` |
| 35 | 181 | `ERROR` | `,` |
| 62 | 194 | `ERROR` | `,` |
| 72 | 241 | `ERROR` | `,` |
| 80 | 116 | `ERROR` | `,` |
| 87 | 113 | `ERROR` | `,` |
| 89 | 200 | `ERROR` | `, , ,` |
| 93 | 155 | `ERROR` | `, , ,` |
| 96 | 192 | `ERROR` | `,` |
| 98 | 153 | `ERROR` | `,` |
| 100 | 214 | `ERROR` | `,` |
| 100 | 226 | `ERROR` | `,` |
| 100 | 238 | `ERROR` | `,` |
| 103 | 154 | `ERROR` | `,` |
| 106 | 207 | `ERROR` | `,` |
| 106 | 219 | `ERROR` | `,` |
| 106 | 231 | `ERROR` | `,` |
| 108 | 211 | `ERROR` | `,` |
| 108 | 223 | `ERROR` | `,` |
| … | … | … | *(22 more)* |

#### `services/camera_service/src/hcapture_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 86 | 132 | `ERROR` | `,` |
| 94 | 137 | `ERROR` | `, , ,` |
| 100 | 210 | `ERROR` | `,` |
| 100 | 219 | `ERROR` | `,` |
| 100 | 228 | `ERROR` | `,` |
| 118 | 137 | `ERROR` | `, , ,` |
| 120 | 164 | `ERROR` | `, , ,` |
| 123 | 143 | `ERROR` | `, , ,` |
| 126 | 153 | `ERROR` | `, , ,` |
| 142 | 137 | `ERROR` | `, , ,` |
| 156 | 145 | `ERROR` | `, , ,` |
| 160 | 188 | `ERROR` | `,` |
| 177 | 137 | `ERROR` | `, , ,` |
| 194 | 145 | `ERROR` | `, , ,` |
| 198 | 162 | `ERROR` | `,` |
| 209 | 137 | `ERROR` | `, , ,` |
| 211 | 132 | `ERROR` | `, , ,` |
| 218 | 148 | `ERROR` | `,` |
| 330 | 157 | `ERROR` | `, , ,` |
| 372 | 189 | `ERROR` | `,` |
| … | … | … | *(219 more)* |

#### `services/camera_service/src/hcapture_session_wrapper.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 242 | `ERROR` | `,` |
| 19 | 123 | `ERROR` | `,` |
| 29 | 127 | `ERROR` | `,` |
| 31 | 132 | `ERROR` | `,` |
| 37 | 167 | `ERROR` | `,` |
| 40 | 122 | `ERROR` | `,` |
| 46 | 180 | `ERROR` | `, , ,` |
| 91 | 175 | `ERROR` | `,` |
| 100 | 189 | `ERROR` | `,` |
| 107 | 147 | `ERROR` | `,` |
| 114 | 148 | `ERROR` | `,` |
| 118 | 121 | `ERROR` | `,` |
| 146 | 138 | `ERROR` | `,` |
| 163 | 141 | `ERROR` | `,` |
| 172 | 202 | `ERROR` | `,` |
| 182 | 195 | `ERROR` | `,` |
| 187 | 188 | `ERROR` | `,` |
| 201 | 194 | `ERROR` | `,` |
| 205 | 229 | `ERROR` | `,` |
| 246 | 206 | `ERROR` | `,` |
| … | … | … | *(59 more)* |

#### `services/camera_service/src/hmech_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 168 | `ERROR` | `,` |
| 15 | 140 | `ERROR` | `,` |
| 20 | 180 | `ERROR` | `,` |
| 34 | 140 | `ERROR` | `,` |
| 43 | 135 | `ERROR` | `,` |
| 49 | 163 | `ERROR` | `,` |
| 55 | 140 | `ERROR` | `,` |

#### `services/camera_service/src/hshared_camera_device.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 126 | `ERROR` | `,` |
| 20 | 207 | `ERROR` | `,` |
| 35 | 127 | `ERROR` | `,` |
| 67 | 207 | `ERROR` | `,` |
| 74 | 146 | `ERROR` | `,` |
| 82 | 204 | `ERROR` | `,` |
| 93 | 189 | `ERROR` | `,` |
| 96 | 193 | `ERROR` | `,` |
| 106 | 179 | `ERROR` | `,` |
| 111 | 133 | `ERROR` | `,` |
| 116 | 141 | `ERROR` | `,` |
| 122 | 178 | `ERROR` | `,` |
| 126 | 170 | `ERROR` | `,` |
| 139 | 185 | `ERROR` | `,` |
| 176 | 190 | `ERROR` | `,` |
| 194 | 191 | `ERROR` | `,` |
| 202 | 197 | `ERROR` | `,` |
| 216 | 136 | `ERROR` | `,` |
| 221 | 137 | `ERROR` | `,` |
| 227 | 170 | `ERROR` | `,` |
| … | … | … | *(36 more)* |

#### `services/camera_service/src/hshared_capture_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 137 | `ERROR` | `,` |
| 21 | 124 | `ERROR` | `,` |
| 26 | 125 | `ERROR` | `,` |
| 45 | 182 | `ERROR` | `,` |
| 53 | 187 | `ERROR` | `,` |
| 62 | 167 | `ERROR` | `,` |
| 67 | 104 | `ERROR` | `,` |
| 75 | 192 | `ERROR` | `,` |
| 87 | 193 | `ERROR` | `,` |
| 94 | 17 | `ERROR` | `,` |
| 102 | 125 | `ERROR` | `,` |
| 145 | 131 | `ERROR` | `,` |
| 156 | 194 | `ERROR` | `,` |
| 222 | 126 | `ERROR` | `,` |
| 229 | 127 | `ERROR` | `,` |
| 234 | 210 | `ERROR` | `,` |
| 237 | 135 | `ERROR` | `,` |
| 243 | 141 | `ERROR` | `,` |
| 246 | 144 | `ERROR` | `,` |
| 260 | 126 | `ERROR` | `,` |
| … | … | … | *(51 more)* |

#### `services/camera_service/src/hstream_capture.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 104 | `ERROR` | `,` |
| 66 | 93 | `ERROR` | `,` |
| 69 | 98 | `ERROR` | `,` |
| 72 | 175 | `ERROR` | `,` |
| 74 | 137 | `ERROR` | `, , ,` |
| 84 | 96 | `ERROR` | `,` |
| 85 | 160 | `ERROR` | `, , ,` |
| 87 | 160 | `ERROR` | `, , ,` |
| 147 | 164 | `ERROR` | `,` |
| 189 | 169 | `ERROR` | `,` |
| 235 | 187 | `ERROR` | `,` |
| 247 | 189 | `ERROR` | `,` |
| 263 | 136 | `ERROR` | `,` |
| 295 | 210 | `ERROR` | `,` |
| 302 | 201 | `ERROR` | `,` |
| 339 | 169 | `ERROR` | `,` |
| 344 | 163 | `ERROR` | `, , ,` |
| 351 | 180 | `ERROR` | `,` |
| 357 | 153 | `ERROR` | `,` |
| 365 | 148 | `ERROR` | `, , ,` |
| … | … | … | *(131 more)* |

#### `services/camera_service/src/hstream_common.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 111 | `ERROR` | `,` |
| 76 | 111 | `ERROR` | `,` |
| 88 | 225 | `ERROR` | `,` |
| 99 | 14 | `ERROR` | `,` |
| 102 | 189 | `ERROR` | `,` |
| 115 | 172 | `ERROR` | `,` |
| 132 | 243 | `ERROR` | `,` |
| 147 | 179 | `ERROR` | `, , ,` |
| 183 | 150 | `ERROR` | `,` |
| 185 | 237 | `ERROR` | `,` |
| 197 | 106 | `ERROR` | `,` |
| 200 | 116 | `ERROR` | `,` |
| 285 | 178 | `ERROR` | `,` |
| 310 | 182 | `ERROR` | `, , ,` |
| 314 | 124 | `ERROR` | `,` |
| 316 | 183 | `ERROR` | `,` |
| 323 | 134 | `ERROR` | `,` |
| 325 | 193 | `ERROR` | `,` |
| 332 | 128 | `ERROR` | `,` |
| 334 | 188 | `ERROR` | `,` |
| … | … | … | *(3 more)* |

#### `services/camera_service/src/hstream_depth_data.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 171 | `ERROR` | `,` |
| 33 | 249 | `ERROR` | `,` |
| 34 | 26 | `ERROR` | `,` |
| 35 | 26 | `ERROR` | `,` |
| 53 | 170 | `ERROR` | `,` |
| 71 | 134 | `ERROR` | `,` |
| 75 | 135 | `ERROR` | `,` |
| 79 | 173 | `ERROR` | `, , ,` |
| 93 | 230 | `ERROR` | `,` |
| 128 | 258 | `ERROR` | `,` |
| 136 | 168 | `ERROR` | `,` |
| 153 | 146 | `ERROR` | `, , ,` |
| 156 | 258 | `ERROR` | `,` |
| 159 | 160 | `ERROR` | `, , ,` |
| 165 | 215 | `ERROR` | `,` |
| 192 | 168 | `ERROR` | `, , ,` |
| 254 | 135 | `ERROR` | `,` |
| 263 | 163 | `ERROR` | `,` |

#### `services/camera_service/src/hstream_metadata.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 106 | 135 | `ERROR` | `,` |
| 112 | 163 | `ERROR` | `,` |
| 119 | 161 | `ERROR` | `, , ,` |
| 129 | 162 | `ERROR` | `, , ,` |
| 137 | 114 | `ERROR` | `,` |
| 147 | 117 | `ERROR` | `,` |
| 167 | 163 | `ERROR` | `, , ,` |
| 182 | 236 | `ERROR` | `,` |
| 203 | 165 | `ERROR` | `,` |
| 211 | 139 | `ERROR` | `, , ,` |
| 263 | 146 | `ERROR` | `, , ,` |
| 326 | 154 | `ERROR` | `, , ,` |
| 373 | 185 | `ERROR` | `,` |
| 373 | 193 | `ERROR` | `,` |
| 373 | 201 | `ERROR` | `,` |
| 375 | 185 | `ERROR` | `,` |
| 375 | 193 | `ERROR` | `,` |
| 375 | 201 | `ERROR` | `,` |
| 378 | 185 | `ERROR` | `,` |
| 378 | 193 | `ERROR` | `,` |
| … | … | … | *(5 more)* |

#### `services/camera_service/src/hstream_operator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 162 | `ERROR` | `,` |
| 76 | 185 | `ERROR` | `,` |
| 93 | 226 | `ERROR` | `,` |
| 141 | 167 | `ERROR` | `, , ,` |
| 142 | 235 | `ERROR` | `,` |
| 145 | 224 | `ERROR` | `, , ,` |
| 149 | 140 | `ERROR` | `, , ,` |
| 154 | 184 | `ERROR` | `,` |
| 165 | 164 | `ERROR` | `, , ,` |
| 217 | 113 | `ERROR` | `,` |
| 242 | 194 | `ERROR` | `,` |
| 250 | 116 | `ERROR` | `,` |
| 257 | 147 | `ERROR` | `, , ,` |
| 277 | 170 | `ERROR` | `, , ,` |
| 278 | 239 | `ERROR` | `,` |
| 282 | 145 | `ERROR` | `, , ,` |
| 289 | 197 | `ERROR` | `,` |
| 300 | 140 | `ERROR` | `, , ,` |
| 317 | 183 | `ERROR` | `, , ,` |
| 337 | 150 | `ERROR` | `, , ,` |
| … | … | … | *(163 more)* |

#### `services/camera_service/src/hstream_operator_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 94 | `ERROR` | `,` |
| 15 | 105 | `ERROR` | `,` |
| 20 | 111 | `ERROR` | `,` |
| 29 | 116 | `ERROR` | `,` |
| 39 | 118 | `ERROR` | `,` |
| 43 | 201 | `ERROR` | `,` |
| 49 | 201 | `ERROR` | `,` |
| 53 | 249 | `ERROR` | `,` |
| 54 | 21 | `ERROR` | `,` |
| 55 | 21 | `ERROR` | `,` |
| 61 | 119 | `ERROR` | `,` |
| 69 | 201 | `ERROR` | `,` |
| 76 | 194 | `ERROR` | `,` |
| 83 | 192 | `ERROR` | `,` |
| 95 | 114 | `ERROR` | `,` |
| 99 | 29 | `ERROR` | `,` |

#### `services/camera_service/src/hstream_repeat.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 246 | `ERROR` | `,` |
| 40 | 26 | `ERROR` | `,` |
| 41 | 26 | `ERROR` | `,` |
| 56 | 165 | `ERROR` | `,` |
| 87 | 164 | `ERROR` | `, , ,` |
| 152 | 203 | `ERROR` | `,` |
| 167 | 110 | `ERROR` | `,` |
| 178 | 108 | `ERROR` | `,` |
| 184 | 114 | `ERROR` | `,` |
| 188 | 106 | `ERROR` | `,` |
| 210 | 122 | `ERROR` | `,` |
| 279 | 108 | `ERROR` | `,` |
| 285 | 189 | `ERROR` | `,` |
| 294 | 129 | `ERROR` | `,` |
| 305 | 127 | `ERROR` | `,` |
| 311 | 132 | `ERROR` | `,` |
| 322 | 130 | `ERROR` | `,` |
| 333 | 112 | `ERROR` | `,` |
| 345 | 110 | `ERROR` | `,` |
| 350 | 117 | `ERROR` | `,` |
| … | … | … | *(106 more)* |

#### `services/camera_service/src/json_cache_converter/json_cache_converter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 101 | `ERROR` | `,` |
| 24 | 147 | `ERROR` | `, , ,` |
| 29 | 170 | `ERROR` | `, , ,` |
| 30 | 170 | `ERROR` | `, , ,` |
| 40 | 114 | `ERROR` | `,` |
| 41 | 165 | `ERROR` | `, , ,` |
| 59 | 114 | `ERROR` | `,` |
| 60 | 165 | `ERROR` | `, , ,` |
| 64 | 171 | `ERROR` | `, , ,` |
| 65 | 167 | `ERROR` | `, , ,` |
| 66 | 170 | `ERROR` | `, , ,` |
| 67 | 166 | `ERROR` | `, , ,` |
| 83 | 115 | `ERROR` | `,` |
| 84 | 167 | `ERROR` | `, , ,` |
| 87 | 171 | `ERROR` | `, , ,` |
| 88 | 167 | `ERROR` | `, , ,` |
| 101 | 115 | `ERROR` | `,` |
| 102 | 162 | `ERROR` | `, , ,` |
| 103 | 169 | `ERROR` | `, , ,` |
| 149 | 121 | `ERROR` | `,` |
| … | … | … | *(34 more)* |

#### `services/camera_service/src/media_library/photo_asset_adapter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 93 | `ERROR` | `,` |
| 14 | 165 | `ERROR` | `, , ,` |
| 16 | 156 | `ERROR` | `, , ,` |
| 18 | 135 | `ERROR` | `, , ,` |
| 23 | 193 | `ERROR` | `,` |
| 23 | 201 | `ERROR` | `,` |
| 23 | 209 | `ERROR` | `,` |
| 25 | 160 | `ERROR` | `,` |
| 26 | 110 | `ERROR` | `,` |

#### `services/camera_service/src/media_library/photo_asset_proxy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 153 | `ERROR` | `, , ,` |
| 16 | 173 | `ERROR` | `, , ,` |
| 19 | 161 | `ERROR` | `, , ,` |
| 29 | 179 | `ERROR` | `, , ,` |
| 30 | 177 | `ERROR` | `, , ,` |
| 37 | 183 | `ERROR` | `, , ,` |
| 46 | 160 | `ERROR` | `, , ,` |
| 52 | 180 | `ERROR` | `, , ,` |
| 59 | 167 | `ERROR` | `, , ,` |
| 65 | 179 | `ERROR` | `, , ,` |

#### `services/camera_service/src/param_update/camera_rotate_param_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 80 | `ERROR` | `,` |
| 62 | 78 | `ERROR` | `,` |
| 63 | 147 | `ERROR` | `, , ,` |
| 65 | 158 | `ERROR` | `,` |
| 74 | 147 | `ERROR` | `, , ,` |
| 85 | 181 | `ERROR` | `, , ,` |
| 87 | 178 | `ERROR` | `, , ,` |
| 96 | 103 | `ERROR` | `,` |
| 101 | 108 | `ERROR` | `,` |
| 104 | 94 | `ERROR` | `,` |
| 109 | 152 | `ERROR` | `, , ,` |
| 111 | 83 | `ERROR` | `,` |
| 112 | 138 | `ERROR` | `, , ,` |
| 122 | 199 | `ERROR` | `,` |
| 122 | 218 | `ERROR` | `,` |
| 122 | 237 | `ERROR` | `,` |
| 123 | 83 | `ERROR` | `,` |
| 129 | 147 | `ERROR` | `, , ,` |
| 153 | 156 | `ERROR` | `, , ,` |
| 159 | 152 | `ERROR` | `, , ,` |
| … | … | … | *(16 more)* |

#### `services/camera_service/src/param_update/camera_rotate_param_reader.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 147 | `ERROR` | `,` |
| 40 | 37 | `ERROR` | `,` |
| 48 | 168 | `ERROR` | `,` |
| 69 | 167 | `ERROR` | `,` |
| 80 | 200 | `ERROR` | `,` |
| 80 | 226 | `ERROR` | `,` |
| 80 | 252 | `ERROR` | `,` |
| 82 | 224 | `ERROR` | `,` |
| 82 | 249 | `ERROR` | `,` |
| 82 | 274 | `ERROR` | `,` |
| 93 | 170 | `ERROR` | `, , ,` |
| 96 | 204 | `ERROR` | `,` |
| 96 | 227 | `ERROR` | `,` |
| 96 | 250 | `ERROR` | `,` |
| 112 | 198 | `ERROR` | `,` |
| 112 | 223 | `ERROR` | `,` |
| 112 | 248 | `ERROR` | `,` |
| 131 | 169 | `ERROR` | `, , ,` |

#### `services/camera_service/src/param_update/camera_rotate_param_sign_tools.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 86 | `ERROR` | `,` |
| 24 | 183 | `ERROR` | `, , ,` |
| 30 | 126 | `ERROR` | `,` |
| 42 | 137 | `ERROR` | `, , ,` |
| 45 | 110 | `ERROR` | `,` |
| 51 | 86 | `ERROR` | `,` |
| 57 | 119 | `ERROR` | `,` |
| 63 | 129 | `ERROR` | `,` |
| 69 | 150 | `ERROR` | `,` |
| 99 | 168 | `ERROR` | `, , ,` |
| 123 | 193 | `ERROR` | `,` |

#### `services/camera_service/src/recorder/movie_file_recorder.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 54 | 104 | `ERROR` | `,` |
| 61 | 103 | `ERROR` | `,` |
| 70 | 104 | `ERROR` | `,` |
| 72 | 134 | `ERROR` | `, , ,` |
| 81 | 107 | `ERROR` | `,` |
| 83 | 134 | `ERROR` | `, , ,` |
| 89 | 105 | `ERROR` | `,` |
| 91 | 134 | `ERROR` | `, , ,` |
| 93 | 140 | `ERROR` | `, , ,` |
| 102 | 91 | `ERROR` | `,` |
| 112 | 105 | `ERROR` | `,` |
| 114 | 134 | `ERROR` | `, , ,` |
| 123 | 106 | `ERROR` | `,` |
| 125 | 134 | `ERROR` | `, , ,` |
| 139 | 172 | `ERROR` | `,` |
| 141 | 134 | `ERROR` | `, , ,` |
| 153 | 139 | `ERROR` | `, , ,` |
| 156 | 116 | `ERROR` | `,` |
| 159 | 123 | `ERROR` | `,` |
| 162 | 156 | `ERROR` | `,` |
| … | … | … | *(61 more)* |

#### `services/camera_service/src/rotate_plugin/camera_rotate_plugin.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 111 | `ERROR` | `,` |
| 22 | 165 | `ERROR` | `, , ,` |
| 24 | 166 | `ERROR` | `, , ,` |
| 30 | 99 | `ERROR` | `,` |
| 129 | 120 | `ERROR` | `,` |
| 133 | 203 | `ERROR` | `,` |
| 183 | 168 | `ERROR` | `, , ,` |
| 189 | 179 | `ERROR` | `,` |
| 198 | 222 | `ERROR` | `,` |
| 198 | 248 | `ERROR` | `,` |
| 198 | 274 | `ERROR` | `,` |
| 200 | 201 | `ERROR` | `,` |
| 200 | 227 | `ERROR` | `,` |
| 200 | 253 | `ERROR` | `,` |
| 228 | 90 | `ERROR` | `,` |
| 232 | 147 | `ERROR` | `, , ,` |
| 243 | 168 | `ERROR` | `, , ,` |
| 246 | 163 | `ERROR` | `, , ,` |
| 249 | 151 | `ERROR` | `, , ,` |
| 267 | 158 | `ERROR` | `, , ,` |
| … | … | … | *(21 more)* |

#### `services/camera_service/src/rss/suspend_state_observer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 7 | 112 | `ERROR` | `,` |
| 24 | 112 | `ERROR` | `,` |

#### `services/camera_service/src/smooth_zoom/cubic_bezier.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 164 | `ERROR` | `,` |
| 42 | 157 | `ERROR` | `,` |

#### `services/camera_service/src/window_manager_utils/camera_window_manager_agent.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 191 | `ERROR` | `,` |

#### `services/camera_service/src/window_manager_utils/camera_window_manager_client.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 118 | `ERROR` | `,` |
| 33 | 104 | `ERROR` | `,` |
| 39 | 105 | `ERROR` | `,` |
| 41 | 220 | `ERROR` | `,` |
| 41 | 228 | `ERROR` | `,` |
| 41 | 236 | `ERROR` | `,` |
| 42 | 102 | `ERROR` | `,` |
| 48 | 106 | `ERROR` | `,` |
| 54 | 105 | `ERROR` | `,` |
| 56 | 222 | `ERROR` | `,` |
| 56 | 230 | `ERROR` | `,` |
| 56 | 238 | `ERROR` | `,` |
| 57 | 104 | `ERROR` | `,` |
| 63 | 96 | `ERROR` | `,` |
| 68 | 105 | `ERROR` | `,` |
| 70 | 160 | `ERROR` | `,` |
| 72 | 94 | `ERROR` | `,` |
| 77 | 93 | `ERROR` | `,` |
| 80 | 162 | `ERROR` | `, , ,` |
| 83 | 150 | `ERROR` | `, , ,` |
| … | … | … | *(16 more)* |

#### `services/deferred_processing_service/include/base/blocking_queue.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 0 | 0 | `?` | `line 11 col 66 (missing )) ` |
| 8 | 20 | `ERROR` | `T` |
| 11 | 33 | `ERROR` | `:: string&` |
| 11 | 68 | `ERROR` | `0) :` |
| 12 | 35 | `ERROR` | `> 0? capacity: std:: numeric_limits< size_t>::` |
| 12 | 91 | `ERROR` | `isActive_(true) { } ~` |
| 18 | 4 | `ERROR` | `:: unique_lock< std:` |
| 23 | 4 | `ERROR` | `:: unique_lock< std:` |
| 32 | 4 | `ERROR` | `:: unique_lock< std:` |
| 35 | 18 | `ERROR` | `&` |
| 37 | 4 | `ERROR` | `:: unique_lock< std:` |
| 38 | 9 | `ERROR` | `=` |
| 45 | 18 | `ERROR` | `&` |
| 47 | 4 | `ERROR` | `:: unique_lock< std:` |
| 48 | 9 | `ERROR` | `=` |
| 55 | 12 | `ERROR` | `&&` |
| 57 | 4 | `ERROR` | `:: unique_lock< std:` |
| 58 | 9 | `ERROR` | `=` |
| 60 | 12 | `ERROR` | `std::` |
| 67 | 4 | `ERROR` | `:: unique_lock< std:` |
| … | … | … | *(37 more)* |

#### `services/deferred_processing_service/include/base/dps.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 55 | `ERROR` | `. . .` |
| 16 | 46 | `ERROR` | `. . .` |
| 18 | 66 | `ERROR` | `. . .` |
| 22 | 24 | `ERROR` | `*` |
| 30 | 39 | `ERROR` | `. . .` |
| 31 | 32 | `ERROR` | `. . .` |
| 31 | 42 | `missing ;` | `` |
| 33 | 93 | `ERROR` | `. . .` |
| 37 | 39 | `ERROR` | `. . .` |
| 38 | 38 | `ERROR` | `. . .` |
| 38 | 48 | `missing ;` | `` |
| 40 | 99 | `ERROR` | `. . .` |

#### `services/deferred_processing_service/include/base/enable_shared_create.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 20 | `ERROR` | `. . .` |
| 14 | 47 | `ERROR` | `. . .` |
| 17 | 34 | `ERROR` | `. . .` |
| 17 | 79 | `ERROR` | `. . .` |
| 20 | 101 | `ERROR` | `. . .` |
| 21 | 78 | `ERROR` | `.` |
| 25 | 25 | `missing ::` | `` |
| 48 | 16 | `missing }` | `` |
| 51 | 8 | `ERROR` | `: template` |
| 53 | 1 | `ERROR` | `friend void` |
| 69 | 20 | `ERROR` | `. . .` |
| 70 | 47 | `ERROR` | `. . .` |
| 72 | 80 | `ERROR` | `. . .` |
| 73 | 25 | `ERROR` | `->` |
| 73 | 40 | `ERROR` | `!= 0` |
| 79 | 9 | `ERROR` | `int32_t` |
| 81 | 1 | `ERROR` | `} } }` |

#### `services/deferred_processing_service/include/deferred_processing_service.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 45 | `missing ;` | `` |

#### `services/deferred_processing_service/include/dfx/dps_video_report.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 34 | `missing ;` | `` |

#### `services/deferred_processing_service/include/event_monitor/base/events_strategy.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 80 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/event_monitor/events_monitor.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 33 | `missing ;` | `` |

#### `services/deferred_processing_service/include/post_processor/command/video_process_command.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 80 | `ERROR` | `,` |
| 46 | 80 | `ERROR` | `,` |
| 63 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/post_processor/photo_process_result.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 80 | `ERROR` | `,` |
| 73 | 80 | `ERROR` | `,` |
| 124 | 212 | `ERROR` | `,` |
| 169 | 166 | `ERROR` | `,` |
| 171 | 112 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/post_processor/video_process_result.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 180 | `ERROR` | `,` |
| 36 | 203 | `ERROR` | `,` |
| 36 | 226 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/schedule/base/istate.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 80 | `ERROR` | `,` |
| 45 | 186 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/schedule/state/state_factory.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 143 | `ERROR` | `,` |

#### `services/deferred_processing_service/include/schedule/video_processor/strategy/ivideo_strategy.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 8 | `ERROR` | `~` |
| 13 | 29 | `missing ;` | `` |
| 14 | 28 | `missing ;` | `` |
| 15 | 22 | `missing ;` | `` |

#### `services/deferred_processing_service/include/schedule/video_processor/video_job_repository/ivideo_job_repository_listener.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 8 | `ERROR` | `~` |
| 13 | 9 | `ERROR` | `void` |
| 13 | 57 | `ERROR` | `&` |

#### `services/deferred_processing_service/include/utils/dp_power_manager.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 36 | `missing ;` | `` |

#### `services/deferred_processing_service/include/utils/dp_safe_map.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 20 | `ERROR` | `K` |
| 8 | 33 | `ERROR` | `V` |
| 11 | 13 | `missing ;` | `` |
| 13 | 15 | `missing ;` | `` |
| 15 | 28 | `ERROR` | `& rhs` |
| 16 | 1 | `ERROR` | `{` |
| 20 | 23 | `ERROR` | `const DpsSafeMap` |
| 20 | 45 | `missing ;` | `` |
| 25 | 9 | `missing identifier` | `` |
| 26 | 4 | `ERROR` | `:: lock_guard< std:` |
| 27 | 1 | `ERROR` | `map_= std::` |
| 32 | 22 | `ERROR` | `. .` |
| 33 | 5 | `missing ;` | `` |
| 33 | 13 | `ERROR` | `(const K` |
| 33 | 36 | `ERROR` | `. . .` |
| 35 | 4 | `ERROR` | `:: lock_guard< std:` |
| 36 | 21 | `ERROR` | `std::` |
| 36 | 48 | `ERROR` | `. . .` |
| 39 | 26 | `ERROR` | `&` |
| 39 | 41 | `ERROR` | `&` |
| … | … | … | *(17 more)* |

#### `services/deferred_processing_service/include/utils/dp_timer.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 28 | `missing ;` | `` |

#### `services/deferred_processing_service/include/utils/dp_utils.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 33 | `ERROR` | `. . .` |
| 73 | 34 | `ERROR` | `. . .` |
| 73 | 75 | `ERROR` | `. . .` |
| 78 | 33 | `ERROR` | `. . .` |
| 79 | 42 | `ERROR` | `. . .` |
| 79 | 52 | `missing ;` | `` |
| 81 | 53 | `ERROR` | `&& . . .` |
| 81 | 91 | `ERROR` | `. . .` |
| 84 | 33 | `ERROR` | `. . .` |
| 86 | 34 | `ERROR` | `. . .` |
| 86 | 75 | `ERROR` | `. . .` |
| 91 | 33 | `ERROR` | `. . .` |
| 92 | 42 | `ERROR` | `. . .` |
| 92 | 52 | `missing ;` | `` |
| 94 | 53 | `ERROR` | `&& . . .` |
| 94 | 91 | `ERROR` | `. . .` |
| 116 | 248 | `ERROR` | `,` |
| 116 | 265 | `ERROR` | `,` |
| 116 | 282 | `ERROR` | `,` |
| 118 | 134 | `ERROR` | `, , ,` |
| … | … | … | *(6 more)* |

#### `services/deferred_processing_service/src/base/basic_definitions.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 150 | `ERROR` | `,` |
| 72 | 151 | `ERROR` | `,` |
| 101 | 152 | `ERROR` | `,` |
| 124 | 152 | `ERROR` | `,` |
| 150 | 153 | `ERROR` | `,` |
| 170 | 156 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/base/command_server/command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 25 | 188 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/base/command_server/command_server.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/base/command_server/command_server_impl.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 80 | `ERROR` | `,` |
| 22 | 80 | `ERROR` | `,` |
| 28 | 147 | `ERROR` | `, , ,` |
| 32 | 101 | `ERROR` | `,` |
| 38 | 147 | `ERROR` | `, , ,` |
| 42 | 101 | `ERROR` | `,` |
| 48 | 147 | `ERROR` | `, , ,` |
| 54 | 147 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/base/dps.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 92 | `ERROR` | `,` |
| 26 | 80 | `ERROR` | `,` |
| 35 | 94 | `ERROR` | `,` |
| 40 | 201 | `ERROR` | `,` |
| 48 | 80 | `ERROR` | `,` |
| 56 | 91 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/base/image_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 45 | 80 | `ERROR` | `,` |
| 50 | 80 | `ERROR` | `,` |
| 81 | 146 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/base/media_progress_notifier.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/deferred_processing_service.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 21 | 80 | `ERROR` | `,` |
| 24 | 207 | `ERROR` | `,` |
| 24 | 215 | `ERROR` | `,` |
| 24 | 223 | `ERROR` | `,` |
| 35 | 218 | `ERROR` | `,` |
| 35 | 229 | `ERROR` | `,` |
| 35 | 240 | `ERROR` | `,` |
| 41 | 163 | `ERROR` | `,` |
| 51 | 218 | `ERROR` | `,` |
| 51 | 229 | `ERROR` | `,` |
| 51 | 240 | `ERROR` | `,` |
| 57 | 161 | `ERROR` | `,` |
| 65 | 157 | `ERROR` | `,` |
| 77 | 157 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/dfx/dps_event_report.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 97 | `ERROR` | `,` |
| 31 | 108 | `ERROR` | `,` |
| 41 | 103 | `ERROR` | `,` |
| 76 | 100 | `ERROR` | `,` |
| 103 | 98 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/dfx/dps_video_report.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 80 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |
| 22 | 98 | `ERROR` | `,` |
| 34 | 113 | `ERROR` | `,` |
| 47 | 101 | `ERROR` | `,` |
| 63 | 100 | `ERROR` | `,` |
| 69 | 108 | `ERROR` | `,` |
| 75 | 112 | `ERROR` | `,` |
| 90 | 101 | `ERROR` | `,` |
| 96 | 109 | `ERROR` | `,` |
| 99 | 180 | `ERROR` | `,` |
| 118 | 112 | `ERROR` | `,` |
| 124 | 103 | `ERROR` | `,` |
| 131 | 111 | `ERROR` | `,` |
| 137 | 115 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/command/event_status_change_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 183 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/events_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 80 | `ERROR` | `,` |
| 20 | 80 | `ERROR` | `,` |
| 69 | 144 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/events_monitor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 21 | 80 | `ERROR` | `,` |
| 30 | 100 | `ERROR` | `,` |
| 53 | 155 | `ERROR` | `,` |
| 59 | 155 | `ERROR` | `,` |
| 85 | 184 | `ERROR` | `,` |
| 91 | 151 | `ERROR` | `,` |
| 97 | 91 | `ERROR` | `,` |
| 109 | 187 | `ERROR` | `,` |
| 109 | 195 | `ERROR` | `,` |
| 109 | 203 | `ERROR` | `,` |
| 114 | 162 | `ERROR` | `,` |
| 116 | 155 | `ERROR` | `,` |
| 119 | 150 | `ERROR` | `,` |
| 121 | 146 | `ERROR` | `,` |
| 127 | 140 | `ERROR` | `,` |
| 131 | 183 | `ERROR` | `,` |
| 135 | 169 | `ERROR` | `,` |
| 139 | 121 | `ERROR` | `,` |
| … | … | … | *(7 more)* |

#### `services/deferred_processing_service/src/event_monitor/events_subscriber.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 80 | `ERROR` | `,` |
| 36 | 80 | `ERROR` | `,` |
| 41 | 80 | `ERROR` | `,` |
| 53 | 80 | `ERROR` | `,` |
| 63 | 80 | `ERROR` | `,` |
| 69 | 80 | `ERROR` | `,` |
| 102 | 94 | `ERROR` | `,` |
| 109 | 96 | `ERROR` | `,` |
| 115 | 146 | `ERROR` | `,` |
| 118 | 150 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/event_monitor/impl/battery_level_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 80 | `ERROR` | `,` |
| 21 | 80 | `ERROR` | `,` |
| 27 | 139 | `ERROR` | `, , ,` |
| 38 | 165 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/battery_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 21 | 134 | `ERROR` | `, , ,` |
| 30 | 169 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/camera_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 80 | `ERROR` | `,` |
| 32 | 80 | `ERROR` | `,` |
| 54 | 270 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/charging_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 27 | 169 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/screen_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 27 | 167 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/thermal_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 80 | `ERROR` | `,` |
| 20 | 80 | `ERROR` | `,` |
| 26 | 138 | `ERROR` | `, , ,` |
| 31 | 161 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/event_monitor/impl/user_strategy.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 24 | 152 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/post_processor/command/photo_process_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 143 | `ERROR` | `,` |
| 16 | 80 | `ERROR` | `,` |
| 23 | 158 | `ERROR` | `, , ,` |
| 25 | 163 | `ERROR` | `, , ,` |
| 34 | 80 | `ERROR` | `,` |
| 50 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/post_processor/command/service_died_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 18 | 159 | `ERROR` | `, , ,` |
| 30 | 153 | `ERROR` | `, , ,` |
| 41 | 153 | `ERROR` | `, , ,` |
| 45 | 151 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/post_processor/command/video_process_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 143 | `ERROR` | `,` |
| 17 | 158 | `ERROR` | `, , ,` |
| 20 | 161 | `ERROR` | `, , ,` |
| 23 | 160 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/post_processor/photo_post_processor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 80 | `ERROR` | `,` |
| 37 | 80 | `ERROR` | `,` |
| 56 | 80 | `ERROR` | `,` |
| 61 | 154 | `ERROR` | `,` |
| 63 | 157 | `ERROR` | `, , ,` |
| 76 | 154 | `ERROR` | `,` |
| 78 | 157 | `ERROR` | `, , ,` |
| 91 | 154 | `ERROR` | `,` |
| 93 | 157 | `ERROR` | `, , ,` |
| 106 | 173 | `ERROR` | `,` |
| 108 | 157 | `ERROR` | `, , ,` |
| 121 | 173 | `ERROR` | `,` |
| 123 | 157 | `ERROR` | `, , ,` |
| 137 | 157 | `ERROR` | `, , ,` |
| 145 | 145 | `ERROR` | `,` |
| 147 | 157 | `ERROR` | `, , ,` |
| 160 | 79 | `ERROR` | `,` |
| 165 | 78 | `ERROR` | `,` |
| 174 | 79 | `ERROR` | `,` |
| 186 | 208 | `ERROR` | `,` |
| … | … | … | *(30 more)* |

#### `services/deferred_processing_service/src/post_processor/photo_process_result.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 80 | `ERROR` | `,` |
| 17 | 79 | `ERROR` | `,` |
| 23 | 169 | `ERROR` | `,` |
| 32 | 194 | `ERROR` | `,` |
| 40 | 165 | `ERROR` | `,` |
| 46 | 101 | `ERROR` | `,` |
| 48 | 194 | `ERROR` | `,` |
| 48 | 202 | `ERROR` | `,` |
| 48 | 210 | `ERROR` | `,` |
| 57 | 150 | `ERROR` | `, , ,` |
| 67 | 147 | `ERROR` | `, , ,` |
| 80 | 150 | `ERROR` | `, , ,` |
| 90 | 151 | `ERROR` | `, , ,` |
| 103 | 150 | `ERROR` | `, , ,` |
| 113 | 151 | `ERROR` | `, , ,` |
| 135 | 99 | `ERROR` | `,` |
| 137 | 136 | `ERROR` | `, , ,` |
| 141 | 167 | `ERROR` | `,` |
| 143 | 111 | `ERROR` | `,` |
| 147 | 150 | `ERROR` | `, , ,` |
| … | … | … | *(57 more)* |

#### `services/deferred_processing_service/src/post_processor/video_post_processor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 80 | `ERROR` | `,` |
| 23 | 145 | `ERROR` | `, , ,` |
| 36 | 80 | `ERROR` | `,` |
| 42 | 157 | `ERROR` | `, , ,` |
| 57 | 80 | `ERROR` | `,` |
| 72 | 154 | `ERROR` | `,` |
| 74 | 157 | `ERROR` | `, , ,` |
| 83 | 154 | `ERROR` | `,` |
| 85 | 157 | `ERROR` | `, , ,` |
| 89 | 165 | `ERROR` | `,` |
| 100 | 157 | `ERROR` | `, , ,` |
| 108 | 146 | `ERROR` | `,` |
| 110 | 157 | `ERROR` | `, , ,` |
| 119 | 79 | `ERROR` | `,` |
| 124 | 78 | `ERROR` | `,` |
| 143 | 146 | `ERROR` | `, , ,` |
| 145 | 220 | `ERROR` | `,` |
| 152 | 80 | `ERROR` | `,` |
| 157 | 80 | `ERROR` | `,` |
| 166 | 185 | `ERROR` | `,` |
| … | … | … | *(35 more)* |

#### `services/deferred_processing_service/src/post_processor/video_process_result.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 80 | `ERROR` | `,` |
| 20 | 169 | `ERROR` | `,` |
| 28 | 169 | `ERROR` | `,` |
| 36 | 194 | `ERROR` | `,` |
| 44 | 165 | `ERROR` | `,` |
| 50 | 101 | `ERROR` | `,` |
| 52 | 204 | `ERROR` | `,` |
| 52 | 212 | `ERROR` | `,` |
| 52 | 220 | `ERROR` | `,` |
| 76 | 301 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/base/state_machine.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 117 | `ERROR` | `, , ,` |
| 10 | 190 | `ERROR` | `,` |
| 10 | 218 | `ERROR` | `,` |
| 10 | 246 | `ERROR` | `,` |
| 15 | 152 | `ERROR` | `,` |
| 17 | 186 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/command/notify_job_changed_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 143 | `ERROR` | `,` |
| 18 | 158 | `ERROR` | `, , ,` |
| 20 | 160 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 80 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |
| 23 | 151 | `ERROR` | `, , ,` |
| 35 | 143 | `ERROR` | `,` |
| 40 | 142 | `ERROR` | `,` |
| 45 | 143 | `ERROR` | `,` |
| 46 | 163 | `ERROR` | `, , ,` |
| 49 | 165 | `ERROR` | `, , ,` |
| 58 | 79 | `ERROR` | `,` |
| 69 | 233 | `ERROR` | `,` |
| 91 | 161 | `ERROR` | `,` |
| 99 | 256 | `ERROR` | `,` |
| 106 | 163 | `ERROR` | `, , ,` |
| 112 | 163 | `ERROR` | `, , ,` |
| 135 | 205 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_processor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 80 | `ERROR` | `,` |
| 21 | 142 | `ERROR` | `,` |
| 26 | 151 | `ERROR` | `, , ,` |
| 27 | 157 | `ERROR` | `, , ,` |
| 29 | 151 | `ERROR` | `, , ,` |
| 90 | 197 | `ERROR` | `,` |
| 104 | 134 | `ERROR` | `, , ,` |
| 105 | 241 | `ERROR` | `,` |
| 113 | 134 | `ERROR` | `, , ,` |
| 114 | 217 | `ERROR` | `,` |
| 124 | 151 | `ERROR` | `,` |
| 151 | 80 | `ERROR` | `,` |
| 169 | 164 | `ERROR` | `,` |
| 192 | 165 | `ERROR` | `,` |
| 220 | 229 | `ERROR` | `,` |
| 228 | 184 | `ERROR` | `,` |
| 231 | 135 | `ERROR` | `, , ,` |
| 236 | 178 | `ERROR` | `,` |
| 239 | 135 | `ERROR` | `, , ,` |
| 244 | 176 | `ERROR` | `,` |
| … | … | … | *(8 more)* |

#### `services/deferred_processing_service/src/schedule/photo_processor/deferred_photo_result.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 80 | `ERROR` | `,` |
| 20 | 79 | `ERROR` | `,` |
| 36 | 143 | `ERROR` | `,` |
| 43 | 143 | `ERROR` | `,` |
| 54 | 214 | `ERROR` | `,` |
| 73 | 160 | `ERROR` | `,` |
| 80 | 167 | `ERROR` | `,` |
| 123 | 173 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/deferred_photo_job.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 145 | `ERROR` | `, , ,` |
| 17 | 80 | `ERROR` | `,` |
| 23 | 145 | `ERROR` | `, , ,` |
| 30 | 145 | `ERROR` | `, , ,` |
| 37 | 145 | `ERROR` | `, , ,` |
| 45 | 145 | `ERROR` | `, , ,` |
| 52 | 145 | `ERROR` | `, , ,` |
| 61 | 80 | `ERROR` | `,` |
| 119 | 227 | `ERROR` | `,` |
| 131 | 186 | `ERROR` | `,` |
| 142 | 237 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/photo_job_queue.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 125 | 173 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/schedule/photo_processor/photo_job_repository/photo_job_repository.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 190 | `ERROR` | `,` |
| 15 | 154 | `ERROR` | `, , ,` |
| 21 | 168 | `ERROR` | `,` |
| 23 | 154 | `ERROR` | `, , ,` |
| 29 | 85 | `ERROR` | `,` |
| 31 | 154 | `ERROR` | `, , ,` |
| 37 | 84 | `ERROR` | `,` |
| 39 | 154 | `ERROR` | `, , ,` |
| 45 | 143 | `ERROR` | `,` |
| 48 | 236 | `ERROR` | `,` |
| 50 | 236 | `ERROR` | `,` |
| 53 | 131 | `ERROR` | `,` |
| 60 | 142 | `ERROR` | `,` |
| 77 | 199 | `ERROR` | `,` |
| 77 | 220 | `ERROR` | `,` |
| 77 | 241 | `ERROR` | `,` |
| 83 | 226 | `ERROR` | `,` |
| 98 | 212 | `ERROR` | `,` |
| 98 | 233 | `ERROR` | `,` |
| 98 | 254 | `ERROR` | `,` |
| … | … | … | *(28 more)* |

#### `services/deferred_processing_service/src/schedule/photo_processor/strategy/photo_strategy_center.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 79 | `ERROR` | `,` |
| 16 | 140 | `ERROR` | `,` |
| 18 | 153 | `ERROR` | `, , ,` |
| 26 | 80 | `ERROR` | `,` |
| 31 | 79 | `ERROR` | `,` |
| 37 | 80 | `ERROR` | `,` |
| 38 | 151 | `ERROR` | `, , ,` |
| 76 | 158 | `ERROR` | `,` |
| 82 | 173 | `ERROR` | `,` |
| 87 | 149 | `ERROR` | `,` |
| 119 | 143 | `ERROR` | `,` |
| 139 | 140 | `ERROR` | `,` |
| 145 | 145 | `ERROR` | `,` |
| 151 | 150 | `ERROR` | `,` |
| 157 | 148 | `ERROR` | `,` |
| 164 | 86 | `ERROR` | `,` |
| 170 | 87 | `ERROR` | `,` |
| 177 | 151 | `ERROR` | `, , ,` |
| 182 | 158 | `ERROR` | `, , ,` |
| 209 | 148 | `ERROR` | `, , ,` |
| … | … | … | *(5 more)* |

#### `services/deferred_processing_service/src/schedule/scheduler_coordinator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 79 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 20 | 79 | `ERROR` | `,` |
| 25 | 79 | `ERROR` | `,` |
| 30 | 79 | `ERROR` | `,` |
| 35 | 79 | `ERROR` | `,` |
| 40 | 79 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/scheduler_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 22 | 80 | `ERROR` | `,` |
| 28 | 80 | `ERROR` | `,` |
| 37 | 80 | `ERROR` | `,` |
| 46 | 80 | `ERROR` | `,` |
| 55 | 79 | `ERROR` | `,` |
| 74 | 80 | `ERROR` | `,` |
| 83 | 80 | `ERROR` | `,` |
| 92 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/interrupt_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 18 | 92 | `ERROR` | `,` |
| 27 | 86 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_cache_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 147 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_camera_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 148 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_hal_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 145 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_media_library_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 154 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_temperature_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 153 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/photo_trailing_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 80 | `ERROR` | `,` |
| 20 | 150 | `ERROR` | `,` |
| 40 | 204 | `ERROR` | `,` |
| 51 | 160 | `ERROR` | `,` |
| 62 | 97 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_battery_level_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 154 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_battery_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 149 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_camera_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 148 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_charging_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 150 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_hal_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 145 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_media_library_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 154 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_photo_process_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 154 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_process_time_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 80 | `ERROR` | `,` |
| 33 | 243 | `ERROR` | `,` |
| 62 | 86 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_screen_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 148 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/state/video_temperature_state.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 153 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/video_processor/command/notify_video_job_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 143 | `ERROR` | `,` |
| 18 | 158 | `ERROR` | `, , ,` |
| 20 | 160 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/schedule/video_processor/deferred_video_controller.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 80 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |
| 23 | 151 | `ERROR` | `, , ,` |
| 35 | 143 | `ERROR` | `,` |
| 40 | 79 | `ERROR` | `,` |
| 46 | 163 | `ERROR` | `, , ,` |
| 48 | 165 | `ERROR` | `, , ,` |
| 56 | 80 | `ERROR` | `,` |
| 63 | 138 | `ERROR` | `, , ,` |
| 64 | 168 | `ERROR` | `,` |
| 72 | 138 | `ERROR` | `, , ,` |
| 73 | 166 | `ERROR` | `,` |
| 88 | 223 | `ERROR` | `,` |
| 100 | 182 | `ERROR` | `, , ,` |
| 103 | 160 | `ERROR` | `,` |
| 115 | 163 | `ERROR` | `, , ,` |
| 121 | 80 | `ERROR` | `,` |
| 122 | 163 | `ERROR` | `, , ,` |
| 128 | 79 | `ERROR` | `,` |
| 129 | 163 | `ERROR` | `, , ,` |
| … | … | … | *(5 more)* |

#### `services/deferred_processing_service/src/schedule/video_processor/deferred_video_processor.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 80 | `ERROR` | `,` |
| 31 | 80 | `ERROR` | `,` |
| 36 | 79 | `ERROR` | `,` |
| 42 | 155 | `ERROR` | `, , ,` |
| 43 | 158 | `ERROR` | `, , ,` |
| 45 | 152 | `ERROR` | `, , ,` |
| 54 | 155 | `ERROR` | `, , ,` |
| 60 | 155 | `ERROR` | `, , ,` |
| 62 | 230 | `ERROR` | `,` |
| 64 | 158 | `ERROR` | `, , ,` |
| 72 | 155 | `ERROR` | `,` |
| 73 | 155 | `ERROR` | `, , ,` |
| 79 | 155 | `ERROR` | `,` |
| 80 | 155 | `ERROR` | `, , ,` |
| 89 | 155 | `ERROR` | `,` |
| 90 | 155 | `ERROR` | `, , ,` |
| 93 | 158 | `ERROR` | `, , ,` |
| 100 | 80 | `ERROR` | `,` |
| 102 | 155 | `ERROR` | `, , ,` |
| 105 | 158 | `ERROR` | `, , ,` |
| … | … | … | *(45 more)* |

#### `services/deferred_processing_service/src/schedule/video_processor/deferred_video_result.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 80 | `ERROR` | `,` |
| 18 | 79 | `ERROR` | `,` |
| 34 | 160 | `ERROR` | `,` |
| 41 | 167 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/video_processor/strategy/video_strategy_center.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 79 | `ERROR` | `,` |
| 25 | 79 | `ERROR` | `,` |
| 30 | 140 | `ERROR` | `,` |
| 32 | 153 | `ERROR` | `, , ,` |
| 44 | 80 | `ERROR` | `,` |
| 49 | 79 | `ERROR` | `,` |
| 55 | 155 | `ERROR` | `, , ,` |
| 104 | 173 | `ERROR` | `,` |
| 109 | 149 | `ERROR` | `,` |
| 146 | 143 | `ERROR` | `,` |
| 152 | 140 | `ERROR` | `,` |
| 158 | 150 | `ERROR` | `,` |
| 164 | 143 | `ERROR` | `,` |
| 171 | 145 | `ERROR` | `,` |
| 179 | 144 | `ERROR` | `,` |
| 185 | 149 | `ERROR` | `,` |
| 191 | 148 | `ERROR` | `,` |
| 198 | 149 | `ERROR` | `,` |
| 205 | 86 | `ERROR` | `,` |
| 212 | 151 | `ERROR` | `, , ,` |
| … | … | … | *(9 more)* |

#### `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/deferred_video_job.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 170 | `ERROR` | `,` |
| 16 | 88 | `ERROR` | `,` |
| 19 | 115 | `ERROR` | `,` |
| 25 | 84 | `ERROR` | `,` |
| 35 | 164 | `ERROR` | `,` |
| 40 | 163 | `ERROR` | `,` |
| 46 | 88 | `ERROR` | `,` |
| 48 | 177 | `ERROR` | `,` |
| 48 | 197 | `ERROR` | `,` |
| 48 | 217 | `ERROR` | `,` |
| 66 | 228 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/video_job_queue.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 112 | 173 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/schedule/video_processor/video_job_repository/video_job_repository.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 143 | `ERROR` | `,` |
| 14 | 142 | `ERROR` | `,` |
| 28 | 173 | `ERROR` | `,` |
| 28 | 194 | `ERROR` | `,` |
| 28 | 215 | `ERROR` | `,` |
| 30 | 188 | `ERROR` | `,` |
| 30 | 209 | `ERROR` | `,` |
| 30 | 230 | `ERROR` | `,` |
| 34 | 228 | `ERROR` | `,` |
| 46 | 206 | `ERROR` | `,` |
| 51 | 220 | `ERROR` | `,` |
| 74 | 189 | `ERROR` | `,` |
| 74 | 210 | `ERROR` | `,` |
| 74 | 231 | `ERROR` | `,` |
| 75 | 170 | `ERROR` | `,` |
| 88 | 165 | `ERROR` | `,` |
| 91 | 162 | `ERROR` | `,` |
| 104 | 216 | `ERROR` | `,` |
| 104 | 237 | `ERROR` | `,` |
| 104 | 258 | `ERROR` | `,` |
| … | … | … | *(33 more)* |

#### `services/deferred_processing_service/src/session/command/photo_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 184 | `ERROR` | `,` |
| 18 | 80 | `ERROR` | `,` |
| 26 | 159 | `ERROR` | `, , ,` |
| 28 | 150 | `ERROR` | `, , ,` |
| 37 | 202 | `ERROR` | `,` |
| 52 | 203 | `ERROR` | `,` |
| 76 | 207 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/session/command/session_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 80 | `ERROR` | `,` |
| 16 | 80 | `ERROR` | `,` |
| 24 | 155 | `ERROR` | `, , ,` |
| 26 | 159 | `ERROR` | `, , ,` |
| 33 | 80 | `ERROR` | `,` |
| 54 | 80 | `ERROR` | `,` |
| 71 | 80 | `ERROR` | `,` |
| 90 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/session/command/sync_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 143 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |
| 25 | 159 | `ERROR` | `, , ,` |
| 27 | 155 | `ERROR` | `, , ,` |
| 37 | 192 | `ERROR` | `,` |
| 42 | 80 | `ERROR` | `,` |
| 52 | 149 | `ERROR` | `, , ,` |
| 90 | 192 | `ERROR` | `,` |
| 95 | 80 | `ERROR` | `,` |
| 105 | 149 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/session/command/video_command.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 184 | `ERROR` | `,` |
| 18 | 80 | `ERROR` | `,` |
| 28 | 159 | `ERROR` | `, , ,` |
| 30 | 150 | `ERROR` | `, , ,` |
| 39 | 162 | `ERROR` | `,` |
| 54 | 203 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/session/photo_session/deferred_photo_processing_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 143 | `ERROR` | `,` |
| 16 | 80 | `ERROR` | `,` |
| 22 | 99 | `ERROR` | `,` |
| 32 | 199 | `ERROR` | `,` |
| 38 | 194 | `ERROR` | `,` |
| 38 | 202 | `ERROR` | `,` |
| 38 | 210 | `ERROR` | `,` |
| 51 | 88 | `ERROR` | `,` |
| 67 | 90 | `ERROR` | `,` |
| 81 | 91 | `ERROR` | `,` |
| 95 | 91 | `ERROR` | `,` |
| 109 | 97 | `ERROR` | `,` |
| 126 | 203 | `ERROR` | `,` |
| 126 | 211 | `ERROR` | `,` |
| 126 | 219 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/session/photo_session/photo_session_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 80 | `ERROR` | `,` |
| 19 | 80 | `ERROR` | `,` |
| 29 | 80 | `ERROR` | `,` |
| 39 | 90 | `ERROR` | `,` |
| 41 | 119 | `ERROR` | `, , ,` |
| 51 | 144 | `ERROR` | `,` |
| 57 | 143 | `ERROR` | `,` |
| 70 | 143 | `ERROR` | `, , ,` |
| 92 | 137 | `ERROR` | `, , ,` |
| 97 | 92 | `ERROR` | `,` |
| 104 | 143 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/session/session_coordinator.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 149 | `ERROR` | `, , ,` |
| 27 | 149 | `ERROR` | `, , ,` |
| 34 | 149 | `ERROR` | `, , ,` |
| 44 | 80 | `ERROR` | `,` |
| 49 | 80 | `ERROR` | `,` |
| 77 | 180 | `ERROR` | `,` |
| 83 | 150 | `ERROR` | `,` |
| 93 | 152 | `ERROR` | `,` |
| 100 | 198 | `ERROR` | `,` |
| 105 | 162 | `ERROR` | `,` |
| 111 | 265 | `ERROR` | `,` |
| 117 | 196 | `ERROR` | `,` |
| 123 | 80 | `ERROR` | `,` |
| 128 | 80 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/session/session_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 80 | `ERROR` | `,` |
| 15 | 79 | `ERROR` | `,` |
| 28 | 80 | `ERROR` | `,` |
| 33 | 80 | `ERROR` | `,` |
| 39 | 154 | `ERROR` | `, , ,` |
| 41 | 157 | `ERROR` | `,` |
| 45 | 97 | `ERROR` | `,` |
| 48 | 101 | `ERROR` | `,` |
| 52 | 192 | `ERROR` | `,` |
| 52 | 200 | `ERROR` | `,` |
| 52 | 208 | `ERROR` | `,` |
| 63 | 174 | `ERROR` | `,` |
| 71 | 150 | `ERROR` | `,` |
| 79 | 153 | `ERROR` | `,` |
| 94 | 154 | `ERROR` | `, , ,` |
| 96 | 157 | `ERROR` | `,` |
| 100 | 97 | `ERROR` | `,` |
| 104 | 101 | `ERROR` | `,` |
| 108 | 192 | `ERROR` | `,` |
| 108 | 200 | `ERROR` | `,` |
| … | … | … | *(3 more)* |

#### `services/deferred_processing_service/src/session/video_session/deferred_video_processing_session.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 143 | `ERROR` | `,` |
| 17 | 80 | `ERROR` | `,` |
| 24 | 99 | `ERROR` | `,` |
| 33 | 199 | `ERROR` | `,` |
| 37 | 194 | `ERROR` | `,` |
| 37 | 202 | `ERROR` | `,` |
| 37 | 210 | `ERROR` | `,` |
| 48 | 87 | `ERROR` | `,` |
| 54 | 186 | `ERROR` | `,` |
| 65 | 94 | `ERROR` | `,` |
| 71 | 186 | `ERROR` | `,` |
| 82 | 87 | `ERROR` | `,` |
| 88 | 186 | `ERROR` | `,` |
| 99 | 94 | `ERROR` | `,` |
| 105 | 186 | `ERROR` | `,` |
| 112 | 111 | `ERROR` | `, , ,` |
| 115 | 189 | `ERROR` | `,` |
| 122 | 112 | `ERROR` | `, , ,` |
| 125 | 190 | `ERROR` | `,` |
| 131 | 112 | `ERROR` | `, , ,` |
| … | … | … | *(3 more)* |

#### `services/deferred_processing_service/src/session/video_session/video_session_info.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 80 | `ERROR` | `,` |
| 19 | 80 | `ERROR` | `,` |
| 29 | 80 | `ERROR` | `,` |
| 39 | 90 | `ERROR` | `,` |
| 41 | 119 | `ERROR` | `, , ,` |
| 52 | 144 | `ERROR` | `,` |
| 58 | 143 | `ERROR` | `,` |
| 71 | 143 | `ERROR` | `, , ,` |
| 93 | 137 | `ERROR` | `, , ,` |
| 98 | 92 | `ERROR` | `,` |
| 105 | 143 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/utils/dp_power_manager.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 80 | `ERROR` | `,` |
| 20 | 80 | `ERROR` | `,` |
| 27 | 80 | `ERROR` | `,` |
| 38 | 80 | `ERROR` | `,` |
| 40 | 89 | `ERROR` | `,` |
| 49 | 100 | `ERROR` | `,` |
| 56 | 108 | `ERROR` | `,` |

#### `services/deferred_processing_service/src/utils/dp_timer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 139 | `ERROR` | `, , ,` |
| 25 | 160 | `ERROR` | `, , ,` |
| 27 | 168 | `ERROR` | `,` |
| 33 | 157 | `ERROR` | `,` |
| 34 | 172 | `ERROR` | `, , ,` |
| 35 | 139 | `ERROR` | `, , ,` |

#### `services/deferred_processing_service/src/utils/dp_utils.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 98 | `ERROR` | `,` |
| 20 | 87 | `ERROR` | `,` |
| 26 | 103 | `ERROR` | `,` |
| 32 | 96 | `ERROR` | `,` |
| 41 | 112 | `ERROR` | `,` |
| 44 | 198 | `ERROR` | `,` |
| 56 | 194 | `ERROR` | `,` |
| 112 | 197 | `ERROR` | `,` |
| 113 | 160 | `ERROR` | `, , ,` |

#### `test/fuzztest/audiocapturersession_fuzzer/audio_capturer_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 158 | `ERROR` | `,` |
| 47 | 161 | `ERROR` | `, , ,` |
| 64 | 105 | `ERROR` | `,` |
| 88 | 156 | `ERROR` | `,` |

#### `test/fuzztest/audiodeferredprocess_fuzzer/audio_deferred_process_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 31 | `missing ;` | `` |
| 10 | 32 | `ERROR` | `__attribute__((used))=` |
| 12 | 25 | `missing ;` | `` |
| 12 | 26 | `ERROR` | `__attribute__((used))=` |
| 13 | 20 | `missing ;` | `` |
| 13 | 21 | `ERROR` | `__attribute__((used))` |
| 36 | 158 | `ERROR` | `,` |
| 48 | 161 | `ERROR` | `, , ,` |
| 65 | 105 | `ERROR` | `,` |
| 89 | 156 | `ERROR` | `,` |

#### `test/fuzztest/audioencoder_fuzzer/audio_encoder_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 158 | `ERROR` | `,` |
| 47 | 129 | `ERROR` | `, , ,` |
| 64 | 91 | `ERROR` | `,` |
| 88 | 156 | `ERROR` | `,` |

#### `test/fuzztest/audiovideomuxer_fuzzer/audio_video_muxer_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 129 | `ERROR` | `, , ,` |
| 38 | 129 | `ERROR` | `, , ,` |
| 50 | 94 | `ERROR` | `,` |

#### `test/fuzztest/avcodecproxy_fuzzer/av_codec_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 147 | `ERROR` | `, , ,` |
| 44 | 147 | `ERROR` | `, , ,` |
| 70 | 147 | `ERROR` | `, , ,` |
| 88 | 147 | `ERROR` | `, , ,` |
| 106 | 97 | `ERROR` | `,` |

#### `test/fuzztest/avcodectaskmanager_fuzzer/avcodec_task_manager_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 129 | `ERROR` | `, , ,` |
| 160 | 147 | `ERROR` | `, , ,` |

#### `test/fuzztest/cameraability_fuzzer/camera_ability_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 162 | `ERROR` | `, , ,` |
| 17 | 129 | `ERROR` | `, , ,` |
| 76 | 92 | `ERROR` | `,` |

#### `test/fuzztest/cameraabilitybuilder_fuzzer/camera_ability_builder_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 49 | 129 | `ERROR` | `, , ,` |
| 59 | 99 | `ERROR` | `,` |
| 83 | 156 | `ERROR` | `,` |

#### `test/fuzztest/camerademuxer_fuzzer/camera_demuxer_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 129 | `ERROR` | `, , ,` |
| 32 | 136 | `ERROR` | `,` |
| 56 | 92 | `ERROR` | `,` |

#### `test/fuzztest/cameradevice_fuzzer/camera_device_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 123 | `ERROR` | `,` |
| 28 | 176 | `ERROR` | `,` |
| 32 | 121 | `ERROR` | `,` |
| 36 | 105 | `ERROR` | `,` |
| 188 | 177 | `ERROR` | `, , ,` |
| 207 | 169 | `ERROR` | `, , ,` |
| 299 | 186 | `ERROR` | `, , ,` |

#### `test/fuzztest/cameradeviceserviceproxy_fuzzer/camera_device_service_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 124 | `ERROR` | `, , ,` |
| 77 | 124 | `ERROR` | `, , ,` |
| 102 | 124 | `ERROR` | `, , ,` |
| 129 | 124 | `ERROR` | `, , ,` |
| 166 | 103 | `ERROR` | `,` |

#### `test/fuzztest/cameradeviceservicestub_fuzzer/camera_device_service_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 127 | `ERROR` | `, , ,` |
| 52 | 129 | `ERROR` | `, , ,` |
| 54 | 127 | `ERROR` | `, , ,` |
| 68 | 133 | `ERROR` | `, , ,` |
| 134 | 133 | `ERROR` | `, , ,` |
| 209 | 102 | `ERROR` | `,` |
| 214 | 84 | `ERROR` | `,` |

#### `test/fuzztest/camerainput_fuzzer/camera_input_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 162 | `ERROR` | `, , ,` |
| 23 | 164 | `ERROR` | `, , ,` |
| 25 | 172 | `ERROR` | `, , ,` |
| 27 | 151 | `ERROR` | `, , ,` |
| 29 | 153 | `ERROR` | `, , ,` |
| 57 | 95 | `ERROR` | `,` |

#### `test/fuzztest/cameralistenerstub_fuzzer/camera_listener_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 170 | `ERROR` | `, , ,` |
| 24 | 129 | `ERROR` | `, , ,` |

#### `test/fuzztest/cameramanager_fuzzer/camera_manager_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 130 | `ERROR` | `, , ,` |
| 70 | 130 | `ERROR` | `, , ,` |
| 104 | 130 | `ERROR` | `, , ,` |
| 195 | 130 | `ERROR` | `, , ,` |
| 318 | 92 | `ERROR` | `,` |

#### `test/fuzztest/cameraoutputcapability_fuzzer/camera_outputcapability_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 100 | 101 | `ERROR` | `,` |

#### `test/fuzztest/camerareportdfxutils_fuzzer/camera_report_dfx_utils_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 129 | `ERROR` | `, , ,` |
| 29 | 99 | `ERROR` | `,` |

#### `test/fuzztest/cameraserverphotoproxy_fuzzer/camera_server_photo_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 129 | `ERROR` | `, , ,` |
| 37 | 101 | `ERROR` | `,` |

#### `test/fuzztest/cameraservicejson_fuzzer/camera_service_json_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 324 | 162 | `ERROR` | `, , ,` |
| 327 | 156 | `ERROR` | `,` |
| 332 | 183 | `ERROR` | `,` |

#### `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 127 | `ERROR` | `, , ,` |
| 19 | 129 | `ERROR` | `, , ,` |
| 21 | 127 | `ERROR` | `, , ,` |
| 33 | 127 | `ERROR` | `, , ,` |
| 35 | 129 | `ERROR` | `, , ,` |
| 37 | 127 | `ERROR` | `, , ,` |
| 39 | 147 | `ERROR` | `, , ,` |
| 41 | 143 | `ERROR` | `, , ,` |
| 43 | 141 | `ERROR` | `, , ,` |
| 45 | 141 | `ERROR` | `, , ,` |
| 69 | 127 | `ERROR` | `, , ,` |
| 71 | 129 | `ERROR` | `, , ,` |
| 73 | 127 | `ERROR` | `, , ,` |
| 87 | 127 | `ERROR` | `, , ,` |
| 89 | 129 | `ERROR` | `, , ,` |
| 91 | 127 | `ERROR` | `, , ,` |
| 96 | 141 | `ERROR` | `, , ,` |
| 98 | 133 | `ERROR` | `, , ,` |
| 113 | 127 | `ERROR` | `, , ,` |
| 115 | 129 | `ERROR` | `, , ,` |
| … | … | … | *(67 more)* |

#### `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

#### `test/fuzztest/cameratypes_fuzzer/camera_types_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 296 | 90 | `ERROR` | `,` |

#### `test/fuzztest/camerawindowmanagerclient_fuzzer/camera_window_manager_client_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 129 | `ERROR` | `, , ,` |
| 45 | 129 | `ERROR` | `, , ,` |
| 57 | 104 | `ERROR` | `,` |

#### `test/fuzztest/captureoutput_fuzzer/capture_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 160 | `ERROR` | `, , ,` |
| 17 | 135 | `ERROR` | `, , ,` |
| 22 | 129 | `ERROR` | `, , ,` |
| 39 | 160 | `ERROR` | `, , ,` |
| 41 | 135 | `ERROR` | `, , ,` |
| 46 | 129 | `ERROR` | `, , ,` |
| 78 | 92 | `ERROR` | `,` |

#### `test/fuzztest/capturesession_fuzzer/capture_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 98 | `ERROR` | `,` |
| 27 | 175 | `ERROR` | `, , ,` |
| 29 | 154 | `ERROR` | `, , ,` |
| 46 | 175 | `ERROR` | `, , ,` |
| 48 | 154 | `ERROR` | `, , ,` |
| 57 | 162 | `ERROR` | `, , ,` |
| 62 | 155 | `ERROR` | `, , ,` |
| 64 | 134 | `ERROR` | `, , ,` |
| 67 | 156 | `ERROR` | `, , ,` |
| 69 | 152 | `ERROR` | `, , ,` |
| 140 | 98 | `ERROR` | `,` |
| 142 | 167 | `ERROR` | `, , ,` |
| 143 | 154 | `ERROR` | `, , ,` |
| 145 | 172 | `ERROR` | `, , ,` |
| 147 | 165 | `ERROR` | `, , ,` |
| 150 | 165 | `ERROR` | `, , ,` |
| 152 | 160 | `ERROR` | `, , ,` |
| 158 | 98 | `ERROR` | `,` |
| 192 | 98 | `ERROR` | `,` |
| 219 | 98 | `ERROR` | `,` |
| … | … | … | *(94 more)* |

#### `test/fuzztest/capturesessionadd_fuzzer/capture_sessionadd_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 98 | `ERROR` | `,` |
| 27 | 175 | `ERROR` | `, , ,` |
| 29 | 154 | `ERROR` | `, , ,` |
| 46 | 162 | `ERROR` | `, , ,` |
| 51 | 155 | `ERROR` | `, , ,` |
| 53 | 134 | `ERROR` | `, , ,` |
| 56 | 156 | `ERROR` | `, , ,` |
| 58 | 152 | `ERROR` | `, , ,` |
| 102 | 98 | `ERROR` | `,` |
| 104 | 167 | `ERROR` | `, , ,` |
| 105 | 154 | `ERROR` | `, , ,` |
| 107 | 172 | `ERROR` | `, , ,` |
| 109 | 165 | `ERROR` | `, , ,` |
| 112 | 165 | `ERROR` | `, , ,` |
| 114 | 160 | `ERROR` | `, , ,` |
| 121 | 98 | `ERROR` | `,` |
| 139 | 98 | `ERROR` | `,` |
| 166 | 98 | `ERROR` | `,` |
| 196 | 98 | `ERROR` | `,` |
| 240 | 98 | `ERROR` | `,` |
| … | … | … | *(16 more)* |

#### `test/fuzztest/capturesessioncallback_fuzzer/capture_session_callback_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 99 | `ERROR` | `,` |
| 27 | 104 | `ERROR` | `,` |
| 30 | 145 | `ERROR` | `, , ,` |
| 33 | 145 | `ERROR` | `, , ,` |
| 36 | 134 | `ERROR` | `, , ,` |
| 53 | 115 | `ERROR` | `,` |
| 56 | 145 | `ERROR` | `, , ,` |
| 59 | 145 | `ERROR` | `, , ,` |
| 62 | 134 | `ERROR` | `, , ,` |
| 81 | 109 | `ERROR` | `,` |
| 84 | 145 | `ERROR` | `, , ,` |
| 87 | 145 | `ERROR` | `, , ,` |
| 90 | 134 | `ERROR` | `, , ,` |
| 118 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/capturesessionproxy_fuzzer/capture_session_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 108 | 124 | `ERROR` | `, , ,` |
| 110 | 126 | `ERROR` | `, , ,` |
| 246 | 98 | `ERROR` | `,` |
| 250 | 125 | `ERROR` | `, , ,` |
| 252 | 129 | `ERROR` | `, , ,` |
| 254 | 153 | `ERROR` | `, , ,` |

#### `test/fuzztest/cloudenhancesession_fuzzer/cloud_enhance_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 103 | `ERROR` | `,` |
| 27 | 180 | `ERROR` | `, , ,` |
| 31 | 159 | `ERROR` | `, , ,` |
| 37 | 103 | `ERROR` | `,` |
| 39 | 172 | `ERROR` | `, , ,` |
| 42 | 159 | `ERROR` | `, , ,` |
| 44 | 177 | `ERROR` | `, , ,` |
| 46 | 170 | `ERROR` | `, , ,` |
| 49 | 170 | `ERROR` | `, , ,` |
| 51 | 165 | `ERROR` | `, , ,` |
| 57 | 103 | `ERROR` | `,` |
| 60 | 187 | `ERROR` | `, , ,` |
| 95 | 162 | `ERROR` | `, , ,` |
| 102 | 166 | `ERROR` | `, , ,` |

#### `test/fuzztest/commandserver_fuzzer/command_server_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 49 | 129 | `ERROR` | `, , ,` |
| 59 | 92 | `ERROR` | `,` |
| 83 | 156 | `ERROR` | `,` |

#### `test/fuzztest/commandserverimpl_fuzzer/command_server_impl_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 52 | 129 | `ERROR` | `, , ,` |
| 62 | 96 | `ERROR` | `,` |
| 86 | 156 | `ERROR` | `,` |

#### `test/fuzztest/compositionfeature_fuzzer/compositionfeature_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 136 | `ERROR` | `, , ,` |
| 76 | 132 | `ERROR` | `, , ,` |
| 105 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/deferredprocessingserviceeventmonitor_fuzzer/deferred_processingservice_event_monitor_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 108 | 92 | `ERROR` | `,` |

#### `test/fuzztest/deferredprocessingstub_fuzzer/deferred_processing_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 99 | `ERROR` | `,` |

#### `test/fuzztest/deferredvideocontroller_fuzzer/deferred_video_controller_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 158 | `ERROR` | `,` |
| 56 | 139 | `ERROR` | `, , ,` |
| 64 | 133 | `ERROR` | `, , ,` |
| 70 | 129 | `ERROR` | `, , ,` |
| 103 | 102 | `ERROR` | `,` |
| 127 | 156 | `ERROR` | `,` |

#### `test/fuzztest/deferredvideoprocessor_fuzzer/deferred_video_processor_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 139 | `ERROR` | `, , ,` |
| 34 | 133 | `ERROR` | `, , ,` |
| 67 | 101 | `ERROR` | `,` |

#### `test/fuzztest/deferredvideoprocsession_fuzzer/deferredvideoprocsession_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 145 | `ERROR` | `, , ,` |
| 46 | 162 | `ERROR` | `, , ,` |
| 64 | 137 | `ERROR` | `, , ,` |
| 79 | 107 | `ERROR` | `,` |

#### `test/fuzztest/dpsvideoreport_fuzzer/dps_video_report_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 129 | `ERROR` | `, , ,` |
| 38 | 93 | `ERROR` | `,` |

#### `test/fuzztest/hcameradevice_fuzzer/hcamera_device_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 255 | 162 | `ERROR` | `, , ,` |
| 259 | 146 | `ERROR` | `, , ,` |
| 267 | 157 | `ERROR` | `, , ,` |

#### `test/fuzztest/hcameradevicemanager_fuzzer/hcamera_device_manager_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 173 | `ERROR` | `, , ,` |
| 25 | 168 | `ERROR` | `, , ,` |
| 36 | 99 | `ERROR` | `,` |
| 67 | 99 | `ERROR` | `,` |
| 84 | 105 | `ERROR` | `,` |

#### `test/fuzztest/hcamerahostmanager_fuzzer/hcamera_host_manager_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 129 | `ERROR` | `, , ,` |
| 35 | 103 | `ERROR` | `,` |

#### `test/fuzztest/hcameramoviefileoutput_fuzzer/hcamera_movie_file_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 97 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/hcamerapreconfig_fuzzer/hcamera_preconfig_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 89 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 421 | 162 | `ERROR` | `, , ,` |
| 433 | 157 | `ERROR` | `, , ,` |

#### `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 122 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

#### `test/fuzztest/hcameraservicecallbackstub_fuzzer/hcamera_service_callback_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 121 | 105 | `ERROR` | `,` |
| 156 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hcameraswitchsession_fuzzer/hcamera_switch_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 162 | `ERROR` | `, , ,` |
| 39 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/hcapturesession_fuzzer/hcapture_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 127 | `ERROR` | `, , ,` |
| 28 | 129 | `ERROR` | `, , ,` |
| 38 | 133 | `ERROR` | `, , ,` |
| 69 | 133 | `ERROR` | `, , ,` |
| 98 | 133 | `ERROR` | `, , ,` |
| 137 | 133 | `ERROR` | `, , ,` |
| 158 | 131 | `ERROR` | `, , ,` |
| 183 | 131 | `ERROR` | `, , ,` |
| 551 | 162 | `ERROR` | `, , ,` |
| 631 | 139 | `ERROR` | `, , ,` |

#### `test/fuzztest/hmechsession_fuzzer/hmech_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 162 | `ERROR` | `, , ,` |
| 34 | 154 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreamcapture_fuzzer/hstream_capture_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 235 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreamcapturestub_fuzzer/hstream_capture_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 133 | `ERROR` | `, , ,` |
| 35 | 133 | `ERROR` | `, , ,` |
| 135 | 147 | `ERROR` | `, , ,` |
| 158 | 127 | `ERROR` | `, , ,` |
| 160 | 129 | `ERROR` | `, , ,` |
| 162 | 127 | `ERROR` | `, , ,` |
| 215 | 127 | `ERROR` | `, , ,` |
| 217 | 129 | `ERROR` | `, , ,` |
| 219 | 127 | `ERROR` | `, , ,` |
| 264 | 137 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreamdepthdatacallbackproxy_fuzzer/hstream_depth_data_callback_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 56 | 129 | `ERROR` | `, , ,` |
| 63 | 108 | `ERROR` | `,` |
| 87 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hstreamdepthdatacallbackstub_fuzzer/hstream_depth_data_callback_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 158 | `ERROR` | `,` |
| 49 | 129 | `ERROR` | `, , ,` |
| 63 | 107 | `ERROR` | `,` |
| 89 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hstreamdepthdataproxy_fuzzer/hstream_depth_data_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 158 | `ERROR` | `,` |
| 55 | 129 | `ERROR` | `, , ,` |
| 67 | 100 | `ERROR` | `,` |
| 91 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hstreamdepthdatastub_fuzzer/hstream_depth_data_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 129 | `ERROR` | `, , ,` |
| 35 | 99 | `ERROR` | `,` |

#### `test/fuzztest/hstreammetadata_fuzzer/hstream_metadata_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 162 | `ERROR` | `, , ,` |
| 65 | 159 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreammetadatacallbackproxy_fuzzer/hstream_metadata_callback_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 129 | `ERROR` | `, , ,` |
| 28 | 107 | `ERROR` | `,` |

#### `test/fuzztest/hstreammetadatacallbackstub_fuzzer/hstream_metadata_callback_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 129 | `ERROR` | `, , ,` |
| 31 | 176 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreammetadatastub_fuzzer/hstream_metadata_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 158 | `ERROR` | `,` |
| 50 | 129 | `ERROR` | `, , ,` |
| 64 | 98 | `ERROR` | `,` |
| 101 | 156 | `ERROR` | `,` |

#### `test/fuzztest/hstreamoperator_fuzzer/hstream_operator_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 79 | 106 | `ERROR` | `,` |
| 110 | 134 | `ERROR` | `, , ,` |
| 135 | 134 | `ERROR` | `, , ,` |
| 148 | 134 | `ERROR` | `, , ,` |
| 182 | 134 | `ERROR` | `, , ,` |
| 236 | 134 | `ERROR` | `, , ,` |
| 249 | 134 | `ERROR` | `, , ,` |
| 261 | 134 | `ERROR` | `, , ,` |
| 289 | 134 | `ERROR` | `, , ,` |
| 308 | 142 | `ERROR` | `, , ,` |
| 338 | 134 | `ERROR` | `, , ,` |
| 355 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreamrepeat_fuzzer/hstream_repeat_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 168 | 147 | `ERROR` | `, , ,` |
| 207 | 147 | `ERROR` | `, , ,` |
| 237 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/hstreamrepeatcallbackstub_fuzzer/hstream_repeat_callback_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 83 | 104 | `ERROR` | `,` |
| 88 | 84 | `ERROR` | `,` |

#### `test/fuzztest/hstreamrepeatstub_fuzzer/hstream_repeat_stub_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 127 | `ERROR` | `, , ,` |
| 38 | 129 | `ERROR` | `, , ,` |
| 40 | 127 | `ERROR` | `, , ,` |
| 65 | 133 | `ERROR` | `, , ,` |
| 157 | 133 | `ERROR` | `, , ,` |
| 216 | 96 | `ERROR` | `,` |

#### `test/fuzztest/lightscansession_fuzzer/light_scan_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 139 | `ERROR` | `, , ,` |
| 35 | 137 | `ERROR` | `, , ,` |
| 48 | 95 | `ERROR` | `,` |
| 54 | 86 | `ERROR` | `,` |

#### `test/fuzztest/mediamanager_fuzzer/media_manager_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 129 | `ERROR` | `, , ,` |
| 55 | 141 | `ERROR` | `, , ,` |
| 80 | 129 | `ERROR` | `, , ,` |
| 88 | 131 | `ERROR` | `, , ,` |
| 113 | 129 | `ERROR` | `, , ,` |
| 139 | 129 | `ERROR` | `, , ,` |
| 150 | 91 | `ERROR` | `,` |

#### `test/fuzztest/mediamanagerproxy_fuzzer/media_manager_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 157 | `ERROR` | `, , ,` |
| 44 | 157 | `ERROR` | `, , ,` |
| 56 | 97 | `ERROR` | `,` |

#### `test/fuzztest/metadataoutput_fuzzer/metadata_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 177 | `ERROR` | `, , ,` |
| 41 | 134 | `ERROR` | `, , ,` |
| 116 | 93 | `ERROR` | `,` |
| 124 | 152 | `ERROR` | `, , ,` |

#### `test/fuzztest/metadataoutput_fuzzer/metadata_output_fuzzer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 127 | `ERROR` | `,` |
| 65 | 156 | `ERROR` | `,` |

#### `test/fuzztest/mooncaptureboostfeature_fuzzer/moon_capture_boost_feature_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 50 | 129 | `ERROR` | `, , ,` |
| 60 | 102 | `ERROR` | `,` |
| 84 | 156 | `ERROR` | `,` |

#### `test/fuzztest/movingphotoproxy_fuzzer/moving_photo_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 91 | `ERROR` | `,` |
| 59 | 157 | `ERROR` | `, , ,` |
| 62 | 163 | `ERROR` | `, , ,` |
| 102 | 157 | `ERROR` | `, , ,` |
| 105 | 163 | `ERROR` | `, , ,` |
| 124 | 157 | `ERROR` | `, , ,` |
| 127 | 163 | `ERROR` | `, , ,` |
| 147 | 157 | `ERROR` | `, , ,` |
| 150 | 163 | `ERROR` | `, , ,` |
| 167 | 95 | `ERROR` | `,` |

#### `test/fuzztest/movingphotosurfacewrapper_fuzzer/moving_photo_surface_wrapper_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 129 | `ERROR` | `, , ,` |
| 37 | 104 | `ERROR` | `,` |

#### `test/fuzztest/photojobrepository_fuzzer/photo_job_repository_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 50 | 129 | `ERROR` | `, , ,` |
| 77 | 97 | `ERROR` | `,` |
| 101 | 156 | `ERROR` | `,` |

#### `test/fuzztest/photooutput_fuzzer/photo_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 162 | `ERROR` | `, , ,` |
| 20 | 164 | `ERROR` | `, , ,` |
| 22 | 172 | `ERROR` | `, , ,` |
| 26 | 145 | `ERROR` | `, , ,` |
| 29 | 169 | `ERROR` | `, , ,` |
| 31 | 162 | `ERROR` | `, , ,` |
| 34 | 162 | `ERROR` | `, , ,` |
| 36 | 150 | `ERROR` | `, , ,` |
| 38 | 154 | `ERROR` | `, , ,` |
| 49 | 95 | `ERROR` | `,` |
| 56 | 162 | `ERROR` | `, , ,` |
| 58 | 150 | `ERROR` | `, , ,` |
| 97 | 95 | `ERROR` | `,` |
| 145 | 95 | `ERROR` | `,` |
| 165 | 95 | `ERROR` | `,` |

#### `test/fuzztest/photopostprocessor_fuzzer/photo_post_processor_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 103 | `ERROR` | `,` |

#### `test/fuzztest/photoprocessresult_fuzzer/photo_process_result_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/photosession_fuzzer/photo_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 158 | `ERROR` | `,` |
| 84 | 165 | `ERROR` | `, , ,` |
| 95 | 91 | `ERROR` | `,` |
| 119 | 156 | `ERROR` | `,` |

#### `test/fuzztest/photostrategycenter_fuzzer/photo_strategy_center_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 139 | `ERROR` | `, , ,` |
| 41 | 129 | `ERROR` | `, , ,` |
| 61 | 139 | `ERROR` | `, , ,` |
| 63 | 129 | `ERROR` | `, , ,` |
| 81 | 139 | `ERROR` | `, , ,` |
| 83 | 129 | `ERROR` | `, , ,` |
| 102 | 139 | `ERROR` | `, , ,` |
| 106 | 129 | `ERROR` | `, , ,` |
| 110 | 145 | `ERROR` | `, , ,` |

#### `test/fuzztest/portraitsession_fuzzer/portrait_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 95 | 178 | `ERROR` | `, , ,` |
| 97 | 134 | `ERROR` | `, , ,` |
| 132 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/previewoutput_fuzzer/preview_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 162 | `ERROR` | `, , ,` |
| 15 | 166 | `ERROR` | `, , ,` |
| 17 | 174 | `ERROR` | `, , ,` |
| 19 | 147 | `ERROR` | `, , ,` |
| 22 | 171 | `ERROR` | `, , ,` |
| 24 | 166 | `ERROR` | `, , ,` |
| 27 | 168 | `ERROR` | `, , ,` |
| 29 | 152 | `ERROR` | `, , ,` |
| 31 | 152 | `ERROR` | `, , ,` |
| 33 | 156 | `ERROR` | `, , ,` |
| 41 | 97 | `ERROR` | `,` |
| 43 | 168 | `ERROR` | `, , ,` |
| 45 | 152 | `ERROR` | `, , ,` |
| 77 | 162 | `ERROR` | `, , ,` |
| 79 | 166 | `ERROR` | `, , ,` |
| 81 | 174 | `ERROR` | `, , ,` |
| 83 | 147 | `ERROR` | `, , ,` |
| 86 | 171 | `ERROR` | `, , ,` |
| 88 | 166 | `ERROR` | `, , ,` |
| 91 | 168 | `ERROR` | `, , ,` |
| … | … | … | *(3 more)* |

#### `test/fuzztest/professionsession_fuzzer/profession_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 158 | `ERROR` | `,` |
| 127 | 96 | `ERROR` | `,` |
| 130 | 162 | `ERROR` | `, , ,` |
| 135 | 155 | `ERROR` | `, , ,` |
| 139 | 155 | `ERROR` | `, , ,` |
| 141 | 135 | `ERROR` | `, , ,` |
| 186 | 156 | `ERROR` | `,` |

#### `test/fuzztest/sessioncoordinator_fuzzer/session_coordinator_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 129 | `ERROR` | `, , ,` |
| 59 | 101 | `ERROR` | `,` |

#### `test/fuzztest/sketchwrapper_fuzzer/sketch_wrapper_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 158 | `ERROR` | `,` |
| 67 | 129 | `ERROR` | `, , ,` |
| 108 | 129 | `ERROR` | `, , ,` |
| 138 | 92 | `ERROR` | `,` |
| 163 | 156 | `ERROR` | `,` |

#### `test/fuzztest/slowmotionsession_fuzzer/slow_motion_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 180 | `ERROR` | `, , ,` |
| 53 | 134 | `ERROR` | `, , ,` |
| 96 | 162 | `ERROR` | `, , ,` |

#### `test/fuzztest/streamcapture_fuzzer/stream_capture_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 162 | `ERROR` | `, , ,` |
| 54 | 164 | `ERROR` | `, , ,` |

#### `test/fuzztest/streamcaptureproxy_fuzzer/stream_capture_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 124 | `ERROR` | `, , ,` |
| 27 | 124 | `ERROR` | `, , ,` |
| 38 | 124 | `ERROR` | `, , ,` |
| 41 | 124 | `ERROR` | `, , ,` |
| 53 | 124 | `ERROR` | `, , ,` |
| 56 | 124 | `ERROR` | `, , ,` |
| 67 | 124 | `ERROR` | `, , ,` |
| 70 | 124 | `ERROR` | `, , ,` |
| 82 | 124 | `ERROR` | `, , ,` |
| 85 | 124 | `ERROR` | `, , ,` |
| 96 | 124 | `ERROR` | `, , ,` |
| 99 | 124 | `ERROR` | `, , ,` |
| 111 | 124 | `ERROR` | `, , ,` |
| 114 | 124 | `ERROR` | `, , ,` |
| 125 | 124 | `ERROR` | `, , ,` |
| 128 | 124 | `ERROR` | `, , ,` |
| 139 | 124 | `ERROR` | `, , ,` |
| 142 | 124 | `ERROR` | `, , ,` |
| 163 | 124 | `ERROR` | `, , ,` |
| 166 | 124 | `ERROR` | `, , ,` |
| … | … | … | *(23 more)* |

#### `test/fuzztest/streamdepthdataproxy_fuzzer/stream_depth_data_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 122 | `ERROR` | `, , ,` |
| 14 | 126 | `ERROR` | `, , ,` |
| 16 | 124 | `ERROR` | `, , ,` |
| 25 | 122 | `ERROR` | `, , ,` |
| 27 | 126 | `ERROR` | `, , ,` |
| 29 | 124 | `ERROR` | `, , ,` |
| 43 | 99 | `ERROR` | `,` |

#### `test/fuzztest/streammetadatacallbackproxy_fuzzer/stream_metadata_callback_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 137 | `ERROR` | `, , ,` |
| 33 | 127 | `ERROR` | `, , ,` |
| 47 | 127 | `ERROR` | `, , ,` |
| 62 | 106 | `ERROR` | `,` |

#### `test/fuzztest/streamrepeatproxy_fuzzer/stream_repeat_proxy_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 124 | `ERROR` | `, , ,` |
| 28 | 141 | `ERROR` | `, , ,` |
| 30 | 133 | `ERROR` | `, , ,` |
| 49 | 96 | `ERROR` | `,` |

#### `test/fuzztest/timebroker_fuzzer/time_broker_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 49 | 129 | `ERROR` | `, , ,` |
| 66 | 89 | `ERROR` | `,` |
| 90 | 156 | `ERROR` | `,` |

#### `test/fuzztest/timelapsephotosession_fuzzer/time_lapse_photo_session_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 161 | `ERROR` | `, , ,` |
| 22 | 167 | `ERROR` | `, , ,` |
| 26 | 165 | `ERROR` | `, , ,` |
| 28 | 155 | `ERROR` | `, , ,` |
| 30 | 163 | `ERROR` | `, , ,` |
| 233 | 169 | `ERROR` | `, , ,` |
| 235 | 187 | `ERROR` | `,` |
| 240 | 175 | `ERROR` | `,` |
| 449 | 162 | `ERROR` | `, , ,` |
| 456 | 182 | `ERROR` | `, , ,` |
| 458 | 155 | `ERROR` | `, , ,` |
| 460 | 163 | `ERROR` | `, , ,` |

#### `test/fuzztest/timercore_fuzzer/timer_core_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 49 | 129 | `ERROR` | `, , ,` |
| 64 | 88 | `ERROR` | `,` |
| 88 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videoencoder_fuzzer/video_encoder_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 158 | `ERROR` | `,` |
| 47 | 129 | `ERROR` | `, , ,` |
| 69 | 91 | `ERROR` | `,` |
| 93 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videojobqueue_fuzzer/video_job_queue_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 158 | `ERROR` | `,` |
| 64 | 138 | `ERROR` | `, , ,` |
| 65 | 138 | `ERROR` | `, , ,` |
| 84 | 92 | `ERROR` | `,` |
| 108 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videojobrepository_fuzzer/video_job_repository_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 158 | `ERROR` | `,` |
| 56 | 129 | `ERROR` | `, , ,` |
| 82 | 97 | `ERROR` | `,` |
| 106 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videooutput_fuzzer/video_output_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 49 | 145 | `ERROR` | `, , ,` |
| 53 | 149 | `ERROR` | `, , ,` |
| 55 | 137 | `ERROR` | `, , ,` |
| 97 | 97 | `ERROR` | `,` |
| 121 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videopostprocessor_fuzzer/video_post_processor_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 158 | `ERROR` | `,` |
| 120 | 97 | `ERROR` | `,` |
| 153 | 156 | `ERROR` | `,` |

#### `test/fuzztest/videostrategycenter_fuzzer/video_strategy_center_fuzzer.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 158 | `ERROR` | `,` |
| 53 | 139 | `ERROR` | `, , ,` |
| 62 | 129 | `ERROR` | `, , ,` |
| 77 | 98 | `ERROR` | `,` |
| 101 | 156 | `ERROR` | `,` |

---
