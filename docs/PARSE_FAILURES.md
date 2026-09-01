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
| `drivers_hdf_core` | `/home/sergei/drivers_hdf_core` | 245 | generic ERROR nodes (mixed C++ constructs) |
| `hiviewdfx_hiview` | `/home/sergei/hiviewdfx_hiview` | 174 | gtest/HWTEST macros (`missing ;`) |
| `multimedia_camera_framework` | `/home/sergei/multimedia_camera_framework` | 198 | generic ERROR nodes (mixed C++ constructs) |

## Cross-corpus category totals

| Category | HDF | Hiview | Camera | Total |
|----------|----:|-------:|-------:|------:|
| generic ERROR nodes (mixed C++ constructs) | 184 | 40 | 112 | 336 |
| gtest/HWTEST macros (`missing ;`) | 60 | 121 | 71 | 252 |
| missing type identifiers (often macro-expanded types) | 1 | 7 | 14 | 22 |
| other / mixed | 0 | 4 | 1 | 5 |
| extern template instantiations | 0 | 2 | 0 | 2 |

## drivers_hdf_core

Generated from `trace analyze /home/sergei/drivers_hdf_core` (245 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 245

### Failure categories

| Category | Files |
|----------|------:|
| generic ERROR nodes (mixed C++ constructs) | 184 |
| gtest/HWTEST macros (`missing ;`) | 60 |
| missing type identifiers (often macro-expanded types) | 1 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `adapter/khdf/hongmeng/osal/src/osal_cdev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 2 | `adapter/khdf/hongmeng/osal/src/osal_workqueue.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 3 | `adapter/khdf/linux/manager/src/devmgr_load.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 4 | `adapter/khdf/linux/manager/src/hdf_kevent.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 5 | `adapter/khdf/linux/model/camera/src/contig_dma.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 6 | `adapter/khdf/linux/model/camera/src/sg_dma.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 7 | `adapter/khdf/linux/model/camera/src/virtual_malloc.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 8 | `adapter/khdf/linux/model/storage/emmc_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 9 | `adapter/khdf/linux/model/storage/sdio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 10 | `adapter/khdf/linux/model/usb/host/include/usb_net_adapter.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 11 | `adapter/khdf/linux/model/usb/host/src/usb_net_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 152 |
| 12 | `adapter/khdf/linux/model/usb/host/src/usb_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 13 | `adapter/khdf/linux/osal/src/osal_cdev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 14 | `adapter/khdf/linux/platform/adc/adc_iio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 15 | `adapter/khdf/linux/platform/clock/clock_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 16 | `adapter/khdf/linux/platform/gpio/gpio_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 17 | `adapter/khdf/linux/platform/i2c/i2c_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 18 | `adapter/khdf/linux/platform/mipi_csi/mipi_v4l2_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 19 | `adapter/khdf/linux/platform/mipi_dsi/mipi_drm_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 20 | `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_dev.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 21 | `adapter/khdf/linux/platform/mipi_dsi/mipi_tx_hi35xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 22 | `adapter/khdf/linux/platform/pwm/pwm_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 23 | `adapter/khdf/linux/platform/pwm/pwm_hi35xx_linux.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 24 | `adapter/khdf/linux/platform/regulator/regulator_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 25 | `adapter/khdf/linux/platform/regulator/regulator_adapter_consumer.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 26 | `adapter/khdf/linux/platform/rtc/rtc_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 27 | `adapter/khdf/linux/platform/spi/spi_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 28 | `adapter/khdf/linux/platform/watchdog/watchdog_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 29 | `adapter/khdf/linux/test/platform/i2c/i2c_adapter_dummy.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 30 | `adapter/khdf/linux/test/test_helper_driver/src/test_helper_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 31 | `adapter/khdf/liteos/model/storage/src/mmc/mmc_block_lite.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 32 | `adapter/khdf/liteos/model/storage/src/mtd/mtd_char_lite.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 33 | `adapter/khdf/liteos/model/usb/host/src/usb_pnp_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 34 | `adapter/khdf/liteos/model/usb/host/src/usb_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 35 | `adapter/khdf/liteos/model/usb/host/src/usb_test_pnp_notify.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 36 | `adapter/khdf/liteos/platform/include/gpio_dev.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 37 | `adapter/khdf/liteos_m/test/sample_driver/src/sample_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 38 | `adapter/khdf/uniproton/test/sample_driver/src/platform_device_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 99 |
| 39 | `adapter/khdf/uniproton/test/sample_driver/src/platform_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 40 | `adapter/khdf/uniproton/test/sample_driver/src/platform_manager_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 41 | `adapter/platform/can/can_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 42 | `adapter/platform/gpio/gpio_asr.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 43 | `adapter/platform/gpio/gpio_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 44 | `adapter/platform/gpio/gpio_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 45 | `adapter/platform/gpio/gpio_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 46 | `adapter/platform/i2c/i2c_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 47 | `adapter/platform/i2c/i2c_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 48 | `adapter/platform/i2c/i2c_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 49 | `adapter/platform/mipi_dsi/mipi_drm_imx8mm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 50 | `adapter/platform/pwm/pwm_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 51 | `adapter/platform/pwm/pwm_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 52 | `adapter/platform/pwm/pwm_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 53 | `adapter/platform/spi/spi_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 54 | `adapter/platform/spi/spi_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 55 | `adapter/platform/spi/spi_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 56 | `adapter/platform/uart/uart_asr.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 57 | `adapter/platform/uart/uart_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 58 | `adapter/platform/uart/uart_gr5xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 59 | `adapter/platform/uart/uart_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 60 | `adapter/platform/uart/uart_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 61 | `adapter/platform/watchdog/watchdog_bes.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 62 | `adapter/platform/watchdog/watchdog_stm32f4xx.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 63 | `adapter/platform/watchdog/watchdog_wm.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 64 | `adapter/uhdf2/hdi/test/buffer_handle/buffer_handle_test.cpp` | tree-sitter-cpp node `missing ;` at 19 site(s) | 19 |
| 65 | `adapter/uhdf2/hdi/test/buffer_handle/native_buffer_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 66 | `adapter/uhdf2/hdi/test/object_collector/object_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 67 | `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_c_test.cpp` | tree-sitter-cpp node `missing ;` at 11 site(s) | 11 |
| 68 | `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 21 |
| 69 | `adapter/uhdf2/hdi/test/smq_test/smq_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 70 | `adapter/uhdf2/hdi/test/stub_collector/stub_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 71 | `adapter/uhdf2/host/test/unittest/devhost_test.cpp` | tree-sitter-cpp node `missing ;` at 12 site(s) | 12 |
| 72 | `adapter/uhdf2/host/test/unittest/devmgr_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 73 | `adapter/uhdf2/host/test/unittest/sample1_driver/sample1_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 74 | `adapter/uhdf2/host/test/unittest/sample_driver/sample_driver.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 75 | `adapter/uhdf2/osal/test/unittest/common/osal_slist_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 76 | `adapter/uhdf2/osal/test/unittest/common/sample_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 77 | `adapter/uhdf2/shared/test/dev_attribute_serialize_test.cpp` | tree-sitter-cpp node `missing ;` at 15 site(s) | 15 |
| 78 | `adapter/uhdf2/test/unittest/platform/i2c/ui2c_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 79 | `framework/core/host/test/unittest/hdf_vdi_test.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 80 | `framework/core/manager/test/unittest/common/devmgr_uevent_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 81 | `framework/core/manager/test/unittest/common/hdf_ioservice_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 82 | `framework/core/manager/test/unittest/common/hdf_lite_manager_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 83 | `framework/core/manager/test/unittest/common/hdf_pm_test.cpp` | tree-sitter-cpp node `missing ;` at 17 site(s) | 17 |
| 84 | `framework/core/manager/test/unittest/common/hdf_remote_adapter_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 85 | `framework/core/manager/test/unittest/common/hdf_sbuf_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 86 | `framework/core/shared/test/unittest/common/hdf_core_shared_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 87 | `framework/model/audio/common/test/unittest/common/audio_codec_base_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 88 | `framework/model/audio/common/test/unittest/common/audio_dai_base_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 89 | `framework/model/audio/common/test/unittest/common/audio_dma_base_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 90 | `framework/model/audio/common/test/unittest/common/audio_dsp_base_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 91 | `framework/model/audio/common/test/unittest/common/audio_platform_base_test.cpp` | tree-sitter-cpp node `missing ;` at 21 site(s) | 21 |
| 92 | `framework/model/audio/core/src/audio_host.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 93 | `framework/model/audio/core/test/unittest/common/audio_core_test.cpp` | tree-sitter-cpp node `missing ;` at 26 site(s) | 26 |
| 94 | `framework/model/audio/core/test/unittest/common/audio_host_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 95 | `framework/model/audio/core/test/unittest/common/audio_parse_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 96 | `framework/model/audio/dispatch/src/audio_control_dispatch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 97 | `framework/model/audio/dispatch/src/audio_stream_dispatch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 98 | `framework/model/audio/hdmi/src/audio_hdmi_codec_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 99 | `framework/model/audio/sapm/test/unittest/common/audio_sapm_test.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 100 | `framework/model/audio/usb/src/audio_usb_codec_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 101 | `framework/model/audio/usb/src/audio_usb_dma_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 102 | `framework/model/audio/usb/src/audio_usb_endpoints.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 103 | `framework/model/audio/usb/src/audio_usb_mixer.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 104 | `framework/model/camera/buffer_manager/src/buffer_queue.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 105 | `framework/model/display/driver/adapter_soc/hi35xx_disp.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 106 | `framework/model/display/driver/backlight/hdf_bl.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 107 | `framework/model/display/driver/backlight/pwm_bl.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 108 | `framework/model/display/driver/hdf_disp.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 109 | `framework/model/display/driver/hdf_drm_panel.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 110 | `framework/model/display/driver/lcdkit/lite_lcdkit.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 111 | `framework/model/display/driver/panel/ili9881_st_5p5.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 112 | `framework/model/display/driver/panel/ili9881c_boe.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 113 | `framework/model/display/driver/panel/mipi_icn9700.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 114 | `framework/model/display/driver/panel/ssp_st7789.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 115 | `framework/model/input/driver/hdf_encoder.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 116 | `framework/model/input/driver/hdf_encoder.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 117 | `framework/model/input/driver/hdf_hid_adapter.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 118 | `framework/model/input/driver/hdf_infrared.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 119 | `framework/model/input/driver/hdf_input_device_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 120 | `framework/model/input/driver/hdf_key.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 121 | `framework/model/input/driver/hdf_touch.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 122 | `framework/model/input/driver/touchscreen/touch_ft5406.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 123 | `framework/model/input/driver/touchscreen/touch_ft5x06.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 124 | `framework/model/input/driver/touchscreen/touch_ft6336.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 125 | `framework/model/input/driver/touchscreen/touch_gt911.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 126 | `framework/model/misc/dsoftbus/src/hdf_dsoftbus_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 127 | `framework/model/misc/light/driver/src/light_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 128 | `framework/model/misc/vibrator/driver/src/vibrator_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 129 | `framework/model/network/ethernet/src/hdf_eth_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 130 | `framework/model/network/wifi/core/hdf_wifi_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 131 | `framework/model/sensor/driver/accel/sensor_accel_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 132 | `framework/model/sensor/driver/accel/sensor_gravity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 133 | `framework/model/sensor/driver/als/sensor_als_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 134 | `framework/model/sensor/driver/barometer/sensor_barometer_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 135 | `framework/model/sensor/driver/common/src/sensor_device_manager.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 136 | `framework/model/sensor/driver/gas/sensor_gas_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 137 | `framework/model/sensor/driver/gyro/sensor_gyro_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 138 | `framework/model/sensor/driver/hall/sensor_hall_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 139 | `framework/model/sensor/driver/humidity/sensor_humidity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 140 | `framework/model/sensor/driver/magnetic/sensor_magnetic_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 141 | `framework/model/sensor/driver/pedometer/sensor_pedometer_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 142 | `framework/model/sensor/driver/ppg/sensor_ppg_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 143 | `framework/model/sensor/driver/proximity/sensor_proximity_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 144 | `framework/model/sensor/driver/temperature/sensor_temperature_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 145 | `framework/model/storage/src/mtd/mtd_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 146 | `framework/sample/platform/uart/src/uart_sample.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 147 | `framework/support/platform/include/fwk/platform_errno.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 148 | `framework/support/platform/src/adc/adc_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 149 | `framework/support/platform/src/clock/clock_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 150 | `framework/support/platform/src/dac/dac_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 151 | `framework/support/platform/src/gpio/gpio_service.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 152 | `framework/support/platform/src/i2c/i2c_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 153 | `framework/support/platform/src/i3c/i3c_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 154 | `framework/support/platform/src/pin/pin_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 155 | `framework/support/platform/src/regulator/regulator_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 58 |
| 156 | `framework/support/platform/src/timer/timer_core.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 20 |
| 157 | `framework/support/platform/test/unittest/common/hdf_adc_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 158 | `framework/support/platform/test/unittest/common/hdf_can_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 159 | `framework/support/platform/test/unittest/common/hdf_clock_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 160 | `framework/support/platform/test/unittest/common/hdf_dac_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 161 | `framework/support/platform/test/unittest/common/hdf_emmc_mini_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 162 | `framework/support/platform/test/unittest/common/hdf_emmc_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 163 | `framework/support/platform/test/unittest/common/hdf_gpio_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 164 | `framework/support/platform/test/unittest/common/hdf_hdmi_test.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 165 | `framework/support/platform/test/unittest/common/hdf_i2c_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 166 | `framework/support/platform/test/unittest/common/hdf_i2s_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 167 | `framework/support/platform/test/unittest/common/hdf_i3c_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 168 | `framework/support/platform/test/unittest/common/hdf_mipi_csi_test.cpp` | tree-sitter-cpp node `missing ;` at 12 site(s) | 12 |
| 169 | `framework/support/platform/test/unittest/common/hdf_mipi_dsi_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 170 | `framework/support/platform/test/unittest/common/hdf_pcie_bus_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 171 | `framework/support/platform/test/unittest/common/hdf_pcie_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 172 | `framework/support/platform/test/unittest/common/hdf_pin_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 173 | `framework/support/platform/test/unittest/common/hdf_platform_device_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 174 | `framework/support/platform/test/unittest/common/hdf_platform_dumper_test.cpp` | tree-sitter-cpp node `missing ;` at 18 site(s) | 18 |
| 175 | `framework/support/platform/test/unittest/common/hdf_platform_event_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 176 | `framework/support/platform/test/unittest/common/hdf_platform_manager_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 177 | `framework/support/platform/test/unittest/common/hdf_platform_queue_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 178 | `framework/support/platform/test/unittest/common/hdf_platform_trace_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 179 | `framework/support/platform/test/unittest/common/hdf_pwm_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 180 | `framework/support/platform/test/unittest/common/hdf_regulator_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 181 | `framework/support/platform/test/unittest/common/hdf_rtc_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 182 | `framework/support/platform/test/unittest/common/hdf_sdio_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 183 | `framework/support/platform/test/unittest/common/hdf_spi_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 184 | `framework/support/platform/test/unittest/common/hdf_timer_test.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 185 | `framework/support/platform/test/unittest/common/hdf_uart_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 186 | `framework/support/platform/test/unittest/common/hdf_watchdog_mini_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 187 | `framework/support/platform/test/unittest/common/hdf_watchdog_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 188 | `framework/support/posix/test/unittest/common/hdf_osal_test.cpp` | tree-sitter-cpp node `missing ;` at 93 site(s) | 93 |
| 189 | `framework/support/posix/test/unittest/common/hdf_osal_test_posix.cpp` | tree-sitter-cpp node `missing ;` at 48 site(s) | 48 |
| 190 | `framework/test/unittest/common/hdf_main_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 191 | `framework/test/unittest/manager/sample_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 192 | `framework/test/unittest/model/network/wifi/unittest/message/hdf_single_node_message_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 193 | `framework/test/unittest/platform/common/adc_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 194 | `framework/test/unittest/platform/common/can_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 49 |
| 195 | `framework/test/unittest/platform/common/clock_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 196 | `framework/test/unittest/platform/common/dac_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 197 | `framework/test/unittest/platform/common/emmc_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 198 | `framework/test/unittest/platform/common/gpio_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 199 | `framework/test/unittest/platform/common/hdmi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 200 | `framework/test/unittest/platform/common/i2c_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 201 | `framework/test/unittest/platform/common/i2s_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 202 | `framework/test/unittest/platform/common/i3c_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 203 | `framework/test/unittest/platform/common/mipi_csi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 204 | `framework/test/unittest/platform/common/mipi_dsi_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 205 | `framework/test/unittest/platform/common/pcie_bus_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 206 | `framework/test/unittest/platform/common/pcie_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 207 | `framework/test/unittest/platform/common/pin_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 208 | `framework/test/unittest/platform/common/platform_device_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 98 |
| 209 | `framework/test/unittest/platform/common/platform_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 210 | `framework/test/unittest/platform/common/platform_event_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 54 |
| 211 | `framework/test/unittest/platform/common/platform_manager_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 50 |
| 212 | `framework/test/unittest/platform/common/platform_queue_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 213 | `framework/test/unittest/platform/common/pwm_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 214 | `framework/test/unittest/platform/common/regulator_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 215 | `framework/test/unittest/platform/common/rtc_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 216 | `framework/test/unittest/platform/common/sdio_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 217 | `framework/test/unittest/platform/common/spi_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 218 | `framework/test/unittest/platform/common/timer_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 219 | `framework/test/unittest/platform/common/uart_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 220 | `framework/test/unittest/platform/common/watchdog_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 221 | `framework/test/unittest/platform/config/can_test_config.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 222 | `framework/test/unittest/platform/virtual/adc_linux_virtual_iio_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 223 | `framework/test/unittest/platform/virtual/adc_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 224 | `framework/test/unittest/platform/virtual/clock_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 225 | `framework/test/unittest/platform/virtual/dac_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 226 | `framework/test/unittest/platform/virtual/i3c_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 227 | `framework/test/unittest/platform/virtual/pcie_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 228 | `framework/test/unittest/platform/virtual/pin_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 229 | `framework/test/unittest/platform/virtual/pwm_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 230 | `framework/test/unittest/platform/virtual/regulator_linux_current_virtual_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 231 | `framework/test/unittest/platform/virtual/regulator_linux_voltage_virtual_driver.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 232 | `framework/test/unittest/platform/virtual/regulator_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 233 | `framework/test/unittest/platform/virtual/spi_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 234 | `framework/test/unittest/platform/virtual/watchdog_virtual.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 235 | `framework/test/unittest/pm/hdf_pm_driver_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 236 | `framework/test/unittest/sensor/hdf_sensor_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 237 | `framework/test/unittest/uevent/devmgr_uevent_test.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 238 | `framework/test/unittest/utils/hcs_parser/unittest/hcs_macro_cases.c` | generic tree-sitter ERROR node(s) in preprocessed C++ | 64 |
| 239 | `framework/tools/hdi-gen/ast/ast.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 53 |
| 240 | `framework/tools/hdi-gen/lexer/lexer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 241 | `framework/tools/hdi-gen/lexer/token.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 242 | `framework/tools/hdi-gen/parser/parser.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 243 | `framework/utils/src/hcs_parser/test/unittest/common/hdf_config_test.cpp` | tree-sitter-cpp node `missing ;` at 55 site(s) | 55 |
| 244 | `interfaces/inner_api/hdi/iservstat_listener_hdi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 245 | `interfaces/inner_api/utils/hdf_trace.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |

### Per-file details

#### `adapter/khdf/hongmeng/osal/src/osal_cdev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 41 | `ERROR` | `struct` |
| 48 | 41 | `ERROR` | `struct` |
| 57 | 41 | `ERROR` | `struct` |
| 66 | 41 | `ERROR` | `struct` |
| 74 | 41 | `ERROR` | `struct` |
| 82 | 41 | `ERROR` | `struct` |
| 91 | 41 | `ERROR` | `struct` |
| 202 | 113 | `ERROR` | `udkDev-> name` |
| 251 | 128 | `ERROR` | `dev-> name` |
| 284 | 128 | `ERROR` | `dev-> name` |

#### `adapter/khdf/hongmeng/osal/src/osal_workqueue.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 30 | `ERROR` | `struct` |
| 104 | 28 | `ERROR` | `struct` |
| 105 | 32 | `ERROR` | `struct` |

#### `adapter/khdf/linux/manager/src/devmgr_load.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 4 | 12 | `ERROR` | `__init` |

#### `adapter/khdf/linux/manager/src/hdf_kevent.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 426 | 66 | `missing ;` | `` |
| 426 | 106 | `ERROR` | `=(` |
| 426 | 141 | `ERROR` | `)` |

#### `adapter/khdf/linux/model/camera/src/contig_dma.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 41 | `missing ;` | `` |
| 69 | 55 | `ERROR` | `struct` |
| 141 | 55 | `ERROR` | `struct` |
| 324 | 55 | `ERROR` | `struct` |

#### `adapter/khdf/linux/model/camera/src/sg_dma.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 95 | 55 | `ERROR` | `struct` |
| 185 | 55 | `ERROR` | `struct` |
| 339 | 55 | `ERROR` | `struct` |
| 404 | 45 | `ERROR` | `*` |
| 404 | 68 | `ERROR` | `*` |

#### `adapter/khdf/linux/model/camera/src/virtual_malloc.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 55 | `ERROR` | `struct` |
| 57 | 55 | `ERROR` | `struct` |
| 87 | 22 | `ERROR` | `void` |
| 95 | 31 | `ERROR` | `*` |
| 95 | 54 | `ERROR` | `*` |
| 127 | 18 | `ERROR` | `void __iomem*` |
| 127 | 33 | `ERROR` | `buf-> vaddr` |
| 217 | 55 | `ERROR` | `struct` |

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
| 500 | 125 | `ERROR` | `cntlr-> index` |
| 573 | 61 | `missing ;` | `` |
| 573 | 101 | `ERROR` | `=(` |
| 573 | 131 | `ERROR` | `)` |

#### `adapter/khdf/linux/model/usb/host/include/usb_net_adapter.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 25 | `ERROR` | `__percpu` |

#### `adapter/khdf/linux/model/usb/host/src/usb_net_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 244 | `ERROR` | `,` |
| 72 | 51 | `ERROR` | `struct` |
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
| … | … | … | *(132 more)* |

#### `adapter/khdf/linux/model/usb/host/src/usb_pnp_notify.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 787 | 63 | `missing ;` | `` |
| 787 | 103 | `ERROR` | `=(` |
| 787 | 135 | `ERROR` | `)` |

#### `adapter/khdf/linux/osal/src/osal_cdev.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 125 | 63 | `ERROR` | `struct` |
| 131 | 63 | `ERROR` | `struct` |
| 137 | 63 | `ERROR` | `struct` |
| 143 | 63 | `ERROR` | `struct` |
| 149 | 63 | `ERROR` | `struct` |
| 155 | 53 | `ERROR` | `struct` |
| 161 | 53 | `ERROR` | `struct` |

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
| 43 | 162 | `ERROR` | `config-> number` |
| 129 | 52 | `missing ;` | `` |
| 129 | 92 | `ERROR` | `=(` |
| 129 | 113 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/pwm/pwm_hi35xx_linux.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 6 | `ERROR` | `__iomem` |
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

#### `adapter/khdf/linux/platform/regulator/regulator_adapter_consumer.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 5 | `ERROR` | `__init` |
| 74 | 6 | `ERROR` | `__exit` |

#### `adapter/khdf/linux/platform/rtc/rtc_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 196 | 118 | `ERROR` | `HdfDeviceGetServiceName(device)` |
| 221 | 60 | `missing ;` | `` |
| 221 | 100 | `ERROR` | `=(` |
| 221 | 129 | `ERROR` | `)` |

#### `adapter/khdf/linux/platform/spi/spi_adapter.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 275 | 122 | `ERROR` | `GetSpiDevName(& spidev-> dev)` |
| 278 | 125 | `ERROR` | `GetSpiDevName(& spidev-> dev)` |
| 279 | 136 | `ERROR` | `spidev-> master-> bus_num` |
| 523 | 58 | `missing ;` | `` |
| 523 | 98 | `ERROR` | `=(` |
| 523 | 125 | `ERROR` | `)` |

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

#### `adapter/khdf/linux/test/test_helper_driver/src/test_helper_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 65 | 12 | `ERROR` | `__init` |
| 65 | 308 | `ERROR` | `__exit` |

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
| 100 | 36 | `ERROR` | `struct Vnode` |
| 100 | 62 | `ERROR` | `struct geometry` |
| 125 | 44 | `ERROR` | `struct Vnode` |
| 174 | 37 | `ERROR` | `struct Vnode` |
| 194 | 21 | `ERROR` | `struct` |
| 204 | 30 | `ERROR` | `int` |

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
| 878 | 63 | `missing ;` | `` |
| 878 | 103 | `ERROR` | `=(` |
| 878 | 135 | `ERROR` | `)` |

#### `adapter/khdf/liteos/model/usb/host/src/usb_test_pnp_notify.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 67 | `missing ;` | `` |
| 59 | 107 | `ERROR` | `=(` |
| 59 | 143 | `ERROR` | `)` |

#### `adapter/khdf/liteos/platform/include/gpio_dev.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 1 | `ERROR` | `typedef struct GpioBitInfo{ unsigned int groupnumber ; unsigned int bitnumber ;  unsigned char value ; unsigned char dir…` |

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
| 19 | 219 | `ERROR` | `#(` |
| 19 | 297 | `ERROR` | `)` |
| 21 | 209 | `ERROR` | `#(` |
| 21 | 217 | `ERROR` | `(0)` |
| 21 | 277 | `ERROR` | `)` |
| 25 | 222 | `ERROR` | `#(` |
| 25 | 240 | `ERROR` | `(NULL)` |
| 25 | 303 | `ERROR` | `)` |
| 29 | 219 | `ERROR` | `#(` |
| 29 | 297 | `ERROR` | `)` |
| 31 | 209 | `ERROR` | `#(` |
| 31 | 217 | `ERROR` | `(0)` |
| 31 | 277 | `ERROR` | `)` |
| 52 | 219 | `ERROR` | `#(` |
| 52 | 297 | `ERROR` | `)` |
| 56 | 237 | `ERROR` | `#(` |
| 56 | 256 | `ERROR` | `(refCntBeforeGet+ 1)` |
| 56 | 333 | `ERROR` | `)` |
| 60 | 219 | `ERROR` | `#(` |
| 60 | 297 | `ERROR` | `)` |
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
| 70 | 220 | `ERROR` | `#(` |
| 70 | 298 | `ERROR` | `)` |
| 73 | 238 | `ERROR` | `#(` |
| 73 | 257 | `ERROR` | `(refCntBeforeAdd+ 1)` |
| 73 | 334 | `ERROR` | `)` |
| 78 | 220 | `ERROR` | `#(` |
| 78 | 298 | `ERROR` | `)` |
| 82 | 223 | `ERROR` | `#(` |
| 82 | 304 | `ERROR` | `)` |
| 87 | 223 | `ERROR` | `#(` |
| 87 | 304 | `ERROR` | `)` |
| 95 | 220 | `ERROR` | `#(` |
| 95 | 298 | `ERROR` | `)` |
| 101 | 220 | `ERROR` | `#(` |
| 101 | 298 | `ERROR` | `)` |
| 107 | 238 | `ERROR` | `#(` |
| 107 | 334 | `ERROR` | `)` |
| 111 | 220 | `ERROR` | `#(` |
| 111 | 235 | `ERROR` | `(NULL)` |
| 111 | 298 | `ERROR` | `)` |
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
| 253 | 65 | `missing ;` | `` |
| 253 | 105 | `ERROR` | `=(` |
| 253 | 139 | `ERROR` | `)` |

#### `adapter/uhdf2/hdi/test/buffer_handle/buffer_handle_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 19 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 80 | 68 | `missing ;` | `` |
| 89 | 68 | `missing ;` | `` |
| 98 | 68 | `missing ;` | `` |
| 113 | 68 | `missing ;` | `` |
| 135 | 68 | `missing ;` | `` |
| 144 | 68 | `missing ;` | `` |
| 157 | 68 | `missing ;` | `` |
| 165 | 68 | `missing ;` | `` |
| 175 | 68 | `missing ;` | `` |
| 189 | 68 | `missing ;` | `` |
| 204 | 68 | `missing ;` | `` |
| 219 | 68 | `missing ;` | `` |
| 254 | 68 | `missing ;` | `` |
| 291 | 68 | `missing ;` | `` |
| 328 | 68 | `missing ;` | `` |
| 372 | 68 | `missing ;` | `` |
| 420 | 68 | `missing ;` | `` |
| 440 | 68 | `missing ;` | `` |
| 460 | 68 | `missing ;` | `` |

#### `adapter/uhdf2/hdi/test/buffer_handle/native_buffer_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 68 | `missing ;` | `` |
| 56 | 68 | `missing ;` | `` |
| 79 | 68 | `missing ;` | `` |
| 97 | 68 | `missing ;` | `` |
| 114 | 68 | `missing ;` | `` |
| 136 | 68 | `missing ;` | `` |
| 158 | 68 | `missing ;` | `` |
| 185 | 68 | `missing ;` | `` |
| 200 | 68 | `missing ;` | `` |
| 216 | 68 | `missing ;` | `` |
| 227 | 68 | `missing ;` | `` |
| 241 | 68 | `missing ;` | `` |
| 257 | 68 | `missing ;` | `` |
| 273 | 68 | `missing ;` | `` |
| 303 | 68 | `missing ;` | `` |
| 338 | 68 | `missing ;` | `` |
| 343 | 97 | `ERROR` | `sbuffer .c_str()` |
| 358 | 98 | `ERROR` | `dbuffer .c_str()` |

#### `adapter/uhdf2/hdi/test/object_collector/object_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 74 | `missing ;` | `` |

#### `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_c_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 11 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 71 | `missing ;` | `` |
| 35 | 71 | `missing ;` | `` |
| 63 | 90 | `missing ;` | `` |
| 74 | 90 | `missing ;` | `` |
| 84 | 90 | `missing ;` | `` |
| 95 | 84 | `missing ;` | `` |
| 108 | 86 | `missing ;` | `` |
| 121 | 71 | `missing ;` | `` |
| 143 | 67 | `missing ;` | `` |
| 165 | 71 | `missing ;` | `` |
| 171 | 71 | `missing ;` | `` |

#### `adapter/uhdf2/hdi/test/servmgr/service_manager_hdi_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 70 | `missing ;` | `` |
| 64 | 89 | `missing ;` | `` |
| 74 | 89 | `missing ;` | `` |
| 84 | 89 | `missing ;` | `` |
| 95 | 74 | `missing ;` | `` |
| 106 | 73 | `missing ;` | `` |
| 117 | 71 | `missing ;` | `` |
| 134 | 71 | `missing ;` | `` |
| 139 | 121 | `ERROR` | `status .serviceName .data()` |
| 155 | 71 | `missing ;` | `` |
| 160 | 121 | `ERROR` | `status .serviceName .data()` |
| 173 | 71 | `missing ;` | `` |
| 178 | 121 | `ERROR` | `status .serviceName .data()` |
| 194 | 71 | `missing ;` | `` |
| 199 | 121 | `ERROR` | `status .serviceName .data()` |
| 215 | 71 | `missing ;` | `` |
| 220 | 121 | `ERROR` | `status .serviceName .data()` |
| 239 | 71 | `missing ;` | `` |
| 244 | 121 | `ERROR` | `status .serviceName .data()` |
| 266 | 74 | `missing ;` | `` |
| … | … | … | *(1 more)* |

#### `adapter/uhdf2/hdi/test/smq_test/smq_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 60 | 50 | `missing ;` | `` |

#### `adapter/uhdf2/hdi/test/stub_collector/stub_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 70 | `missing ;` | `` |
| 52 | 70 | `missing ;` | `` |
| 62 | 70 | `missing ;` | `` |
| 81 | 70 | `missing ;` | `` |
| 92 | 70 | `missing ;` | `` |
| 103 | 70 | `missing ;` | `` |
| 124 | 70 | `missing ;` | `` |

#### `adapter/uhdf2/host/test/unittest/devhost_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 12 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 73 | `missing ;` | `` |
| 72 | 69 | `missing ;` | `` |
| 118 | 62 | `missing ;` | `` |
| 176 | 66 | `missing ;` | `` |
| 233 | 66 | `missing ;` | `` |
| 280 | 66 | `missing ;` | `` |
| 322 | 61 | `missing ;` | `` |
| 367 | 71 | `missing ;` | `` |
| 383 | 72 | `missing ;` | `` |
| 457 | 62 | `missing ;` | `` |
| 489 | 62 | `missing ;` | `` |
| 528 | 66 | `missing ;` | `` |

#### `adapter/uhdf2/host/test/unittest/devmgr_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 47 | 61 | `missing ;` | `` |
| 72 | 62 | `missing ;` | `` |
| 97 | 57 | `missing ;` | `` |
| 120 | 55 | `missing ;` | `` |

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

#### `adapter/uhdf2/osal/test/unittest/common/osal_slist_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 65 | `missing ;` | `` |
| 80 | 66 | `missing ;` | `` |
| 90 | 67 | `missing ;` | `` |
| 103 | 64 | `missing ;` | `` |

#### `adapter/uhdf2/osal/test/unittest/common/sample_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 128 | 63 | `missing ;` | `` |
| 128 | 103 | `ERROR` | `=(` |
| 128 | 135 | `ERROR` | `)` |

#### `adapter/uhdf2/shared/test/dev_attribute_serialize_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 15 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 86 | `missing ;` | `` |
| 22 | 86 | `missing ;` | `` |
| 32 | 86 | `missing ;` | `` |
| 43 | 86 | `missing ;` | `` |
| 58 | 86 | `missing ;` | `` |
| 79 | 86 | `missing ;` | `` |
| 85 | 86 | `missing ;` | `` |
| 98 | 86 | `missing ;` | `` |
| 115 | 86 | `missing ;` | `` |
| 135 | 86 | `missing ;` | `` |
| 158 | 86 | `missing ;` | `` |
| 184 | 86 | `missing ;` | `` |
| 213 | 86 | `missing ;` | `` |
| 245 | 86 | `missing ;` | `` |
| 278 | 86 | `missing ;` | `` |

#### `adapter/uhdf2/test/unittest/platform/i2c/ui2c_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 56 | `missing ;` | `` |
| 28 | 56 | `missing ;` | `` |

#### `framework/core/host/test/unittest/hdf_vdi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 64 | `missing ;` | `` |
| 54 | 67 | `missing ;` | `` |
| 62 | 64 | `missing ;` | `` |
| 83 | 67 | `missing ;` | `` |
| 91 | 71 | `missing ;` | `` |
| 107 | 70 | `missing ;` | `` |
| 114 | 69 | `missing ;` | `` |
| 122 | 61 | `missing ;` | `` |

#### `framework/core/manager/test/unittest/common/devmgr_uevent_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 70 | `missing ;` | `` |

#### `framework/core/manager/test/unittest/common/hdf_ioservice_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 84 | 132 | `ERROR` | `static_cast< char*>(listener-> priv)` |
| 93 | 104 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 103 | 157 | `ERROR` | `static_cast< char*>(listener-> priv)` |
| 114 | 128 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 127 | 116 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 128 | 110 | `ERROR` | `static_cast< char*>(service-> priv)` |
| 174 | 119 | `ERROR` | `time .sec` |
| 183 | 61 | `missing ;` | `` |
| 191 | 61 | `missing ;` | `` |
| 236 | 61 | `missing ;` | `` |
| 286 | 61 | `missing ;` | `` |
| 306 | 61 | `missing ;` | `` |
| 341 | 61 | `missing ;` | `` |
| 404 | 61 | `missing ;` | `` |
| 436 | 61 | `missing ;` | `` |
| 468 | 61 | `missing ;` | `` |
| 500 | 61 | `missing ;` | `` |
| 535 | 61 | `missing ;` | `` |
| 559 | 61 | `missing ;` | `` |
| 584 | 61 | `missing ;` | `` |
| … | … | … | *(12 more)* |

#### `framework/core/manager/test/unittest/common/hdf_lite_manager_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 66 | `missing ;` | `` |
| 39 | 66 | `missing ;` | `` |
| 47 | 67 | `missing ;` | `` |
| 78 | 80 | `missing ;` | `` |

#### `framework/core/manager/test/unittest/common/hdf_pm_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 17 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 64 | `missing ;` | `` |
| 49 | 65 | `missing ;` | `` |
| 58 | 63 | `missing ;` | `` |
| 67 | 67 | `missing ;` | `` |
| 77 | 68 | `missing ;` | `` |
| 87 | 64 | `missing ;` | `` |
| 96 | 65 | `missing ;` | `` |
| 105 | 63 | `missing ;` | `` |
| 114 | 67 | `missing ;` | `` |
| 124 | 68 | `missing ;` | `` |
| 134 | 66 | `missing ;` | `` |
| 144 | 67 | `missing ;` | `` |
| 154 | 65 | `missing ;` | `` |
| 163 | 69 | `missing ;` | `` |
| 173 | 70 | `missing ;` | `` |
| 183 | 72 | `missing ;` | `` |
| 193 | 77 | `missing ;` | `` |

#### `framework/core/manager/test/unittest/common/hdf_remote_adapter_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 76 | `missing ;` | `` |
| 36 | 76 | `missing ;` | `` |
| 63 | 76 | `missing ;` | `` |
| 88 | 76 | `missing ;` | `` |
| 97 | 76 | `missing ;` | `` |
| 117 | 76 | `missing ;` | `` |
| 135 | 76 | `missing ;` | `` |

#### `framework/core/manager/test/unittest/common/hdf_sbuf_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 230 | 61 | `missing ;` | `` |
| 238 | 66 | `missing ;` | `` |
| 248 | 70 | `missing ;` | `` |
| 260 | 69 | `missing ;` | `` |
| 276 | 1 | `ERROR` | `ASSERT_EQ(val , static_cast< uint64_t> INT64_MAX)` |
| 285 | 59 | `missing ;` | `` |
| 313 | 60 | `missing ;` | `` |
| 341 | 60 | `missing ;` | `` |
| 369 | 60 | `missing ;` | `` |
| 397 | 61 | `missing ;` | `` |
| 425 | 61 | `missing ;` | `` |
| 453 | 60 | `missing ;` | `` |
| 481 | 61 | `missing ;` | `` |
| 505 | 65 | `missing ;` | `` |
| 528 | 61 | `missing ;` | `` |
| 556 | 65 | `missing ;` | `` |
| 584 | 68 | `missing ;` | `` |
| 604 | 70 | `missing ;` | `` |
| 632 | 63 | `missing ;` | `` |
| 664 | 67 | `missing ;` | `` |
| … | … | … | *(16 more)* |

#### `framework/core/shared/test/unittest/common/hdf_core_shared_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 79 | `missing ;` | `` |
| 23 | 80 | `missing ;` | `` |
| 33 | 79 | `missing ;` | `` |
| 45 | 78 | `missing ;` | `` |
| 62 | 80 | `missing ;` | `` |

#### `framework/model/audio/common/test/unittest/common/audio_codec_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 84 | `missing ;` | `` |
| 39 | 80 | `missing ;` | `` |
| 45 | 83 | `missing ;` | `` |
| 51 | 83 | `missing ;` | `` |
| 57 | 80 | `missing ;` | `` |

#### `framework/model/audio/common/test/unittest/common/audio_dai_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 78 | `missing ;` | `` |
| 39 | 79 | `missing ;` | `` |
| 45 | 81 | `missing ;` | `` |
| 51 | 79 | `missing ;` | `` |

#### `framework/model/audio/common/test/unittest/common/audio_dma_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 77 | `missing ;` | `` |
| 39 | 76 | `missing ;` | `` |
| 45 | 83 | `missing ;` | `` |
| 51 | 82 | `missing ;` | `` |
| 57 | 73 | `missing ;` | `` |
| 63 | 75 | `missing ;` | `` |
| 69 | 76 | `missing ;` | `` |
| 75 | 74 | `missing ;` | `` |
| 81 | 75 | `missing ;` | `` |
| 87 | 76 | `missing ;` | `` |

#### `framework/model/audio/common/test/unittest/common/audio_dsp_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 83 | `missing ;` | `` |
| 39 | 79 | `missing ;` | `` |

#### `framework/model/audio/common/test/unittest/common/audio_platform_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 21 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 100 | `missing ;` | `` |
| 39 | 98 | `missing ;` | `` |
| 45 | 104 | `missing ;` | `` |
| 51 | 101 | `missing ;` | `` |
| 57 | 95 | `missing ;` | `` |
| 63 | 101 | `missing ;` | `` |
| 69 | 102 | `missing ;` | `` |
| 75 | 93 | `missing ;` | `` |
| 81 | 92 | `missing ;` | `` |
| 87 | 97 | `missing ;` | `` |
| 93 | 96 | `missing ;` | `` |
| 99 | 95 | `missing ;` | `` |
| 105 | 96 | `missing ;` | `` |
| 111 | 96 | `missing ;` | `` |
| 117 | 95 | `missing ;` | `` |
| 123 | 97 | `missing ;` | `` |
| 129 | 93 | `missing ;` | `` |
| 135 | 98 | `missing ;` | `` |
| 141 | 99 | `missing ;` | `` |
| 147 | 98 | `missing ;` | `` |
| … | … | … | *(1 more)* |

#### `framework/model/audio/core/src/audio_host.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 443 | 62 | `missing ;` | `` |
| 443 | 102 | `ERROR` | `=(` |
| 443 | 133 | `ERROR` | `)` |

#### `framework/model/audio/core/test/unittest/common/audio_core_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 26 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 78 | `missing ;` | `` |
| 38 | 79 | `missing ;` | `` |
| 44 | 84 | `missing ;` | `` |
| 50 | 79 | `missing ;` | `` |
| 56 | 76 | `missing ;` | `` |
| 62 | 78 | `missing ;` | `` |
| 68 | 76 | `missing ;` | `` |
| 74 | 83 | `missing ;` | `` |
| 80 | 81 | `missing ;` | `` |
| 86 | 82 | `missing ;` | `` |
| 92 | 81 | `missing ;` | `` |
| 98 | 76 | `missing ;` | `` |
| 104 | 75 | `missing ;` | `` |
| 110 | 79 | `missing ;` | `` |
| 116 | 78 | `missing ;` | `` |
| 122 | 78 | `missing ;` | `` |
| 128 | 79 | `missing ;` | `` |
| 134 | 75 | `missing ;` | `` |
| 140 | 76 | `missing ;` | `` |
| 146 | 77 | `missing ;` | `` |
| … | … | … | *(6 more)* |

#### `framework/model/audio/core/test/unittest/common/audio_host_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 82 | `missing ;` | `` |
| 39 | 75 | `missing ;` | `` |

#### `framework/model/audio/core/test/unittest/common/audio_parse_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 81 | `missing ;` | `` |
| 39 | 79 | `missing ;` | `` |

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

#### `framework/model/audio/sapm/test/unittest/common/audio_sapm_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 82 | `missing ;` | `` |
| 38 | 78 | `missing ;` | `` |
| 44 | 80 | `missing ;` | `` |
| 50 | 74 | `missing ;` | `` |
| 56 | 76 | `missing ;` | `` |
| 62 | 84 | `missing ;` | `` |
| 68 | 84 | `missing ;` | `` |
| 74 | 84 | `missing ;` | `` |

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

#### `framework/model/camera/buffer_manager/src/buffer_queue.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 226 | 55 | `ERROR` | `struct` |
| 227 | 57 | `ERROR` | `struct` |

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
| 6 | 29 | `ERROR` | `struct` |
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

#### `framework/model/network/ethernet/src/hdf_eth_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 207 | 54 | `missing ;` | `` |
| 207 | 94 | `ERROR` | `=(` |
| 207 | 117 | `ERROR` | `)` |

#### `framework/model/network/wifi/core/hdf_wifi_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 546 | 58 | `missing ;` | `` |
| 546 | 98 | `ERROR` | `=(` |
| 546 | 125 | `ERROR` | `)` |

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

#### `framework/sample/platform/uart/src/uart_sample.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 67 | `missing ;` | `` |
| 22 | 107 | `ERROR` | `=(` |
| 22 | 143 | `ERROR` | `)` |

#### `framework/support/platform/include/fwk/platform_errno.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 6 | 1 | `ERROR` | `enum PlatformErrno{ HDF_PLT_ERR_OS_API= HDF_ERR_BSP_PLT_API_ERR , HDF_PLT_ERR_OPEN_DEV= HDF_PAL_ERR_DEV_CREATE , HDF_PLT…` |

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

#### `framework/support/platform/src/gpio/gpio_service.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 397 | 62 | `missing ;` | `` |
| 397 | 102 | `ERROR` | `=(` |
| 397 | 133 | `ERROR` | `)` |

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

#### `framework/support/platform/src/pin/pin_core.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 108 | `ERROR` | `cntlr-> pinCount` |
| 159 | 126 | `ERROR` | `desc-> pinName` |
| 740 | 61 | `missing ;` | `` |
| 740 | 101 | `ERROR` | `=(` |
| 740 | 131 | `ERROR` | `)` |

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
| 452 | 110 | `ERROR` | `pos-> info .number` |
| 475 | 108 | `ERROR` | `cntrl-> info .number` |
| 482 | 112 | `ERROR` | `cntrl-> info .number` |
| 487 | 112 | `ERROR` | `cntrl-> info .number` |
| 493 | 119 | `ERROR` | `cntrl-> info .number` |
| 513 | 116 | `ERROR` | `pos-> info .number` |
| 589 | 63 | `missing ;` | `` |
| 589 | 103 | `ERROR` | `=(` |
| 589 | 135 | `ERROR` | `)` |

#### `framework/support/platform/test/unittest/common/hdf_adc_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 57 | `missing ;` | `` |
| 41 | 64 | `missing ;` | `` |
| 49 | 64 | `missing ;` | `` |
| 57 | 66 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_can_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 68 | `missing ;` | `` |
| 37 | 68 | `missing ;` | `` |
| 44 | 66 | `missing ;` | `` |
| 51 | 72 | `missing ;` | `` |
| 58 | 72 | `missing ;` | `` |
| 65 | 68 | `missing ;` | `` |
| 72 | 84 | `missing ;` | `` |
| 79 | 85 | `missing ;` | `` |
| 86 | 85 | `missing ;` | `` |
| 93 | 86 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_clock_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 63 | `missing ;` | `` |
| 42 | 64 | `missing ;` | `` |
| 50 | 64 | `missing ;` | `` |
| 58 | 64 | `missing ;` | `` |
| 66 | 66 | `missing ;` | `` |
| 74 | 66 | `missing ;` | `` |
| 82 | 68 | `missing ;` | `` |
| 90 | 68 | `missing ;` | `` |
| 98 | 70 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_dac_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 62 | `missing ;` | `` |
| 41 | 68 | `missing ;` | `` |
| 49 | 68 | `missing ;` | `` |
| 57 | 70 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_emmc_mini_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 101 | 75 | `missing ;` | `` |
| 108 | 73 | `missing ;` | `` |
| 115 | 74 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_emmc_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 57 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_gpio_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 64 | `missing ;` | `` |
| 41 | 64 | `missing ;` | `` |
| 49 | 63 | `missing ;` | `` |
| 57 | 67 | `missing ;` | `` |
| 65 | 66 | `missing ;` | `` |
| 73 | 68 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_hdmi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 147 | 67 | `missing ;` | `` |
| 154 | 67 | `missing ;` | `` |
| 161 | 67 | `missing ;` | `` |
| 168 | 65 | `missing ;` | `` |
| 175 | 64 | `missing ;` | `` |
| 182 | 69 | `missing ;` | `` |
| 189 | 73 | `missing ;` | `` |
| 196 | 77 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_i2c_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 67 | `missing ;` | `` |
| 59 | 67 | `missing ;` | `` |
| 67 | 66 | `missing ;` | `` |
| 73 | 69 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_i2s_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 63 | `missing ;` | `` |
| 58 | 66 | `missing ;` | `` |
| 65 | 61 | `missing ;` | `` |
| 72 | 65 | `missing ;` | `` |
| 79 | 67 | `missing ;` | `` |
| 87 | 62 | `missing ;` | `` |
| 95 | 66 | `missing ;` | `` |
| 102 | 63 | `missing ;` | `` |
| 109 | 68 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_i3c_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 64 | `missing ;` | `` |
| 44 | 65 | `missing ;` | `` |
| 51 | 65 | `missing ;` | `` |
| 58 | 66 | `missing ;` | `` |
| 65 | 63 | `missing ;` | `` |
| 72 | 67 | `missing ;` | `` |
| 79 | 67 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_mipi_csi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 12 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 74 | `missing ;` | `` |
| 57 | 81 | `missing ;` | `` |
| 64 | 80 | `missing ;` | `` |
| 71 | 72 | `missing ;` | `` |
| 78 | 83 | `missing ;` | `` |
| 85 | 82 | `missing ;` | `` |
| 92 | 76 | `missing ;` | `` |
| 99 | 80 | `missing ;` | `` |
| 106 | 78 | `missing ;` | `` |
| 113 | 79 | `missing ;` | `` |
| 120 | 74 | `missing ;` | `` |
| 127 | 78 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_mipi_dsi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 71 | `missing ;` | `` |
| 56 | 71 | `missing ;` | `` |
| 62 | 69 | `missing ;` | `` |
| 68 | 69 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_pcie_bus_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 67 | `missing ;` | `` |
| 38 | 69 | `missing ;` | `` |
| 45 | 77 | `missing ;` | `` |
| 52 | 78 | `missing ;` | `` |
| 59 | 79 | `missing ;` | `` |
| 66 | 76 | `missing ;` | `` |
| 73 | 80 | `missing ;` | `` |
| 80 | 75 | `missing ;` | `` |
| 87 | 75 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_pcie_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 63 | `missing ;` | `` |
| 41 | 69 | `missing ;` | `` |
| 49 | 69 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_pin_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 59 | `missing ;` | `` |
| 59 | 63 | `missing ;` | `` |
| 67 | 62 | `missing ;` | `` |
| 75 | 64 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_device_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 85 | `missing ;` | `` |
| 40 | 87 | `missing ;` | `` |
| 47 | 87 | `missing ;` | `` |
| 54 | 87 | `missing ;` | `` |
| 61 | 91 | `missing ;` | `` |
| 68 | 88 | `missing ;` | `` |
| 75 | 89 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_dumper_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 90 | `missing ;` | `` |
| 37 | 91 | `missing ;` | `` |
| 44 | 91 | `missing ;` | `` |
| 51 | 91 | `missing ;` | `` |
| 58 | 89 | `missing ;` | `` |
| 65 | 90 | `missing ;` | `` |
| 72 | 90 | `missing ;` | `` |
| 79 | 90 | `missing ;` | `` |
| 86 | 90 | `missing ;` | `` |
| 93 | 91 | `missing ;` | `` |
| 100 | 89 | `missing ;` | `` |
| 107 | 91 | `missing ;` | `` |
| 114 | 93 | `missing ;` | `` |
| 121 | 90 | `missing ;` | `` |
| 128 | 84 | `missing ;` | `` |
| 135 | 89 | `missing ;` | `` |
| 142 | 85 | `missing ;` | `` |
| 149 | 89 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_event_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 89 | `missing ;` | `` |
| 40 | 87 | `missing ;` | `` |
| 47 | 93 | `missing ;` | `` |
| 54 | 87 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_manager_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 89 | `missing ;` | `` |
| 40 | 89 | `missing ;` | `` |
| 47 | 91 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_queue_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 86 | `missing ;` | `` |
| 40 | 87 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_platform_trace_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 90 | `missing ;` | `` |
| 36 | 83 | `missing ;` | `` |
| 43 | 87 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_pwm_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 62 | `missing ;` | `` |
| 41 | 60 | `missing ;` | `` |
| 49 | 64 | `missing ;` | `` |
| 57 | 65 | `missing ;` | `` |
| 65 | 59 | `missing ;` | `` |
| 73 | 60 | `missing ;` | `` |
| 81 | 66 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_regulator_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 75 | `missing ;` | `` |
| 51 | 76 | `missing ;` | `` |
| 57 | 81 | `missing ;` | `` |
| 63 | 79 | `missing ;` | `` |
| 69 | 79 | `missing ;` | `` |
| 75 | 79 | `missing ;` | `` |
| 81 | 79 | `missing ;` | `` |
| 87 | 78 | `missing ;` | `` |
| 93 | 80 | `missing ;` | `` |
| 99 | 80 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_rtc_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 66 | `missing ;` | `` |
| 42 | 69 | `missing ;` | `` |
| 51 | 69 | `missing ;` | `` |
| 60 | 71 | `missing ;` | `` |
| 69 | 74 | `missing ;` | `` |
| 77 | 74 | `missing ;` | `` |
| 86 | 64 | `missing ;` | `` |
| 95 | 64 | `missing ;` | `` |
| 104 | 71 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_sdio_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 66 | `missing ;` | `` |
| 50 | 65 | `missing ;` | `` |
| 57 | 67 | `missing ;` | `` |
| 64 | 80 | `missing ;` | `` |
| 71 | 81 | `missing ;` | `` |
| 78 | 77 | `missing ;` | `` |
| 85 | 74 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_spi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 61 | `missing ;` | `` |
| 44 | 64 | `missing ;` | `` |
| 53 | 64 | `missing ;` | `` |
| 62 | 64 | `missing ;` | `` |
| 71 | 66 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_timer_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 64 | `missing ;` | `` |
| 40 | 68 | `missing ;` | `` |
| 48 | 64 | `missing ;` | `` |
| 56 | 66 | `missing ;` | `` |
| 64 | 65 | `missing ;` | `` |
| 72 | 72 | `missing ;` | `` |
| 80 | 72 | `missing ;` | `` |
| 88 | 74 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_uart_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 67 | `missing ;` | `` |
| 41 | 59 | `missing ;` | `` |
| 49 | 66 | `missing ;` | `` |
| 57 | 68 | `missing ;` | `` |
| 63 | 69 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_watchdog_mini_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 87 | 86 | `missing ;` | `` |
| 94 | 75 | `missing ;` | `` |

#### `framework/support/platform/test/unittest/common/hdf_watchdog_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 87 | `missing ;` | `` |
| 41 | 83 | `missing ;` | `` |
| 49 | 78 | `missing ;` | `` |
| 57 | 85 | `missing ;` | `` |
| 65 | 87 | `missing ;` | `` |

#### `framework/support/posix/test/unittest/common/hdf_osal_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 93 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 56 | `missing ;` | `` |
| 61 | 56 | `missing ;` | `` |
| 67 | 56 | `missing ;` | `` |
| 73 | 56 | `missing ;` | `` |
| 79 | 56 | `missing ;` | `` |
| 85 | 56 | `missing ;` | `` |
| 91 | 56 | `missing ;` | `` |
| 97 | 56 | `missing ;` | `` |
| 103 | 55 | `missing ;` | `` |
| 109 | 55 | `missing ;` | `` |
| 115 | 55 | `missing ;` | `` |
| 121 | 55 | `missing ;` | `` |
| 127 | 55 | `missing ;` | `` |
| 133 | 55 | `missing ;` | `` |
| 139 | 55 | `missing ;` | `` |
| 145 | 55 | `missing ;` | `` |
| 151 | 55 | `missing ;` | `` |
| 157 | 55 | `missing ;` | `` |
| 163 | 55 | `missing ;` | `` |
| 169 | 55 | `missing ;` | `` |
| … | … | … | *(73 more)* |

#### `framework/support/posix/test/unittest/common/hdf_osal_test_posix.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 48 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 61 | `missing ;` | `` |
| 68 | 61 | `missing ;` | `` |
| 74 | 61 | `missing ;` | `` |
| 80 | 61 | `missing ;` | `` |
| 86 | 61 | `missing ;` | `` |
| 92 | 61 | `missing ;` | `` |
| 98 | 61 | `missing ;` | `` |
| 104 | 61 | `missing ;` | `` |
| 110 | 60 | `missing ;` | `` |
| 116 | 60 | `missing ;` | `` |
| 122 | 60 | `missing ;` | `` |
| 128 | 60 | `missing ;` | `` |
| 134 | 60 | `missing ;` | `` |
| 140 | 60 | `missing ;` | `` |
| 146 | 60 | `missing ;` | `` |
| 152 | 60 | `missing ;` | `` |
| 158 | 60 | `missing ;` | `` |
| 164 | 60 | `missing ;` | `` |
| 170 | 60 | `missing ;` | `` |
| 176 | 60 | `missing ;` | `` |
| … | … | … | *(28 more)* |

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

#### `framework/test/unittest/platform/common/adc_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 117 | 58 | `missing ;` | `` |
| 117 | 98 | `ERROR` | `=(` |
| 117 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/can_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 132 | `ERROR` | `(* config)` |
| 96 | 136 | `ERROR` | `config-> busNum` |
| 110 | 215 | `ERROR` | `NULL` |
| 131 | 225 | `ERROR` | `NULL` |
| 183 | 87 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 195 | 103 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 202 | 259 | `ERROR` | `CanBusReadMsg(g_handle ,& msg , 0)` |
| 218 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 219 | 99 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 223 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 236 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 237 | 96 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 238 | 99 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 243 | 103 | `ERROR` | `"[""E" "/""HDF_LOG_TAG" "] ""/home/sergei/drivers_hdf_core/framework/test/unittest/platform/common/can_test.c" "(line:%d…` |
| 324 | 211 | `ERROR` | `NULL` |
| 340 | 211 | `ERROR` | `NULL` |
| 355 | 211 | `ERROR` | `NULL` |
| 369 | 211 | `ERROR` | `NULL` |
| 387 | 201 | `ERROR` | `#(` |
| 387 | 279 | `ERROR` | `)` |
| … | … | … | *(29 more)* |

#### `framework/test/unittest/platform/common/clock_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 60 | `missing ;` | `` |
| 94 | 100 | `ERROR` | `=(` |
| 94 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/dac_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 118 | 58 | `missing ;` | `` |
| 118 | 98 | `ERROR` | `=(` |
| 118 | 125 | `ERROR` | `)` |

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

#### `framework/test/unittest/platform/common/pin_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 114 | 58 | `missing ;` | `` |
| 114 | 98 | `ERROR` | `=(` |
| 114 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/platform_device_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 213 | `ERROR` | `#(` |
| 21 | 291 | `ERROR` | `)` |
| 23 | 203 | `ERROR` | `#(` |
| 23 | 211 | `ERROR` | `(0)` |
| 23 | 271 | `ERROR` | `)` |
| 27 | 216 | `ERROR` | `#(` |
| 27 | 234 | `ERROR` | `(NULL)` |
| 27 | 297 | `ERROR` | `)` |
| 31 | 213 | `ERROR` | `#(` |
| 31 | 291 | `ERROR` | `)` |
| 33 | 203 | `ERROR` | `#(` |
| 33 | 211 | `ERROR` | `(0)` |
| 33 | 271 | `ERROR` | `)` |
| 54 | 213 | `ERROR` | `#(` |
| 54 | 291 | `ERROR` | `)` |
| 58 | 231 | `ERROR` | `#(` |
| 58 | 250 | `ERROR` | `(refCntBeforeGet+ 1)` |
| 58 | 327 | `ERROR` | `)` |
| 62 | 213 | `ERROR` | `#(` |
| 62 | 291 | `ERROR` | `)` |
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
| 18 | 214 | `ERROR` | `#(` |
| 18 | 234 | `ERROR` | `(0)` |
| 18 | 294 | `ERROR` | `)` |
| 22 | 215 | `ERROR` | `#(` |
| 22 | 293 | `ERROR` | `)` |
| 29 | 215 | `ERROR` | `#(` |
| 29 | 293 | `ERROR` | `)` |
| 46 | 216 | `ERROR` | `#(` |
| 46 | 298 | `ERROR` | `)` |
| 50 | 212 | `ERROR` | `#(` |
| 50 | 290 | `ERROR` | `)` |
| 54 | 212 | `ERROR` | `#(` |
| 54 | 290 | `ERROR` | `)` |
| 55 | 221 | `ERROR` | `#(` |
| 55 | 308 | `ERROR` | `)` |
| 59 | 216 | `ERROR` | `#(` |
| 59 | 298 | `ERROR` | `)` |
| 63 | 212 | `ERROR` | `#(` |
| 63 | 290 | `ERROR` | `)` |
| 68 | 216 | `ERROR` | `#(` |
| … | … | … | *(34 more)* |

#### `framework/test/unittest/platform/common/platform_manager_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 73 | 214 | `ERROR` | `#(` |
| 73 | 292 | `ERROR` | `)` |
| 76 | 232 | `ERROR` | `#(` |
| 76 | 251 | `ERROR` | `(refCntBeforeAdd+ 1)` |
| 76 | 328 | `ERROR` | `)` |
| 81 | 214 | `ERROR` | `#(` |
| 81 | 292 | `ERROR` | `)` |
| 85 | 217 | `ERROR` | `#(` |
| 85 | 298 | `ERROR` | `)` |
| 90 | 217 | `ERROR` | `#(` |
| 90 | 298 | `ERROR` | `)` |
| 98 | 214 | `ERROR` | `#(` |
| 98 | 292 | `ERROR` | `)` |
| 104 | 214 | `ERROR` | `#(` |
| 104 | 292 | `ERROR` | `)` |
| 110 | 232 | `ERROR` | `#(` |
| 110 | 328 | `ERROR` | `)` |
| 114 | 214 | `ERROR` | `#(` |
| 114 | 229 | `ERROR` | `(NULL)` |
| 114 | 292 | `ERROR` | `)` |
| … | … | … | *(30 more)* |

#### `framework/test/unittest/platform/common/platform_queue_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 117 | `ERROR` | `msg-> code` |
| 58 | 215 | `ERROR` | `#(` |
| 58 | 293 | `ERROR` | `)` |
| 65 | 215 | `ERROR` | `#(` |
| 65 | 293 | `ERROR` | `)` |
| 69 | 224 | `ERROR` | `#(` |
| 69 | 311 | `ERROR` | `)` |
| 86 | 212 | `ERROR` | `#(` |
| 86 | 290 | `ERROR` | `)` |
| 89 | 212 | `ERROR` | `#(` |
| 89 | 290 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/pwm_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 58 | `missing ;` | `` |
| 123 | 98 | `ERROR` | `=(` |
| 123 | 125 | `ERROR` | `)` |

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

#### `framework/test/unittest/platform/common/timer_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 109 | `ERROR` | `config-> number` |
| 109 | 60 | `missing ;` | `` |
| 109 | 100 | `ERROR` | `=(` |
| 109 | 129 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/uart_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 126 | 59 | `missing ;` | `` |
| 126 | 99 | `ERROR` | `=(` |
| 126 | 127 | `ERROR` | `)` |

#### `framework/test/unittest/platform/common/watchdog_driver_test.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 104 | 63 | `missing ;` | `` |
| 104 | 103 | `ERROR` | `=(` |
| 104 | 135 | `ERROR` | `)` |

#### `framework/test/unittest/platform/config/can_test_config.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 112 | 58 | `missing ;` | `` |
| 112 | 98 | `ERROR` | `=(` |
| 112 | 125 | `ERROR` | `)` |

#### `framework/test/unittest/platform/virtual/adc_linux_virtual_iio_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 174 | 12 | `ERROR` | `__init` |
| 186 | 13 | `ERROR` | `__exit` |

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
| 181 | 12 | `ERROR` | `__init` |
| 190 | 13 | `ERROR` | `__exit` |

#### `framework/test/unittest/platform/virtual/regulator_linux_voltage_virtual_driver.c`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 143 | 145 | `ERROR` | `g_virtualVoltageRegulatorDesc .name` |
| 184 | 12 | `ERROR` | `__init` |
| 194 | 13 | `ERROR` | `__exit` |

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

#### `framework/utils/src/hcs_parser/test/unittest/common/hdf_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 55 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 91 | 73 | `missing ;` | `` |
| 99 | 81 | `missing ;` | `` |
| 106 | 78 | `missing ;` | `` |
| 113 | 79 | `missing ;` | `` |
| 120 | 76 | `missing ;` | `` |
| 127 | 80 | `missing ;` | `` |
| 134 | 77 | `missing ;` | `` |
| 141 | 80 | `missing ;` | `` |
| 148 | 77 | `missing ;` | `` |
| 155 | 76 | `missing ;` | `` |
| 162 | 73 | `missing ;` | `` |
| 169 | 81 | `missing ;` | `` |
| 176 | 78 | `missing ;` | `` |
| 183 | 81 | `missing ;` | `` |
| 190 | 78 | `missing ;` | `` |
| 197 | 77 | `missing ;` | `` |
| 204 | 74 | `missing ;` | `` |
| 211 | 81 | `missing ;` | `` |
| 218 | 78 | `missing ;` | `` |
| 225 | 81 | `missing ;` | `` |
| … | … | … | *(35 more)* |

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

Generated from `trace analyze /home/sergei/hiviewdfx_hiview` (174 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 174

### Failure categories

| Category | Files |
|----------|------:|
| gtest/HWTEST macros (`missing ;`) | 121 |
| generic ERROR nodes (mixed C++ constructs) | 40 |
| missing type identifiers (often macro-expanded types) | 7 |
| other / mixed | 4 |
| extern template instantiations | 2 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `adapter/plugins/eventservice/service/idl/dfx/include/listener_status_util.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 2 | `adapter/plugins/eventservice/service/idl/include/iquery_sys_event_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 3 | `adapter/plugins/eventservice/service/idl/include/parcelable_vector_rw.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 4 | `adapter/plugins/eventservice/service/test/unittest/common/data_share_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 5 | `adapter/plugins/eventservice/service/test/unittest/common/event_checker_test.cpp` | tree-sitter-cpp node `missing ;` at 12 site(s) | 12 |
| 6 | `adapter/plugins/eventservice/service/test/unittest/common/event_query_wrapper_builder_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 7 | `adapter/plugins/eventservice/service/test/unittest/common/event_service_adapter_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 8 | `adapter/plugins/eventservice/service/test/unittest/common/listener_status_monitor_test.cpp` | tree-sitter-cpp node `missing ;` at 17 site(s) | 17 |
| 9 | `adapter/plugins/eventservice/service/test/unittest/common/sys_event_service_ohos_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 97 |
| 10 | `adapter/service/test/unittest/common/adapter_loglibrary_ability_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 11 | `adapter/service/test/unittest/common/hiview_sa_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 12 | `base/event_loop.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 13 | `base/event_publish/test/unittest/common/app_event_handler_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 14 | `base/event_publish/test/unittest/common/event_publish_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 15 | `base/event_publish/test/unittest/common/log_file_name_converter_test.cpp` | tree-sitter-cpp node `missing ;` at 78 site(s) | 78 |
| 16 | `base/event_raw/encoded/encoded_param.cpp` | tree-sitter-cpp node `missing identifier` at 4 site(s) | 4 |
| 17 | `base/event_raw/include/encoded/encoded_param.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 18 | `base/event_raw/include/encoded/raw_data_builder.h` | explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp | 10 |
| 19 | `base/event_raw/test/unittest/common/event_raw_base_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 20 | `base/event_raw/test/unittest/common/event_raw_encoded_and_decoded_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 70 |
| 21 | `base/event_report/test/unittest/common/event_report_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 22 | `base/event_store/include/sys_event_query.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 23 | `base/event_store/test/unittest/common/event_store_config_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 24 | `base/event_store/test/unittest/common/sys_event_backup_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 25 | `base/event_store/test/unittest/common/sys_event_dao_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 164 |
| 26 | `base/event_store/test/unittest/common/sys_event_database_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 27 | `base/event_store/test/unittest/common/sys_event_doc_lru_cache_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 28 | `base/event_store/test/unittest/common/sys_event_doc_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 29 | `base/event_store/test/unittest/common/sys_event_repeat_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 30 | `base/event_store/test/unittest/common/sys_event_sequence_mgr_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 31 | `base/event_store/test/unittest/common/sys_event_store_utility_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 32 | `base/include/event.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 33 | `base/include/plugin.h` | tree-sitter-cpp node `missing type_identifier` at 3 site(s) | 3 |
| 34 | `base/include/sys_event.h` | explicit template instantiation declarations (`extern template …`) not supported by tree-sitter-cpp | 8 |
| 35 | `base/logstore/test/unittest/common/log_store_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 36 | `base/running_status_logger/test/unittest/common/running_status_logger_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 37 | `base/test/unittest/common/dispatch_rule_parser_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 38 | `base/test/unittest/common/domain_json_parser_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 39 | `base/test/unittest/common/event_base_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 40 | `base/test/unittest/common/event_json_parser_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 41 | `base/test/unittest/common/event_loop_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 42 | `base/test/unittest/common/pipeline_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 43 | `base/test/unittest/common/plugin_factory_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 44 | `base/test/unittest/common/plugin_test.cpp` | tree-sitter-cpp node `missing ;` at 18 site(s) | 18 |
| 45 | `base/test/unittest/common/sys_event_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 100 |
| 46 | `base/test/unittest/common/version_config_parser_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 47 | `base/utility/test/unittest/common/adapter_utility_ohos_test.cpp` | tree-sitter-cpp node `missing ;` at 31 site(s) | 31 |
| 48 | `base/utility/test/unittest/common/base_utility_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 17 site(s) | 17 |
| 49 | `base/utility/test/unittest/common/bundle_util_test.cpp` | tree-sitter-cpp node `missing ;` at 14 site(s) | 14 |
| 50 | `base/utility/test/unittest/common/system_service_ohos_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 51 | `core/param_update/include/param_event_manager.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 52 | `core/test/unittest/common/event_dispatch_queue_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 53 | `core/test/unittest/common/hiview_platform_config_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 54 | `core/test/unittest/common/param_update_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 55 | `core/test/unittest/common/platform_config_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 56 | `core/test/unittest/common/plugin_bundle_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 57 | `core/test/unittest/common/plugin_platform_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 58 | `framework/native/unified_collection/collector/config/test/unittest/perf_collect_config_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 59 | `framework/native/unified_collection/collector/impl/cpu/device_client/collect_device_client.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 31 |
| 60 | `framework/native/unified_collection/collector/impl/memory/utils/test/unittest/memory_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 61 | `framework/native/unified_collection/collector/impl/trace/strategy/include/trace_handler.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 62 | `framework/native/unified_collection/collector/impl/trace/test/trace_impl_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 63 | `framework/native/unified_collection/collector/impl/trace/test/trace_strategy_ex_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 64 | `framework/native/unified_collection/collector/impl/trace/test/trace_strategy_test.cpp` | tree-sitter-cpp node `missing ;` at 38 site(s) | 38 |
| 65 | `framework/native/unified_collection/collector/impl/trace/test/trace_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 66 | `framework/native/unified_collection/collector/test/collect_device_client_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 67 | `framework/native/unified_collection/collector/test/common_util_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 68 | `framework/native/unified_collection/collector/test/cpu_calculator_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 69 | `framework/native/unified_collection/collector/test/process_status_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 70 | `framework/native/unified_collection/decorator/test/decorator_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 71 | `framework/native/unified_collection/graphic_memory/graphic_memory.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 72 | `framework/native/unified_collection/trace_manager/test/trace_manager_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 73 | `hiretrieval/frameworks/include/hiretrieval_base_def.h` | tree-sitter-cpp node `missing identifier` at 1 site(s) | 1 |
| 74 | `hiretrieval/frameworks/test/unittest/common/hiretrieval_dynamic_loader_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 75 | `hiretrieval/frameworks/test/unittest/common/hiretrieval_mgr_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 76 | `hiretrieval/interfaces/ets/ani/src/hiretrieval_ani.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 77 | `hiretrieval/interfaces/js/napi/src/hiretrieval_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 78 | `interfaces/ets/ani/loglibrary/src/loglibrary_ani.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 79 | `interfaces/inner_api/unified_collection/client/src/trace_collector_client_impl.cpp` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 80 | `interfaces/inner_api/unified_collection/client/trace_collector_client.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 81 | `interfaces/js/napi/src/napi_hiview_js.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 82 | `interfaces/js/napi/test/unittest/common/interface_js_napi_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 83 | `interfaces/js/napi/test/unittest/common/loglibrary_agent_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 84 | `plugins/event_store/event_export/test/unittest/common/event_export_config_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 85 | `plugins/event_store/event_export/test/unittest/common/event_export_db_mgr_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 86 | `plugins/event_store/event_export/test/unittest/common/event_export_mgr_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 87 | `plugins/event_store/event_export/test/unittest/common/event_export_write_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 88 | `plugins/event_store/event_export/test/unittest/common/trigger_export_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 89 | `plugins/event_validator/test/unittest/common/daily_controller_test.cpp` | tree-sitter-cpp node `missing ;` at 13 site(s) | 13 |
| 90 | `plugins/event_validator/test/unittest/common/event_validator_test.cpp` | tree-sitter-cpp node `missing ;` at 14 site(s) | 14 |
| 91 | `plugins/eventlogger/config/test/unittest/common/event_logger_config_test.cpp` | tree-sitter-cpp node `missing ;` at 23 site(s) | 23 |
| 92 | `plugins/eventlogger/log_catcher/summary_log_info_catcher.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 93 | `plugins/eventlogger/log_catcher/test/unittest/common/event_logger_catcher_test.cpp` | tree-sitter-cpp node `missing ;` at 18 site(s) | 18 |
| 94 | `plugins/eventlogger/test/unittest/common/event_logger_plugin_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 95 | `plugins/eventlogger/test/unittest/common/event_logger_test.cpp` | tree-sitter-cpp node `missing ;` at 102 site(s) | 102 |
| 96 | `plugins/faultlogger/interfaces/cj/faultlogger_ffi.h` | tree-sitter-cpp node `missing ::` at 1 site(s) | 1 |
| 97 | `plugins/faultlogger/interfaces/cpp/innerkits/test/common/moduletest/faultlogger_native_interface_test.cpp` | tree-sitter-cpp node `missing ;` at 12 site(s) | 12 |
| 98 | `plugins/faultlogger/interfaces/js/napi/napi_faultlogger.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 99 | `plugins/faultlogger/interfaces/js/test/unittest/cpp/faultlogger_test_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 100 | `plugins/faultlogger/service/bdfr_base/fault_file/faultlog_dump.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 39 |
| 101 | `plugins/faultlogger/service/bdfr_base/test/unittest/asan_unittest.cpp` | tree-sitter-cpp node `missing ;` at 11 site(s) | 11 |
| 102 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_bootscan_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 103 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_bundle_util_unittest.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 104 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cjerror_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 105 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cppcrash_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 167 |
| 106 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_database_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 65 |
| 107 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_event_factory_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 108 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_formatter_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 224 |
| 109 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_freeze_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 110 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_hilog_helper_test.cpp` | tree-sitter-cpp node `missing identifier` at 9 site(s) | 9 |
| 111 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_jserror_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 66 |
| 112 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_manager_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 113 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_sanitizer_test.cpp` | tree-sitter-cpp node `missing ;` at 13 site(s) | 13 |
| 114 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 16 site(s) | 16 |
| 115 | `plugins/faultlogger/service/bdfr_base/test/unittest/faultlogger_base_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 116 | `plugins/faultlogger/service/bdfr_base/test/unittest/freeze_json_generator_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 117 | `plugins/faultlogger/service/bdfr_base/test/unittest/log_analyzer_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 118 | `plugins/faultlogger/service/idl/include/ifaultlog_query_result.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 119 | `plugins/faultlogger/service/idl/include/ifaultlogger_service.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 120 | `plugins/faultlogger/test/common/unittest/extension_manager_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 121 | `plugins/faultlogger/test/common/unittest/faultlogger_client_unittest.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 122 | `plugins/faultlogger/test/common/unittest/faultlogger_unittest.cpp` | tree-sitter-cpp node `missing ;` at 16 site(s) | 16 |
| 123 | `plugins/faultlogger/test/common/unittest/rom_baseline.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 124 | `plugins/freeze_detector/test/moduletest/common/freeze_detector_plugin_module_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 125 | `plugins/freeze_detector/test/unittest/common/freeze_detector_getratio_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 126 | `plugins/freeze_detector/test/unittest/common/freeze_detector_ipcfull_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 127 | `plugins/freeze_detector/test/unittest/common/freeze_detector_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 128 | `plugins/freeze_detector/test/unittest/common/freeze_detector_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 103 |
| 129 | `plugins/freeze_detector/test/unittest/common/freeze_manager_unittest.cpp` | tree-sitter-cpp node `missing ;` at 22 site(s) | 22 |
| 130 | `plugins/performance/perfmonitor/common/event_builder/xperf_event_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 6 |
| 131 | `plugins/performance/perfmonitor/interfaces/inner_api/include/perf_model.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 132 | `plugins/performance/perfmonitor/src/perf_reporter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 133 | `plugins/performance/perfmonitor/src/scene_monitor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 134 | `plugins/performance/xperf_service/services/utils/time_util.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 135 | `plugins/privacy_controller/test/unittest/common/privacy_controller_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 136 | `plugins/reliability/bbox_detectors/bdfr_base/panic_report_recovery.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 137 | `plugins/reliability/bbox_detectors/bdfr_base/test/unittest/bbox_detector_base_unit_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 138 | `plugins/reliability/bbox_detectors/test/moduletest/bbox_detector_module_test.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 139 | `plugins/reliability/bbox_detectors/test/unittest/bbox_detector_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 140 | `plugins/reliability/leak_detectors/test/moduletest/leak_detector_module_test.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 141 | `plugins/reliability/leak_detectors/test/unittest/leak_detector_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 142 | `plugins/sysevent_source/test/unittest/common/event_server_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 143 | `plugins/unified_collector/test/unittest/common/cpu_storage_test.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 144 | `plugins/unified_collector/test/unittest/common/uc_state_observer_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 145 | `plugins/usage_event_report/fold/cache/include/fold_app_usage_db_helper.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 146 | `plugins/usage_event_report/test/unittest/fold_app_usage_test.cpp` | tree-sitter-cpp node `missing ;` at 33 site(s) | 33 |
| 147 | `plugins/usage_event_report/test/unittest/usage_event_report_cache_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 148 | `plugins/usage_event_report/test/unittest/usage_event_report_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 149 | `plugins/usage_event_report/usage_event_report.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 150 | `test/moduletest/common/hiview_plugin_platform_module_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 151 | `test/unittest/common/holistic_platform_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 152 | `test/unittest/unified_collection/client/cpu_collector_client_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 153 | `test/unittest/unified_collection/client/memory_collector_client_test.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 154 | `test/unittest/unified_collection/client/trace_collector_client_test.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 155 | `test/unittest/unified_collection/utility/cpu_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 156 | `test/unittest/unified_collection/utility/gpu_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 157 | `test/unittest/unified_collection/utility/graphic_memory_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 158 | `test/unittest/unified_collection/utility/hilog_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 159 | `test/unittest/unified_collection/utility/io_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 160 | `test/unittest/unified_collection/utility/memory_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 161 | `test/unittest/unified_collection/utility/perf_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 162 | `test/unittest/unified_collection/utility/thermal_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 163 | `test/unittest/unified_collection/utility/trace_collector_test.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 164 | `test/unittest/xpower_event/xpower_event_test.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 165 | `test/unittest/xpower_event/xpower_jsevent_test.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 166 | `utility/common_utils/test/unittest/common/utility_common_utils_test.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 167 | `utility/smart_parser/test/moduletest/common/smart_parser_module_test.cpp` | tree-sitter-cpp node `missing ;` at 24 site(s) | 24 |
| 168 | `utility/test/unittest/cpp_crash_unittest.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 169 | `utility/test/unittest/hw_watchdog_unittest.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 170 | `utility/test/unittest/iom3_exception_unittest.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 171 | `utility/test/unittest/lpm3_exception_unittest.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 172 | `utility/test/unittest/panic_unittest.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 173 | `utility/test/unittest/rgm_manager_sysfreeze_unittest.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 174 | `utility/test/unittest/syswarning_unittest.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |

### Per-file details

#### `adapter/plugins/eventservice/service/idl/dfx/include/listener_status_util.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 91 | `missing type_identifier` | `` |
| 12 | 43 | `missing type_identifier` | `` |

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

#### `adapter/plugins/eventservice/service/test/unittest/common/data_share_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 83 | `missing ;` | `` |
| 67 | 81 | `missing ;` | `` |
| 94 | 82 | `missing ;` | `` |
| 117 | 82 | `missing ;` | `` |
| 146 | 82 | `missing ;` | `` |
| 153 | 98 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/event_checker_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 12 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 84 | `missing ;` | `` |
| 28 | 84 | `missing ;` | `` |
| 39 | 84 | `missing ;` | `` |
| 50 | 84 | `missing ;` | `` |
| 61 | 84 | `missing ;` | `` |
| 72 | 84 | `missing ;` | `` |
| 83 | 84 | `missing ;` | `` |
| 94 | 84 | `missing ;` | `` |
| 105 | 84 | `missing ;` | `` |
| 116 | 84 | `missing ;` | `` |
| 127 | 84 | `missing ;` | `` |
| 138 | 84 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/event_query_wrapper_builder_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 108 | `missing ;` | `` |
| 30 | 108 | `missing ;` | `` |
| 40 | 108 | `missing ;` | `` |
| 51 | 108 | `missing ;` | `` |
| 62 | 108 | `missing ;` | `` |
| 73 | 108 | `missing ;` | `` |
| 84 | 108 | `missing ;` | `` |
| 95 | 108 | `missing ;` | `` |
| 106 | 108 | `missing ;` | `` |
| 122 | 108 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/event_service_adapter_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 75 | 92 | `missing ;` | `` |
| 88 | 92 | `missing ;` | `` |
| 106 | 92 | `missing ;` | `` |
| 139 | 101 | `missing ;` | `` |
| 167 | 106 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/listener_status_monitor_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 17 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 102 | `missing ;` | `` |
| 65 | 102 | `missing ;` | `` |
| 89 | 102 | `missing ;` | `` |
| 109 | 102 | `missing ;` | `` |
| 129 | 102 | `missing ;` | `` |
| 149 | 102 | `missing ;` | `` |
| 169 | 102 | `missing ;` | `` |
| 190 | 102 | `missing ;` | `` |
| 213 | 102 | `missing ;` | `` |
| 227 | 102 | `missing ;` | `` |
| 241 | 102 | `missing ;` | `` |
| 257 | 102 | `missing ;` | `` |
| 273 | 102 | `missing ;` | `` |
| 289 | 102 | `missing ;` | `` |
| 315 | 102 | `missing ;` | `` |
| 346 | 102 | `missing ;` | `` |
| 366 | 102 | `missing ;` | `` |

#### `adapter/plugins/eventservice/service/test/unittest/common/sys_event_service_ohos_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 98 | 98 | `missing ;` | `` |
| 112 | 94 | `missing ;` | `` |
| 131 | 94 | `missing ;` | `` |
| 145 | 104 | `missing ;` | `` |
| 174 | 93 | `missing ;` | `` |
| 178 | 102 | `ERROR` | `=` |
| 179 | 28 | `ERROR` | `}] } })~ "` |
| 179 | 39 | `ERROR` | `"` |
| 185 | 93 | `missing ;` | `` |
| 189 | 103 | `ERROR` | `=` |
| 190 | 28 | `ERROR` | `}` |
| 190 | 31 | `missing identifier` | `` |
| 190 | 32 | `ERROR` | `"param":` |
| 190 | 50 | `ERROR` | `"op":` |
| 190 | 69 | `ERROR` | `: 1201` |
| 190 | 77 | `ERROR` | `] } })~ "` |
| 190 | 87 | `ERROR` | `"` |
| 196 | 93 | `missing ;` | `` |
| 200 | 71 | `ERROR` | `":[{"` |
| 201 | 1 | `ERROR` | `{` |
| … | … | … | *(77 more)* |

#### `adapter/service/test/unittest/common/adapter_loglibrary_ability_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 105 | `missing ;` | `` |
| 52 | 105 | `missing ;` | `` |
| 61 | 105 | `missing ;` | `` |
| 71 | 105 | `missing ;` | `` |
| 80 | 105 | `missing ;` | `` |
| 90 | 105 | `missing ;` | `` |
| 99 | 107 | `missing ;` | `` |
| 109 | 107 | `missing ;` | `` |
| 118 | 88 | `missing ;` | `` |

#### `adapter/service/test/unittest/common/hiview_sa_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 74 | `missing ;` | `` |
| 49 | 74 | `missing ;` | `` |
| 67 | 74 | `missing ;` | `` |
| 88 | 74 | `missing ;` | `` |

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

#### `base/event_publish/test/unittest/common/app_event_handler_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 74 | `missing ;` | `` |

#### `base/event_publish/test/unittest/common/event_publish_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 68 | `missing ;` | `` |

#### `base/event_publish/test/unittest/common/log_file_name_converter_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 78 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 84 | `missing ;` | `` |
| 32 | 84 | `missing ;` | `` |
| 43 | 84 | `missing ;` | `` |
| 54 | 92 | `missing ;` | `` |
| 65 | 92 | `missing ;` | `` |
| 76 | 92 | `missing ;` | `` |
| 87 | 87 | `missing ;` | `` |
| 98 | 87 | `missing ;` | `` |
| 109 | 87 | `missing ;` | `` |
| 120 | 92 | `missing ;` | `` |
| 131 | 92 | `missing ;` | `` |
| 142 | 92 | `missing ;` | `` |
| 155 | 87 | `missing ;` | `` |
| 166 | 87 | `missing ;` | `` |
| 177 | 87 | `missing ;` | `` |
| 188 | 87 | `missing ;` | `` |
| 199 | 87 | `missing ;` | `` |
| 210 | 87 | `missing ;` | `` |
| 221 | 85 | `missing ;` | `` |
| 232 | 85 | `missing ;` | `` |
| … | … | … | *(58 more)* |

#### `base/event_raw/encoded/encoded_param.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 51 | `missing identifier` | `` |
| 10 | 50 | `missing identifier` | `` |
| 11 | 53 | `missing identifier` | `` |
| 83 | 146 | `missing identifier` | `` |

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

#### `base/event_raw/test/unittest/common/event_raw_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 81 | `missing ;` | `` |
| 40 | 81 | `missing ;` | `` |
| 51 | 96 | `missing ;` | `` |
| 58 | 84 | `missing ;` | `` |
| 73 | 79 | `missing ;` | `` |
| 91 | 79 | `missing ;` | `` |
| 107 | 79 | `missing ;` | `` |

#### `base/event_raw/test/unittest/common/event_raw_encoded_and_decoded_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 87 | `missing ;` | `` |
| 63 | 87 | `missing ;` | `` |
| 95 | 87 | `missing ;` | `` |
| 125 | 87 | `missing ;` | `` |
| 157 | 87 | `missing ;` | `` |
| 187 | 87 | `missing ;` | `` |
| 219 | 87 | `missing ;` | `` |
| 249 | 87 | `missing ;` | `` |
| 281 | 99 | `missing ;` | `` |
| 299 | 99 | `missing ;` | `` |
| 308 | 99 | `missing ;` | `` |
| 311 | 8 | `ERROR` | `: 1` |
| 311 | 62 | `ERROR` | `: 1751` |
| 311 | 77 | `ERROR` | `: 1751` |
| 311 | 92 | `ERROR` | `: 0` |
| 312 | 1 | `ERROR` | `"id_":` |
| 312 | 46 | `ERROR` | `: 3` |
| 313 | 1 | `ERROR` | `"spanid_":` |
| 313 | 18 | `ERROR` | `"pspanid_":` |
| 313 | 36 | `ERROR` | `"key1":` |
| … | … | … | *(50 more)* |

#### `base/event_report/test/unittest/common/event_report_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 90 | `missing ;` | `` |
| 56 | 90 | `missing ;` | `` |
| 64 | 90 | `missing ;` | `` |
| 72 | 90 | `missing ;` | `` |
| 80 | 90 | `missing ;` | `` |
| 99 | 86 | `missing ;` | `` |

#### `base/event_store/include/sys_event_query.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 89 | 48 | `ERROR` | `*` |
| 271 | 59 | `missing type_identifier` | `` |

#### `base/event_store/test/unittest/common/event_store_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 76 | `missing ;` | `` |
| 45 | 76 | `missing ;` | `` |
| 55 | 76 | `missing ;` | `` |
| 65 | 76 | `missing ;` | `` |
| 75 | 76 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_backup_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 45 | 72 | `missing ;` | `` |
| 52 | 72 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_dao_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 89 | `missing ;` | `` |
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
| … | … | … | *(144 more)* |

#### `base/event_store/test/unittest/common/sys_event_database_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 89 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_doc_lru_cache_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 82 | `missing ;` | `` |
| 47 | 82 | `missing ;` | `` |
| 57 | 82 | `missing ;` | `` |
| 69 | 82 | `missing ;` | `` |
| 79 | 82 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_doc_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 82 | `missing ;` | `` |
| 67 | 82 | `missing ;` | `` |
| 87 | 82 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_repeat_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 90 | `missing ;` | `` |
| 61 | 90 | `missing ;` | `` |
| 100 | 90 | `missing ;` | `` |
| 125 | 90 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_sequence_mgr_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 64 | 98 | `missing ;` | `` |
| 74 | 98 | `missing ;` | `` |
| 84 | 98 | `missing ;` | `` |

#### `base/event_store/test/unittest/common/sys_event_store_utility_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 104 | 142 | `ERROR` | `,` |
| 138 | 61 | `ERROR` | `0800 ","` |
| 139 | 36 | `ERROR` | `12254568215815823881` |
| 164 | 100 | `missing ;` | `` |
| 175 | 100 | `missing ;` | `` |
| 206 | 100 | `missing ;` | `` |
| 232 | 100 | `missing ;` | `` |
| 256 | 100 | `missing ;` | `` |
| 263 | 100 | `missing ;` | `` |
| 271 | 100 | `missing ;` | `` |
| 282 | 100 | `missing ;` | `` |
| 311 | 100 | `missing ;` | `` |
| 324 | 100 | `missing ;` | `` |

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

#### `base/logstore/test/unittest/common/log_store_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 51 | 84 | `missing ;` | `` |
| 82 | 84 | `missing ;` | `` |

#### `base/running_status_logger/test/unittest/common/running_status_logger_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 118 | 99 | `missing ;` | `` |
| 131 | 99 | `missing ;` | `` |
| 144 | 99 | `missing ;` | `` |
| 157 | 99 | `missing ;` | `` |
| 170 | 99 | `missing ;` | `` |
| 183 | 99 | `missing ;` | `` |
| 196 | 99 | `missing ;` | `` |
| 210 | 99 | `missing ;` | `` |
| 224 | 99 | `missing ;` | `` |

#### `base/test/unittest/common/dispatch_rule_parser_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 85 | 92 | `missing ;` | `` |
| 98 | 92 | `missing ;` | `` |
| 110 | 92 | `missing ;` | `` |
| 118 | 92 | `missing ;` | `` |
| 130 | 92 | `missing ;` | `` |
| 142 | 92 | `missing ;` | `` |
| 154 | 92 | `missing ;` | `` |

#### `base/test/unittest/common/domain_json_parser_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 92 | `missing ;` | `` |

#### `base/test/unittest/common/event_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 78 | `missing ;` | `` |
| 59 | 80 | `missing ;` | `` |
| 71 | 74 | `missing ;` | `` |
| 102 | 82 | `missing ;` | `` |

#### `base/test/unittest/common/event_json_parser_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 94 | `missing ;` | `` |
| 59 | 90 | `missing ;` | `` |
| 77 | 90 | `missing ;` | `` |
| 131 | 90 | `missing ;` | `` |

#### `base/test/unittest/common/event_loop_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 48 | `ERROR` | `PRId64` |
| 50 | 49 | `ERROR` | `PRId64` |
| 134 | 65 | `missing ;` | `` |
| 146 | 74 | `missing ;` | `` |
| 179 | 74 | `missing ;` | `` |
| 224 | 74 | `missing ;` | `` |
| 236 | 74 | `missing ;` | `` |
| 249 | 74 | `missing ;` | `` |
| 264 | 74 | `missing ;` | `` |
| 287 | 70 | `missing ;` | `` |
| 299 | 37 | `ERROR` | `PRIu64` |
| 300 | 35 | `ERROR` | `PRIu64` |
| 301 | 34 | `ERROR` | `PRIu64` |
| 306 | 72 | `missing ;` | `` |
| 324 | 78 | `missing ;` | `` |
| 340 | 62 | `missing ;` | `` |

#### `base/test/unittest/common/pipeline_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 78 | 69 | `missing ;` | `` |
| 100 | 72 | `missing ;` | `` |
| 107 | 72 | `missing ;` | `` |
| 114 | 72 | `missing ;` | `` |
| 121 | 72 | `missing ;` | `` |
| 128 | 72 | `missing ;` | `` |

#### `base/test/unittest/common/plugin_factory_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 78 | `missing ;` | `` |
| 19 | 78 | `missing ;` | `` |

#### `base/test/unittest/common/plugin_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 72 | `missing ;` | `` |
| 97 | 72 | `missing ;` | `` |
| 131 | 72 | `missing ;` | `` |
| 141 | 82 | `missing ;` | `` |
| 155 | 79 | `missing ;` | `` |
| 192 | 79 | `missing ;` | `` |
| 200 | 79 | `missing ;` | `` |
| 208 | 79 | `missing ;` | `` |
| 216 | 79 | `missing ;` | `` |
| 226 | 79 | `missing ;` | `` |
| 234 | 79 | `missing ;` | `` |
| 242 | 79 | `missing ;` | `` |
| 250 | 79 | `missing ;` | `` |
| 258 | 79 | `missing ;` | `` |
| 266 | 79 | `missing ;` | `` |
| 274 | 79 | `missing ;` | `` |
| 282 | 77 | `missing ;` | `` |
| 299 | 77 | `missing ;` | `` |

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
| 76 | 80 | `missing ;` | `` |
| 83 | 113 | `ERROR` | `\` |
| 83 | 116 | `ERROR` | `\ d` |
| 84 | 1 | `ERROR` | `R` |
| 84 | 65 | `ERROR` | `\` |
| 84 | 70 | `ERROR` | `","` |
| 93 | 85 | `missing ;` | `` |
| 101 | 113 | `ERROR` | `\` |
| 101 | 116 | `ERROR` | `\ d` |
| 102 | 1 | `ERROR` | `R` |
| 102 | 65 | `ERROR` | `\` |
| 102 | 70 | `ERROR` | `","` |
| 111 | 86 | `missing ;` | `` |
| … | … | … | *(80 more)* |

#### `base/test/unittest/common/version_config_parser_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 90 | `missing ;` | `` |
| 38 | 90 | `missing ;` | `` |
| 55 | 90 | `missing ;` | `` |
| 65 | 90 | `missing ;` | `` |
| 77 | 90 | `missing ;` | `` |
| 89 | 92 | `missing ;` | `` |
| 114 | 93 | `missing ;` | `` |

#### `base/utility/test/unittest/common/adapter_utility_ohos_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 31 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 98 | 93 | `missing ;` | `` |
| 106 | 93 | `missing ;` | `` |
| 115 | 90 | `missing ;` | `` |
| 125 | 90 | `missing ;` | `` |
| 134 | 90 | `missing ;` | `` |
| 146 | 90 | `missing ;` | `` |
| 152 | 90 | `missing ;` | `` |
| 162 | 90 | `missing ;` | `` |
| 171 | 90 | `missing ;` | `` |
| 182 | 90 | `missing ;` | `` |
| 190 | 90 | `missing ;` | `` |
| 204 | 90 | `missing ;` | `` |
| 219 | 90 | `missing ;` | `` |
| 242 | 90 | `missing ;` | `` |
| 255 | 90 | `missing ;` | `` |
| 271 | 90 | `missing ;` | `` |
| 285 | 90 | `missing ;` | `` |
| 303 | 96 | `missing ;` | `` |
| 329 | 87 | `missing ;` | `` |
| 363 | 87 | `missing ;` | `` |
| … | … | … | *(11 more)* |

#### `base/utility/test/unittest/common/base_utility_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 17 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 109 | 90 | `missing ;` | `` |
| 121 | 90 | `missing ;` | `` |
| 137 | 90 | `missing ;` | `` |
| 148 | 90 | `missing ;` | `` |
| 170 | 90 | `missing ;` | `` |
| 184 | 90 | `missing ;` | `` |
| 198 | 90 | `missing ;` | `` |
| 207 | 90 | `missing ;` | `` |
| 223 | 90 | `missing ;` | `` |
| 235 | 90 | `missing ;` | `` |
| 253 | 90 | `missing ;` | `` |
| 264 | 90 | `missing ;` | `` |
| 285 | 90 | `missing ;` | `` |
| 302 | 90 | `missing ;` | `` |
| 308 | 90 | `missing ;` | `` |
| 317 | 90 | `missing ;` | `` |
| 329 | 90 | `missing ;` | `` |

#### `base/utility/test/unittest/common/bundle_util_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 14 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 80 | `missing ;` | `` |
| 46 | 80 | `missing ;` | `` |
| 54 | 80 | `missing ;` | `` |
| 62 | 80 | `missing ;` | `` |
| 72 | 80 | `missing ;` | `` |
| 80 | 80 | `missing ;` | `` |
| 89 | 80 | `missing ;` | `` |
| 96 | 80 | `missing ;` | `` |
| 103 | 80 | `missing ;` | `` |
| 110 | 80 | `missing ;` | `` |
| 117 | 80 | `missing ;` | `` |
| 124 | 80 | `missing ;` | `` |
| 131 | 80 | `missing ;` | `` |
| 146 | 80 | `missing ;` | `` |

#### `base/utility/test/unittest/common/system_service_ohos_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 90 | `missing ;` | `` |
| 26 | 90 | `missing ;` | `` |
| 41 | 90 | `missing ;` | `` |
| 67 | 90 | `missing ;` | `` |

#### `core/param_update/include/param_event_manager.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 37 | `missing ;` | `` |

#### `core/test/unittest/common/event_dispatch_queue_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 86 | `missing ;` | `` |
| 67 | 86 | `missing ;` | `` |
| 87 | 86 | `missing ;` | `` |

#### `core/test/unittest/common/hiview_platform_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 79 | `missing ;` | `` |
| 37 | 79 | `missing ;` | `` |

#### `core/test/unittest/common/param_update_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 69 | 66 | `missing ;` | `` |

#### `core/test/unittest/common/platform_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 73 | `missing ;` | `` |
| 43 | 73 | `missing ;` | `` |
| 67 | 73 | `missing ;` | `` |
| 74 | 73 | `missing ;` | `` |

#### `core/test/unittest/common/plugin_bundle_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 72 | `missing ;` | `` |

#### `core/test/unittest/common/plugin_platform_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 86 | `missing ;` | `` |
| 42 | 84 | `missing ;` | `` |
| 57 | 75 | `missing ;` | `` |
| 70 | 91 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/config/test/unittest/perf_collect_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 78 | `missing ;` | `` |
| 49 | 78 | `missing ;` | `` |
| 58 | 78 | `missing ;` | `` |

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

#### `framework/native/unified_collection/collector/impl/memory/utils/test/unittest/memory_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 68 | 66 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/impl/trace/strategy/include/trace_handler.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 100 | `missing type_identifier` | `` |
| 35 | 92 | `missing type_identifier` | `` |

#### `framework/native/unified_collection/collector/impl/trace/test/trace_impl_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 62 | `missing ;` | `` |
| 55 | 62 | `missing ;` | `` |
| 78 | 62 | `missing ;` | `` |
| 101 | 62 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/impl/trace/test/trace_strategy_ex_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 74 | `missing ;` | `` |
| 72 | 74 | `missing ;` | `` |
| 104 | 74 | `missing ;` | `` |
| 148 | 74 | `missing ;` | `` |
| 179 | 75 | `missing ;` | `` |
| 214 | 75 | `missing ;` | `` |
| 238 | 75 | `missing ;` | `` |
| 256 | 75 | `missing ;` | `` |
| 275 | 75 | `missing ;` | `` |
| 308 | 75 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/impl/trace/test/trace_strategy_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 38 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 70 | `missing ;` | `` |
| 73 | 70 | `missing ;` | `` |
| 89 | 70 | `missing ;` | `` |
| 109 | 70 | `missing ;` | `` |
| 145 | 70 | `missing ;` | `` |
| 167 | 70 | `missing ;` | `` |
| 201 | 70 | `missing ;` | `` |
| 223 | 70 | `missing ;` | `` |
| 237 | 70 | `missing ;` | `` |
| 276 | 70 | `missing ;` | `` |
| 288 | 70 | `missing ;` | `` |
| 302 | 70 | `missing ;` | `` |
| 320 | 70 | `missing ;` | `` |
| 342 | 70 | `missing ;` | `` |
| 361 | 70 | `missing ;` | `` |
| 397 | 70 | `missing ;` | `` |
| 427 | 70 | `missing ;` | `` |
| 479 | 70 | `missing ;` | `` |
| 496 | 70 | `missing ;` | `` |
| 523 | 70 | `missing ;` | `` |
| … | … | … | *(18 more)* |

#### `framework/native/unified_collection/collector/impl/trace/test/trace_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 64 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/test/collect_device_client_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 82 | `missing ;` | `` |
| 32 | 82 | `missing ;` | `` |
| 42 | 82 | `missing ;` | `` |
| 52 | 82 | `missing ;` | `` |
| 62 | 82 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/test/common_util_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 64 | `missing ;` | `` |
| 34 | 64 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/test/cpu_calculator_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 70 | `missing ;` | `` |
| 29 | 70 | `missing ;` | `` |
| 41 | 70 | `missing ;` | `` |

#### `framework/native/unified_collection/collector/test/process_status_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 70 | `missing ;` | `` |
| 28 | 70 | `missing ;` | `` |
| 37 | 70 | `missing ;` | `` |

#### `framework/native/unified_collection/decorator/test/decorator_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 62 | `missing ;` | `` |

#### `framework/native/unified_collection/graphic_memory/graphic_memory.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 74 | `ERROR` | `PRId64` |
| 60 | 49 | `ERROR` | `PRId64` |

#### `framework/native/unified_collection/trace_manager/test/trace_manager_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 1 | `ERROR` | `.args` |
| 65 | 1 | `ERROR` | `.args` |
| 73 | 1 | `ERROR` | `.args` |
| 130 | 68 | `missing ;` | `` |
| 160 | 68 | `missing ;` | `` |
| 180 | 68 | `missing ;` | `` |
| 221 | 68 | `missing ;` | `` |
| 233 | 68 | `missing ;` | `` |
| 295 | 68 | `missing ;` | `` |
| 315 | 68 | `missing ;` | `` |
| 401 | 68 | `missing ;` | `` |
| 515 | 68 | `missing ;` | `` |
| 584 | 68 | `missing ;` | `` |
| 643 | 68 | `missing ;` | `` |
| 672 | 68 | `missing ;` | `` |
| 729 | 68 | `missing ;` | `` |
| 747 | 68 | `missing ;` | `` |
| 760 | 68 | `missing ;` | `` |
| 771 | 68 | `missing ;` | `` |
| 783 | 68 | `missing ;` | `` |
| … | … | … | *(23 more)* |

#### `hiretrieval/frameworks/include/hiretrieval_base_def.h`

**Summary:** tree-sitter-cpp node `missing identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 2 | `missing identifier` | `` |

#### `hiretrieval/frameworks/test/unittest/common/hiretrieval_dynamic_loader_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 116 | `missing ;` | `` |

#### `hiretrieval/frameworks/test/unittest/common/hiretrieval_mgr_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 53 | 96 | `missing ;` | `` |
| 74 | 96 | `missing ;` | `` |
| 103 | 96 | `missing ;` | `` |
| 128 | 96 | `missing ;` | `` |
| 153 | 96 | `missing ;` | `` |

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

#### `interfaces/js/napi/test/unittest/common/interface_js_napi_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 65 | `missing ;` | `` |

#### `interfaces/js/napi/test/unittest/common/loglibrary_agent_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 94 | `missing ;` | `` |
| 37 | 94 | `missing ;` | `` |
| 45 | 94 | `missing ;` | `` |
| 53 | 96 | `missing ;` | `` |

#### `plugins/event_store/event_export/test/unittest/common/event_export_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 57 | 104 | `missing ;` | `` |
| 78 | 104 | `missing ;` | `` |
| 97 | 104 | `missing ;` | `` |
| 106 | 104 | `missing ;` | `` |
| 118 | 104 | `missing ;` | `` |
| 155 | 103 | `missing ;` | `` |
| 170 | 103 | `missing ;` | `` |

#### `plugins/event_store/event_export/test/unittest/common/event_export_db_mgr_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 92 | `missing ;` | `` |
| 88 | 92 | `missing ;` | `` |
| 109 | 92 | `missing ;` | `` |
| 128 | 92 | `missing ;` | `` |

#### `plugins/event_store/event_export/test/unittest/common/event_export_mgr_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 44 | 88 | `missing ;` | `` |
| 61 | 87 | `missing ;` | `` |
| 77 | 88 | `missing ;` | `` |
| 89 | 88 | `missing ;` | `` |
| 101 | 89 | `missing ;` | `` |
| 134 | 86 | `missing ;` | `` |

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
| 85 | 92 | `missing ;` | `` |
| 98 | 92 | `missing ;` | `` |
| 115 | 92 | `missing ;` | `` |
| 128 | 92 | `missing ;` | `` |
| 144 | 92 | `missing ;` | `` |

#### `plugins/event_store/event_export/test/unittest/common/trigger_export_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 90 | `missing ;` | `` |
| 72 | 90 | `missing ;` | `` |
| 84 | 90 | `missing ;` | `` |
| 103 | 92 | `missing ;` | `` |

#### `plugins/event_validator/test/unittest/common/daily_controller_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 74 | `missing ;` | `` |
| 73 | 74 | `missing ;` | `` |
| 84 | 74 | `missing ;` | `` |
| 95 | 74 | `missing ;` | `` |
| 106 | 74 | `missing ;` | `` |
| 117 | 74 | `missing ;` | `` |
| 128 | 74 | `missing ;` | `` |
| 139 | 74 | `missing ;` | `` |
| 150 | 74 | `missing ;` | `` |
| 159 | 74 | `missing ;` | `` |
| 168 | 74 | `missing ;` | `` |
| 177 | 74 | `missing ;` | `` |
| 186 | 74 | `missing ;` | `` |

#### `plugins/event_validator/test/unittest/common/event_validator_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 14 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 72 | `missing ;` | `` |
| 65 | 72 | `missing ;` | `` |
| 74 | 72 | `missing ;` | `` |
| 83 | 72 | `missing ;` | `` |
| 92 | 72 | `missing ;` | `` |
| 102 | 72 | `missing ;` | `` |
| 111 | 72 | `missing ;` | `` |
| 123 | 72 | `missing ;` | `` |
| 149 | 72 | `missing ;` | `` |
| 158 | 72 | `missing ;` | `` |
| 167 | 72 | `missing ;` | `` |
| 176 | 72 | `missing ;` | `` |
| 185 | 72 | `missing ;` | `` |
| 199 | 72 | `missing ;` | `` |

#### `plugins/eventlogger/config/test/unittest/common/event_logger_config_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 23 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 78 | `missing ;` | `` |
| 34 | 78 | `missing ;` | `` |
| 45 | 78 | `missing ;` | `` |
| 56 | 78 | `missing ;` | `` |
| 66 | 68 | `missing ;` | `` |
| 77 | 68 | `missing ;` | `` |
| 88 | 68 | `missing ;` | `` |
| 99 | 68 | `missing ;` | `` |
| 109 | 68 | `missing ;` | `` |
| 120 | 68 | `missing ;` | `` |
| 131 | 68 | `missing ;` | `` |
| 142 | 68 | `missing ;` | `` |
| 152 | 68 | `missing ;` | `` |
| 162 | 68 | `missing ;` | `` |
| 172 | 70 | `missing ;` | `` |
| 183 | 70 | `missing ;` | `` |
| 193 | 70 | `missing ;` | `` |
| 203 | 72 | `missing ;` | `` |
| 214 | 72 | `missing ;` | `` |
| 225 | 74 | `missing ;` | `` |
| … | … | … | *(3 more)* |

#### `plugins/eventlogger/log_catcher/summary_log_info_catcher.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 127 | `missing identifier` | `` |
| 58 | 45 | `ERROR` | `, int32_t` |
| 60 | 161 | `ERROR` | `,` |
| 63 | 141 | `ERROR` | `,` |
| 66 | 128 | `ERROR` | `,` |

#### `plugins/eventlogger/log_catcher/test/unittest/common/event_logger_catcher_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 78 | `missing ;` | `` |
| 43 | 71 | `missing ;` | `` |
| 68 | 71 | `missing ;` | `` |
| 89 | 71 | `missing ;` | `` |
| 119 | 71 | `missing ;` | `` |
| 171 | 75 | `missing ;` | `` |
| 195 | 75 | `missing ;` | `` |
| 214 | 75 | `missing ;` | `` |
| 222 | 75 | `missing ;` | `` |
| 230 | 78 | `missing ;` | `` |
| 269 | 78 | `missing ;` | `` |
| 314 | 78 | `missing ;` | `` |
| 325 | 78 | `missing ;` | `` |
| 348 | 78 | `missing ;` | `` |
| 362 | 78 | `missing ;` | `` |
| 375 | 78 | `missing ;` | `` |
| 391 | 81 | `missing ;` | `` |
| 409 | 84 | `missing ;` | `` |

#### `plugins/eventlogger/test/unittest/common/event_logger_plugin_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 78 | `missing ;` | `` |
| 46 | 78 | `missing ;` | `` |
| 59 | 78 | `missing ;` | `` |
| 73 | 78 | `missing ;` | `` |
| 89 | 78 | `missing ;` | `` |
| 105 | 78 | `missing ;` | `` |

#### `plugins/eventlogger/test/unittest/common/event_logger_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 102 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 75 | `missing ;` | `` |
| 39 | 75 | `missing ;` | `` |
| 52 | 75 | `missing ;` | `` |
| 77 | 75 | `missing ;` | `` |
| 114 | 93 | `missing ;` | `` |
| 134 | 92 | `missing ;` | `` |
| 144 | 83 | `missing ;` | `` |
| 165 | 88 | `missing ;` | `` |
| 177 | 87 | `missing ;` | `` |
| 208 | 87 | `missing ;` | `` |
| 229 | 87 | `missing ;` | `` |
| 244 | 80 | `missing ;` | `` |
| 260 | 88 | `missing ;` | `` |
| 285 | 83 | `missing ;` | `` |
| 312 | 88 | `missing ;` | `` |
| 330 | 76 | `missing ;` | `` |
| 344 | 85 | `missing ;` | `` |
| 354 | 86 | `missing ;` | `` |
| 370 | 83 | `missing ;` | `` |
| 377 | 77 | `missing ;` | `` |
| … | … | … | *(82 more)* |

#### `plugins/faultlogger/interfaces/cj/faultlogger_ffi.h`

**Summary:** tree-sitter-cpp node `missing ::` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 28 | `missing ::` | `` |

#### `plugins/faultlogger/interfaces/cpp/innerkits/test/common/moduletest/faultlogger_native_interface_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 12 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 97 | `missing ;` | `` |
| 29 | 97 | `missing ;` | `` |
| 40 | 97 | `missing ;` | `` |
| 63 | 103 | `missing ;` | `` |
| 108 | 103 | `missing ;` | `` |
| 119 | 103 | `missing ;` | `` |
| 131 | 103 | `missing ;` | `` |
| 154 | 103 | `missing ;` | `` |
| 174 | 98 | `missing ;` | `` |
| 201 | 110 | `missing ;` | `` |
| 210 | 106 | `missing ;` | `` |
| 217 | 106 | `missing ;` | `` |

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

#### `plugins/faultlogger/interfaces/js/test/unittest/cpp/faultlogger_test_napi.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 8 | `ERROR` | `napi_value` |
| 31 | 20 | `ERROR` | `demoModule` |

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

#### `plugins/faultlogger/service/bdfr_base/test/unittest/asan_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 11 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 85 | `missing ;` | `` |
| 143 | 85 | `missing ;` | `` |
| 155 | 85 | `missing ;` | `` |
| 166 | 85 | `missing ;` | `` |
| 192 | 85 | `missing ;` | `` |
| 216 | 85 | `missing ;` | `` |
| 243 | 85 | `missing ;` | `` |
| 267 | 75 | `missing ;` | `` |
| 289 | 75 | `missing ;` | `` |
| 311 | 73 | `missing ;` | `` |
| 340 | 72 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_bootscan_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 86 | `missing ;` | `` |
| 16 | 86 | `missing ;` | `` |
| 28 | 85 | `missing ;` | `` |
| 36 | 87 | `missing ;` | `` |
| 55 | 85 | `missing ;` | `` |
| 78 | 85 | `missing ;` | `` |
| 97 | 85 | `missing ;` | `` |
| 134 | 85 | `missing ;` | `` |
| 174 | 84 | `missing ;` | `` |
| 188 | 84 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_bundle_util_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 99 | `missing ;` | `` |
| 21 | 89 | `missing ;` | `` |
| 30 | 100 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_cjerror_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 84 | `missing ;` | `` |
| 63 | 96 | `missing ;` | `` |
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
| … | … | … | *(147 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_database_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 90 | `missing ;` | `` |
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
| … | … | … | *(45 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_event_factory_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 101 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_formatter_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 101 | `missing ;` | `` |
| 32 | 98 | `missing ;` | `` |
| 46 | 101 | `missing ;` | `` |
| 49 | 6 | `ERROR` | `: 1234` |
| 51 | 1 | `ERROR` | `"PNAME":` |
| 52 | 1 | `ERROR` | `"REASON":` |
| 54 | 1 | `ERROR` | `"thread_name":` |
| 55 | 1 | `ERROR` | `"tid":` |
| 56 | 9 | `ERROR` | `:` |
| 57 | 2 | `ERROR` | `"pc":` |
| 57 | 21 | `ERROR` | `"symbol":` |
| 57 | 53 | `ERROR` | `: 100` |
| 57 | 61 | `ERROR` | `"file":` |
| 57 | 86 | `ERROR` | `"buildId":` |
| 59 | 1 | `ERROR` | `}` |
| 60 | 20 | `ERROR` | `:` |
| 61 | 2 | `ERROR` | `"thread_name":` |
| 61 | 34 | `ERROR` | `: 1235` |
| 61 | 51 | `ERROR` | `:[]` |
| 63 | 1 | `ERROR` | `})~ "` |
| … | … | … | *(204 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_freeze_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 94 | `missing ;` | `` |
| 32 | 85 | `missing ;` | `` |
| 47 | 79 | `missing ;` | `` |
| 62 | 79 | `missing ;` | `` |
| 77 | 89 | `missing ;` | `` |
| 97 | 89 | `missing ;` | `` |
| 123 | 89 | `missing ;` | `` |
| 149 | 82 | `missing ;` | `` |
| 169 | 82 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_hilog_helper_test.cpp`

**Summary:** tree-sitter-cpp node `missing identifier` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 91 | `missing ;` | `` |
| 15 | 119 | `missing identifier` | `` |
| 36 | 117 | `missing identifier` | `` |
| 45 | 91 | `missing ;` | `` |
| 48 | 119 | `missing identifier` | `` |
| 69 | 117 | `missing identifier` | `` |
| 78 | 91 | `missing ;` | `` |
| 81 | 119 | `missing identifier` | `` |
| 99 | 117 | `missing identifier` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_jserror_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 86 | `missing ;` | `` |
| 37 | 84 | `missing ;` | `` |
| 92 | 96 | `missing ;` | `` |
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
| 118 | 96 | `missing ;` | `` |
| 122 | 14 | `ERROR` | `: BussinessError 2501000: Operation failed` |
| 123 | 6 | `missing ;` | `` |
| 123 | 13 | `ERROR` | `2501000` |
| 125 | 8 | `ERROR` | `get SourceMap` |
| 125 | 29 | `ERROR` | `dump raw stack: at` |
| 126 | 54 | `ERROR` | `: 76: 10` |
| … | … | … | *(46 more)* |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_manager_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 84 | `missing ;` | `` |
| 20 | 84 | `missing ;` | `` |
| 29 | 88 | `missing ;` | `` |
| 38 | 88 | `missing ;` | `` |
| 68 | 87 | `missing ;` | `` |
| 103 | 88 | `missing ;` | `` |
| 133 | 92 | `missing ;` | `` |
| 149 | 90 | `missing ;` | `` |
| 170 | 88 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_sanitizer_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 88 | `missing ;` | `` |
| 39 | 88 | `missing ;` | `` |
| 70 | 88 | `missing ;` | `` |
| 96 | 88 | `missing ;` | `` |
| 130 | 88 | `missing ;` | `` |
| 162 | 88 | `missing ;` | `` |
| 190 | 88 | `missing ;` | `` |
| 214 | 88 | `missing ;` | `` |
| 230 | 88 | `missing ;` | `` |
| 252 | 88 | `missing ;` | `` |
| 279 | 88 | `missing ;` | `` |
| 307 | 88 | `missing ;` | `` |
| 323 | 88 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlog_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 16 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 96 | `missing ;` | `` |
| 45 | 96 | `missing ;` | `` |
| 58 | 89 | `missing ;` | `` |
| 75 | 89 | `missing ;` | `` |
| 107 | 94 | `missing ;` | `` |
| 124 | 101 | `missing ;` | `` |
| 135 | 101 | `missing ;` | `` |
| 144 | 101 | `missing ;` | `` |
| 155 | 101 | `missing ;` | `` |
| 184 | 103 | `missing ;` | `` |
| 201 | 103 | `missing ;` | `` |
| 211 | 92 | `missing ;` | `` |
| 228 | 98 | `missing ;` | `` |
| 237 | 98 | `missing ;` | `` |
| 246 | 98 | `missing ;` | `` |
| 257 | 99 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/faultlogger_base_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 86 | `missing ;` | `` |
| 20 | 93 | `missing ;` | `` |
| 44 | 85 | `missing ;` | `` |
| 91 | 87 | `missing ;` | `` |
| 105 | 87 | `missing ;` | `` |
| 119 | 85 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/freeze_json_generator_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 97 | `missing ;` | `` |
| 20 | 97 | `missing ;` | `` |
| 46 | 97 | `missing ;` | `` |
| 97 | 97 | `missing ;` | `` |

#### `plugins/faultlogger/service/bdfr_base/test/unittest/log_analyzer_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 94 | `missing ;` | `` |
| 25 | 94 | `missing ;` | `` |

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

#### `plugins/faultlogger/test/common/unittest/extension_manager_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 8 | 97 | `missing ;` | `` |
| 26 | 95 | `missing ;` | `` |
| 39 | 94 | `missing ;` | `` |
| 46 | 94 | `missing ;` | `` |

#### `plugins/faultlogger/test/common/unittest/faultlogger_client_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 100 | `missing ;` | `` |
| 33 | 103 | `missing ;` | `` |
| 40 | 92 | `missing ;` | `` |
| 70 | 98 | `missing ;` | `` |
| 105 | 93 | `missing ;` | `` |

#### `plugins/faultlogger/test/common/unittest/faultlogger_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 16 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 132 | 79 | `missing ;` | `` |
| 166 | 93 | `missing ;` | `` |
| 183 | 83 | `missing ;` | `` |
| 191 | 79 | `missing ;` | `` |
| 198 | 93 | `missing ;` | `` |
| 244 | 98 | `missing ;` | `` |
| 316 | 97 | `missing ;` | `` |
| 344 | 101 | `missing ;` | `` |
| 400 | 93 | `missing ;` | `` |
| 411 | 92 | `missing ;` | `` |
| 420 | 92 | `missing ;` | `` |
| 429 | 92 | `missing ;` | `` |
| 438 | 84 | `missing ;` | `` |
| 454 | 80 | `missing ;` | `` |
| 492 | 89 | `missing ;` | `` |
| 536 | 97 | `missing ;` | `` |

#### `plugins/faultlogger/test/common/unittest/rom_baseline.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 93 | `missing ;` | `` |

#### `plugins/freeze_detector/test/moduletest/common/freeze_detector_plugin_module_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 77 | `missing ;` | `` |
| 78 | 95 | `missing ;` | `` |
| 102 | 96 | `missing ;` | `` |
| 118 | 91 | `missing ;` | `` |

#### `plugins/freeze_detector/test/unittest/common/freeze_detector_getratio_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 36 | 99 | `missing ;` | `` |
| 43 | 99 | `missing ;` | `` |
| 50 | 99 | `missing ;` | `` |
| 57 | 99 | `missing ;` | `` |

#### `plugins/freeze_detector/test/unittest/common/freeze_detector_ipcfull_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 79 | 86 | `missing ;` | `` |
| 106 | 86 | `missing ;` | `` |
| 133 | 86 | `missing ;` | `` |
| 160 | 86 | `missing ;` | `` |

#### `plugins/freeze_detector/test/unittest/common/freeze_detector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 123 | 77 | `missing ;` | `` |
| 166 | 72 | `missing ;` | `` |
| 208 | 72 | `missing ;` | `` |
| 250 | 72 | `missing ;` | `` |
| 277 | 72 | `missing ;` | `` |
| 306 | 72 | `missing ;` | `` |
| 326 | 72 | `missing ;` | `` |
| 346 | 68 | `missing ;` | `` |
| 389 | 68 | `missing ;` | `` |
| 432 | 70 | `missing ;` | `` |

#### `plugins/freeze_detector/test/unittest/common/freeze_detector_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 73 | `missing ;` | `` |
| 40 | 73 | `missing ;` | `` |
| 48 | 73 | `missing ;` | `` |
| 58 | 73 | `missing ;` | `` |
| 70 | 73 | `missing ;` | `` |
| 87 | 73 | `missing ;` | `` |
| 102 | 73 | `missing ;` | `` |
| 118 | 73 | `missing ;` | `` |
| 154 | 73 | `missing ;` | `` |
| 178 | 71 | `missing ;` | `` |
| 185 | 71 | `missing ;` | `` |
| 195 | 71 | `missing ;` | `` |
| 206 | 71 | `missing ;` | `` |
| 230 | 71 | `missing ;` | `` |
| 275 | 71 | `missing ;` | `` |
| 301 | 71 | `missing ;` | `` |
| 341 | 71 | `missing ;` | `` |
| 373 | 71 | `missing ;` | `` |
| 415 | 90 | `missing ;` | `` |
| 457 | 71 | `missing ;` | `` |
| … | … | … | *(83 more)* |

#### `plugins/freeze_detector/test/unittest/common/freeze_manager_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 22 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 73 | `missing ;` | `` |
| 51 | 66 | `missing ;` | `` |
| 60 | 68 | `missing ;` | `` |
| 68 | 68 | `missing ;` | `` |
| 76 | 68 | `missing ;` | `` |
| 84 | 70 | `missing ;` | `` |
| 92 | 77 | `missing ;` | `` |
| 104 | 75 | `missing ;` | `` |
| 113 | 75 | `missing ;` | `` |
| 122 | 75 | `missing ;` | `` |
| 134 | 73 | `missing ;` | `` |
| 145 | 83 | `missing ;` | `` |
| 156 | 79 | `missing ;` | `` |
| 167 | 67 | `missing ;` | `` |
| 179 | 73 | `missing ;` | `` |
| 195 | 73 | `missing ;` | `` |
| 209 | 71 | `missing ;` | `` |
| 224 | 74 | `missing ;` | `` |
| 236 | 65 | `missing ;` | `` |
| 248 | 77 | `missing ;` | `` |
| … | … | … | *(2 more)* |

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

#### `plugins/performance/perfmonitor/interfaces/inner_api/include/perf_model.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 244 | 32 | `ERROR` | `"OHOS.HiviewDFX.IFrameCallback"` |
| 253 | 32 | `ERROR` | `"OHOS.HiviewDFX.IAnimatorCallback"` |
| 262 | 32 | `ERROR` | `"OHOS.HiviewDFX.ISceneCallback"` |

#### `plugins/performance/perfmonitor/src/perf_reporter.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 116 | 72 | `ERROR` | `,` |
| 125 | 72 | `ERROR` | `,` |
| 134 | 72 | `ERROR` | `,` |
| 193 | 94 | `ERROR` | `PRId32` |
| 206 | 119 | `missing identifier` | `` |
| 298 | 55 | `ERROR` | `,` |
| 465 | 77 | `ERROR` | `,` |
| 509 | 146 | `ERROR` | `PRIu64` |

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
| 484 | 98 | `ERROR` | `PRIu64` |
| 493 | 137 | `ERROR` | `,` |

#### `plugins/performance/xperf_service/services/utils/time_util.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 1 | `ERROR` | `static int64_t GetCurrTimeMs()` |
| 11 | 20 | `ERROR` | `:: duration_cast< std:` |
| 11 | 51 | `ERROR` | `:` |
| 12 | 1 | `ERROR` | `std:: chrono:: system_clock::` |
| 16 | 1 | `ERROR` | `}` |

#### `plugins/privacy_controller/test/unittest/common/privacy_controller_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 78 | `missing ;` | `` |

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
| 27 | 94 | `missing ;` | `` |
| 30 | 47 | `missing ;` | `` |
| 46 | 1 | `ERROR` | `) "` |
| 46 | 5 | `ERROR` | `"` |
| 59 | 78 | `missing ;` | `` |
| 70 | 78 | `missing ;` | `` |
| 81 | 78 | `missing ;` | `` |
| 94 | 78 | `missing ;` | `` |
| 138 | 82 | `missing ;` | `` |
| 147 | 68 | `missing ;` | `` |
| 160 | 80 | `missing ;` | `` |
| 168 | 96 | `missing ;` | `` |
| 187 | 78 | `missing ;` | `` |
| 211 | 64 | `missing ;` | `` |
| 235 | 64 | `missing ;` | `` |

#### `plugins/reliability/bbox_detectors/test/moduletest/bbox_detector_module_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 22 | 80 | `missing ;` | `` |
| 52 | 80 | `missing ;` | `` |
| 82 | 80 | `missing ;` | `` |
| 112 | 80 | `missing ;` | `` |
| 139 | 80 | `missing ;` | `` |
| 153 | 80 | `missing ;` | `` |
| 171 | 80 | `missing ;` | `` |
| 201 | 80 | `missing ;` | `` |
| 228 | 80 | `missing ;` | `` |

#### `plugins/reliability/bbox_detectors/test/unittest/bbox_detector_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 41 | 76 | `missing ;` | `` |
| 54 | 76 | `missing ;` | `` |
| 68 | 72 | `missing ;` | `` |

#### `plugins/reliability/leak_detectors/test/moduletest/leak_detector_module_test.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 80 | `missing ;` | `` |
| 49 | 145 | `ERROR` | `,` |
| 53 | 135 | `missing identifier` | `` |
| 62 | 151 | `ERROR` | `,` |
| 67 | 149 | `ERROR` | `,` |

#### `plugins/reliability/leak_detectors/test/unittest/leak_detector_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 76 | `missing ;` | `` |

#### `plugins/sysevent_source/test/unittest/common/event_server_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 66 | `missing ;` | `` |

#### `plugins/unified_collector/test/unittest/common/cpu_storage_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 46 | 64 | `missing ;` | `` |
| 58 | 64 | `missing ;` | `` |
| 82 | 64 | `missing ;` | `` |

#### `plugins/unified_collector/test/unittest/common/uc_state_observer_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 74 | `missing ;` | `` |
| 36 | 74 | `missing ;` | `` |

#### `plugins/usage_event_report/fold/cache/include/fold_app_usage_db_helper.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 92 | 101 | `missing type_identifier` | `` |

#### `plugins/usage_event_report/test/unittest/fold_app_usage_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 33 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 43 | 68 | `missing ;` | `` |
| 91 | 68 | `missing ;` | `` |
| 134 | 68 | `missing ;` | `` |
| 148 | 68 | `missing ;` | `` |
| 200 | 68 | `missing ;` | `` |
| 233 | 68 | `missing ;` | `` |
| 255 | 68 | `missing ;` | `` |
| 267 | 68 | `missing ;` | `` |
| 284 | 68 | `missing ;` | `` |
| 298 | 68 | `missing ;` | `` |
| 333 | 68 | `missing ;` | `` |
| 353 | 68 | `missing ;` | `` |
| 381 | 68 | `missing ;` | `` |
| 401 | 68 | `missing ;` | `` |
| 429 | 68 | `missing ;` | `` |
| 449 | 68 | `missing ;` | `` |
| 465 | 68 | `missing ;` | `` |
| 485 | 68 | `missing ;` | `` |
| 506 | 68 | `missing ;` | `` |
| 530 | 68 | `missing ;` | `` |
| … | … | … | *(13 more)* |

#### `plugins/usage_event_report/test/unittest/usage_event_report_cache_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 86 | `missing ;` | `` |

#### `plugins/usage_event_report/test/unittest/usage_event_report_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 76 | `missing ;` | `` |
| 34 | 76 | `missing ;` | `` |

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
| 250 | 32 | `ERROR` | `"%"` |
| 255 | 33 | `ERROR` | `"%"` |
| 266 | 154 | `ERROR` | `,` |
| 271 | 166 | `ERROR` | `,` |
| 273 | 139 | `ERROR` | `,` |

#### `test/moduletest/common/hiview_plugin_platform_module_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 56 | 82 | `missing ;` | `` |

#### `test/unittest/common/holistic_platform_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 86 | `missing ;` | `` |
| 139 | 86 | `missing ;` | `` |
| 190 | 86 | `missing ;` | `` |
| 243 | 75 | `missing ;` | `` |
| 348 | 75 | `missing ;` | `` |

#### `test/unittest/unified_collection/client/cpu_collector_client_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 67 | `missing ;` | `` |

#### `test/unittest/unified_collection/client/memory_collector_client_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 86 | `missing ;` | `` |
| 48 | 86 | `missing ;` | `` |

#### `test/unittest/unified_collection/client/trace_collector_client_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 72 | `missing ;` | `` |
| 86 | 72 | `missing ;` | `` |
| 114 | 72 | `missing ;` | `` |
| 138 | 72 | `missing ;` | `` |
| 173 | 72 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/cpu_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 68 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/gpu_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 68 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/graphic_memory_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 88 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/hilog_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 72 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/io_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 66 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/memory_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 74 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/perf_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 70 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/thermal_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 76 | `missing ;` | `` |

#### `test/unittest/unified_collection/utility/trace_collector_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 72 | `missing ;` | `` |
| 29 | 72 | `missing ;` | `` |
| 37 | 72 | `missing ;` | `` |
| 45 | 72 | `missing ;` | `` |
| 54 | 72 | `missing ;` | `` |
| 63 | 72 | `missing ;` | `` |

#### `test/unittest/xpower_event/xpower_event_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 28 | 78 | `missing ;` | `` |

#### `test/unittest/xpower_event/xpower_jsevent_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 40 | 94 | `missing ;` | `` |
| 52 | 94 | `missing ;` | `` |
| 82 | 94 | `missing ;` | `` |
| 118 | 94 | `missing ;` | `` |

#### `utility/common_utils/test/unittest/common/utility_common_utils_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 47 | 93 | `missing ;` | `` |
| 68 | 93 | `missing ;` | `` |
| 82 | 86 | `missing ;` | `` |
| 100 | 86 | `missing ;` | `` |
| 110 | 86 | `missing ;` | `` |
| 127 | 82 | `missing ;` | `` |
| 150 | 82 | `missing ;` | `` |
| 161 | 82 | `missing ;` | `` |
| 173 | 82 | `missing ;` | `` |
| 175 | 26 | `missing ;` | `` |

#### `utility/smart_parser/test/moduletest/common/smart_parser_module_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 24 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 72 | `missing ;` | `` |
| 51 | 72 | `missing ;` | `` |
| 78 | 72 | `missing ;` | `` |
| 110 | 72 | `missing ;` | `` |
| 133 | 72 | `missing ;` | `` |
| 156 | 72 | `missing ;` | `` |
| 168 | 72 | `missing ;` | `` |
| 182 | 72 | `missing ;` | `` |
| 207 | 72 | `missing ;` | `` |
| 238 | 72 | `missing ;` | `` |
| 270 | 72 | `missing ;` | `` |
| 309 | 72 | `missing ;` | `` |
| 346 | 72 | `missing ;` | `` |
| 384 | 72 | `missing ;` | `` |
| 422 | 72 | `missing ;` | `` |
| 473 | 72 | `missing ;` | `` |
| 512 | 72 | `missing ;` | `` |
| 535 | 72 | `missing ;` | `` |
| 558 | 72 | `missing ;` | `` |
| 581 | 72 | `missing ;` | `` |
| … | … | … | *(4 more)* |

#### `utility/test/unittest/cpp_crash_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 96 | `missing ;` | `` |

#### `utility/test/unittest/hw_watchdog_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 100 | `missing ;` | `` |
| 25 | 100 | `missing ;` | `` |

#### `utility/test/unittest/iom3_exception_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 106 | `missing ;` | `` |

#### `utility/test/unittest/lpm3_exception_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 106 | `missing ;` | `` |
| 25 | 106 | `missing ;` | `` |

#### `utility/test/unittest/panic_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 90 | `missing ;` | `` |
| 44 | 90 | `missing ;` | `` |
| 78 | 90 | `missing ;` | `` |

#### `utility/test/unittest/rgm_manager_sysfreeze_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 118 | `missing ;` | `` |
| 23 | 118 | `missing ;` | `` |
| 36 | 118 | `missing ;` | `` |
| 49 | 118 | `missing ;` | `` |
| 62 | 118 | `missing ;` | `` |
| 75 | 118 | `missing ;` | `` |
| 88 | 118 | `missing ;` | `` |

#### `utility/test/unittest/syswarning_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 100 | `missing ;` | `` |

---

## multimedia_camera_framework

Generated from `trace analyze /home/sergei/multimedia_camera_framework` (198 files with parse warnings).
Each entry is a translation unit or header indexed as its own file; reasons come from tree-sitter ERROR sites in preprocessed source.

**Total failing files:** 198

### Failure categories

| Category | Files |
|----------|------:|
| generic ERROR nodes (mixed C++ constructs) | 112 |
| gtest/HWTEST macros (`missing ;`) | 71 |
| missing type identifiers (often macro-expanded types) | 14 |
| other / mixed | 1 |

### File list

| # | File | Reason | Error sites |
|---|------|--------|-------------|
| 1 | `common/test/unittest/src/camera_common_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 77 |
| 2 | `common/test/unittest/src/camera_hdi_const_unittest.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 3 | `common/utils/media_capability_interface.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 4 | `dynamic_libs/media_manager/include/media_manager/mpeg_manager_factory.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 5 | `dynamic_libs/media_manager/include/media_manager/track_factory.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 6 | `dynamic_libs/moving_photo/src/moving_photo_adapter.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 74 |
| 7 | `frameworks/cj/camera/include/camera_ffi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 164 |
| 8 | `frameworks/cj/camera/include/camera_input_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 9 | `frameworks/cj/camera/include/camera_manager_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 10 | `frameworks/cj/camera/include/camera_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 11 | `frameworks/cj/camera/include/camera_session_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 12 | `frameworks/cj/camera/include/metadata_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 13 | `frameworks/cj/camera/include/photo_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 14 | `frameworks/cj/camera/include/preview_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 15 | `frameworks/cj/camera/include/video_output_impl.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 16 | `frameworks/cj/camera/src/camera_ffi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 45 |
| 17 | `frameworks/cj/camera_picker/include/camera_picker_ffi.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 18 | `frameworks/js/camera_napi/src/output/photo_output_napi.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 297 |
| 19 | `frameworks/native/camera/base/src/ability/camera_ability_builder.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 20 | `frameworks/native/camera/test/moduletest/camera_base_function/src/camera_base_function_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 249 |
| 21 | `frameworks/native/camera/test/moduletest/camera_deferred_photo/src/camera_deferred_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 22 | `frameworks/native/camera/test/moduletest/camera_deferred_video/src/camera_deferred_video_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 23 | `frameworks/native/camera/test/moduletest/camera_format_YUV/src/camera_format_YUV_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 24 | `frameworks/native/camera/test/moduletest/camera_moving_photo/src/camera_moving_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 71 |
| 25 | `frameworks/native/camera/test/moduletest/camera_photo/src/camera_photo_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 58 |
| 26 | `frameworks/native/camera/test/moduletest/camera_preview/src/camera_preview_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 27 | `frameworks/native/camera/test/moduletest/camera_session/src/camera_session_moduletest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 234 |
| 28 | `frameworks/native/camera/test/ndktest/camera_ndk_demo/entry/src/main/cpp/main.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 29 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_base_test/src/camera_deferred_base_unittest.cpp` | tree-sitter-cpp node `missing ;` at 46 site(s) | 46 |
| 30 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_event_report_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 43 |
| 31 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_video_report_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 15 |
| 32 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_event_test/src/events_info_unittest.cpp` | tree-sitter-cpp node `missing ;` at 19 site(s) | 19 |
| 33 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_event_test/src/events_subscriber_unittest.cpp` | tree-sitter-cpp node `missing ;` at 15 site(s) | 15 |
| 34 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_manager_test/src/camera_deferred_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 35 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_media_manager_test/src/media_manager_adapter_unittest.cpp` | tree-sitter-cpp node `missing ;` at 13 site(s) | 13 |
| 36 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_post_processor_test/src/camera_deferred_post_processor_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 86 |
| 37 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_proc_test/src/camera_deferred_proc_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 38 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/include/camera_deferred_video_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 17 |
| 39 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_job_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 64 |
| 40 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_stratety_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 41 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_unittest.cpp` | tree-sitter-cpp node `missing type_identifier` at 50 site(s) | 50 |
| 42 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_controller_unittest.cpp` | tree-sitter-cpp node `missing ;` at 21 site(s) | 21 |
| 43 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_job_unittest.cpp` | tree-sitter-cpp node `missing ;` at 52 site(s) | 52 |
| 44 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_stratety_unittest.cpp` | tree-sitter-cpp node `missing ;` at 17 site(s) | 17 |
| 45 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_unittest.cpp` | tree-sitter-cpp node `missing ;` at 41 site(s) | 41 |
| 46 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/camera_deferred_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 47 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 38 |
| 48 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_session_command_unittest.cpp` | tree-sitter-cpp node `missing ;` at 15 site(s) | 15 |
| 49 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 50 | `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_utils_test/src/deferred_utils_unittest.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 51 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_capture_session_test/src/camera_capture_session_unittest.cpp` | tree-sitter-cpp node `missing ;` at 131 site(s) | 131 |
| 52 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_input_test/src/camera_input_unittest.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 53 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_manager_test/src/camera_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 59 |
| 54 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_metadata_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 2 site(s) | 2 |
| 55 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_photo_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 44 site(s) | 44 |
| 56 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_preview_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 15 site(s) | 15 |
| 57 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_video_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 16 site(s) | 16 |
| 58 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_photo_native_test/src/camera_photo_native_unittest.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 59 | `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_photo_native_test/src/photo_listener_impl_unittest.cpp` | tree-sitter-cpp node `missing ;` at 20 site(s) | 20 |
| 60 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_capturer_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 61 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_deferred_process_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 62 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_encoder_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 63 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_video_muxer_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 64 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/avcodec_task_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 65 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/camera_server_photo_proxy_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 25 |
| 66 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/frame_record_unittest.cpp` | tree-sitter-cpp node `missing ;` at 13 site(s) | 13 |
| 67 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/moving_photo_video_cache_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 68 | `frameworks/native/camera/test/unittest/camera_service/avcodec/src/video_encoder_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 69 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_client_unittest.cpp` | tree-sitter-cpp node `missing ;` at 3 site(s) | 3 |
| 70 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 71 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_beauty_notification_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 72 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_fwk_metadata_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 73 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_info_dumper_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 74 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_privacy_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 75 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_util_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 76 | `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/icamera_util_unittest.cpp` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 77 | `frameworks/native/camera/test/unittest/camera_service/client/src/camera_service_client_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 68 |
| 78 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_device_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 79 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/include/hcamera_service_unittest.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 14 |
| 80 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 30 |
| 81 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 72 |
| 82 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_host_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 41 |
| 83 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_preconfig_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 84 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_restore_param_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 85 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_service_unittest.cpp` | tree-sitter-cpp node `missing ;` at 85 site(s) | 85 |
| 86 | `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hshared_camera_device_unittest.cpp` | tree-sitter-cpp node `missing ;` at 34 site(s) | 34 |
| 87 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/include/hstream_operator_unittest.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 88 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hcapture_session_unittest.cpp` | tree-sitter-cpp node `missing ;` at 68 site(s) | 68 |
| 89 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hshared_capture_session_unittest.cpp` | tree-sitter-cpp node `missing ;` at 36 site(s) | 36 |
| 90 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_capture_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 91 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_depth_data_unittest.cpp` | tree-sitter-cpp node `missing ;` at 12 site(s) | 12 |
| 92 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_metadata_unittest.cpp` | tree-sitter-cpp node `missing ;` at 13 site(s) | 13 |
| 93 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_operator_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 58 |
| 94 | `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_repeat_unittest.cpp` | tree-sitter-cpp node `missing ;` at 86 site(s) | 86 |
| 95 | `frameworks/native/camera/test/unittest/camera_service/media_library/src/photo_asset_adapter_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 96 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 32 |
| 97 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_reader_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 28 |
| 98 | `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_sign_tools_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 99 | `frameworks/native/camera/test/unittest/camera_service/pipeline/src/camera_common_pipeline_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 44 |
| 100 | `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/cubic_bezier_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 18 |
| 101 | `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/smooth_zoom_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 5 |
| 102 | `frameworks/native/camera/test/unittest/framework_native/ability/src/camera_ability_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 103 | `frameworks/native/camera/test/unittest/framework_native/camera_utils/src/camera_utils_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 19 |
| 104 | `frameworks/native/camera/test/unittest/framework_native/device/src/camera_device_unittest.cpp` | tree-sitter-cpp node `missing ;` at 9 site(s) | 9 |
| 105 | `frameworks/native/camera/test/unittest/framework_native/input/src/camera_framework_input_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 64 |
| 106 | `frameworks/native/camera/test/unittest/framework_native/manager/src/camera_framework_manager_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 94 |
| 107 | `frameworks/native/camera/test/unittest/framework_native/manager/src/prelaunch_config_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 108 | `frameworks/native/camera/test/unittest/framework_native/output/src/camera_output_capability_unittest.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 109 | `frameworks/native/camera/test/unittest/framework_native/output/src/camera_photo_proxy_unittest.cpp` | tree-sitter-cpp node `missing ;` at 5 site(s) | 5 |
| 110 | `frameworks/native/camera/test/unittest/framework_native/output/src/capture_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 4 site(s) | 4 |
| 111 | `frameworks/native/camera/test/unittest/framework_native/output/src/deferred_photo_proxy_unittest.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 112 | `frameworks/native/camera/test/unittest/framework_native/output/src/depth_data_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 113 | `frameworks/native/camera/test/unittest/framework_native/output/src/metadata_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 35 |
| 114 | `frameworks/native/camera/test/unittest/framework_native/output/src/photo_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 77 |
| 115 | `frameworks/native/camera/test/unittest/framework_native/output/src/preview_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 33 site(s) | 33 |
| 116 | `frameworks/native/camera/test/unittest/framework_native/output/src/sketch_wrapper_unittest.cpp` | tree-sitter-cpp node `missing ;` at 8 site(s) | 8 |
| 117 | `frameworks/native/camera/test/unittest/framework_native/output/src/video_output_unittest.cpp` | tree-sitter-cpp node `missing ;` at 17 site(s) | 17 |
| 118 | `frameworks/native/camera/test/unittest/framework_native/session/include/composition_feature_unittest.h` | tree-sitter-cpp node `missing type_identifier` at 2 site(s) | 2 |
| 119 | `frameworks/native/camera/test/unittest/framework_native/session/src/camera_switch_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 120 | `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_dfx_utils_unittest.cpp` | tree-sitter-cpp node `missing ;` at 6 site(s) | 6 |
| 121 | `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 425 |
| 122 | `frameworks/native/camera/test/unittest/framework_native/session/src/cinematic_video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 123 | `frameworks/native/camera/test/unittest/framework_native/session/src/composition_feature_unittest.cpp` | tree-sitter-cpp node `missing ;` at 14 site(s) | 14 |
| 124 | `frameworks/native/camera/test/unittest/framework_native/session/src/mech_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 125 | `frameworks/native/camera/test/unittest/framework_native/session/src/moon_capture_boost_feature_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 126 | `frameworks/native/camera/test/unittest/framework_native/session/src/night_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 127 | `frameworks/native/camera/test/unittest/framework_native/session/src/panorama_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 128 | `frameworks/native/camera/test/unittest/framework_native/session/src/photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 129 | `frameworks/native/camera/test/unittest/framework_native/session/src/portrait_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 130 | `frameworks/native/camera/test/unittest/framework_native/session/src/profession_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 13 |
| 131 | `frameworks/native/camera/test/unittest/framework_native/session/src/scan_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 132 | `frameworks/native/camera/test/unittest/framework_native/session/src/secure_camera_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 9 |
| 133 | `frameworks/native/camera/test/unittest/framework_native/session/src/slow_motion_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 7 |
| 134 | `frameworks/native/camera/test/unittest/framework_native/session/src/stitching_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 16 |
| 135 | `frameworks/native/camera/test/unittest/framework_native/session/src/time_lapse_photo_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 136 | `frameworks/native/camera/test/unittest/framework_native/session/src/video_session_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 26 |
| 137 | `frameworks/native/camera/test/unittest/movie_file/src/hcamera_movie_file_output_unittest.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 91 |
| 138 | `frameworks/native/camera/test/unittest/movie_file/src/movie_file_audio_metadata_unittest.cpp` | tree-sitter-cpp node `missing ;` at 10 site(s) | 10 |
| 139 | `frameworks/native/ndk/impl/camera_manager_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 115 |
| 140 | `frameworks/native/ndk/impl/metadata_output_impl.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 11 |
| 141 | `frameworks/taihe/include/camera_event_emitter_taihe.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 142 | `frameworks/taihe/src/camera_constructor_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 143 | `frameworks/taihe/src/camera_picker_constructor_taihe.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 144 | `interfaces/inner_api/native/camera/include/ability/camera_ability_builder.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 145 | `interfaces/inner_api/native/camera/include/input/camera_manager.h` | tree-sitter-cpp node `missing type_identifier` at 1 site(s) | 1 |
| 146 | `interfaces/inner_api/native/camera/include/input/i_standard_camera_listener.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 147 | `interfaces/inner_api/native/camera/include/output/photo_output.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 148 | `interfaces/kits/js/camera_napi/include/camera_napi_event_emitter.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 8 |
| 149 | `interfaces/kits/js/camera_napi/include/session/camera_napi_adaptor.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 36 |
| 150 | `mediastream/test/unittest/filter/include/audio_capture_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 8 site(s) | 8 |
| 151 | `mediastream/test/unittest/filter/include/audio_encoder_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 24 site(s) | 24 |
| 152 | `mediastream/test/unittest/filter/include/audio_fork_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 18 site(s) | 18 |
| 153 | `mediastream/test/unittest/filter/include/audio_process_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 18 site(s) | 18 |
| 154 | `mediastream/test/unittest/filter/include/cfilter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 155 | `mediastream/test/unittest/filter/include/cinematic_video_cache_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 14 site(s) | 14 |
| 156 | `mediastream/test/unittest/filter/include/metadata_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 14 site(s) | 14 |
| 157 | `mediastream/test/unittest/filter/include/muxer_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 22 site(s) | 22 |
| 158 | `mediastream/test/unittest/filter/include/video_encoder_filter_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 159 | `mediastream/test/unittest/filter/src/audio_capture_adapter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 29 site(s) | 29 |
| 160 | `mediastream/test/unittest/filter/src/audio_capture_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 27 site(s) | 27 |
| 161 | `mediastream/test/unittest/filter/src/audio_encoder_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 29 site(s) | 29 |
| 162 | `mediastream/test/unittest/filter/src/audio_fork_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 20 site(s) | 20 |
| 163 | `mediastream/test/unittest/filter/src/audio_process_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 25 site(s) | 25 |
| 164 | `mediastream/test/unittest/filter/src/cfilter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 34 site(s) | 34 |
| 165 | `mediastream/test/unittest/filter/src/cinematic_video_cache_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 18 site(s) | 18 |
| 166 | `mediastream/test/unittest/filter/src/metadata_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 24 site(s) | 24 |
| 167 | `mediastream/test/unittest/filter/src/muxer_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 21 site(s) | 21 |
| 168 | `mediastream/test/unittest/filter/src/video_encoder_adapter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 32 site(s) | 32 |
| 169 | `mediastream/test/unittest/filter/src/video_encoder_filter_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 22 site(s) | 22 |
| 170 | `mediastream/test/unittest/pipeline/pipeline_unit_test.cpp` | tree-sitter-cpp node `missing ;` at 7 site(s) | 7 |
| 171 | `mediastream/test/unittest/pipeline/pipeline_unit_test.h` | tree-sitter-cpp node `missing type_identifier` at 4 site(s) | 4 |
| 172 | `moviefile/include/movie_file/plugin/movie_file_video_filter_plugin.h` | tree-sitter-cpp node `missing field_identifier` at 1 site(s) | 1 |
| 173 | `services/camera_service/binder/base/include/icamera_broker.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 174 | `services/camera_service/binder/base/include/icamera_multi_stream_output.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 175 | `services/camera_service/binder/base/include/istream_capture_photo_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 176 | `services/camera_service/binder/base/include/istream_capture_thumbnail_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 177 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_mock_session_manager_interface.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 178 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_scene_session_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 179 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_manager_callback.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 180 | `services/camera_service/binder/base/include/window_manager_service_utils/icamera_window_session_manager_service.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 181 | `services/camera_service/include/param_update/camera_rotate_param_manager.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 2 |
| 182 | `services/camera_service/src/camera_beauty_notification.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 183 | `services/camera_service/src/hcamera_device.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 225 |
| 184 | `services/camera_service/src/hcamera_service.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 339 |
| 185 | `services/deferred_processing_service/include/base/blocking_queue.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 57 |
| 186 | `services/deferred_processing_service/include/base/dps.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 12 |
| 187 | `services/deferred_processing_service/include/deferred_processing_service.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 188 | `services/deferred_processing_service/include/dfx/dps_video_report.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 189 | `services/deferred_processing_service/include/event_monitor/events_monitor.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 190 | `services/deferred_processing_service/include/schedule/video_processor/strategy/ivideo_strategy.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 4 |
| 191 | `services/deferred_processing_service/include/schedule/video_processor/video_job_repository/ivideo_job_repository_listener.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 3 |
| 192 | `services/deferred_processing_service/include/utils/dp_power_manager.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 193 | `services/deferred_processing_service/include/utils/dp_safe_map.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 37 |
| 194 | `services/deferred_processing_service/include/utils/dp_timer.h` | tree-sitter-cpp node `missing ;` at 1 site(s) | 1 |
| 195 | `services/deferred_processing_service/src/post_processor/video_post_processor.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 56 |
| 196 | `test/fuzztest/audiodeferredprocess_fuzzer/audio_deferred_process_fuzzer.cpp` | generic tree-sitter ERROR node(s) in preprocessed C++ | 10 |
| 197 | `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |
| 198 | `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.h` | generic tree-sitter ERROR node(s) in preprocessed C++ | 1 |

### Per-file details

#### `common/test/unittest/src/camera_common_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 86 | `missing ;` | `` |
| 32 | 96 | `missing ;` | `` |
| 41 | 87 | `missing ;` | `` |
| 49 | 95 | `missing ;` | `` |
| 56 | 95 | `missing ;` | `` |
| 72 | 95 | `missing ;` | `` |
| 92 | 87 | `missing ;` | `` |
| 110 | 97 | `missing ;` | `` |
| 126 | 82 | `missing ;` | `` |
| 142 | 83 | `missing ;` | `` |
| 162 | 95 | `missing ;` | `` |
| 177 | 101 | `missing ;` | `` |
| 192 | 103 | `missing ;` | `` |
| 212 | 109 | `missing ;` | `` |
| 232 | 102 | `missing ;` | `` |
| 249 | 93 | `missing ;` | `` |
| 262 | 92 | `missing ;` | `` |
| 276 | 89 | `missing ;` | `` |
| 278 | 108 | `ERROR` | `,` |
| 293 | 106 | `ERROR` | `,` |
| … | … | … | *(57 more)* |

#### `common/test/unittest/src/camera_hdi_const_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 81 | `missing ;` | `` |

#### `common/utils/media_capability_interface.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 5 | 11 | `ERROR` | `OHOS::` |
| 6 | 26 | `ERROR` | `: public Parcelable` |
| 8 | 8 | `ERROR` | `~` |
| 9 | 9 | `ERROR` | `bool` |
| 10 | 49 | `ERROR` | `&` |

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

#### `frameworks/cj/camera_picker/include/camera_picker_ffi.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 12 | `ERROR` | `void` |

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
| … | … | … | *(277 more)* |

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
| … | … | … | *(229 more)* |

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
| … | … | … | *(18 more)* |

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
| 175 | 98 | `missing ;` | `` |
| 192 | 192 | `ERROR` | `, , ,` |
| 204 | 98 | `missing ;` | `` |
| 221 | 192 | `ERROR` | `, , ,` |
| 233 | 98 | `missing ;` | `` |
| 257 | 98 | `missing ;` | `` |

#### `frameworks/native/camera/test/moduletest/camera_format_YUV/src/camera_format_YUV_moduletest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 65 | 121 | `ERROR` | `,` |
| 71 | 124 | `ERROR` | `,` |
| 95 | 116 | `ERROR` | `,` |
| 111 | 90 | `missing ;` | `` |
| 148 | 90 | `missing ;` | `` |

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
| … | … | … | *(51 more)* |

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
| 211 | 81 | `missing ;` | `` |
| 240 | 81 | `missing ;` | `` |
| 269 | 81 | `missing ;` | `` |
| 280 | 175 | `ERROR` | `, , ,` |
| … | … | … | *(38 more)* |

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
| 154 | 85 | `missing ;` | `` |

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
| 151 | 216 | `ERROR` | `PRIu64` |
| 158 | 222 | `ERROR` | `PRIu64` |
| 165 | 216 | `ERROR` | `PRIu64` |
| 171 | 177 | `ERROR` | `,` |
| 184 | 99 | `ERROR` | `,` |
| 189 | 167 | `ERROR` | `,` |
| … | … | … | *(214 more)* |

#### `frameworks/native/camera/test/ndktest/camera_ndk_demo/entry/src/main/cpp/main.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 468 | 8 | `ERROR` | `napi_value` |
| 500 | 20 | `ERROR` | `demoModule` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_base_test/src/camera_deferred_base_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 46 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 86 | `missing ;` | `` |
| 39 | 86 | `missing ;` | `` |
| 62 | 86 | `missing ;` | `` |
| 76 | 86 | `missing ;` | `` |
| 85 | 86 | `missing ;` | `` |
| 106 | 86 | `missing ;` | `` |
| 127 | 86 | `missing ;` | `` |
| 141 | 86 | `missing ;` | `` |
| 155 | 86 | `missing ;` | `` |
| 170 | 86 | `missing ;` | `` |
| 180 | 86 | `missing ;` | `` |
| 190 | 86 | `missing ;` | `` |
| 201 | 86 | `missing ;` | `` |
| 217 | 86 | `missing ;` | `` |
| 228 | 86 | `missing ;` | `` |
| 240 | 86 | `missing ;` | `` |
| 253 | 86 | `missing ;` | `` |
| 263 | 86 | `missing ;` | `` |
| 284 | 86 | `missing ;` | `` |
| 308 | 86 | `missing ;` | `` |
| … | … | … | *(26 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_event_report_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 117 | `ERROR` | `,` |
| 24 | 120 | `ERROR` | `,` |
| 29 | 109 | `ERROR` | `,` |
| 34 | 112 | `ERROR` | `,` |
| 38 | 70 | `missing ;` | `` |
| 47 | 70 | `missing ;` | `` |
| 56 | 82 | `missing ;` | `` |
| 73 | 82 | `missing ;` | `` |
| 90 | 70 | `missing ;` | `` |
| 105 | 70 | `missing ;` | `` |
| 119 | 70 | `missing ;` | `` |
| 133 | 70 | `missing ;` | `` |
| 150 | 73 | `missing ;` | `` |
| 167 | 73 | `missing ;` | `` |
| 183 | 70 | `missing ;` | `` |
| 199 | 70 | `missing ;` | `` |
| 215 | 70 | `missing ;` | `` |
| 230 | 73 | `missing ;` | `` |
| 247 | 73 | `missing ;` | `` |
| 264 | 73 | `missing ;` | `` |
| … | … | … | *(23 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_dfx_test/src/dps_video_report_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 117 | `ERROR` | `,` |
| 20 | 120 | `ERROR` | `,` |
| 25 | 109 | `ERROR` | `,` |
| 30 | 112 | `ERROR` | `,` |
| 34 | 77 | `missing ;` | `` |
| 48 | 77 | `missing ;` | `` |
| 63 | 80 | `missing ;` | `` |
| 77 | 80 | `missing ;` | `` |
| 92 | 79 | `missing ;` | `` |
| 107 | 79 | `missing ;` | `` |
| 123 | 80 | `missing ;` | `` |
| 138 | 80 | `missing ;` | `` |
| 153 | 80 | `missing ;` | `` |
| 168 | 82 | `missing ;` | `` |
| 183 | 82 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_event_test/src/events_info_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 19 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 75 | `missing ;` | `` |
| 30 | 75 | `missing ;` | `` |
| 39 | 75 | `missing ;` | `` |
| 48 | 75 | `missing ;` | `` |
| 57 | 75 | `missing ;` | `` |
| 66 | 75 | `missing ;` | `` |
| 75 | 75 | `missing ;` | `` |
| 89 | 75 | `missing ;` | `` |
| 98 | 75 | `missing ;` | `` |
| 107 | 75 | `missing ;` | `` |
| 116 | 75 | `missing ;` | `` |
| 125 | 75 | `missing ;` | `` |
| 136 | 75 | `missing ;` | `` |
| 147 | 75 | `missing ;` | `` |
| 156 | 75 | `missing ;` | `` |
| 165 | 75 | `missing ;` | `` |
| 175 | 75 | `missing ;` | `` |
| 184 | 75 | `missing ;` | `` |
| 193 | 75 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_event_test/src/events_subscriber_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 15 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 87 | `missing ;` | `` |
| 30 | 87 | `missing ;` | `` |
| 41 | 87 | `missing ;` | `` |
| 50 | 87 | `missing ;` | `` |
| 66 | 87 | `missing ;` | `` |
| 82 | 87 | `missing ;` | `` |
| 99 | 87 | `missing ;` | `` |
| 115 | 87 | `missing ;` | `` |
| 131 | 87 | `missing ;` | `` |
| 147 | 87 | `missing ;` | `` |
| 163 | 87 | `missing ;` | `` |
| 179 | 87 | `missing ;` | `` |
| 195 | 87 | `missing ;` | `` |
| 211 | 87 | `missing ;` | `` |
| 227 | 87 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_manager_test/src/camera_deferred_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 119 | `ERROR` | `,` |
| 23 | 122 | `ERROR` | `,` |
| 28 | 77 | `ERROR` | `,` |
| 36 | 114 | `ERROR` | `,` |
| 49 | 92 | `missing ;` | `` |
| 63 | 92 | `missing ;` | `` |
| 89 | 92 | `missing ;` | `` |
| 106 | 92 | `missing ;` | `` |
| 124 | 92 | `missing ;` | `` |
| 140 | 92 | `missing ;` | `` |
| 148 | 92 | `missing ;` | `` |
| 172 | 92 | `missing ;` | `` |
| 188 | 92 | `missing ;` | `` |
| 205 | 92 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_media_manager_test/src/media_manager_adapter_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 94 | `missing ;` | `` |
| 59 | 94 | `missing ;` | `` |
| 69 | 94 | `missing ;` | `` |
| 79 | 94 | `missing ;` | `` |
| 89 | 94 | `missing ;` | `` |
| 99 | 94 | `missing ;` | `` |
| 109 | 94 | `missing ;` | `` |
| 128 | 94 | `missing ;` | `` |
| 147 | 94 | `missing ;` | `` |
| 166 | 94 | `missing ;` | `` |
| 185 | 94 | `missing ;` | `` |
| 204 | 94 | `missing ;` | `` |
| 223 | 94 | `missing ;` | `` |

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
| … | … | … | *(66 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_proc_test/src/camera_deferred_proc_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 116 | `ERROR` | `,` |
| 23 | 119 | `ERROR` | `,` |
| 47 | 111 | `ERROR` | `,` |
| 50 | 86 | `missing ;` | `` |
| 86 | 86 | `missing ;` | `` |
| 119 | 86 | `missing ;` | `` |
| 161 | 86 | `missing ;` | `` |
| 200 | 86 | `missing ;` | `` |
| 236 | 86 | `missing ;` | `` |
| 273 | 86 | `missing ;` | `` |
| 304 | 86 | `missing ;` | `` |
| 331 | 86 | `missing ;` | `` |
| 355 | 86 | `missing ;` | `` |
| 379 | 86 | `missing ;` | `` |
| 403 | 86 | `missing ;` | `` |
| 427 | 86 | `missing ;` | `` |

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
| 59 | 85 | `missing ;` | `` |
| 72 | 85 | `missing ;` | `` |
| 86 | 85 | `missing ;` | `` |
| 104 | 85 | `missing ;` | `` |
| 129 | 85 | `missing ;` | `` |
| 158 | 85 | `missing ;` | `` |
| 186 | 85 | `missing ;` | `` |
| 203 | 85 | `missing ;` | `` |
| 226 | 85 | `missing ;` | `` |
| 274 | 85 | `missing ;` | `` |
| 312 | 85 | `missing ;` | `` |
| 351 | 85 | `missing ;` | `` |
| 390 | 85 | `missing ;` | `` |
| 430 | 85 | `missing ;` | `` |
| 478 | 85 | `missing ;` | `` |
| 524 | 90 | `missing ;` | `` |
| … | … | … | *(44 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_stratety_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 80 | `ERROR` | `,` |
| 34 | 80 | `ERROR` | `,` |
| 39 | 80 | `ERROR` | `,` |
| 43 | 117 | `missing ;` | `` |
| 60 | 117 | `missing ;` | `` |
| 75 | 117 | `missing ;` | `` |
| 86 | 117 | `missing ;` | `` |
| 102 | 117 | `missing ;` | `` |
| 119 | 117 | `missing ;` | `` |
| 125 | 117 | `missing ;` | `` |
| 131 | 117 | `missing ;` | `` |
| 138 | 117 | `missing ;` | `` |
| 162 | 117 | `missing ;` | `` |
| 189 | 117 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_photo_processor_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 50 site(s)

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
| … | … | … | *(30 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_controller_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 21 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 43 | `missing type_identifier` | `` |
| 23 | 74 | `missing type_identifier` | `` |
| 24 | 32 | `missing type_identifier` | `` |
| 25 | 39 | `missing type_identifier` | `` |
| 25 | 60 | `missing type_identifier` | `` |
| 26 | 45 | `missing type_identifier` | `` |
| 26 | 93 | `missing type_identifier` | `` |
| 75 | 102 | `missing ;` | `` |
| 89 | 102 | `missing ;` | `` |
| 96 | 102 | `missing ;` | `` |
| 112 | 102 | `missing ;` | `` |
| 127 | 102 | `missing ;` | `` |
| 142 | 102 | `missing ;` | `` |
| 152 | 102 | `missing ;` | `` |
| 162 | 102 | `missing ;` | `` |
| 171 | 102 | `missing ;` | `` |
| 181 | 102 | `missing ;` | `` |
| 192 | 102 | `missing ;` | `` |
| 208 | 102 | `missing ;` | `` |
| 226 | 102 | `missing ;` | `` |
| … | … | … | *(1 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_job_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 52 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 71 | 88 | `missing ;` | `` |
| 82 | 88 | `missing ;` | `` |
| 95 | 88 | `missing ;` | `` |
| 111 | 88 | `missing ;` | `` |
| 133 | 88 | `missing ;` | `` |
| 158 | 88 | `missing ;` | `` |
| 182 | 88 | `missing ;` | `` |
| 194 | 88 | `missing ;` | `` |
| 216 | 88 | `missing ;` | `` |
| 259 | 88 | `missing ;` | `` |
| 293 | 88 | `missing ;` | `` |
| 333 | 88 | `missing ;` | `` |
| 368 | 88 | `missing ;` | `` |
| 403 | 88 | `missing ;` | `` |
| 439 | 88 | `missing ;` | `` |
| 483 | 88 | `missing ;` | `` |
| 525 | 88 | `missing ;` | `` |
| 537 | 88 | `missing ;` | `` |
| 549 | 88 | `missing ;` | `` |
| 560 | 88 | `missing ;` | `` |
| … | … | … | *(32 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_stratety_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 17 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 117 | `missing ;` | `` |
| 59 | 117 | `missing ;` | `` |
| 75 | 117 | `missing ;` | `` |
| 86 | 117 | `missing ;` | `` |
| 100 | 117 | `missing ;` | `` |
| 113 | 117 | `missing ;` | `` |
| 126 | 117 | `missing ;` | `` |
| 139 | 117 | `missing ;` | `` |
| 153 | 117 | `missing ;` | `` |
| 171 | 117 | `missing ;` | `` |
| 192 | 117 | `missing ;` | `` |
| 210 | 117 | `missing ;` | `` |
| 226 | 117 | `missing ;` | `` |
| 237 | 117 | `missing ;` | `` |
| 248 | 117 | `missing ;` | `` |
| 280 | 117 | `missing ;` | `` |
| 292 | 117 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_schedule_test/src/deferred_video_processor_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 41 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 24 | 43 | `missing type_identifier` | `` |
| 24 | 74 | `missing type_identifier` | `` |
| 25 | 32 | `missing type_identifier` | `` |
| 26 | 39 | `missing type_identifier` | `` |
| 26 | 60 | `missing type_identifier` | `` |
| 27 | 45 | `missing type_identifier` | `` |
| 27 | 93 | `missing type_identifier` | `` |
| 81 | 100 | `missing ;` | `` |
| 91 | 100 | `missing ;` | `` |
| 101 | 100 | `missing ;` | `` |
| 112 | 100 | `missing ;` | `` |
| 124 | 100 | `missing ;` | `` |
| 135 | 100 | `missing ;` | `` |
| 150 | 100 | `missing ;` | `` |
| 163 | 100 | `missing ;` | `` |
| 177 | 100 | `missing ;` | `` |
| 190 | 100 | `missing ;` | `` |
| 205 | 100 | `missing ;` | `` |
| 217 | 100 | `missing ;` | `` |
| 231 | 100 | `missing ;` | `` |
| … | … | … | *(21 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/camera_deferred_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 50 | 134 | `ERROR` | `, , ,` |
| 54 | 158 | `ERROR` | `, , ,` |
| 61 | 134 | `ERROR` | `, , ,` |
| 65 | 158 | `ERROR` | `, , ,` |
| 70 | 98 | `missing ;` | `` |
| 110 | 98 | `missing ;` | `` |
| 150 | 98 | `missing ;` | `` |
| 174 | 98 | `missing ;` | `` |
| 205 | 98 | `missing ;` | `` |
| 228 | 98 | `missing ;` | `` |
| 266 | 98 | `missing ;` | `` |
| 286 | 98 | `missing ;` | `` |

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
| 80 | 79 | `missing ;` | `` |
| 89 | 79 | `missing ;` | `` |
| 99 | 79 | `missing ;` | `` |
| 107 | 79 | `missing ;` | `` |
| 116 | 79 | `missing ;` | `` |
| … | … | … | *(18 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_session_test/src/deferred_session_command_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 15 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 87 | 100 | `missing ;` | `` |
| 98 | 100 | `missing ;` | `` |
| 107 | 100 | `missing ;` | `` |
| 116 | 100 | `missing ;` | `` |
| 128 | 100 | `missing ;` | `` |
| 145 | 100 | `missing ;` | `` |
| 161 | 100 | `missing ;` | `` |
| 175 | 100 | `missing ;` | `` |
| 187 | 100 | `missing ;` | `` |
| 199 | 100 | `missing ;` | `` |
| 213 | 100 | `missing ;` | `` |
| 228 | 100 | `missing ;` | `` |
| 247 | 100 | `missing ;` | `` |
| 261 | 100 | `missing ;` | `` |
| 274 | 100 | `missing ;` | `` |

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
| 82 | 89 | `missing ;` | `` |
| 93 | 89 | `missing ;` | `` |
| 119 | 89 | `missing ;` | `` |
| 141 | 89 | `missing ;` | `` |
| 151 | 89 | `missing ;` | `` |
| 161 | 89 | `missing ;` | `` |
| 175 | 40 | `missing type_identifier` | `` |
| 175 | 71 | `missing type_identifier` | `` |
| 176 | 29 | `missing type_identifier` | `` |
| 176 | 91 | `missing type_identifier` | `` |
| 177 | 36 | `missing type_identifier` | `` |
| 177 | 66 | `missing type_identifier` | `` |
| … | … | … | *(6 more)* |

#### `frameworks/native/camera/test/unittest/camera_deferred_unittest/camera_deferred_utils_test/src/deferred_utils_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 81 | `missing ;` | `` |
| 26 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_capture_session_test/src/camera_capture_session_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 131 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 96 | `missing ;` | `` |
| 65 | 96 | `missing ;` | `` |
| 87 | 96 | `missing ;` | `` |
| 101 | 96 | `missing ;` | `` |
| 119 | 96 | `missing ;` | `` |
| 135 | 96 | `missing ;` | `` |
| 149 | 96 | `missing ;` | `` |
| 189 | 96 | `missing ;` | `` |
| 205 | 96 | `missing ;` | `` |
| 219 | 96 | `missing ;` | `` |
| 258 | 96 | `missing ;` | `` |
| 294 | 96 | `missing ;` | `` |
| 330 | 96 | `missing ;` | `` |
| 351 | 96 | `missing ;` | `` |
| 370 | 96 | `missing ;` | `` |
| 421 | 96 | `missing ;` | `` |
| 465 | 96 | `missing ;` | `` |
| 499 | 96 | `missing ;` | `` |
| 551 | 96 | `missing ;` | `` |
| 584 | 96 | `missing ;` | `` |
| … | … | … | *(111 more)* |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_input_test/src/camera_input_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 77 | `missing ;` | `` |
| 48 | 77 | `missing ;` | `` |
| 70 | 77 | `missing ;` | `` |
| 116 | 77 | `missing ;` | `` |
| 151 | 77 | `missing ;` | `` |
| 175 | 77 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_manager_test/src/camera_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 27 | 81 | `missing ;` | `` |
| 40 | 81 | `missing ;` | `` |
| 62 | 81 | `missing ;` | `` |
| 72 | 81 | `missing ;` | `` |
| 86 | 81 | `missing ;` | `` |
| 100 | 81 | `missing ;` | `` |
| 134 | 81 | `missing ;` | `` |
| 153 | 81 | `missing ;` | `` |
| 176 | 81 | `missing ;` | `` |
| 198 | 81 | `missing ;` | `` |
| 232 | 81 | `missing ;` | `` |
| 278 | 81 | `missing ;` | `` |
| 297 | 81 | `missing ;` | `` |
| 322 | 81 | `missing ;` | `` |
| 356 | 81 | `missing ;` | `` |
| 399 | 81 | `missing ;` | `` |
| 423 | 81 | `missing ;` | `` |
| 446 | 81 | `missing ;` | `` |
| 491 | 81 | `missing ;` | `` |
| 515 | 81 | `missing ;` | `` |
| … | … | … | *(39 more)* |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_metadata_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 2 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 96 | `missing ;` | `` |
| 44 | 96 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_photo_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 44 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 90 | `missing ;` | `` |
| 64 | 90 | `missing ;` | `` |
| 102 | 90 | `missing ;` | `` |
| 140 | 90 | `missing ;` | `` |
| 178 | 90 | `missing ;` | `` |
| 216 | 90 | `missing ;` | `` |
| 254 | 90 | `missing ;` | `` |
| 296 | 90 | `missing ;` | `` |
| 334 | 90 | `missing ;` | `` |
| 375 | 90 | `missing ;` | `` |
| 416 | 90 | `missing ;` | `` |
| 469 | 90 | `missing ;` | `` |
| 522 | 90 | `missing ;` | `` |
| 544 | 90 | `missing ;` | `` |
| 575 | 90 | `missing ;` | `` |
| 624 | 90 | `missing ;` | `` |
| 666 | 90 | `missing ;` | `` |
| 677 | 90 | `missing ;` | `` |
| 727 | 90 | `missing ;` | `` |
| 767 | 90 | `missing ;` | `` |
| … | … | … | *(24 more)* |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_preview_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 15 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 94 | `missing ;` | `` |
| 73 | 94 | `missing ;` | `` |
| 115 | 94 | `missing ;` | `` |
| 171 | 94 | `missing ;` | `` |
| 185 | 94 | `missing ;` | `` |
| 237 | 94 | `missing ;` | `` |
| 291 | 94 | `missing ;` | `` |
| 331 | 94 | `missing ;` | `` |
| 351 | 94 | `missing ;` | `` |
| 380 | 94 | `missing ;` | `` |
| 409 | 94 | `missing ;` | `` |
| 464 | 94 | `missing ;` | `` |
| 518 | 94 | `missing ;` | `` |
| 566 | 94 | `missing ;` | `` |
| 605 | 94 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_output_test/src/camera_video_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 16 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 90 | `missing ;` | `` |
| 67 | 90 | `missing ;` | `` |
| 122 | 90 | `missing ;` | `` |
| 136 | 90 | `missing ;` | `` |
| 187 | 90 | `missing ;` | `` |
| 240 | 90 | `missing ;` | `` |
| 283 | 90 | `missing ;` | `` |
| 315 | 90 | `missing ;` | `` |
| 333 | 90 | `missing ;` | `` |
| 353 | 90 | `missing ;` | `` |
| 382 | 90 | `missing ;` | `` |
| 411 | 90 | `missing ;` | `` |
| 451 | 90 | `missing ;` | `` |
| 489 | 90 | `missing ;` | `` |
| 536 | 90 | `missing ;` | `` |
| 583 | 90 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_photo_native_test/src/camera_photo_native_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 84 | `missing ;` | `` |
| 36 | 84 | `missing ;` | `` |
| 51 | 84 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_ndk_unittest/camera_ndk_photo_native_test/src/photo_listener_impl_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 20 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 34 | 86 | `missing ;` | `` |
| 49 | 86 | `missing ;` | `` |
| 65 | 86 | `missing ;` | `` |
| 85 | 86 | `missing ;` | `` |
| 105 | 87 | `missing ;` | `` |
| 112 | 94 | `missing ;` | `` |
| 122 | 94 | `missing ;` | `` |
| 132 | 92 | `missing ;` | `` |
| 139 | 99 | `missing ;` | `` |
| 149 | 99 | `missing ;` | `` |
| 159 | 74 | `missing ;` | `` |
| 171 | 74 | `missing ;` | `` |
| 180 | 78 | `missing ;` | `` |
| 188 | 77 | `missing ;` | `` |
| 201 | 77 | `missing ;` | `` |
| 210 | 77 | `missing ;` | `` |
| 223 | 77 | `missing ;` | `` |
| 233 | 73 | `missing ;` | `` |
| 241 | 73 | `missing ;` | `` |
| 248 | 80 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_capturer_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 18 | 127 | `ERROR` | `,` |
| 23 | 116 | `ERROR` | `,` |
| 28 | 119 | `ERROR` | `,` |
| 32 | 96 | `missing ;` | `` |
| 43 | 96 | `missing ;` | `` |
| 55 | 96 | `missing ;` | `` |
| 69 | 96 | `missing ;` | `` |
| 83 | 96 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_deferred_process_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 124 | `ERROR` | `,` |
| 17 | 127 | `ERROR` | `,` |
| 22 | 116 | `ERROR` | `,` |
| 27 | 119 | `ERROR` | `,` |
| 31 | 96 | `missing ;` | `` |
| 33 | 112 | `ERROR` | `,` |
| 48 | 110 | `ERROR` | `,` |
| 52 | 96 | `missing ;` | `` |
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
| 31 | 79 | `missing ;` | `` |
| 50 | 79 | `missing ;` | `` |
| 68 | 79 | `missing ;` | `` |
| 87 | 79 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/audio_video_muxer_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 119 | `ERROR` | `,` |
| 19 | 122 | `ERROR` | `,` |
| 24 | 111 | `ERROR` | `,` |
| 29 | 114 | `ERROR` | `,` |
| 33 | 86 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/avcodec_task_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 91 | `ERROR` | `,` |
| 19 | 122 | `ERROR` | `,` |
| 24 | 125 | `ERROR` | `,` |
| 29 | 114 | `ERROR` | `,` |
| 34 | 117 | `ERROR` | `,` |
| 38 | 92 | `missing ;` | `` |
| 63 | 92 | `missing ;` | `` |
| 81 | 92 | `missing ;` | `` |
| 117 | 92 | `missing ;` | `` |
| 131 | 92 | `missing ;` | `` |
| 145 | 92 | `missing ;` | `` |
| 164 | 92 | `missing ;` | `` |
| 182 | 92 | `missing ;` | `` |
| 198 | 92 | `missing ;` | `` |
| 224 | 92 | `missing ;` | `` |
| 250 | 92 | `missing ;` | `` |
| 266 | 92 | `missing ;` | `` |
| 283 | 92 | `missing ;` | `` |
| 319 | 92 | `missing ;` | `` |
| 355 | 92 | `missing ;` | `` |
| … | … | … | *(12 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/camera_server_photo_proxy_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 126 | `ERROR` | `,` |
| 19 | 129 | `ERROR` | `,` |
| 24 | 118 | `ERROR` | `,` |
| 29 | 121 | `ERROR` | `,` |
| 33 | 101 | `missing ;` | `` |
| 54 | 101 | `missing ;` | `` |
| 67 | 101 | `missing ;` | `` |
| 87 | 101 | `missing ;` | `` |
| 96 | 101 | `missing ;` | `` |
| 105 | 101 | `missing ;` | `` |
| 114 | 101 | `missing ;` | `` |
| 123 | 101 | `missing ;` | `` |
| 132 | 101 | `missing ;` | `` |
| 141 | 101 | `missing ;` | `` |
| 150 | 101 | `missing ;` | `` |
| 161 | 101 | `missing ;` | `` |
| 170 | 101 | `missing ;` | `` |
| 179 | 101 | `missing ;` | `` |
| 188 | 101 | `missing ;` | `` |
| 201 | 101 | `missing ;` | `` |
| … | … | … | *(5 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/frame_record_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 17 | 74 | `missing ;` | `` |
| 33 | 74 | `missing ;` | `` |
| 54 | 74 | `missing ;` | `` |
| 70 | 74 | `missing ;` | `` |
| 92 | 74 | `missing ;` | `` |
| 114 | 74 | `missing ;` | `` |
| 130 | 74 | `missing ;` | `` |
| 147 | 74 | `missing ;` | `` |
| 163 | 74 | `missing ;` | `` |
| 184 | 74 | `missing ;` | `` |
| 200 | 74 | `missing ;` | `` |
| 222 | 74 | `missing ;` | `` |
| 244 | 74 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/moving_photo_video_cache_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 91 | `ERROR` | `,` |
| 19 | 125 | `ERROR` | `,` |
| 24 | 128 | `ERROR` | `,` |
| 29 | 117 | `ERROR` | `,` |
| 34 | 120 | `ERROR` | `,` |
| 38 | 99 | `missing ;` | `` |
| 59 | 99 | `missing ;` | `` |
| 74 | 99 | `missing ;` | `` |
| 104 | 99 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/avcodec/src/video_encoder_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 116 | `ERROR` | `,` |
| 18 | 119 | `ERROR` | `,` |
| 23 | 108 | `ERROR` | `,` |
| 28 | 111 | `ERROR` | `,` |
| 32 | 79 | `missing ;` | `` |
| 51 | 79 | `missing ;` | `` |
| 77 | 79 | `missing ;` | `` |
| 86 | 79 | `missing ;` | `` |
| 99 | 79 | `missing ;` | `` |
| 123 | 79 | `missing ;` | `` |
| 135 | 79 | `missing ;` | `` |
| 159 | 79 | `missing ;` | `` |
| 173 | 79 | `missing ;` | `` |
| 186 | 79 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_client_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 3 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 97 | `missing ;` | `` |
| 28 | 97 | `missing ;` | `` |
| 38 | 97 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_app_manager_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 10 | 117 | `ERROR` | `,` |
| 16 | 120 | `ERROR` | `,` |
| 21 | 109 | `ERROR` | `,` |
| 26 | 112 | `ERROR` | `,` |
| 30 | 91 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_beauty_notification_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 100 | `missing ;` | `` |
| 31 | 100 | `missing ;` | `` |
| 53 | 100 | `missing ;` | `` |
| 75 | 100 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_fwk_metadata_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 126 | `ERROR` | `,` |
| 17 | 129 | `ERROR` | `,` |
| 22 | 118 | `ERROR` | `,` |
| 27 | 121 | `ERROR` | `,` |
| 31 | 101 | `missing ;` | `` |
| 46 | 101 | `missing ;` | `` |
| 74 | 101 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_info_dumper_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 120 | `ERROR` | `,` |
| 17 | 123 | `ERROR` | `,` |
| 22 | 112 | `ERROR` | `,` |
| 27 | 115 | `ERROR` | `,` |
| 31 | 88 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_privacy_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 117 | `ERROR` | `,` |
| 17 | 120 | `ERROR` | `,` |
| 22 | 109 | `ERROR` | `,` |
| 27 | 112 | `ERROR` | `,` |
| 31 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/camera_util_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 114 | `ERROR` | `,` |
| 25 | 117 | `ERROR` | `,` |
| 30 | 106 | `ERROR` | `,` |
| 35 | 109 | `ERROR` | `,` |
| 39 | 75 | `missing ;` | `` |
| 63 | 75 | `missing ;` | `` |
| 108 | 75 | `missing ;` | `` |
| 144 | 75 | `missing ;` | `` |
| 154 | 75 | `missing ;` | `` |
| 196 | 75 | `missing ;` | `` |
| 206 | 75 | `missing ;` | `` |
| 219 | 75 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/camera_service_common/src/icamera_util_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 19 | 73 | `missing ;` | `` |

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
| 145 | 216 | `ERROR` | `PRIu64` |
| 152 | 222 | `ERROR` | `PRIu64` |
| 159 | 216 | `ERROR` | `PRIu64` |
| 165 | 177 | `ERROR` | `,` |
| 178 | 99 | `ERROR` | `,` |
| 183 | 167 | `ERROR` | `,` |
| … | … | … | *(48 more)* |

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
| 66 | 96 | `missing ;` | `` |
| 90 | 96 | `missing ;` | `` |
| 102 | 96 | `missing ;` | `` |
| 120 | 96 | `missing ;` | `` |
| 137 | 96 | `missing ;` | `` |
| 164 | 96 | `missing ;` | `` |
| 177 | 96 | `missing ;` | `` |
| 197 | 96 | `missing ;` | `` |
| 205 | 96 | `missing ;` | `` |
| 228 | 96 | `missing ;` | `` |
| 240 | 96 | `missing ;` | `` |
| 257 | 96 | `missing ;` | `` |
| 265 | 96 | `missing ;` | `` |
| 290 | 96 | `missing ;` | `` |
| … | … | … | *(10 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_device_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 14 | 113 | `ERROR` | `,` |
| 22 | 116 | `ERROR` | `,` |
| 27 | 77 | `ERROR` | `,` |
| 34 | 85 | `ERROR` | `,` |
| 37 | 83 | `ERROR` | `,` |
| 41 | 77 | `missing ;` | `` |
| 57 | 77 | `missing ;` | `` |
| 79 | 77 | `missing ;` | `` |
| 102 | 77 | `missing ;` | `` |
| 125 | 77 | `missing ;` | `` |
| 140 | 77 | `missing ;` | `` |
| 155 | 77 | `missing ;` | `` |
| 170 | 77 | `missing ;` | `` |
| 194 | 77 | `missing ;` | `` |
| 213 | 77 | `missing ;` | `` |
| 256 | 77 | `missing ;` | `` |
| 295 | 77 | `missing ;` | `` |
| 330 | 77 | `missing ;` | `` |
| 343 | 77 | `missing ;` | `` |
| 362 | 77 | `missing ;` | `` |
| … | … | … | *(52 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_host_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 118 | `ERROR` | `,` |
| 19 | 121 | `ERROR` | `,` |
| 24 | 77 | `ERROR` | `,` |
| 32 | 85 | `ERROR` | `,` |
| 48 | 88 | `missing ;` | `` |
| 67 | 88 | `missing ;` | `` |
| 83 | 88 | `missing ;` | `` |
| 106 | 88 | `missing ;` | `` |
| 141 | 88 | `missing ;` | `` |
| 163 | 88 | `missing ;` | `` |
| 217 | 88 | `missing ;` | `` |
| 238 | 88 | `missing ;` | `` |
| 275 | 88 | `missing ;` | `` |
| 297 | 88 | `missing ;` | `` |
| 318 | 88 | `missing ;` | `` |
| 340 | 88 | `missing ;` | `` |
| 366 | 88 | `missing ;` | `` |
| 387 | 88 | `missing ;` | `` |
| 410 | 88 | `missing ;` | `` |
| 425 | 88 | `missing ;` | `` |
| … | … | … | *(21 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_preconfig_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 9 | 122 | `ERROR` | `,` |
| 14 | 125 | `ERROR` | `,` |
| 19 | 77 | `ERROR` | `,` |
| 24 | 85 | `ERROR` | `,` |
| 28 | 78 | `missing ;` | `` |
| 53 | 78 | `missing ;` | `` |
| 78 | 78 | `missing ;` | `` |
| 103 | 78 | `missing ;` | `` |
| 126 | 78 | `missing ;` | `` |
| 149 | 78 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_restore_param_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 11 | 123 | `ERROR` | `,` |
| 16 | 126 | `ERROR` | `,` |
| 21 | 115 | `ERROR` | `,` |
| 26 | 118 | `ERROR` | `,` |
| 30 | 94 | `missing ;` | `` |
| 52 | 94 | `missing ;` | `` |
| 65 | 94 | `missing ;` | `` |
| 79 | 94 | `missing ;` | `` |
| 97 | 94 | `missing ;` | `` |
| 109 | 94 | `missing ;` | `` |
| 121 | 94 | `missing ;` | `` |
| 135 | 94 | `missing ;` | `` |
| 147 | 94 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hcamera_service_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 85 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 76 | 79 | `missing ;` | `` |
| 89 | 79 | `missing ;` | `` |
| 122 | 79 | `missing ;` | `` |
| 154 | 79 | `missing ;` | `` |
| 200 | 79 | `missing ;` | `` |
| 244 | 79 | `missing ;` | `` |
| 258 | 79 | `missing ;` | `` |
| 267 | 79 | `missing ;` | `` |
| 280 | 79 | `missing ;` | `` |
| 288 | 79 | `missing ;` | `` |
| 295 | 79 | `missing ;` | `` |
| 315 | 79 | `missing ;` | `` |
| 348 | 79 | `missing ;` | `` |
| 377 | 79 | `missing ;` | `` |
| 413 | 79 | `missing ;` | `` |
| 440 | 79 | `missing ;` | `` |
| 463 | 79 | `missing ;` | `` |
| 517 | 79 | `missing ;` | `` |
| 571 | 79 | `missing ;` | `` |
| 602 | 79 | `missing ;` | `` |
| … | … | … | *(65 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_camera_test/src/hshared_camera_device_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 34 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 55 | 95 | `missing ;` | `` |
| 77 | 95 | `missing ;` | `` |
| 105 | 95 | `missing ;` | `` |
| 126 | 95 | `missing ;` | `` |
| 148 | 95 | `missing ;` | `` |
| 169 | 95 | `missing ;` | `` |
| 191 | 95 | `missing ;` | `` |
| 213 | 95 | `missing ;` | `` |
| 240 | 95 | `missing ;` | `` |
| 264 | 95 | `missing ;` | `` |
| 291 | 95 | `missing ;` | `` |
| 330 | 95 | `missing ;` | `` |
| 359 | 95 | `missing ;` | `` |
| 379 | 95 | `missing ;` | `` |
| 403 | 95 | `missing ;` | `` |
| 423 | 95 | `missing ;` | `` |
| 443 | 95 | `missing ;` | `` |
| 468 | 94 | `missing ;` | `` |
| 500 | 94 | `missing ;` | `` |
| 527 | 94 | `missing ;` | `` |
| … | … | … | *(14 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/include/hstream_operator_unittest.h`

**Summary:** tree-sitter-cpp node `missing type_identifier` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 79 | `missing type_identifier` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hcapture_session_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 68 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 70 | 86 | `missing ;` | `` |
| 117 | 86 | `missing ;` | `` |
| 152 | 86 | `missing ;` | `` |
| 202 | 86 | `missing ;` | `` |
| 217 | 86 | `missing ;` | `` |
| 272 | 86 | `missing ;` | `` |
| 309 | 86 | `missing ;` | `` |
| 364 | 86 | `missing ;` | `` |
| 379 | 86 | `missing ;` | `` |
| 393 | 86 | `missing ;` | `` |
| 439 | 86 | `missing ;` | `` |
| 494 | 86 | `missing ;` | `` |
| 515 | 86 | `missing ;` | `` |
| 540 | 86 | `missing ;` | `` |
| 592 | 86 | `missing ;` | `` |
| 640 | 86 | `missing ;` | `` |
| 694 | 86 | `missing ;` | `` |
| 742 | 86 | `missing ;` | `` |
| 780 | 86 | `missing ;` | `` |
| 821 | 86 | `missing ;` | `` |
| … | … | … | *(48 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hshared_capture_session_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 36 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 52 | 99 | `missing ;` | `` |
| 71 | 99 | `missing ;` | `` |
| 92 | 99 | `missing ;` | `` |
| 136 | 99 | `missing ;` | `` |
| 155 | 99 | `missing ;` | `` |
| 173 | 99 | `missing ;` | `` |
| 194 | 99 | `missing ;` | `` |
| 215 | 99 | `missing ;` | `` |
| 236 | 99 | `missing ;` | `` |
| 255 | 99 | `missing ;` | `` |
| 274 | 99 | `missing ;` | `` |
| 292 | 99 | `missing ;` | `` |
| 314 | 99 | `missing ;` | `` |
| 338 | 99 | `missing ;` | `` |
| 359 | 99 | `missing ;` | `` |
| 377 | 99 | `missing ;` | `` |
| 403 | 99 | `missing ;` | `` |
| 429 | 99 | `missing ;` | `` |
| 460 | 99 | `missing ;` | `` |
| 487 | 99 | `missing ;` | `` |
| … | … | … | *(16 more)* |

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
| 136 | 70 | `missing ;` | `` |
| 149 | 75 | `missing ;` | `` |
| 166 | 79 | `missing ;` | `` |
| 197 | 79 | `missing ;` | `` |
| 220 | 71 | `missing ;` | `` |
| 244 | 71 | `missing ;` | `` |
| 266 | 71 | `missing ;` | `` |
| 288 | 71 | `missing ;` | `` |
| 310 | 69 | `missing ;` | `` |
| 329 | 69 | `missing ;` | `` |
| 348 | 72 | `missing ;` | `` |
| 368 | 72 | `missing ;` | `` |
| 388 | 72 | `missing ;` | `` |
| 411 | 72 | `missing ;` | `` |
| … | … | … | *(37 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_depth_data_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 12 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 88 | `missing ;` | `` |
| 53 | 88 | `missing ;` | `` |
| 75 | 88 | `missing ;` | `` |
| 96 | 88 | `missing ;` | `` |
| 118 | 88 | `missing ;` | `` |
| 140 | 88 | `missing ;` | `` |
| 163 | 88 | `missing ;` | `` |
| 179 | 88 | `missing ;` | `` |
| 193 | 88 | `missing ;` | `` |
| 232 | 88 | `missing ;` | `` |
| 264 | 88 | `missing ;` | `` |
| 298 | 88 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_metadata_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 13 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 98 | 81 | `missing ;` | `` |
| 113 | 81 | `missing ;` | `` |
| 135 | 81 | `missing ;` | `` |
| 156 | 81 | `missing ;` | `` |
| 174 | 81 | `missing ;` | `` |
| 202 | 81 | `missing ;` | `` |
| 220 | 81 | `missing ;` | `` |
| 244 | 81 | `missing ;` | `` |
| 261 | 81 | `missing ;` | `` |
| 290 | 81 | `missing ;` | `` |
| 320 | 82 | `missing ;` | `` |
| 346 | 82 | `missing ;` | `` |
| 364 | 82 | `missing ;` | `` |

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
| 77 | 81 | `missing ;` | `` |
| 87 | 81 | `missing ;` | `` |
| 98 | 83 | `missing ;` | `` |
| 107 | 83 | `missing ;` | `` |
| 115 | 69 | `missing ;` | `` |
| 122 | 69 | `missing ;` | `` |
| 129 | 72 | `missing ;` | `` |
| 136 | 72 | `missing ;` | `` |
| … | … | … | *(38 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/hdi_stream_test/src/hstream_repeat_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 86 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 156 | 77 | `missing ;` | `` |
| 168 | 77 | `missing ;` | `` |
| 180 | 77 | `missing ;` | `` |
| 193 | 77 | `missing ;` | `` |
| 204 | 77 | `missing ;` | `` |
| 214 | 77 | `missing ;` | `` |
| 225 | 77 | `missing ;` | `` |
| 235 | 77 | `missing ;` | `` |
| 256 | 77 | `missing ;` | `` |
| 275 | 77 | `missing ;` | `` |
| 290 | 77 | `missing ;` | `` |
| 301 | 77 | `missing ;` | `` |
| 312 | 77 | `missing ;` | `` |
| 324 | 77 | `missing ;` | `` |
| 336 | 77 | `missing ;` | `` |
| 348 | 83 | `missing ;` | `` |
| 360 | 83 | `missing ;` | `` |
| 372 | 83 | `missing ;` | `` |
| 384 | 77 | `missing ;` | `` |
| 396 | 77 | `missing ;` | `` |
| … | … | … | *(66 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/media_library/src/photo_asset_adapter_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 86 | `missing ;` | `` |
| 44 | 86 | `missing ;` | `` |
| 79 | 86 | `missing ;` | `` |
| 89 | 86 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 128 | `ERROR` | `,` |
| 17 | 131 | `ERROR` | `,` |
| 23 | 91 | `ERROR` | `,` |
| 32 | 120 | `ERROR` | `,` |
| 37 | 123 | `ERROR` | `,` |
| 41 | 105 | `missing ;` | `` |
| 79 | 105 | `missing ;` | `` |
| 88 | 105 | `missing ;` | `` |
| 92 | 5 | `ERROR` | `=` |
| 93 | 8 | `ERROR` | `=` |
| 94 | 8 | `ERROR` | `=` |
| 95 | 9 | `missing literal_suffix` | `` |
| 95 | 11 | `ERROR` | `1` |
| 96 | 26 | `ERROR` | `.11.10.20.100) "` |
| 96 | 43 | `ERROR` | `"` |
| 104 | 101 | `ERROR` | `,` |
| 120 | 105 | `missing ;` | `` |
| 130 | 105 | `missing ;` | `` |
| 132 | 56 | `ERROR` | `1.0` |
| 132 | 78 | `ERROR` | `8` |
| … | … | … | *(12 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_reader_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 13 | 127 | `ERROR` | `,` |
| 18 | 130 | `ERROR` | `,` |
| 24 | 91 | `ERROR` | `,` |
| 33 | 119 | `ERROR` | `,` |
| 38 | 122 | `ERROR` | `,` |
| 42 | 103 | `missing ;` | `` |
| 51 | 103 | `missing ;` | `` |
| 62 | 103 | `missing ;` | `` |
| 66 | 5 | `ERROR` | `=` |
| 67 | 8 | `ERROR` | `=` |
| 68 | 8 | `ERROR` | `=` |
| 69 | 9 | `missing literal_suffix` | `` |
| 69 | 11 | `ERROR` | `1` |
| 70 | 26 | `ERROR` | `.11.10.20.100) "` |
| 70 | 43 | `ERROR` | `"` |
| 76 | 91 | `ERROR` | `,` |
| 85 | 101 | `ERROR` | `,` |
| 99 | 103 | `missing ;` | `` |
| 113 | 103 | `missing ;` | `` |
| 133 | 103 | `missing ;` | `` |
| … | … | … | *(8 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/param_update/src/camera_rotate_param_sign_tools_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 130 | `ERROR` | `,` |
| 21 | 133 | `ERROR` | `,` |
| 26 | 122 | `ERROR` | `,` |
| 31 | 125 | `ERROR` | `,` |
| 35 | 110 | `missing ;` | `` |
| 51 | 110 | `missing ;` | `` |
| 62 | 110 | `missing ;` | `` |
| 72 | 110 | `missing ;` | `` |
| 84 | 110 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/pipeline/src/camera_common_pipeline_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 28 | `missing type_identifier` | `` |
| 20 | 32 | `missing type_identifier` | `` |
| 43 | 63 | `missing ;` | `` |
| 63 | 62 | `missing ;` | `` |
| 79 | 70 | `missing ;` | `` |
| 102 | 66 | `missing ;` | `` |
| 146 | 65 | `missing ;` | `` |
| 161 | 67 | `missing ;` | `` |
| 191 | 65 | `missing ;` | `` |
| 220 | 68 | `missing ;` | `` |
| 243 | 71 | `missing ;` | `` |
| 273 | 65 | `missing ;` | `` |
| 305 | 75 | `missing ;` | `` |
| 315 | 84 | `ERROR` | `,` |
| 329 | 77 | `missing ;` | `` |
| 360 | 77 | `missing ;` | `` |
| 397 | 81 | `missing ;` | `` |
| 413 | 84 | `missing ;` | `` |
| 481 | 79 | `missing ;` | `` |
| 488 | 83 | `missing ;` | `` |
| … | … | … | *(24 more)* |

#### `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/cubic_bezier_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 115 | `ERROR` | `,` |
| 17 | 118 | `ERROR` | `,` |
| 22 | 107 | `ERROR` | `,` |
| 27 | 110 | `ERROR` | `,` |
| 31 | 77 | `missing ;` | `` |
| 41 | 77 | `missing ;` | `` |
| 51 | 77 | `missing ;` | `` |
| 64 | 77 | `missing ;` | `` |
| 74 | 77 | `missing ;` | `` |
| 84 | 77 | `missing ;` | `` |
| 94 | 77 | `missing ;` | `` |
| 104 | 77 | `missing ;` | `` |
| 113 | 77 | `missing ;` | `` |
| 124 | 77 | `missing ;` | `` |
| 135 | 77 | `missing ;` | `` |
| 146 | 77 | `missing ;` | `` |
| 157 | 77 | `missing ;` | `` |
| 170 | 77 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/camera_service/smooth_zoom/src/smooth_zoom_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 114 | `ERROR` | `,` |
| 17 | 117 | `ERROR` | `,` |
| 22 | 106 | `ERROR` | `,` |
| 27 | 109 | `ERROR` | `,` |
| 31 | 75 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/ability/src/camera_ability_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 12 | 117 | `ERROR` | `,` |
| 17 | 120 | `ERROR` | `,` |
| 22 | 109 | `ERROR` | `,` |
| 27 | 112 | `ERROR` | `,` |
| 31 | 81 | `missing ;` | `` |
| 44 | 81 | `missing ;` | `` |
| 58 | 81 | `missing ;` | `` |
| 68 | 81 | `missing ;` | `` |
| 79 | 81 | `missing ;` | `` |
| 87 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/camera_utils/src/camera_utils_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 18 | 115 | `ERROR` | `,` |
| 23 | 118 | `ERROR` | `,` |
| 28 | 107 | `ERROR` | `,` |
| 33 | 110 | `ERROR` | `,` |
| 37 | 77 | `missing ;` | `` |
| 68 | 77 | `missing ;` | `` |
| 89 | 77 | `missing ;` | `` |
| 107 | 77 | `missing ;` | `` |
| 125 | 77 | `missing ;` | `` |
| 143 | 77 | `missing ;` | `` |
| 161 | 77 | `missing ;` | `` |
| 179 | 77 | `missing ;` | `` |
| 197 | 77 | `missing ;` | `` |
| 207 | 77 | `missing ;` | `` |
| 214 | 77 | `missing ;` | `` |
| 225 | 77 | `missing ;` | `` |
| 244 | 77 | `missing ;` | `` |
| 263 | 77 | `missing ;` | `` |
| 270 | 77 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/device/src/camera_device_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 9 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 75 | `missing ;` | `` |
| 39 | 75 | `missing ;` | `` |
| 52 | 75 | `missing ;` | `` |
| 63 | 75 | `missing ;` | `` |
| 75 | 75 | `missing ;` | `` |
| 84 | 75 | `missing ;` | `` |
| 99 | 75 | `missing ;` | `` |
| 110 | 75 | `missing ;` | `` |
| 118 | 75 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/input/src/camera_framework_input_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 16 | 235 | `ERROR` | `,` |
| 40 | 92 | `missing ;` | `` |
| 54 | 92 | `missing ;` | `` |
| 68 | 92 | `missing ;` | `` |
| 82 | 92 | `missing ;` | `` |
| 99 | 92 | `missing ;` | `` |
| 114 | 92 | `missing ;` | `` |
| 158 | 92 | `missing ;` | `` |
| 175 | 92 | `missing ;` | `` |
| 206 | 92 | `missing ;` | `` |
| 237 | 92 | `missing ;` | `` |
| 269 | 92 | `missing ;` | `` |
| 302 | 92 | `missing ;` | `` |
| 319 | 92 | `missing ;` | `` |
| 363 | 92 | `missing ;` | `` |
| 382 | 92 | `missing ;` | `` |
| 403 | 92 | `missing ;` | `` |
| 433 | 92 | `missing ;` | `` |
| 452 | 92 | `missing ;` | `` |
| 504 | 92 | `missing ;` | `` |
| … | … | … | *(44 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/manager/src/camera_framework_manager_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 280 | `ERROR` | `,` |
| 47 | 96 | `missing ;` | `` |
| 60 | 96 | `missing ;` | `` |
| 72 | 96 | `missing ;` | `` |
| 84 | 96 | `missing ;` | `` |
| 101 | 96 | `missing ;` | `` |
| 110 | 96 | `missing ;` | `` |
| 137 | 96 | `missing ;` | `` |
| 155 | 96 | `missing ;` | `` |
| 179 | 96 | `missing ;` | `` |
| 200 | 96 | `missing ;` | `` |
| 216 | 96 | `missing ;` | `` |
| 230 | 96 | `missing ;` | `` |
| 243 | 96 | `missing ;` | `` |
| 254 | 96 | `missing ;` | `` |
| 276 | 96 | `missing ;` | `` |
| 309 | 96 | `missing ;` | `` |
| 319 | 96 | `missing ;` | `` |
| 331 | 96 | `missing ;` | `` |
| 341 | 96 | `missing ;` | `` |
| … | … | … | *(74 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/manager/src/prelaunch_config_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 81 | `missing ;` | `` |
| 35 | 81 | `missing ;` | `` |
| 45 | 81 | `missing ;` | `` |
| 55 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/camera_output_capability_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 96 | `missing ;` | `` |
| 40 | 96 | `missing ;` | `` |
| 54 | 96 | `missing ;` | `` |
| 72 | 96 | `missing ;` | `` |
| 90 | 96 | `missing ;` | `` |
| 124 | 96 | `missing ;` | `` |
| 140 | 96 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/camera_photo_proxy_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 5 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 84 | `missing ;` | `` |
| 42 | 84 | `missing ;` | `` |
| 60 | 84 | `missing ;` | `` |
| 78 | 84 | `missing ;` | `` |
| 91 | 84 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/capture_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 4 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 63 | 83 | `missing ;` | `` |
| 109 | 83 | `missing ;` | `` |
| 136 | 83 | `missing ;` | `` |
| 160 | 83 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/deferred_photo_proxy_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 88 | `missing ;` | `` |
| 39 | 88 | `missing ;` | `` |
| 48 | 88 | `missing ;` | `` |
| 61 | 88 | `missing ;` | `` |
| 71 | 88 | `missing ;` | `` |
| 107 | 88 | `missing ;` | `` |
| 133 | 88 | `missing ;` | `` |
| 144 | 88 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/depth_data_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 39 | 82 | `missing ;` | `` |
| 85 | 82 | `missing ;` | `` |
| 133 | 82 | `missing ;` | `` |
| 179 | 82 | `missing ;` | `` |
| 232 | 82 | `missing ;` | `` |
| 281 | 82 | `missing ;` | `` |
| 334 | 82 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/metadata_output_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 38 | 128 | `ERROR` | `,` |
| 42 | 157 | `ERROR` | `,` |
| 47 | 85 | `missing ;` | `` |
| 88 | 85 | `missing ;` | `` |
| 129 | 85 | `missing ;` | `` |
| 166 | 85 | `missing ;` | `` |
| 187 | 85 | `missing ;` | `` |
| 212 | 85 | `missing ;` | `` |
| 237 | 85 | `missing ;` | `` |
| 251 | 85 | `missing ;` | `` |
| 266 | 85 | `missing ;` | `` |
| 283 | 85 | `missing ;` | `` |
| 310 | 85 | `missing ;` | `` |
| 359 | 85 | `missing ;` | `` |
| 408 | 85 | `missing ;` | `` |
| 453 | 85 | `missing ;` | `` |
| 512 | 85 | `missing ;` | `` |
| 552 | 85 | `missing ;` | `` |
| 595 | 85 | `missing ;` | `` |
| 608 | 85 | `missing ;` | `` |
| … | … | … | *(15 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/photo_output_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 74 | `missing ;` | `` |
| 70 | 79 | `missing ;` | `` |
| 108 | 79 | `missing ;` | `` |
| 122 | 79 | `missing ;` | `` |
| 160 | 79 | `missing ;` | `` |
| 180 | 79 | `missing ;` | `` |
| 198 | 79 | `missing ;` | `` |
| 214 | 79 | `missing ;` | `` |
| 262 | 79 | `missing ;` | `` |
| 310 | 79 | `missing ;` | `` |
| 357 | 79 | `missing ;` | `` |
| 369 | 79 | `missing ;` | `` |
| 417 | 79 | `missing ;` | `` |
| 458 | 79 | `missing ;` | `` |
| 496 | 79 | `missing ;` | `` |
| 545 | 79 | `missing ;` | `` |
| 567 | 79 | `missing ;` | `` |
| 583 | 79 | `missing ;` | `` |
| 598 | 79 | `missing ;` | `` |
| 613 | 79 | `missing ;` | `` |
| … | … | … | *(57 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/preview_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 33 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 61 | 83 | `missing ;` | `` |
| 97 | 83 | `missing ;` | `` |
| 138 | 83 | `missing ;` | `` |
| 155 | 83 | `missing ;` | `` |
| 204 | 83 | `missing ;` | `` |
| 266 | 83 | `missing ;` | `` |
| 323 | 83 | `missing ;` | `` |
| 344 | 83 | `missing ;` | `` |
| 387 | 83 | `missing ;` | `` |
| 427 | 83 | `missing ;` | `` |
| 448 | 83 | `missing ;` | `` |
| 510 | 83 | `missing ;` | `` |
| 530 | 83 | `missing ;` | `` |
| 550 | 83 | `missing ;` | `` |
| 567 | 83 | `missing ;` | `` |
| 588 | 83 | `missing ;` | `` |
| 613 | 83 | `missing ;` | `` |
| 631 | 83 | `missing ;` | `` |
| 643 | 83 | `missing ;` | `` |
| 664 | 83 | `missing ;` | `` |
| … | … | … | *(13 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/sketch_wrapper_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 8 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 37 | 89 | `missing ;` | `` |
| 86 | 89 | `missing ;` | `` |
| 129 | 89 | `missing ;` | `` |
| 166 | 89 | `missing ;` | `` |
| 209 | 89 | `missing ;` | `` |
| 281 | 89 | `missing ;` | `` |
| 329 | 89 | `missing ;` | `` |
| 377 | 89 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/output/src/video_output_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 17 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 79 | `missing ;` | `` |
| 104 | 79 | `missing ;` | `` |
| 156 | 79 | `missing ;` | `` |
| 210 | 79 | `missing ;` | `` |
| 243 | 79 | `missing ;` | `` |
| 277 | 79 | `missing ;` | `` |
| 312 | 79 | `missing ;` | `` |
| 335 | 79 | `missing ;` | `` |
| 356 | 79 | `missing ;` | `` |
| 376 | 79 | `missing ;` | `` |
| 402 | 79 | `missing ;` | `` |
| 422 | 79 | `missing ;` | `` |
| 452 | 79 | `missing ;` | `` |
| 472 | 79 | `missing ;` | `` |
| 492 | 88 | `missing ;` | `` |
| 515 | 88 | `missing ;` | `` |
| 525 | 88 | `missing ;` | `` |

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
| 30 | 94 | `missing ;` | `` |
| 38 | 94 | `missing ;` | `` |
| 48 | 94 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_dfx_utils_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 6 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 49 | 101 | `missing ;` | `` |
| 85 | 101 | `missing ;` | `` |
| 117 | 101 | `missing ;` | `` |
| 133 | 101 | `missing ;` | `` |
| 151 | 101 | `missing ;` | `` |
| 169 | 101 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/capture_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 174 | 83 | `missing ;` | `` |
| 188 | 83 | `missing ;` | `` |
| 197 | 83 | `missing ;` | `` |
| 213 | 83 | `missing ;` | `` |
| 246 | 83 | `missing ;` | `` |
| 277 | 83 | `missing ;` | `` |
| 291 | 83 | `missing ;` | `` |
| 322 | 83 | `missing ;` | `` |
| 332 | 83 | `missing ;` | `` |
| 361 | 83 | `missing ;` | `` |
| 397 | 83 | `missing ;` | `` |
| 432 | 83 | `missing ;` | `` |
| 444 | 83 | `missing ;` | `` |
| 477 | 83 | `missing ;` | `` |
| 487 | 83 | `missing ;` | `` |
| 525 | 83 | `missing ;` | `` |
| 558 | 83 | `missing ;` | `` |
| 592 | 83 | `missing ;` | `` |
| 607 | 83 | `missing ;` | `` |
| 620 | 83 | `missing ;` | `` |
| … | … | … | *(405 more)* |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/cinematic_video_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 112 | `ERROR` | `,` |
| 34 | 107 | `missing ;` | `` |
| 49 | 107 | `missing ;` | `` |
| 73 | 107 | `missing ;` | `` |
| 97 | 107 | `missing ;` | `` |
| 126 | 107 | `missing ;` | `` |
| 165 | 107 | `missing ;` | `` |
| 206 | 107 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/composition_feature_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 14 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 69 | 98 | `missing ;` | `` |
| 77 | 93 | `missing ;` | `` |
| 89 | 98 | `missing ;` | `` |
| 97 | 89 | `missing ;` | `` |
| 112 | 89 | `missing ;` | `` |
| 126 | 98 | `missing ;` | `` |
| 141 | 98 | `missing ;` | `` |
| 151 | 100 | `missing ;` | `` |
| 166 | 100 | `missing ;` | `` |
| 176 | 92 | `missing ;` | `` |
| 196 | 72 | `missing ;` | `` |
| 206 | 72 | `missing ;` | `` |
| 216 | 81 | `missing ;` | `` |
| 236 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/mech_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 29 | 100 | `ERROR` | `,` |
| 103 | 77 | `missing ;` | `` |
| 106 | 167 | `ERROR` | `,` |
| 110 | 77 | `missing ;` | `` |
| 120 | 77 | `missing ;` | `` |
| 134 | 77 | `missing ;` | `` |
| 143 | 77 | `missing ;` | `` |
| 158 | 77 | `missing ;` | `` |
| 175 | 77 | `missing ;` | `` |
| 193 | 77 | `missing ;` | `` |
| 223 | 77 | `missing ;` | `` |
| 245 | 77 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/moon_capture_boost_feature_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 105 | `ERROR` | `,` |
| 29 | 97 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/night_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 101 | 105 | `ERROR` | `,` |
| 105 | 81 | `missing ;` | `` |
| 165 | 81 | `missing ;` | `` |
| 223 | 81 | `missing ;` | `` |
| 283 | 81 | `missing ;` | `` |
| 360 | 81 | `missing ;` | `` |
| 439 | 81 | `missing ;` | `` |
| 518 | 81 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/panorama_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 106 | `ERROR` | `,` |
| 34 | 86 | `missing ;` | `` |
| 84 | 86 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 26 | 107 | `ERROR` | `,` |
| 149 | 92 | `missing ;` | `` |
| 188 | 92 | `missing ;` | `` |
| 227 | 107 | `missing ;` | `` |
| 240 | 94 | `missing ;` | `` |
| 265 | 94 | `missing ;` | `` |
| 289 | 105 | `missing ;` | `` |
| 302 | 94 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/portrait_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 233 | 110 | `ERROR` | `,` |
| 237 | 91 | `missing ;` | `` |
| 309 | 91 | `missing ;` | `` |
| 377 | 91 | `missing ;` | `` |
| 446 | 91 | `missing ;` | `` |
| 515 | 91 | `missing ;` | `` |
| 584 | 91 | `missing ;` | `` |
| 661 | 91 | `missing ;` | `` |
| 733 | 91 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/profession_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 116 | `ERROR` | `,` |
| 38 | 119 | `ERROR` | `,` |
| 67 | 111 | `ERROR` | `,` |
| 78 | 105 | `ERROR` | `,` |
| 163 | 96 | `missing ;` | `` |
| 230 | 96 | `missing ;` | `` |
| 249 | 98 | `missing ;` | `` |
| 280 | 98 | `missing ;` | `` |
| 296 | 98 | `missing ;` | `` |
| 323 | 98 | `missing ;` | `` |
| 352 | 98 | `missing ;` | `` |
| 381 | 98 | `missing ;` | `` |
| 410 | 98 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/scan_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 115 | 107 | `ERROR` | `,` |
| 119 | 83 | `missing ;` | `` |
| 169 | 83 | `missing ;` | `` |
| 223 | 83 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/secure_camera_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 62 | 109 | `ERROR` | `,` |
| 66 | 92 | `missing ;` | `` |
| 114 | 92 | `missing ;` | `` |
| 166 | 92 | `missing ;` | `` |
| 216 | 92 | `missing ;` | `` |
| 265 | 92 | `missing ;` | `` |
| 320 | 92 | `missing ;` | `` |
| 373 | 103 | `missing ;` | `` |
| 385 | 103 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/slow_motion_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 15 | 117 | `ERROR` | `,` |
| 37 | 112 | `ERROR` | `,` |
| 133 | 96 | `missing ;` | `` |
| 144 | 98 | `missing ;` | `` |
| 204 | 96 | `missing ;` | `` |
| 283 | 105 | `missing ;` | `` |
| 308 | 105 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/stitching_photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 30 | 112 | `ERROR` | `,` |
| 34 | 107 | `missing ;` | `` |
| 81 | 107 | `missing ;` | `` |
| 151 | 107 | `missing ;` | `` |
| 221 | 107 | `missing ;` | `` |
| 318 | 111 | `missing ;` | `` |
| 340 | 111 | `missing ;` | `` |
| 362 | 111 | `missing ;` | `` |
| 384 | 111 | `missing ;` | `` |
| 411 | 111 | `missing ;` | `` |
| 450 | 111 | `missing ;` | `` |
| 480 | 111 | `missing ;` | `` |
| 512 | 111 | `missing ;` | `` |
| 543 | 111 | `missing ;` | `` |
| 580 | 107 | `missing ;` | `` |
| 614 | 111 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/time_lapse_photo_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 25 | 110 | `ERROR` | `,` |
| 29 | 100 | `missing ;` | `` |

#### `frameworks/native/camera/test/unittest/framework_native/session/src/video_session_unittest.cpp`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 42 | 108 | `ERROR` | `,` |
| 196 | 85 | `missing ;` | `` |
| 233 | 85 | `missing ;` | `` |
| 271 | 85 | `missing ;` | `` |
| 289 | 85 | `missing ;` | `` |
| 319 | 85 | `missing ;` | `` |
| 362 | 85 | `missing ;` | `` |
| 391 | 85 | `missing ;` | `` |
| 437 | 85 | `missing ;` | `` |
| 493 | 85 | `missing ;` | `` |
| 545 | 85 | `missing ;` | `` |
| 604 | 85 | `missing ;` | `` |
| 620 | 85 | `missing ;` | `` |
| 661 | 85 | `missing ;` | `` |
| 711 | 89 | `missing ;` | `` |
| 734 | 89 | `missing ;` | `` |
| 757 | 89 | `missing ;` | `` |
| 780 | 89 | `missing ;` | `` |
| 803 | 89 | `missing ;` | `` |
| 827 | 89 | `missing ;` | `` |
| … | … | … | *(6 more)* |

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
| … | … | … | *(71 more)* |

#### `frameworks/native/camera/test/unittest/movie_file/src/movie_file_audio_metadata_unittest.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 10 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 48 | 99 | `missing ;` | `` |
| 73 | 99 | `missing ;` | `` |
| 86 | 96 | `missing ;` | `` |
| 103 | 121 | `missing ;` | `` |
| 118 | 114 | `missing ;` | `` |
| 135 | 115 | `missing ;` | `` |
| 150 | 116 | `missing ;` | `` |
| 163 | 111 | `missing ;` | `` |
| 181 | 121 | `missing ;` | `` |
| 197 | 117 | `missing ;` | `` |

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

#### `mediastream/test/unittest/filter/src/audio_capture_adapter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 29 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 65 | 81 | `missing ;` | `` |
| 85 | 81 | `missing ;` | `` |
| 93 | 81 | `missing ;` | `` |
| 102 | 81 | `missing ;` | `` |
| 112 | 84 | `missing ;` | `` |
| 134 | 82 | `missing ;` | `` |
| 160 | 81 | `missing ;` | `` |
| 195 | 81 | `missing ;` | `` |
| 205 | 81 | `missing ;` | `` |
| 216 | 81 | `missing ;` | `` |
| 238 | 97 | `missing ;` | `` |
| 254 | 97 | `missing ;` | `` |
| 263 | 91 | `missing ;` | `` |
| 271 | 92 | `missing ;` | `` |
| 302 | 104 | `missing ;` | `` |
| 319 | 95 | `missing ;` | `` |
| 330 | 77 | `missing ;` | `` |
| 339 | 77 | `missing ;` | `` |
| 348 | 77 | `missing ;` | `` |
| 357 | 82 | `missing ;` | `` |
| … | … | … | *(9 more)* |

#### `mediastream/test/unittest/filter/src/audio_capture_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 27 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 23 | 86 | `missing ;` | `` |
| 37 | 101 | `missing ;` | `` |
| 52 | 111 | `missing ;` | `` |
| 61 | 91 | `missing ;` | `` |
| 75 | 89 | `missing ;` | `` |
| 87 | 89 | `missing ;` | `` |
| 97 | 90 | `missing ;` | `` |
| 111 | 88 | `missing ;` | `` |
| 123 | 89 | `missing ;` | `` |
| 130 | 91 | `missing ;` | `` |
| 143 | 94 | `missing ;` | `` |
| 151 | 94 | `missing ;` | `` |
| 160 | 90 | `missing ;` | `` |
| 174 | 95 | `missing ;` | `` |
| 181 | 96 | `missing ;` | `` |
| 192 | 89 | `missing ;` | `` |
| 206 | 110 | `missing ;` | `` |
| 219 | 97 | `missing ;` | `` |
| 233 | 96 | `missing ;` | `` |
| 245 | 92 | `missing ;` | `` |
| … | … | … | *(7 more)* |

#### `mediastream/test/unittest/filter/src/audio_encoder_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 29 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 131 | 32 | `missing type_identifier` | `` |
| 131 | 104 | `missing type_identifier` | `` |
| 168 | 93 | `missing ;` | `` |
| 185 | 113 | `missing ;` | `` |
| 195 | 97 | `missing ;` | `` |
| 206 | 87 | `missing ;` | `` |
| 216 | 92 | `missing ;` | `` |
| 232 | 92 | `missing ;` | `` |
| 246 | 90 | `missing ;` | `` |
| 261 | 90 | `missing ;` | `` |
| 266 | 91 | `missing ;` | `` |
| 271 | 89 | `missing ;` | `` |
| 287 | 90 | `missing ;` | `` |
| 299 | 92 | `missing ;` | `` |
| 315 | 92 | `missing ;` | `` |
| 328 | 95 | `missing ;` | `` |
| 337 | 95 | `missing ;` | `` |
| 344 | 91 | `missing ;` | `` |
| 369 | 93 | `missing ;` | `` |
| 376 | 93 | `missing ;` | `` |
| … | … | … | *(9 more)* |

#### `mediastream/test/unittest/filter/src/audio_fork_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 20 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 64 | `missing ;` | `` |
| 42 | 69 | `missing ;` | `` |
| 53 | 67 | `missing ;` | `` |
| 67 | 67 | `missing ;` | `` |
| 82 | 68 | `missing ;` | `` |
| 99 | 67 | `missing ;` | `` |
| 116 | 66 | `missing ;` | `` |
| 131 | 69 | `missing ;` | `` |
| 148 | 72 | `missing ;` | `` |
| 156 | 72 | `missing ;` | `` |
| 166 | 68 | `missing ;` | `` |
| 175 | 70 | `missing ;` | `` |
| 183 | 70 | `missing ;` | `` |
| 191 | 73 | `missing ;` | `` |
| 198 | 74 | `missing ;` | `` |
| 214 | 68 | `missing ;` | `` |
| 224 | 69 | `missing ;` | `` |
| 234 | 70 | `missing ;` | `` |
| 244 | 76 | `missing ;` | `` |
| 252 | 75 | `missing ;` | `` |

#### `mediastream/test/unittest/filter/src/audio_process_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 25 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 32 | 67 | `missing ;` | `` |
| 41 | 72 | `missing ;` | `` |
| 52 | 70 | `missing ;` | `` |
| 66 | 70 | `missing ;` | `` |
| 81 | 71 | `missing ;` | `` |
| 98 | 70 | `missing ;` | `` |
| 115 | 69 | `missing ;` | `` |
| 130 | 72 | `missing ;` | `` |
| 147 | 75 | `missing ;` | `` |
| 155 | 75 | `missing ;` | `` |
| 165 | 71 | `missing ;` | `` |
| 174 | 73 | `missing ;` | `` |
| 182 | 73 | `missing ;` | `` |
| 190 | 76 | `missing ;` | `` |
| 197 | 77 | `missing ;` | `` |
| 213 | 71 | `missing ;` | `` |
| 223 | 72 | `missing ;` | `` |
| 233 | 73 | `missing ;` | `` |
| 243 | 79 | `missing ;` | `` |
| 251 | 78 | `missing ;` | `` |
| … | … | … | *(5 more)* |

#### `mediastream/test/unittest/filter/src/cfilter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 34 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 63 | `missing ;` | `` |
| 40 | 63 | `missing ;` | `` |
| 49 | 63 | `missing ;` | `` |
| 58 | 63 | `missing ;` | `` |
| 67 | 63 | `missing ;` | `` |
| 76 | 63 | `missing ;` | `` |
| 86 | 63 | `missing ;` | `` |
| 95 | 63 | `missing ;` | `` |
| 104 | 69 | `missing ;` | `` |
| 113 | 69 | `missing ;` | `` |
| 122 | 69 | `missing ;` | `` |
| 131 | 69 | `missing ;` | `` |
| 140 | 69 | `missing ;` | `` |
| 149 | 69 | `missing ;` | `` |
| 159 | 69 | `missing ;` | `` |
| 168 | 69 | `missing ;` | `` |
| 177 | 59 | `missing ;` | `` |
| 188 | 59 | `missing ;` | `` |
| 198 | 59 | `missing ;` | `` |
| 209 | 63 | `missing ;` | `` |
| … | … | … | *(14 more)* |

#### `mediastream/test/unittest/filter/src/cinematic_video_cache_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 18 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 74 | `missing ;` | `` |
| 43 | 79 | `missing ;` | `` |
| 54 | 77 | `missing ;` | `` |
| 68 | 77 | `missing ;` | `` |
| 83 | 78 | `missing ;` | `` |
| 100 | 77 | `missing ;` | `` |
| 117 | 76 | `missing ;` | `` |
| 132 | 79 | `missing ;` | `` |
| 149 | 78 | `missing ;` | `` |
| 158 | 80 | `missing ;` | `` |
| 166 | 80 | `missing ;` | `` |
| 174 | 83 | `missing ;` | `` |
| 181 | 78 | `missing ;` | `` |
| 191 | 79 | `missing ;` | `` |
| 201 | 80 | `missing ;` | `` |
| 211 | 84 | `missing ;` | `` |
| 221 | 85 | `missing ;` | `` |
| 229 | 86 | `missing ;` | `` |

#### `mediastream/test/unittest/filter/src/metadata_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 24 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 63 | `missing ;` | `` |
| 41 | 68 | `missing ;` | `` |
| 49 | 78 | `missing ;` | `` |
| 58 | 78 | `missing ;` | `` |
| 66 | 78 | `missing ;` | `` |
| 73 | 68 | `missing ;` | `` |
| 84 | 66 | `missing ;` | `` |
| 98 | 66 | `missing ;` | `` |
| 113 | 67 | `missing ;` | `` |
| 130 | 66 | `missing ;` | `` |
| 147 | 65 | `missing ;` | `` |
| 162 | 68 | `missing ;` | `` |
| 179 | 68 | `missing ;` | `` |
| 186 | 67 | `missing ;` | `` |
| 195 | 69 | `missing ;` | `` |
| 203 | 69 | `missing ;` | `` |
| 211 | 72 | `missing ;` | `` |
| 218 | 67 | `missing ;` | `` |
| 228 | 68 | `missing ;` | `` |
| 238 | 69 | `missing ;` | `` |
| … | … | … | *(4 more)* |

#### `mediastream/test/unittest/filter/src/muxer_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 21 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 31 | 74 | `missing ;` | `` |
| 38 | 73 | `missing ;` | `` |
| 45 | 71 | `missing ;` | `` |
| 52 | 60 | `missing ;` | `` |
| 62 | 65 | `missing ;` | `` |
| 72 | 63 | `missing ;` | `` |
| 85 | 63 | `missing ;` | `` |
| 99 | 64 | `missing ;` | `` |
| 115 | 63 | `missing ;` | `` |
| 131 | 62 | `missing ;` | `` |
| 145 | 65 | `missing ;` | `` |
| 161 | 64 | `missing ;` | `` |
| 169 | 66 | `missing ;` | `` |
| 177 | 66 | `missing ;` | `` |
| 185 | 69 | `missing ;` | `` |
| 192 | 75 | `missing ;` | `` |
| 200 | 75 | `missing ;` | `` |
| 208 | 78 | `missing ;` | `` |
| 216 | 64 | `missing ;` | `` |
| 233 | 65 | `missing ;` | `` |
| … | … | … | *(1 more)* |

#### `mediastream/test/unittest/filter/src/video_encoder_adapter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 32 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 21 | 68 | `missing ;` | `` |
| 30 | 68 | `missing ;` | `` |
| 42 | 68 | `missing ;` | `` |
| 54 | 73 | `missing ;` | `` |
| 62 | 73 | `missing ;` | `` |
| 76 | 76 | `missing ;` | `` |
| 83 | 76 | `missing ;` | `` |
| 94 | 75 | `missing ;` | `` |
| 101 | 84 | `missing ;` | `` |
| 109 | 89 | `missing ;` | `` |
| 122 | 89 | `missing ;` | `` |
| 131 | 100 | `missing ;` | `` |
| 139 | 81 | `missing ;` | `` |
| 146 | 79 | `missing ;` | `` |
| 158 | 79 | `missing ;` | `` |
| 165 | 69 | `missing ;` | `` |
| 176 | 69 | `missing ;` | `` |
| 184 | 68 | `missing ;` | `` |
| 198 | 69 | `missing ;` | `` |
| 205 | 70 | `missing ;` | `` |
| … | … | … | *(12 more)* |

#### `mediastream/test/unittest/filter/src/video_encoder_filter_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 22 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 33 | 77 | `missing ;` | `` |
| 42 | 77 | `missing ;` | `` |
| 50 | 67 | `missing ;` | `` |
| 58 | 72 | `missing ;` | `` |
| 66 | 75 | `missing ;` | `` |
| 74 | 74 | `missing ;` | `` |
| 81 | 72 | `missing ;` | `` |
| 88 | 70 | `missing ;` | `` |
| 95 | 70 | `missing ;` | `` |
| 102 | 71 | `missing ;` | `` |
| 109 | 69 | `missing ;` | `` |
| 116 | 72 | `missing ;` | `` |
| 123 | 72 | `missing ;` | `` |
| 131 | 71 | `missing ;` | `` |
| 140 | 73 | `missing ;` | `` |
| 148 | 73 | `missing ;` | `` |
| 156 | 76 | `missing ;` | `` |
| 163 | 71 | `missing ;` | `` |
| 171 | 72 | `missing ;` | `` |
| 179 | 73 | `missing ;` | `` |
| … | … | … | *(2 more)* |

#### `mediastream/test/unittest/pipeline/pipeline_unit_test.cpp`

**Summary:** tree-sitter-cpp node `missing ;` at 7 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 35 | 81 | `missing ;` | `` |
| 41 | 81 | `missing ;` | `` |
| 47 | 81 | `missing ;` | `` |
| 54 | 78 | `missing ;` | `` |
| 66 | 74 | `missing ;` | `` |
| 86 | 74 | `missing ;` | `` |
| 93 | 74 | `missing ;` | `` |

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

#### `services/camera_service/include/param_update/camera_rotate_param_manager.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 59 | 14 | `ERROR` | `CameraRoateParamManager::` |
| 59 | 79 | `ERROR` | `const` |

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
| … | … | … | *(205 more)* |

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

#### `services/deferred_processing_service/include/event_monitor/events_monitor.h`

**Summary:** tree-sitter-cpp node `missing ;` at 1 site(s)

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 20 | 33 | `missing ;` | `` |

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
| … | … | … | *(36 more)* |

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

#### `test/fuzztest/cameraserviceproxy_fuzzer/camera_service_proxy_fuzzer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 94 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

#### `test/fuzztest/hcameraservice_fuzzer/hcamera_service_fuzzer.h`

**Summary:** generic tree-sitter ERROR node(s) in preprocessed C++

| Line | Col | Node kind | Snippet |
|-----:|----:|-----------|---------|
| 122 | 32 | `ERROR` | `"OHOS.Anco.Service.Camera"` |

---
