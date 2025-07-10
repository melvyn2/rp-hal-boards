#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
// The actual flash used is the W25Q64JVXGIQ, but this firmware is compatible
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 {
        name: tx,
        aliases: { FunctionUart, PullNone: UartTx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart, PullNone: UartRx }
    },
    Gpio2 {
        name: sda,
        aliases: { FunctionI2C, PullUp: Sda }
    },
    Gpio3 {
        name: scl,
        aliases: { FunctionI2C, PullUp: Scl }
    },
    Gpio4 { name: d4 },
    Gpio5 { name: d5 },
    Gpio6 { name: d6 },
    Gpio7 { name: button },
    Gpio8 {
        name: miso,
        aliases: { FunctionSpi, PullNone: Miso }
    },
    Gpio9 { name: d9 },
    Gpio10 { name: d10 },
    Gpio11 { name: d11 },
    Gpio12 { name: d12 },
    Gpio13 { name: d13 },
    Gpio14 {
        name: sck,
        aliases: { FunctionSpi, PullNone: Sclk }
    },
    Gpio15 {
        name: mosi,
        aliases: { FunctionSpi, PullNone: Mosi }
    },
    Gpio16 {
        name: sd_card_detect,
        aliases: {
            FunctionNull, PullNone: SdioCd
        }
    },
    Gpio17 { name: neopixel },
    Gpio18 {
        name: sd_clk,
        aliases: {
            FunctionSpi, PullNone: SdClk,
            FunctionPio0, PullNone: SdioSckPio0,
            FunctionPio1, PullNone: SdioSckPio1
        }
    },
    Gpio19 {
        name: sd_mosi,
        aliases: {
            FunctionSpi, PullNone: SdMosi,
            FunctionPio0, PullNone: SdioCmdPio0,
            FunctionPio1, PullNone: SdioCmdPio1
        }
    },
    Gpio20 {
        name: sd_miso,
        aliases: {
            FunctionSpi, PullNone: SdMiso,
            FunctionPio0, PullNone: SdioData0Pio0,
            FunctionPio1, PullNone: SdioData0Pio1
        }
    },
    Gpio21 {
        name: sd_dat1,
        aliases: {
            FunctionPio0, PullNone: SdioData1Pio0,
            FunctionPio1, PullNone: SdioData1Pio1
        }
    },
    Gpio22 {
        name: sd_dat2,
        aliases: {
            FunctionPio0, PullNone: SdioData2Pio0,
            FunctionPio1, PullNone: SdioData2Pio1
        }
    },
    Gpio23 {
        name: sd_cs,
        aliases: {
            FunctionSpi, PullNone: SdCs,
            FunctionPio0, PullNone: SdioData3Pio0,
            FunctionPio1, PullNone: SdioData3Pio1
        }
    },
    Gpio24 { name: d24 },
    Gpio25 { name: d25 },
    Gpio26 { name: a0 },
    Gpio27 { name: a1 },
    Gpio28 { name: a2 },
    Gpio29 { name: a3 },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
