#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chipid0: CHIPID0,
    chipid1: CHIPID1,
    chipid2: CHIPID2,
    chipid3: CHIPID3,
    randid0: RANDID0,
    randid1: RANDID1,
    randid2: RANDID2,
    randid3: RANDID3,
    randid4: RANDID4,
    randid5: RANDID5,
    randid6: RANDID6,
    randid7: RANDID7,
    _reserved12: [u8; 0x08],
    rosc_calib: ROSC_CALIB,
    lposc_calib: LPOSC_CALIB,
    _reserved14: [u8; 0x0c],
    num_gpios: NUM_GPIOS,
    _reserved15: [u8; 0x3a],
    info_crc0: INFO_CRC0,
    info_crc1: INFO_CRC1,
    _reserved17: [u8; 0x38],
    flash_devinfo: FLASH_DEVINFO,
    flash_partition_slot_size: FLASH_PARTITION_SLOT_SIZE,
    bootsel_led_cfg: BOOTSEL_LED_CFG,
    bootsel_pll_cfg: BOOTSEL_PLL_CFG,
    bootsel_xosc_cfg: BOOTSEL_XOSC_CFG,
    _reserved22: [u8; 0x06],
    usb_white_label_addr: USB_WHITE_LABEL_ADDR,
    _reserved23: [u8; 0x02],
    otpboot_src: OTPBOOT_SRC,
    otpboot_len: OTPBOOT_LEN,
    otpboot_dst0: OTPBOOT_DST0,
    otpboot_dst1: OTPBOOT_DST1,
    _reserved27: [u8; 0x3c],
    bootkey0_0: BOOTKEY0_0,
    bootkey0_1: BOOTKEY0_1,
    bootkey0_2: BOOTKEY0_2,
    bootkey0_3: BOOTKEY0_3,
    bootkey0_4: BOOTKEY0_4,
    bootkey0_5: BOOTKEY0_5,
    bootkey0_6: BOOTKEY0_6,
    bootkey0_7: BOOTKEY0_7,
    bootkey0_8: BOOTKEY0_8,
    bootkey0_9: BOOTKEY0_9,
    bootkey0_10: BOOTKEY0_10,
    bootkey0_11: BOOTKEY0_11,
    bootkey0_12: BOOTKEY0_12,
    bootkey0_13: BOOTKEY0_13,
    bootkey0_14: BOOTKEY0_14,
    bootkey0_15: BOOTKEY0_15,
    bootkey1_0: BOOTKEY1_0,
    bootkey1_1: BOOTKEY1_1,
    bootkey1_2: BOOTKEY1_2,
    bootkey1_3: BOOTKEY1_3,
    bootkey1_4: BOOTKEY1_4,
    bootkey1_5: BOOTKEY1_5,
    bootkey1_6: BOOTKEY1_6,
    bootkey1_7: BOOTKEY1_7,
    bootkey1_8: BOOTKEY1_8,
    bootkey1_9: BOOTKEY1_9,
    bootkey1_10: BOOTKEY1_10,
    bootkey1_11: BOOTKEY1_11,
    bootkey1_12: BOOTKEY1_12,
    bootkey1_13: BOOTKEY1_13,
    bootkey1_14: BOOTKEY1_14,
    bootkey1_15: BOOTKEY1_15,
    bootkey2_0: BOOTKEY2_0,
    bootkey2_1: BOOTKEY2_1,
    bootkey2_2: BOOTKEY2_2,
    bootkey2_3: BOOTKEY2_3,
    bootkey2_4: BOOTKEY2_4,
    bootkey2_5: BOOTKEY2_5,
    bootkey2_6: BOOTKEY2_6,
    bootkey2_7: BOOTKEY2_7,
    bootkey2_8: BOOTKEY2_8,
    bootkey2_9: BOOTKEY2_9,
    bootkey2_10: BOOTKEY2_10,
    bootkey2_11: BOOTKEY2_11,
    bootkey2_12: BOOTKEY2_12,
    bootkey2_13: BOOTKEY2_13,
    bootkey2_14: BOOTKEY2_14,
    bootkey2_15: BOOTKEY2_15,
    bootkey3_0: BOOTKEY3_0,
    bootkey3_1: BOOTKEY3_1,
    bootkey3_2: BOOTKEY3_2,
    bootkey3_3: BOOTKEY3_3,
    bootkey3_4: BOOTKEY3_4,
    bootkey3_5: BOOTKEY3_5,
    bootkey3_6: BOOTKEY3_6,
    bootkey3_7: BOOTKEY3_7,
    bootkey3_8: BOOTKEY3_8,
    bootkey3_9: BOOTKEY3_9,
    bootkey3_10: BOOTKEY3_10,
    bootkey3_11: BOOTKEY3_11,
    bootkey3_12: BOOTKEY3_12,
    bootkey3_13: BOOTKEY3_13,
    bootkey3_14: BOOTKEY3_14,
    bootkey3_15: BOOTKEY3_15,
    _reserved91: [u8; 0x1d10],
    key1_0: KEY1_0,
    key1_1: KEY1_1,
    key1_2: KEY1_2,
    key1_3: KEY1_3,
    key1_4: KEY1_4,
    key1_5: KEY1_5,
    key1_6: KEY1_6,
    key1_7: KEY1_7,
    key2_0: KEY2_0,
    key2_1: KEY2_1,
    key2_2: KEY2_2,
    key2_3: KEY2_3,
    key2_4: KEY2_4,
    key2_5: KEY2_5,
    key2_6: KEY2_6,
    key2_7: KEY2_7,
    key3_0: KEY3_0,
    key3_1: KEY3_1,
    key3_2: KEY3_2,
    key3_3: KEY3_3,
    key3_4: KEY3_4,
    key3_5: KEY3_5,
    key3_6: KEY3_6,
    key3_7: KEY3_7,
    key4_0: KEY4_0,
    key4_1: KEY4_1,
    key4_2: KEY4_2,
    key4_3: KEY4_3,
    key4_4: KEY4_4,
    key4_5: KEY4_5,
    key4_6: KEY4_6,
    key4_7: KEY4_7,
    key5_0: KEY5_0,
    key5_1: KEY5_1,
    key5_2: KEY5_2,
    key5_3: KEY5_3,
    key5_4: KEY5_4,
    key5_5: KEY5_5,
    key5_6: KEY5_6,
    key5_7: KEY5_7,
    key6_0: KEY6_0,
    key6_1: KEY6_1,
    key6_2: KEY6_2,
    key6_3: KEY6_3,
    key6_4: KEY6_4,
    key6_5: KEY6_5,
    key6_6: KEY6_6,
    key6_7: KEY6_7,
}
impl RegisterBlock {
    #[doc = "0x00 - Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique."]
    #[inline(always)]
    pub const fn chipid0(&self) -> &CHIPID0 {
        &self.chipid0
    }
    #[doc = "0x02 - Bits 31:16 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid1(&self) -> &CHIPID1 {
        &self.chipid1
    }
    #[doc = "0x04 - Bits 47:32 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid2(&self) -> &CHIPID2 {
        &self.chipid2
    }
    #[doc = "0x06 - Bits 63:48 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid3(&self) -> &CHIPID3 {
        &self.chipid3
    }
    #[doc = "0x08 - Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
    #[inline(always)]
    pub const fn randid0(&self) -> &RANDID0 {
        &self.randid0
    }
    #[doc = "0x0a - Bits 31:16 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid1(&self) -> &RANDID1 {
        &self.randid1
    }
    #[doc = "0x0c - Bits 47:32 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid2(&self) -> &RANDID2 {
        &self.randid2
    }
    #[doc = "0x0e - Bits 63:48 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid3(&self) -> &RANDID3 {
        &self.randid3
    }
    #[doc = "0x10 - Bits 79:64 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid4(&self) -> &RANDID4 {
        &self.randid4
    }
    #[doc = "0x12 - Bits 95:80 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid5(&self) -> &RANDID5 {
        &self.randid5
    }
    #[doc = "0x14 - Bits 111:96 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid6(&self) -> &RANDID6 {
        &self.randid6
    }
    #[doc = "0x16 - Bits 127:112 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid7(&self) -> &RANDID7 {
        &self.randid7
    }
    #[doc = "0x20 - Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
    #[inline(always)]
    pub const fn rosc_calib(&self) -> &ROSC_CALIB {
        &self.rosc_calib
    }
    #[doc = "0x22 - Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
    #[inline(always)]
    pub const fn lposc_calib(&self) -> &LPOSC_CALIB {
        &self.lposc_calib
    }
    #[doc = "0x30 - The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
    #[inline(always)]
    pub const fn num_gpios(&self) -> &NUM_GPIOS {
        &self.num_gpios
    }
    #[doc = "0x6c - Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
    #[inline(always)]
    pub const fn info_crc0(&self) -> &INFO_CRC0 {
        &self.info_crc0
    }
    #[doc = "0x6e - Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
    #[inline(always)]
    pub const fn info_crc1(&self) -> &INFO_CRC1 {
        &self.info_crc1
    }
    #[doc = "0xa8 - Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
    #[inline(always)]
    pub const fn flash_devinfo(&self) -> &FLASH_DEVINFO {
        &self.flash_devinfo
    }
    #[doc = "0xaa - Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
    #[inline(always)]
    pub const fn flash_partition_slot_size(&self) -> &FLASH_PARTITION_SLOT_SIZE {
        &self.flash_partition_slot_size
    }
    #[doc = "0xac - Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
    #[inline(always)]
    pub const fn bootsel_led_cfg(&self) -> &BOOTSEL_LED_CFG {
        &self.bootsel_led_cfg
    }
    #[doc = "0xae - Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_pll_cfg(&self) -> &BOOTSEL_PLL_CFG {
        &self.bootsel_pll_cfg
    }
    #[doc = "0xb0 - Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_xosc_cfg(&self) -> &BOOTSEL_XOSC_CFG {
        &self.bootsel_xosc_cfg
    }
    #[doc = "0xb8 - Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> &USB_WHITE_LABEL_ADDR {
        &self.usb_white_label_addr
    }
    #[doc = "0xbc - OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
    #[inline(always)]
    pub const fn otpboot_src(&self) -> &OTPBOOT_SRC {
        &self.otpboot_src
    }
    #[doc = "0xbe - Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
    #[inline(always)]
    pub const fn otpboot_len(&self) -> &OTPBOOT_LEN {
        &self.otpboot_len
    }
    #[doc = "0xc0 - Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst0(&self) -> &OTPBOOT_DST0 {
        &self.otpboot_dst0
    }
    #[doc = "0xc2 - Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst1(&self) -> &OTPBOOT_DST1 {
        &self.otpboot_dst1
    }
    #[doc = "0x100 - Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_0(&self) -> &BOOTKEY0_0 {
        &self.bootkey0_0
    }
    #[doc = "0x102 - Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_1(&self) -> &BOOTKEY0_1 {
        &self.bootkey0_1
    }
    #[doc = "0x104 - Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_2(&self) -> &BOOTKEY0_2 {
        &self.bootkey0_2
    }
    #[doc = "0x106 - Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_3(&self) -> &BOOTKEY0_3 {
        &self.bootkey0_3
    }
    #[doc = "0x108 - Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_4(&self) -> &BOOTKEY0_4 {
        &self.bootkey0_4
    }
    #[doc = "0x10a - Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_5(&self) -> &BOOTKEY0_5 {
        &self.bootkey0_5
    }
    #[doc = "0x10c - Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_6(&self) -> &BOOTKEY0_6 {
        &self.bootkey0_6
    }
    #[doc = "0x10e - Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_7(&self) -> &BOOTKEY0_7 {
        &self.bootkey0_7
    }
    #[doc = "0x110 - Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_8(&self) -> &BOOTKEY0_8 {
        &self.bootkey0_8
    }
    #[doc = "0x112 - Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_9(&self) -> &BOOTKEY0_9 {
        &self.bootkey0_9
    }
    #[doc = "0x114 - Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_10(&self) -> &BOOTKEY0_10 {
        &self.bootkey0_10
    }
    #[doc = "0x116 - Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_11(&self) -> &BOOTKEY0_11 {
        &self.bootkey0_11
    }
    #[doc = "0x118 - Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_12(&self) -> &BOOTKEY0_12 {
        &self.bootkey0_12
    }
    #[doc = "0x11a - Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_13(&self) -> &BOOTKEY0_13 {
        &self.bootkey0_13
    }
    #[doc = "0x11c - Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_14(&self) -> &BOOTKEY0_14 {
        &self.bootkey0_14
    }
    #[doc = "0x11e - Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_15(&self) -> &BOOTKEY0_15 {
        &self.bootkey0_15
    }
    #[doc = "0x120 - Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_0(&self) -> &BOOTKEY1_0 {
        &self.bootkey1_0
    }
    #[doc = "0x122 - Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_1(&self) -> &BOOTKEY1_1 {
        &self.bootkey1_1
    }
    #[doc = "0x124 - Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_2(&self) -> &BOOTKEY1_2 {
        &self.bootkey1_2
    }
    #[doc = "0x126 - Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_3(&self) -> &BOOTKEY1_3 {
        &self.bootkey1_3
    }
    #[doc = "0x128 - Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_4(&self) -> &BOOTKEY1_4 {
        &self.bootkey1_4
    }
    #[doc = "0x12a - Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_5(&self) -> &BOOTKEY1_5 {
        &self.bootkey1_5
    }
    #[doc = "0x12c - Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_6(&self) -> &BOOTKEY1_6 {
        &self.bootkey1_6
    }
    #[doc = "0x12e - Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_7(&self) -> &BOOTKEY1_7 {
        &self.bootkey1_7
    }
    #[doc = "0x130 - Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_8(&self) -> &BOOTKEY1_8 {
        &self.bootkey1_8
    }
    #[doc = "0x132 - Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_9(&self) -> &BOOTKEY1_9 {
        &self.bootkey1_9
    }
    #[doc = "0x134 - Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_10(&self) -> &BOOTKEY1_10 {
        &self.bootkey1_10
    }
    #[doc = "0x136 - Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_11(&self) -> &BOOTKEY1_11 {
        &self.bootkey1_11
    }
    #[doc = "0x138 - Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_12(&self) -> &BOOTKEY1_12 {
        &self.bootkey1_12
    }
    #[doc = "0x13a - Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_13(&self) -> &BOOTKEY1_13 {
        &self.bootkey1_13
    }
    #[doc = "0x13c - Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_14(&self) -> &BOOTKEY1_14 {
        &self.bootkey1_14
    }
    #[doc = "0x13e - Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_15(&self) -> &BOOTKEY1_15 {
        &self.bootkey1_15
    }
    #[doc = "0x140 - Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_0(&self) -> &BOOTKEY2_0 {
        &self.bootkey2_0
    }
    #[doc = "0x142 - Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_1(&self) -> &BOOTKEY2_1 {
        &self.bootkey2_1
    }
    #[doc = "0x144 - Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_2(&self) -> &BOOTKEY2_2 {
        &self.bootkey2_2
    }
    #[doc = "0x146 - Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_3(&self) -> &BOOTKEY2_3 {
        &self.bootkey2_3
    }
    #[doc = "0x148 - Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_4(&self) -> &BOOTKEY2_4 {
        &self.bootkey2_4
    }
    #[doc = "0x14a - Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_5(&self) -> &BOOTKEY2_5 {
        &self.bootkey2_5
    }
    #[doc = "0x14c - Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_6(&self) -> &BOOTKEY2_6 {
        &self.bootkey2_6
    }
    #[doc = "0x14e - Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_7(&self) -> &BOOTKEY2_7 {
        &self.bootkey2_7
    }
    #[doc = "0x150 - Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_8(&self) -> &BOOTKEY2_8 {
        &self.bootkey2_8
    }
    #[doc = "0x152 - Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_9(&self) -> &BOOTKEY2_9 {
        &self.bootkey2_9
    }
    #[doc = "0x154 - Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_10(&self) -> &BOOTKEY2_10 {
        &self.bootkey2_10
    }
    #[doc = "0x156 - Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_11(&self) -> &BOOTKEY2_11 {
        &self.bootkey2_11
    }
    #[doc = "0x158 - Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_12(&self) -> &BOOTKEY2_12 {
        &self.bootkey2_12
    }
    #[doc = "0x15a - Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_13(&self) -> &BOOTKEY2_13 {
        &self.bootkey2_13
    }
    #[doc = "0x15c - Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_14(&self) -> &BOOTKEY2_14 {
        &self.bootkey2_14
    }
    #[doc = "0x15e - Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_15(&self) -> &BOOTKEY2_15 {
        &self.bootkey2_15
    }
    #[doc = "0x160 - Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_0(&self) -> &BOOTKEY3_0 {
        &self.bootkey3_0
    }
    #[doc = "0x162 - Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_1(&self) -> &BOOTKEY3_1 {
        &self.bootkey3_1
    }
    #[doc = "0x164 - Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_2(&self) -> &BOOTKEY3_2 {
        &self.bootkey3_2
    }
    #[doc = "0x166 - Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_3(&self) -> &BOOTKEY3_3 {
        &self.bootkey3_3
    }
    #[doc = "0x168 - Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_4(&self) -> &BOOTKEY3_4 {
        &self.bootkey3_4
    }
    #[doc = "0x16a - Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_5(&self) -> &BOOTKEY3_5 {
        &self.bootkey3_5
    }
    #[doc = "0x16c - Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_6(&self) -> &BOOTKEY3_6 {
        &self.bootkey3_6
    }
    #[doc = "0x16e - Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_7(&self) -> &BOOTKEY3_7 {
        &self.bootkey3_7
    }
    #[doc = "0x170 - Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_8(&self) -> &BOOTKEY3_8 {
        &self.bootkey3_8
    }
    #[doc = "0x172 - Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_9(&self) -> &BOOTKEY3_9 {
        &self.bootkey3_9
    }
    #[doc = "0x174 - Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_10(&self) -> &BOOTKEY3_10 {
        &self.bootkey3_10
    }
    #[doc = "0x176 - Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_11(&self) -> &BOOTKEY3_11 {
        &self.bootkey3_11
    }
    #[doc = "0x178 - Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_12(&self) -> &BOOTKEY3_12 {
        &self.bootkey3_12
    }
    #[doc = "0x17a - Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_13(&self) -> &BOOTKEY3_13 {
        &self.bootkey3_13
    }
    #[doc = "0x17c - Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_14(&self) -> &BOOTKEY3_14 {
        &self.bootkey3_14
    }
    #[doc = "0x17e - Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_15(&self) -> &BOOTKEY3_15 {
        &self.bootkey3_15
    }
    #[doc = "0x1e90 - Bits 15:0 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_0(&self) -> &KEY1_0 {
        &self.key1_0
    }
    #[doc = "0x1e92 - Bits 31:16 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_1(&self) -> &KEY1_1 {
        &self.key1_1
    }
    #[doc = "0x1e94 - Bits 47:32 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_2(&self) -> &KEY1_2 {
        &self.key1_2
    }
    #[doc = "0x1e96 - Bits 63:48 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_3(&self) -> &KEY1_3 {
        &self.key1_3
    }
    #[doc = "0x1e98 - Bits 79:64 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_4(&self) -> &KEY1_4 {
        &self.key1_4
    }
    #[doc = "0x1e9a - Bits 95:80 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_5(&self) -> &KEY1_5 {
        &self.key1_5
    }
    #[doc = "0x1e9c - Bits 111:96 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_6(&self) -> &KEY1_6 {
        &self.key1_6
    }
    #[doc = "0x1e9e - Bits 127:112 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_7(&self) -> &KEY1_7 {
        &self.key1_7
    }
    #[doc = "0x1ea0 - Bits 15:0 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_0(&self) -> &KEY2_0 {
        &self.key2_0
    }
    #[doc = "0x1ea2 - Bits 31:16 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_1(&self) -> &KEY2_1 {
        &self.key2_1
    }
    #[doc = "0x1ea4 - Bits 47:32 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_2(&self) -> &KEY2_2 {
        &self.key2_2
    }
    #[doc = "0x1ea6 - Bits 63:48 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_3(&self) -> &KEY2_3 {
        &self.key2_3
    }
    #[doc = "0x1ea8 - Bits 79:64 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_4(&self) -> &KEY2_4 {
        &self.key2_4
    }
    #[doc = "0x1eaa - Bits 95:80 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_5(&self) -> &KEY2_5 {
        &self.key2_5
    }
    #[doc = "0x1eac - Bits 111:96 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_6(&self) -> &KEY2_6 {
        &self.key2_6
    }
    #[doc = "0x1eae - Bits 127:112 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_7(&self) -> &KEY2_7 {
        &self.key2_7
    }
    #[doc = "0x1eb0 - Bits 15:0 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_0(&self) -> &KEY3_0 {
        &self.key3_0
    }
    #[doc = "0x1eb2 - Bits 31:16 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_1(&self) -> &KEY3_1 {
        &self.key3_1
    }
    #[doc = "0x1eb4 - Bits 47:32 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_2(&self) -> &KEY3_2 {
        &self.key3_2
    }
    #[doc = "0x1eb6 - Bits 63:48 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_3(&self) -> &KEY3_3 {
        &self.key3_3
    }
    #[doc = "0x1eb8 - Bits 79:64 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_4(&self) -> &KEY3_4 {
        &self.key3_4
    }
    #[doc = "0x1eba - Bits 95:80 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_5(&self) -> &KEY3_5 {
        &self.key3_5
    }
    #[doc = "0x1ebc - Bits 111:96 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_6(&self) -> &KEY3_6 {
        &self.key3_6
    }
    #[doc = "0x1ebe - Bits 127:112 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_7(&self) -> &KEY3_7 {
        &self.key3_7
    }
    #[doc = "0x1ec0 - Bits 15:0 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_0(&self) -> &KEY4_0 {
        &self.key4_0
    }
    #[doc = "0x1ec2 - Bits 31:16 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_1(&self) -> &KEY4_1 {
        &self.key4_1
    }
    #[doc = "0x1ec4 - Bits 47:32 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_2(&self) -> &KEY4_2 {
        &self.key4_2
    }
    #[doc = "0x1ec6 - Bits 63:48 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_3(&self) -> &KEY4_3 {
        &self.key4_3
    }
    #[doc = "0x1ec8 - Bits 79:64 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_4(&self) -> &KEY4_4 {
        &self.key4_4
    }
    #[doc = "0x1eca - Bits 95:80 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_5(&self) -> &KEY4_5 {
        &self.key4_5
    }
    #[doc = "0x1ecc - Bits 111:96 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_6(&self) -> &KEY4_6 {
        &self.key4_6
    }
    #[doc = "0x1ece - Bits 127:112 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_7(&self) -> &KEY4_7 {
        &self.key4_7
    }
    #[doc = "0x1ed0 - Bits 15:0 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_0(&self) -> &KEY5_0 {
        &self.key5_0
    }
    #[doc = "0x1ed2 - Bits 31:16 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_1(&self) -> &KEY5_1 {
        &self.key5_1
    }
    #[doc = "0x1ed4 - Bits 47:32 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_2(&self) -> &KEY5_2 {
        &self.key5_2
    }
    #[doc = "0x1ed6 - Bits 63:48 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_3(&self) -> &KEY5_3 {
        &self.key5_3
    }
    #[doc = "0x1ed8 - Bits 79:64 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_4(&self) -> &KEY5_4 {
        &self.key5_4
    }
    #[doc = "0x1eda - Bits 95:80 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_5(&self) -> &KEY5_5 {
        &self.key5_5
    }
    #[doc = "0x1edc - Bits 111:96 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_6(&self) -> &KEY5_6 {
        &self.key5_6
    }
    #[doc = "0x1ede - Bits 127:112 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_7(&self) -> &KEY5_7 {
        &self.key5_7
    }
    #[doc = "0x1ee0 - Bits 15:0 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_0(&self) -> &KEY6_0 {
        &self.key6_0
    }
    #[doc = "0x1ee2 - Bits 31:16 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_1(&self) -> &KEY6_1 {
        &self.key6_1
    }
    #[doc = "0x1ee4 - Bits 47:32 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_2(&self) -> &KEY6_2 {
        &self.key6_2
    }
    #[doc = "0x1ee6 - Bits 63:48 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_3(&self) -> &KEY6_3 {
        &self.key6_3
    }
    #[doc = "0x1ee8 - Bits 79:64 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_4(&self) -> &KEY6_4 {
        &self.key6_4
    }
    #[doc = "0x1eea - Bits 95:80 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_5(&self) -> &KEY6_5 {
        &self.key6_5
    }
    #[doc = "0x1eec - Bits 111:96 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_6(&self) -> &KEY6_6 {
        &self.key6_6
    }
    #[doc = "0x1eee - Bits 127:112 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_7(&self) -> &KEY6_7 {
        &self.key6_7
    }
}
#[doc = "CHIPID0 (rw) register accessor: Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique.  

You can [`read`](crate::Reg::read) this register and get [`chipid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid0`]
module"]
pub type CHIPID0 = crate::Reg<chipid0::CHIPID0_SPEC>;
#[doc = "Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique."]
pub mod chipid0;
#[doc = "CHIPID1 (rw) register accessor: Bits 31:16 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid1`]
module"]
pub type CHIPID1 = crate::Reg<chipid1::CHIPID1_SPEC>;
#[doc = "Bits 31:16 of public device ID (ECC)"]
pub mod chipid1;
#[doc = "CHIPID2 (rw) register accessor: Bits 47:32 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid2`]
module"]
pub type CHIPID2 = crate::Reg<chipid2::CHIPID2_SPEC>;
#[doc = "Bits 47:32 of public device ID (ECC)"]
pub mod chipid2;
#[doc = "CHIPID3 (rw) register accessor: Bits 63:48 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid3`]
module"]
pub type CHIPID3 = crate::Reg<chipid3::CHIPID3_SPEC>;
#[doc = "Bits 63:48 of public device ID (ECC)"]
pub mod chipid3;
#[doc = "RANDID0 (rw) register accessor: Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0.  

You can [`read`](crate::Reg::read) this register and get [`randid0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid0`]
module"]
pub type RANDID0 = crate::Reg<randid0::RANDID0_SPEC>;
#[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
pub mod randid0;
#[doc = "RANDID1 (rw) register accessor: Bits 31:16 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid1`]
module"]
pub type RANDID1 = crate::Reg<randid1::RANDID1_SPEC>;
#[doc = "Bits 31:16 of private per-device random number (ECC)"]
pub mod randid1;
#[doc = "RANDID2 (rw) register accessor: Bits 47:32 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid2`]
module"]
pub type RANDID2 = crate::Reg<randid2::RANDID2_SPEC>;
#[doc = "Bits 47:32 of private per-device random number (ECC)"]
pub mod randid2;
#[doc = "RANDID3 (rw) register accessor: Bits 63:48 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid3`]
module"]
pub type RANDID3 = crate::Reg<randid3::RANDID3_SPEC>;
#[doc = "Bits 63:48 of private per-device random number (ECC)"]
pub mod randid3;
#[doc = "RANDID4 (rw) register accessor: Bits 79:64 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid4`]
module"]
pub type RANDID4 = crate::Reg<randid4::RANDID4_SPEC>;
#[doc = "Bits 79:64 of private per-device random number (ECC)"]
pub mod randid4;
#[doc = "RANDID5 (rw) register accessor: Bits 95:80 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid5`]
module"]
pub type RANDID5 = crate::Reg<randid5::RANDID5_SPEC>;
#[doc = "Bits 95:80 of private per-device random number (ECC)"]
pub mod randid5;
#[doc = "RANDID6 (rw) register accessor: Bits 111:96 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid6`]
module"]
pub type RANDID6 = crate::Reg<randid6::RANDID6_SPEC>;
#[doc = "Bits 111:96 of private per-device random number (ECC)"]
pub mod randid6;
#[doc = "RANDID7 (rw) register accessor: Bits 127:112 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid7`]
module"]
pub type RANDID7 = crate::Reg<randid7::RANDID7_SPEC>;
#[doc = "Bits 127:112 of private per-device random number (ECC)"]
pub mod randid7;
#[doc = "ROSC_CALIB (rw) register accessor: Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state.  

You can [`read`](crate::Reg::read) this register and get [`rosc_calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosc_calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rosc_calib`]
module"]
pub type ROSC_CALIB = crate::Reg<rosc_calib::ROSC_CALIB_SPEC>;
#[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
pub mod rosc_calib;
#[doc = "LPOSC_CALIB (rw) register accessor: Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state.  

You can [`read`](crate::Reg::read) this register and get [`lposc_calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lposc_calib`]
module"]
pub type LPOSC_CALIB = crate::Reg<lposc_calib::LPOSC_CALIB_SPEC>;
#[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
pub mod lposc_calib;
#[doc = "NUM_GPIOS (rw) register accessor: The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)  

You can [`read`](crate::Reg::read) this register and get [`num_gpios::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`num_gpios::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@num_gpios`]
module"]
pub type NUM_GPIOS = crate::Reg<num_gpios::NUM_GPIOS_SPEC>;
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
pub mod num_gpios;
#[doc = "INFO_CRC0 (rw) register accessor: Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_crc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@info_crc0`]
module"]
pub type INFO_CRC0 = crate::Reg<info_crc0::INFO_CRC0_SPEC>;
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
pub mod info_crc0;
#[doc = "INFO_CRC1 (rw) register accessor: Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_crc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@info_crc1`]
module"]
pub type INFO_CRC1 = crate::Reg<info_crc1::INFO_CRC1_SPEC>;
#[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
pub mod info_crc1;
#[doc = "FLASH_DEVINFO (rw) register accessor: Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set.  

You can [`read`](crate::Reg::read) this register and get [`flash_devinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_devinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flash_devinfo`]
module"]
pub type FLASH_DEVINFO = crate::Reg<flash_devinfo::FLASH_DEVINFO_SPEC>;
#[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
pub mod flash_devinfo;
#[doc = "FLASH_PARTITION_SLOT_SIZE (rw) register accessor: Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)  

You can [`read`](crate::Reg::read) this register and get [`flash_partition_slot_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_partition_slot_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flash_partition_slot_size`]
module"]
pub type FLASH_PARTITION_SLOT_SIZE =
    crate::Reg<flash_partition_slot_size::FLASH_PARTITION_SLOT_SIZE_SPEC>;
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
pub mod flash_partition_slot_size;
#[doc = "BOOTSEL_LED_CFG (rw) register accessor: Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_led_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootsel_led_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_led_cfg`]
module"]
pub type BOOTSEL_LED_CFG = crate::Reg<bootsel_led_cfg::BOOTSEL_LED_CFG_SPEC>;
#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
pub mod bootsel_led_cfg;
#[doc = "BOOTSEL_PLL_CFG (rw) register accessor: Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_pll_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootsel_pll_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_pll_cfg`]
module"]
pub type BOOTSEL_PLL_CFG = crate::Reg<bootsel_pll_cfg::BOOTSEL_PLL_CFG_SPEC>;
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
pub mod bootsel_pll_cfg;
#[doc = "BOOTSEL_XOSC_CFG (rw) register accessor: Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_xosc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootsel_xosc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_xosc_cfg`]
module"]
pub type BOOTSEL_XOSC_CFG = crate::Reg<bootsel_xosc_cfg::BOOTSEL_XOSC_CFG_SPEC>;
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
pub mod bootsel_xosc_cfg;
#[doc = "USB_WHITE_LABEL_ADDR (rw) register accessor: Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):  

You can [`read`](crate::Reg::read) this register and get [`usb_white_label_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_white_label_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_white_label_addr`]
module"]
pub type USB_WHITE_LABEL_ADDR = crate::Reg<usb_white_label_addr::USB_WHITE_LABEL_ADDR_SPEC>;
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
pub mod usb_white_label_addr;
#[doc = "OTPBOOT_SRC (rw) register accessor: OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_src`]
module"]
pub type OTPBOOT_SRC = crate::Reg<otpboot_src::OTPBOOT_SRC_SPEC>;
#[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
pub mod otpboot_src;
#[doc = "OTPBOOT_LEN (rw) register accessor: Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).  

You can [`read`](crate::Reg::read) this register and get [`otpboot_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_len`]
module"]
pub type OTPBOOT_LEN = crate::Reg<otpboot_len::OTPBOOT_LEN_SPEC>;
#[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
pub mod otpboot_len;
#[doc = "OTPBOOT_DST0 (rw) register accessor: Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_dst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_dst0`]
module"]
pub type OTPBOOT_DST0 = crate::Reg<otpboot_dst0::OTPBOOT_DST0_SPEC>;
#[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
pub mod otpboot_dst0;
#[doc = "OTPBOOT_DST1 (rw) register accessor: Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_dst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_dst1`]
module"]
pub type OTPBOOT_DST1 = crate::Reg<otpboot_dst1::OTPBOOT_DST1_SPEC>;
#[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
pub mod otpboot_dst1;
#[doc = "BOOTKEY0_0 (rw) register accessor: Bits 15:0 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_0`]
module"]
pub type BOOTKEY0_0 = crate::Reg<bootkey0_0::BOOTKEY0_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_0;
#[doc = "BOOTKEY0_1 (rw) register accessor: Bits 31:16 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_1`]
module"]
pub type BOOTKEY0_1 = crate::Reg<bootkey0_1::BOOTKEY0_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_1;
#[doc = "BOOTKEY0_2 (rw) register accessor: Bits 47:32 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_2`]
module"]
pub type BOOTKEY0_2 = crate::Reg<bootkey0_2::BOOTKEY0_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_2;
#[doc = "BOOTKEY0_3 (rw) register accessor: Bits 63:48 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_3`]
module"]
pub type BOOTKEY0_3 = crate::Reg<bootkey0_3::BOOTKEY0_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_3;
#[doc = "BOOTKEY0_4 (rw) register accessor: Bits 79:64 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_4`]
module"]
pub type BOOTKEY0_4 = crate::Reg<bootkey0_4::BOOTKEY0_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_4;
#[doc = "BOOTKEY0_5 (rw) register accessor: Bits 95:80 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_5`]
module"]
pub type BOOTKEY0_5 = crate::Reg<bootkey0_5::BOOTKEY0_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_5;
#[doc = "BOOTKEY0_6 (rw) register accessor: Bits 111:96 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_6`]
module"]
pub type BOOTKEY0_6 = crate::Reg<bootkey0_6::BOOTKEY0_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_6;
#[doc = "BOOTKEY0_7 (rw) register accessor: Bits 127:112 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_7`]
module"]
pub type BOOTKEY0_7 = crate::Reg<bootkey0_7::BOOTKEY0_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_7;
#[doc = "BOOTKEY0_8 (rw) register accessor: Bits 143:128 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_8`]
module"]
pub type BOOTKEY0_8 = crate::Reg<bootkey0_8::BOOTKEY0_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_8;
#[doc = "BOOTKEY0_9 (rw) register accessor: Bits 159:144 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_9`]
module"]
pub type BOOTKEY0_9 = crate::Reg<bootkey0_9::BOOTKEY0_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_9;
#[doc = "BOOTKEY0_10 (rw) register accessor: Bits 175:160 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_10`]
module"]
pub type BOOTKEY0_10 = crate::Reg<bootkey0_10::BOOTKEY0_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_10;
#[doc = "BOOTKEY0_11 (rw) register accessor: Bits 191:176 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_11`]
module"]
pub type BOOTKEY0_11 = crate::Reg<bootkey0_11::BOOTKEY0_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_11;
#[doc = "BOOTKEY0_12 (rw) register accessor: Bits 207:192 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_12`]
module"]
pub type BOOTKEY0_12 = crate::Reg<bootkey0_12::BOOTKEY0_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_12;
#[doc = "BOOTKEY0_13 (rw) register accessor: Bits 223:208 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_13`]
module"]
pub type BOOTKEY0_13 = crate::Reg<bootkey0_13::BOOTKEY0_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_13;
#[doc = "BOOTKEY0_14 (rw) register accessor: Bits 239:224 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_14`]
module"]
pub type BOOTKEY0_14 = crate::Reg<bootkey0_14::BOOTKEY0_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_14;
#[doc = "BOOTKEY0_15 (rw) register accessor: Bits 255:240 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey0_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_15`]
module"]
pub type BOOTKEY0_15 = crate::Reg<bootkey0_15::BOOTKEY0_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_15;
#[doc = "BOOTKEY1_0 (rw) register accessor: Bits 15:0 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_0`]
module"]
pub type BOOTKEY1_0 = crate::Reg<bootkey1_0::BOOTKEY1_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_0;
#[doc = "BOOTKEY1_1 (rw) register accessor: Bits 31:16 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_1`]
module"]
pub type BOOTKEY1_1 = crate::Reg<bootkey1_1::BOOTKEY1_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_1;
#[doc = "BOOTKEY1_2 (rw) register accessor: Bits 47:32 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_2`]
module"]
pub type BOOTKEY1_2 = crate::Reg<bootkey1_2::BOOTKEY1_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_2;
#[doc = "BOOTKEY1_3 (rw) register accessor: Bits 63:48 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_3`]
module"]
pub type BOOTKEY1_3 = crate::Reg<bootkey1_3::BOOTKEY1_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_3;
#[doc = "BOOTKEY1_4 (rw) register accessor: Bits 79:64 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_4`]
module"]
pub type BOOTKEY1_4 = crate::Reg<bootkey1_4::BOOTKEY1_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_4;
#[doc = "BOOTKEY1_5 (rw) register accessor: Bits 95:80 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_5`]
module"]
pub type BOOTKEY1_5 = crate::Reg<bootkey1_5::BOOTKEY1_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_5;
#[doc = "BOOTKEY1_6 (rw) register accessor: Bits 111:96 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_6`]
module"]
pub type BOOTKEY1_6 = crate::Reg<bootkey1_6::BOOTKEY1_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_6;
#[doc = "BOOTKEY1_7 (rw) register accessor: Bits 127:112 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_7`]
module"]
pub type BOOTKEY1_7 = crate::Reg<bootkey1_7::BOOTKEY1_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_7;
#[doc = "BOOTKEY1_8 (rw) register accessor: Bits 143:128 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_8`]
module"]
pub type BOOTKEY1_8 = crate::Reg<bootkey1_8::BOOTKEY1_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_8;
#[doc = "BOOTKEY1_9 (rw) register accessor: Bits 159:144 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_9`]
module"]
pub type BOOTKEY1_9 = crate::Reg<bootkey1_9::BOOTKEY1_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_9;
#[doc = "BOOTKEY1_10 (rw) register accessor: Bits 175:160 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_10`]
module"]
pub type BOOTKEY1_10 = crate::Reg<bootkey1_10::BOOTKEY1_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_10;
#[doc = "BOOTKEY1_11 (rw) register accessor: Bits 191:176 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_11`]
module"]
pub type BOOTKEY1_11 = crate::Reg<bootkey1_11::BOOTKEY1_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_11;
#[doc = "BOOTKEY1_12 (rw) register accessor: Bits 207:192 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_12`]
module"]
pub type BOOTKEY1_12 = crate::Reg<bootkey1_12::BOOTKEY1_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_12;
#[doc = "BOOTKEY1_13 (rw) register accessor: Bits 223:208 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_13`]
module"]
pub type BOOTKEY1_13 = crate::Reg<bootkey1_13::BOOTKEY1_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_13;
#[doc = "BOOTKEY1_14 (rw) register accessor: Bits 239:224 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_14`]
module"]
pub type BOOTKEY1_14 = crate::Reg<bootkey1_14::BOOTKEY1_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_14;
#[doc = "BOOTKEY1_15 (rw) register accessor: Bits 255:240 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_15`]
module"]
pub type BOOTKEY1_15 = crate::Reg<bootkey1_15::BOOTKEY1_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_15;
#[doc = "BOOTKEY2_0 (rw) register accessor: Bits 15:0 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_0`]
module"]
pub type BOOTKEY2_0 = crate::Reg<bootkey2_0::BOOTKEY2_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_0;
#[doc = "BOOTKEY2_1 (rw) register accessor: Bits 31:16 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_1`]
module"]
pub type BOOTKEY2_1 = crate::Reg<bootkey2_1::BOOTKEY2_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_1;
#[doc = "BOOTKEY2_2 (rw) register accessor: Bits 47:32 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_2`]
module"]
pub type BOOTKEY2_2 = crate::Reg<bootkey2_2::BOOTKEY2_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_2;
#[doc = "BOOTKEY2_3 (rw) register accessor: Bits 63:48 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_3`]
module"]
pub type BOOTKEY2_3 = crate::Reg<bootkey2_3::BOOTKEY2_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_3;
#[doc = "BOOTKEY2_4 (rw) register accessor: Bits 79:64 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_4`]
module"]
pub type BOOTKEY2_4 = crate::Reg<bootkey2_4::BOOTKEY2_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_4;
#[doc = "BOOTKEY2_5 (rw) register accessor: Bits 95:80 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_5`]
module"]
pub type BOOTKEY2_5 = crate::Reg<bootkey2_5::BOOTKEY2_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_5;
#[doc = "BOOTKEY2_6 (rw) register accessor: Bits 111:96 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_6`]
module"]
pub type BOOTKEY2_6 = crate::Reg<bootkey2_6::BOOTKEY2_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_6;
#[doc = "BOOTKEY2_7 (rw) register accessor: Bits 127:112 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_7`]
module"]
pub type BOOTKEY2_7 = crate::Reg<bootkey2_7::BOOTKEY2_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_7;
#[doc = "BOOTKEY2_8 (rw) register accessor: Bits 143:128 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_8`]
module"]
pub type BOOTKEY2_8 = crate::Reg<bootkey2_8::BOOTKEY2_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_8;
#[doc = "BOOTKEY2_9 (rw) register accessor: Bits 159:144 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_9`]
module"]
pub type BOOTKEY2_9 = crate::Reg<bootkey2_9::BOOTKEY2_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_9;
#[doc = "BOOTKEY2_10 (rw) register accessor: Bits 175:160 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_10`]
module"]
pub type BOOTKEY2_10 = crate::Reg<bootkey2_10::BOOTKEY2_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_10;
#[doc = "BOOTKEY2_11 (rw) register accessor: Bits 191:176 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_11`]
module"]
pub type BOOTKEY2_11 = crate::Reg<bootkey2_11::BOOTKEY2_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_11;
#[doc = "BOOTKEY2_12 (rw) register accessor: Bits 207:192 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_12`]
module"]
pub type BOOTKEY2_12 = crate::Reg<bootkey2_12::BOOTKEY2_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_12;
#[doc = "BOOTKEY2_13 (rw) register accessor: Bits 223:208 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_13`]
module"]
pub type BOOTKEY2_13 = crate::Reg<bootkey2_13::BOOTKEY2_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_13;
#[doc = "BOOTKEY2_14 (rw) register accessor: Bits 239:224 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_14`]
module"]
pub type BOOTKEY2_14 = crate::Reg<bootkey2_14::BOOTKEY2_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_14;
#[doc = "BOOTKEY2_15 (rw) register accessor: Bits 255:240 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_15`]
module"]
pub type BOOTKEY2_15 = crate::Reg<bootkey2_15::BOOTKEY2_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_15;
#[doc = "BOOTKEY3_0 (rw) register accessor: Bits 15:0 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_0`]
module"]
pub type BOOTKEY3_0 = crate::Reg<bootkey3_0::BOOTKEY3_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_0;
#[doc = "BOOTKEY3_1 (rw) register accessor: Bits 31:16 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_1`]
module"]
pub type BOOTKEY3_1 = crate::Reg<bootkey3_1::BOOTKEY3_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_1;
#[doc = "BOOTKEY3_2 (rw) register accessor: Bits 47:32 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_2`]
module"]
pub type BOOTKEY3_2 = crate::Reg<bootkey3_2::BOOTKEY3_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_2;
#[doc = "BOOTKEY3_3 (rw) register accessor: Bits 63:48 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_3`]
module"]
pub type BOOTKEY3_3 = crate::Reg<bootkey3_3::BOOTKEY3_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_3;
#[doc = "BOOTKEY3_4 (rw) register accessor: Bits 79:64 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_4`]
module"]
pub type BOOTKEY3_4 = crate::Reg<bootkey3_4::BOOTKEY3_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_4;
#[doc = "BOOTKEY3_5 (rw) register accessor: Bits 95:80 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_5`]
module"]
pub type BOOTKEY3_5 = crate::Reg<bootkey3_5::BOOTKEY3_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_5;
#[doc = "BOOTKEY3_6 (rw) register accessor: Bits 111:96 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_6`]
module"]
pub type BOOTKEY3_6 = crate::Reg<bootkey3_6::BOOTKEY3_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_6;
#[doc = "BOOTKEY3_7 (rw) register accessor: Bits 127:112 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_7`]
module"]
pub type BOOTKEY3_7 = crate::Reg<bootkey3_7::BOOTKEY3_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_7;
#[doc = "BOOTKEY3_8 (rw) register accessor: Bits 143:128 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_8`]
module"]
pub type BOOTKEY3_8 = crate::Reg<bootkey3_8::BOOTKEY3_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_8;
#[doc = "BOOTKEY3_9 (rw) register accessor: Bits 159:144 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_9`]
module"]
pub type BOOTKEY3_9 = crate::Reg<bootkey3_9::BOOTKEY3_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_9;
#[doc = "BOOTKEY3_10 (rw) register accessor: Bits 175:160 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_10`]
module"]
pub type BOOTKEY3_10 = crate::Reg<bootkey3_10::BOOTKEY3_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_10;
#[doc = "BOOTKEY3_11 (rw) register accessor: Bits 191:176 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_11`]
module"]
pub type BOOTKEY3_11 = crate::Reg<bootkey3_11::BOOTKEY3_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_11;
#[doc = "BOOTKEY3_12 (rw) register accessor: Bits 207:192 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_12`]
module"]
pub type BOOTKEY3_12 = crate::Reg<bootkey3_12::BOOTKEY3_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_12;
#[doc = "BOOTKEY3_13 (rw) register accessor: Bits 223:208 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_13`]
module"]
pub type BOOTKEY3_13 = crate::Reg<bootkey3_13::BOOTKEY3_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_13;
#[doc = "BOOTKEY3_14 (rw) register accessor: Bits 239:224 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_14`]
module"]
pub type BOOTKEY3_14 = crate::Reg<bootkey3_14::BOOTKEY3_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_14;
#[doc = "BOOTKEY3_15 (rw) register accessor: Bits 255:240 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey3_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_15`]
module"]
pub type BOOTKEY3_15 = crate::Reg<bootkey3_15::BOOTKEY3_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_15;
#[doc = "KEY1_0 (rw) register accessor: Bits 15:0 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_0`]
module"]
pub type KEY1_0 = crate::Reg<key1_0::KEY1_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
pub mod key1_0;
#[doc = "KEY1_1 (rw) register accessor: Bits 31:16 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_1`]
module"]
pub type KEY1_1 = crate::Reg<key1_1::KEY1_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
pub mod key1_1;
#[doc = "KEY1_2 (rw) register accessor: Bits 47:32 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_2`]
module"]
pub type KEY1_2 = crate::Reg<key1_2::KEY1_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
pub mod key1_2;
#[doc = "KEY1_3 (rw) register accessor: Bits 63:48 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_3`]
module"]
pub type KEY1_3 = crate::Reg<key1_3::KEY1_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
pub mod key1_3;
#[doc = "KEY1_4 (rw) register accessor: Bits 79:64 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_4`]
module"]
pub type KEY1_4 = crate::Reg<key1_4::KEY1_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
pub mod key1_4;
#[doc = "KEY1_5 (rw) register accessor: Bits 95:80 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_5`]
module"]
pub type KEY1_5 = crate::Reg<key1_5::KEY1_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
pub mod key1_5;
#[doc = "KEY1_6 (rw) register accessor: Bits 111:96 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_6`]
module"]
pub type KEY1_6 = crate::Reg<key1_6::KEY1_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
pub mod key1_6;
#[doc = "KEY1_7 (rw) register accessor: Bits 127:112 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_7`]
module"]
pub type KEY1_7 = crate::Reg<key1_7::KEY1_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
pub mod key1_7;
#[doc = "KEY2_0 (rw) register accessor: Bits 15:0 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_0`]
module"]
pub type KEY2_0 = crate::Reg<key2_0::KEY2_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
pub mod key2_0;
#[doc = "KEY2_1 (rw) register accessor: Bits 31:16 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_1`]
module"]
pub type KEY2_1 = crate::Reg<key2_1::KEY2_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
pub mod key2_1;
#[doc = "KEY2_2 (rw) register accessor: Bits 47:32 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_2`]
module"]
pub type KEY2_2 = crate::Reg<key2_2::KEY2_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
pub mod key2_2;
#[doc = "KEY2_3 (rw) register accessor: Bits 63:48 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_3`]
module"]
pub type KEY2_3 = crate::Reg<key2_3::KEY2_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
pub mod key2_3;
#[doc = "KEY2_4 (rw) register accessor: Bits 79:64 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_4`]
module"]
pub type KEY2_4 = crate::Reg<key2_4::KEY2_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
pub mod key2_4;
#[doc = "KEY2_5 (rw) register accessor: Bits 95:80 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_5`]
module"]
pub type KEY2_5 = crate::Reg<key2_5::KEY2_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
pub mod key2_5;
#[doc = "KEY2_6 (rw) register accessor: Bits 111:96 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_6`]
module"]
pub type KEY2_6 = crate::Reg<key2_6::KEY2_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
pub mod key2_6;
#[doc = "KEY2_7 (rw) register accessor: Bits 127:112 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_7`]
module"]
pub type KEY2_7 = crate::Reg<key2_7::KEY2_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
pub mod key2_7;
#[doc = "KEY3_0 (rw) register accessor: Bits 15:0 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_0`]
module"]
pub type KEY3_0 = crate::Reg<key3_0::KEY3_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
pub mod key3_0;
#[doc = "KEY3_1 (rw) register accessor: Bits 31:16 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_1`]
module"]
pub type KEY3_1 = crate::Reg<key3_1::KEY3_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
pub mod key3_1;
#[doc = "KEY3_2 (rw) register accessor: Bits 47:32 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_2`]
module"]
pub type KEY3_2 = crate::Reg<key3_2::KEY3_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
pub mod key3_2;
#[doc = "KEY3_3 (rw) register accessor: Bits 63:48 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_3`]
module"]
pub type KEY3_3 = crate::Reg<key3_3::KEY3_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
pub mod key3_3;
#[doc = "KEY3_4 (rw) register accessor: Bits 79:64 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_4`]
module"]
pub type KEY3_4 = crate::Reg<key3_4::KEY3_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
pub mod key3_4;
#[doc = "KEY3_5 (rw) register accessor: Bits 95:80 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_5`]
module"]
pub type KEY3_5 = crate::Reg<key3_5::KEY3_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
pub mod key3_5;
#[doc = "KEY3_6 (rw) register accessor: Bits 111:96 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_6`]
module"]
pub type KEY3_6 = crate::Reg<key3_6::KEY3_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
pub mod key3_6;
#[doc = "KEY3_7 (rw) register accessor: Bits 127:112 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_7`]
module"]
pub type KEY3_7 = crate::Reg<key3_7::KEY3_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
pub mod key3_7;
#[doc = "KEY4_0 (rw) register accessor: Bits 15:0 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_0`]
module"]
pub type KEY4_0 = crate::Reg<key4_0::KEY4_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
pub mod key4_0;
#[doc = "KEY4_1 (rw) register accessor: Bits 31:16 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_1`]
module"]
pub type KEY4_1 = crate::Reg<key4_1::KEY4_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
pub mod key4_1;
#[doc = "KEY4_2 (rw) register accessor: Bits 47:32 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_2`]
module"]
pub type KEY4_2 = crate::Reg<key4_2::KEY4_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
pub mod key4_2;
#[doc = "KEY4_3 (rw) register accessor: Bits 63:48 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_3`]
module"]
pub type KEY4_3 = crate::Reg<key4_3::KEY4_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
pub mod key4_3;
#[doc = "KEY4_4 (rw) register accessor: Bits 79:64 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_4`]
module"]
pub type KEY4_4 = crate::Reg<key4_4::KEY4_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
pub mod key4_4;
#[doc = "KEY4_5 (rw) register accessor: Bits 95:80 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_5`]
module"]
pub type KEY4_5 = crate::Reg<key4_5::KEY4_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
pub mod key4_5;
#[doc = "KEY4_6 (rw) register accessor: Bits 111:96 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_6`]
module"]
pub type KEY4_6 = crate::Reg<key4_6::KEY4_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
pub mod key4_6;
#[doc = "KEY4_7 (rw) register accessor: Bits 127:112 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_7`]
module"]
pub type KEY4_7 = crate::Reg<key4_7::KEY4_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
pub mod key4_7;
#[doc = "KEY5_0 (rw) register accessor: Bits 15:0 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_0`]
module"]
pub type KEY5_0 = crate::Reg<key5_0::KEY5_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
pub mod key5_0;
#[doc = "KEY5_1 (rw) register accessor: Bits 31:16 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_1`]
module"]
pub type KEY5_1 = crate::Reg<key5_1::KEY5_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
pub mod key5_1;
#[doc = "KEY5_2 (rw) register accessor: Bits 47:32 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_2`]
module"]
pub type KEY5_2 = crate::Reg<key5_2::KEY5_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
pub mod key5_2;
#[doc = "KEY5_3 (rw) register accessor: Bits 63:48 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_3`]
module"]
pub type KEY5_3 = crate::Reg<key5_3::KEY5_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
pub mod key5_3;
#[doc = "KEY5_4 (rw) register accessor: Bits 79:64 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_4`]
module"]
pub type KEY5_4 = crate::Reg<key5_4::KEY5_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
pub mod key5_4;
#[doc = "KEY5_5 (rw) register accessor: Bits 95:80 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_5`]
module"]
pub type KEY5_5 = crate::Reg<key5_5::KEY5_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
pub mod key5_5;
#[doc = "KEY5_6 (rw) register accessor: Bits 111:96 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_6`]
module"]
pub type KEY5_6 = crate::Reg<key5_6::KEY5_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
pub mod key5_6;
#[doc = "KEY5_7 (rw) register accessor: Bits 127:112 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_7`]
module"]
pub type KEY5_7 = crate::Reg<key5_7::KEY5_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
pub mod key5_7;
#[doc = "KEY6_0 (rw) register accessor: Bits 15:0 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_0`]
module"]
pub type KEY6_0 = crate::Reg<key6_0::KEY6_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
pub mod key6_0;
#[doc = "KEY6_1 (rw) register accessor: Bits 31:16 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_1`]
module"]
pub type KEY6_1 = crate::Reg<key6_1::KEY6_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
pub mod key6_1;
#[doc = "KEY6_2 (rw) register accessor: Bits 47:32 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_2`]
module"]
pub type KEY6_2 = crate::Reg<key6_2::KEY6_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
pub mod key6_2;
#[doc = "KEY6_3 (rw) register accessor: Bits 63:48 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_3`]
module"]
pub type KEY6_3 = crate::Reg<key6_3::KEY6_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
pub mod key6_3;
#[doc = "KEY6_4 (rw) register accessor: Bits 79:64 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_4`]
module"]
pub type KEY6_4 = crate::Reg<key6_4::KEY6_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
pub mod key6_4;
#[doc = "KEY6_5 (rw) register accessor: Bits 95:80 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_5`]
module"]
pub type KEY6_5 = crate::Reg<key6_5::KEY6_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
pub mod key6_5;
#[doc = "KEY6_6 (rw) register accessor: Bits 111:96 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_6`]
module"]
pub type KEY6_6 = crate::Reg<key6_6::KEY6_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
pub mod key6_6;
#[doc = "KEY6_7 (rw) register accessor: Bits 127:112 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_7`]
module"]
pub type KEY6_7 = crate::Reg<key6_7::KEY6_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
pub mod key6_7;
