/// Fragments of SVD taken from bl808-pac
/// Want to parse these but need to map them to scrape data first
/// so it's more expedient to encode them here.

pub const HEADER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<device schemaVersion="1.3" xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD.xsd">
  <vendor>Bouffalo Lab</vendor>
  <vendorID>bouffalolab</vendorID>
  <name>BL616</name>
  <series></series> <!-- todo -->
  <version>0.1</version>
  <description>Bouffalo BL808 chip</description>
  <licenseText>
Copyright (c) 2022 Bouffalo Lab
bl616-pac is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
          http://license.coscl.org.cn/MulanPSL2
THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
  </licenseText>
  <addressUnitBits>8</addressUnitBits>
  <width>64</width>
  <size>32</size>
  <access>read-write</access>
  <resetValue>0x00000000</resetValue>
  <resetMask>0xFFFFFFFF</resetMask>
  <peripherals>
"#;

pub const FOOTER: &str = r#"
</peripherals>
</device>"#;

// #define GLB_BASE         ((uint32_t)0x20000000)
// #define MIX_BASE         ((uint32_t)0x20001000)
// #define GPIP_BASE        ((uint32_t)0x20002000)
// #define PHY_BASE         ((uint32_t)0x20002800)
// #define AGC_BASE         ((uint32_t)0x20002c00)
// #define SEC_DBG_BASE     ((uint32_t)0x20003000)
// #define SEC_ENG_BASE     ((uint32_t)0x20004000)
// #define TZ1_BASE         ((uint32_t)0x20005000)
// #define TZC_SEC_BASE     ((uint32_t)0x20005000)
// #define TZ2_BASE         ((uint32_t)0x20006000)
// #define TZC_NSEC_BASE    ((uint32_t)0x20006000)
// #define CCI_BASE         ((uint32_t)0x20008000)
// #define MCU_MISC_BASE    ((uint32_t)0x20009000)
// #define L1C_BASE         ((uint32_t)0x20009000)
// #define UART0_BASE       ((uint32_t)0x2000a000)
// #define UART1_BASE       ((uint32_t)0x2000a100)
// #define SPI_BASE         ((uint32_t)0x2000a200)
// #define I2C0_BASE        ((uint32_t)0x2000a300)
// #define PWM_BASE         ((uint32_t)0x2000a400)
// #define TIMER_BASE       ((uint32_t)0x2000a500)
// #define IR_BASE          ((uint32_t)0x2000a600)
// #define CKS_BASE         ((uint32_t)0x2000a700)
// #define DBI_BASE         ((uint32_t)0x2000A800)
// #define I2C1_BASE        ((uint32_t)0x2000a900)
// #define ISO11898_BASE    ((uint32_t)0x2000aa00)
// #define I2S_BASE         ((uint32_t)0x2000ab00)
// #define AUADC_BASE       ((uint32_t)0x2000a000)
// #define QSPI_BASE        ((uint32_t)0x2000b000)
// #define SF_CTRL_BASE     ((uint32_t)0x2000b000)
// #define SF_CTRL_BUF_BASE ((uint32_t)0x2000b600)
// #define DMA_BASE         ((uint32_t)0x2000c000)
// #define SDU_BASE         ((uint32_t)0x2000d000)
// #define PDS_BASE         ((uint32_t)0x2000e000)
// #define HBN_BASE         ((uint32_t)0x2000f000)
// #define AON_BASE         ((uint32_t)0x2000f000)
// #define MM_MISC_BASE     ((uint32_t)0x20050000)
// #define PSRAM_CTRL_BASE  ((uint32_t)0x20052000)
// #define AUDAC_BASE       ((uint32_t)0x20055000)
// #define EFUSE_BASE       ((uint32_t)0x20056000)
// #define EF_DATA_BASE     ((uint32_t)0x20056000)
// #define EF_CTRL_BASE     ((uint32_t)0x20056000)
// #define DVP2AXI0_BASE    ((uint32_t)0x20057000)
// #define DVP2AXI1_BASE    ((uint32_t)0x20058000)
// #define MJPEG_BASE       ((uint32_t)0x20059000)
// #define SDH_BASE         ((uint32_t)0x20060000)
// #define EMAC_BASE        ((uint32_t)0x20070000)
// #define USB_BASE         ((uint32_t)0x20072000)
// #define HBN_RAM_BASE     ((uint32_t)0x20010000)

// #define GLB_BASE         ((uint32_t)0x20000000)
pub const GLB: &str = r#"<name>GLB</name>
<description>Global configuration register</description>
<baseAddress>0x20000000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define MIX_BASE         ((uint32_t)0x20001000)
pub const MIX: &str = r#""#;

// #define GPIP_BASE        ((uint32_t)0x20002000)
pub const GPIP: &str = r#"<name>GPIP</name>
<description>Generic DAC, ADC and ACOMP interface control</description>
<baseAddress>0x20002000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x400</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define PHY_BASE         ((uint32_t)0x20002800)
pub const PHY: &str = r#""#;

// #define AGC_BASE         ((uint32_t)0x20002c00)
pub const AGC: &str = r#"<name>AGC</name>
<description>Automatic Gain Control</description>
<baseAddress>0x20002C00</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size> <!-- todo -->
  <usage>registers</usage>
</addressBlock>"#;

// #define SEC_DBG_BASE     ((uint32_t)0x20003000)
pub const SEC_DBG: &str = r#"<name>SEC_DBG</name>
<description>Secure debug configuration</description>
<baseAddress>0x20003000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define SEC_ENG_BASE     ((uint32_t)0x20004000)
pub const SEC_ENG: &str = r#"<name>SEC</name>
<description>Secure Engine</description>
<baseAddress>0x20004000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define TZ1_BASE         ((uint32_t)0x20005000)
// #define TZC_SEC_BASE     ((uint32_t)0x20005000)
pub const TZC_SEC: &str = r#"<name>TZC_SEC</name>
<description>Trust zone isolation</description>
<baseAddress>0x20005000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define TZ2_BASE         ((uint32_t)0x20006000)
// #define TZC_NSEC_BASE    ((uint32_t)0x20006000)
pub const TZC_NSEC: &str = r#"<name>TZC_NSEC</name>
<description>Trust zone isolation 2</description>
<baseAddress>0x20006000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define CCI_BASE         ((uint32_t)0x20008000)
/// TODO: verify
pub const CCI: &str = r#"      <name>CCI</name>
<description>Camera Control Interface</description>
<baseAddress>0x20008000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define MCU_MISC_BASE    ((uint32_t)0x20009000)
// #define L1C_BASE         ((uint32_t)0x20009000)
pub const MCU_MISC: &str = r#"<name>MISC</name>
<description>Chip Miscellaneous control</description>
<baseAddress>0x20009000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define UART0_BASE       ((uint32_t)0x2000a000)
pub const UART0: &str = r#"<name>UART[%s]</name>
<dim>2</dim>
<dimIncrement>0x100</dimIncrement>
<description>Universal Asynchronous Receiver Transmitter</description>
<baseAddress>0x2000A000</baseAddress>
<access>read-write</access>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;
// #define UART1_BASE       ((uint32_t)0x2000a100)
pub const UART1: &str = r#""#;

// #define SPI0_BASE        ((uint32_t)0x2000a200)
pub const SPI0: &str = r#"<name>SPI[%s]</name>
<dim>1</dim>
<dimIncrement>0x100</dimIncrement>
<description>Serial Peripheral Interface</description>
<baseAddress>0x2000A200</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define I2C0_BASE        ((uint32_t)0x2000a300)
pub const I2C0: &str = r#"<name>I2C[%s]</name>
<dim>2</dim>
<dimIncrement>0x600</dimIncrement>
<description>Inter-Integrated Circuit bus</description>
<baseAddress>0x2000A300</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define PWM_BASE         ((uint32_t)0x2000a400)
pub const PWM: &str = r#"<name>PWM</name>
<description>Pulse-Width Modulation module</description>
<baseAddress>0x2000A400</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define TIMER0_BASE      ((uint32_t)0x2000a500)
pub const TIMER0: &str = r#"<name>TIMER</name>
<description>Timer control</description>
<baseAddress>0x2000A500</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define IR_BASE          ((uint32_t)0x2000a600)
pub const IR: &str = r#"      <name>IR</name>
<description>Infrared Remote module</description>
<baseAddress>0x2000A600</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define CKS_BASE         ((uint32_t)0x2000a700)
pub const CKS: &str = r#""#;

// #define IPC0_BASE        ((uint32_t)0x2000a800)
pub const IPC0: &str = r#"<name>IPC</name>
<description>Inter-processor Channel</description>
<baseAddress>0x2000A800</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define I2C1_BASE        ((uint32_t)0x2000a900)
pub const I2C1: &str = r#""#;

// #define ISO11898_BASE    ((uint32_t)0x2000aa00)
pub const ISO11898: &str = r#"      <name>ISO11898</name>
<description>ISO 11898 communication protocol</description>
<baseAddress>0x2000AA00</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define DBI_BASE         ((uint32_t)0x2000A800)
// TODO
pub const DBI: &str = r#""#;

// #define I2S_BASE         ((uint32_t)0x2000ab00)
pub const I2S: &str = r#"<name>I2S</name>
<description>Inter-IC Sound controller</description>
<baseAddress>0x2000AB00</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define AUADC_BASE       ((uint32_t)0x2000a000)
// TODO
// #define PDM0_BASE        ((uint32_t)0x2000a000)
pub const PDM0: &str = r#"<name>PDM</name>
<description>Pulse Density Modulation</description>
<baseAddress>0x2000AC00</baseAddress> <!-- todo: verify -->
<addressBlock>
  <offset>0</offset>
  <size>0x100</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define QSPI_BASE        ((uint32_t)0x2000b000)
pub const QSPI: &str = r#""#;

// #define SF_CTRL        ((uint32_t)0x2000b000)
pub const SF_CTRL: &str = r#"<name>FLASH</name>
<description>Quad Serial Flash control</description>
<baseAddress>0x2000B000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x400</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define SF_CTRL_BASE     ((uint32_t)0x2000b000)
pub const SF_CTRL_BUF: &str = r#""#;

// #define DMA0_BASE        ((uint32_t)0x2000c000)
pub const DMA0: &str = r#"<name>DMA%s</name>
<dim>2</dim>
<dimIncrement>0x65000</dimIncrement>
<description>Direct Memory Access</description>
<baseAddress>0x2000C000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define DMA1_BASE        ((uint32_t)0x20071000)
// TODO - pull in bl808-pac version, remove this
pub const DMA1: &str = r#""#;

// #define SDU_BASE         ((uint32_t)0x2000d000)
// TODO
pub const SDU: &str = r#""#;

// #define PDS_BASE         ((uint32_t)0x2000e000)
pub const PDS: &str = r#"<name>PDS_BASE</name>
<description>Sleep control (Power-down sleep?)</description>
<baseAddress>0x2000e000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0xB00</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define HBN_BASE         ((uint32_t)0x2000f000)
pub const HBN: &str = r#"<name>HBN</name>
<description>Hibernate (Deep sleep) control</description>
<baseAddress>0x2000F000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x800</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define AON_BASE         ((uint32_t)0x2000f000)
pub const AON: &str = r#"<name>AON</name>
<description>Always-On function control</description>
<baseAddress>0x2000F800</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define MM_MISC_BASE     ((uint32_t)0x20050000)
// TODO
pub const MM_MISC: &str = r#""#;

// #define PSRAM_CTRL_BASE  ((uint32_t)0x20052000)
pub const PSRAM_CTRL: &str = r#"<name>PSRAM</name>
<description>Pseudo Static Random Access Memory control</description>
<baseAddress>0x20052000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x200</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define AUDAC_BASE       ((uint32_t)0x20055000)
// TODO
pub const AUDAC: &str = r#""#;

// #define EFUSE_BASE       ((uint32_t)0x20056000)
pub const EFUSE: &str = r#"      <name>EFUSE</name>
<description>eFuse memory control</description>
<baseAddress>0x20056000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define AUDAC_BASE       ((uint32_t)0x20055000)
pub const AUDAC_BASE: &str = r#"<name>AUDAC_BASE</name>
<description>Audio codec controller</description>
<baseAddress>0x20055000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define EF_DATA_BASE     ((uint32_t)0x20056000)
pub const EF_DATA: &str = r#"<name>EF_DATA</name>
<description></description>
<baseAddress>0x20056000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define DVP2AXI0_BASE    ((uint32_t)0x20057000)
pub const DVP2AXI0: &str = r#""#;

// #define DVP2AXI1_BASE    ((uint32_t)0x20058000)
pub const DVP2AXI1: &str = r#""#;

// #define MJPEG_BASE       ((uint32_t)0x20059000)
pub const MJPEG: &str = r#""#;

// #define SDH_BASE         ((uint32_t)0x20060000)
pub const SDH: &str = r#"<name>SDH</name>
<description>Secure Digital host control</description>
<baseAddress>0x20060000</baseAddress>
<!-- todo: sdk shows this peripheral is written by 16-bit register, confirm this -->
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define EMAC_BASE        ((uint32_t)0x20070000)
pub const EMAC: &str = r#"<name>EMAC</name>
<description>Ethernet Media Access Control</description>
<baseAddress>0x20070000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// #define USB_BASE         ((uint32_t)0x20072000)
pub const USB: &str = r#"<name>USB</name>
<description>Universal Serial Bus host</description>
<baseAddress>0x20072000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// These aren't listed in bl616.h
// TODO: verify
pub const WIFI: &str = r#"<name>WIFI</name>
<description>Wireless Fidelity control</description>
<baseAddress>0x24B00000</baseAddress> 
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

// TODO: verify
pub const PTA: &str = r#"<name>PTA</name>
<description>Packet Traffic Arbitration</description>
<baseAddress>0x24920000</baseAddress>
<addressBlock>
  <offset>0</offset>
  <size>0x1000</size>
  <usage>registers</usage>
</addressBlock>"#;

pub const _: &str = r#""#;
pub const _: &str = r#""#;
pub const _: &str = r#""#;
pub const _: &str = r#""#;
