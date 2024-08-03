#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_randid0: [u8; 0x0f],
    _reserved1: [u8; 0x01],
    _reserved_1_rosc_calib: [u8; 0x05],
    _reserved2: [u8; 0x21],
    _reserved_2_crit0: [u8; 0x2f],
    _reserved3: [u8; 0x1b],
    _reserved_3_bootkey0_0: [u8; 0x43],
    _reserved4: [u8; 0x0e85],
    _reserved_4_key1_0: [u8; 0xbb],
}
impl RegisterBlock {
    #[doc = "0x00 - Lower 16 bits of test lot/device number (ECC)"]
    #[inline(always)]
    pub const fn chipid_device0(&self) -> &CHIPID_DEVICE0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x01 - Upper 16 bits of test lot/device number (ECC)"]
    #[inline(always)]
    pub const fn chipid_device1(&self) -> &CHIPID_DEVICE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1).cast() }
    }
    #[doc = "0x02 - Lower 16 bits of wafer lot (ECC)"]
    #[inline(always)]
    pub const fn chipid_wafer0(&self) -> &CHIPID_WAFER0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - Upper 16 bits of wafer lot (ECC)"]
    #[inline(always)]
    pub const fn chipid_wafer1(&self) -> &CHIPID_WAFER1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3).cast() }
    }
    #[doc = "0x04 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid0(&self) -> &RANDID0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x05 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid1(&self) -> &RANDID1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(5).cast() }
    }
    #[doc = "0x06 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid2(&self) -> &RANDID2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid3(&self) -> &RANDID3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x08 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid4(&self) -> &RANDID4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x09 - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid5(&self) -> &RANDID5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(9).cast() }
    }
    #[doc = "0x0a - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid6(&self) -> &RANDID6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(10).cast() }
    }
    #[doc = "0x0b - 16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn randid7(&self) -> &RANDID7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(11).cast() }
    }
    #[doc = "0x10 - Ring oscillator calibration constant measured during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn rosc_calib(&self) -> &ROSC_CALIB {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - Low-power oscillator calibration constant measured during manufacturing (ECC)"]
    #[inline(always)]
    pub const fn lposc_calib(&self) -> &LPOSC_CALIB {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x36 - Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
    #[inline(always)]
    pub const fn info_crc0(&self) -> &INFO_CRC0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x37 - Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
    #[inline(always)]
    pub const fn info_crc1(&self) -> &INFO_CRC1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(55).cast() }
    }
    #[doc = "0x38 - Page 0 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit0(&self) -> &CRIT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x39 - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r1(&self) -> &CRIT0_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(57).cast() }
    }
    #[doc = "0x3a - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r2(&self) -> &CRIT0_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3b - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r3(&self) -> &CRIT0_R3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(59).cast() }
    }
    #[doc = "0x3c - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r4(&self) -> &CRIT0_R4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3d - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r5(&self) -> &CRIT0_R5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3e - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r6(&self) -> &CRIT0_R6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3f - Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r7(&self) -> &CRIT0_R7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(63).cast() }
    }
    #[doc = "0x40 - Page 1 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit1(&self) -> &CRIT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r1(&self) -> &CRIT1_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x42 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r2(&self) -> &CRIT1_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r3(&self) -> &CRIT1_R3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x44 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r4(&self) -> &CRIT1_R4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x45 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r5(&self) -> &CRIT1_R5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(69).cast() }
    }
    #[doc = "0x46 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r6(&self) -> &CRIT1_R6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x47 - Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r7(&self) -> &CRIT1_R7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(71).cast() }
    }
    #[doc = "0x48 - Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags0(&self) -> &BOOT_FLAGS0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x49 - Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r1(&self) -> &BOOT_FLAGS0_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(73).cast() }
    }
    #[doc = "0x4a - Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r2(&self) -> &BOOT_FLAGS0_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(74).cast() }
    }
    #[doc = "0x4b - Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags1(&self) -> &BOOT_FLAGS1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(75).cast() }
    }
    #[doc = "0x4c - Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r1(&self) -> &BOOT_FLAGS1_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4d - Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r2(&self) -> &BOOT_FLAGS1_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(77).cast() }
    }
    #[doc = "0x4e - Default boot version thermometer counter, bits 23:0 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version0(&self) -> &DEFAULT_BOOT_VERSION0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(78).cast() }
    }
    #[doc = "0x4f - Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r1(&self) -> &DEFAULT_BOOT_VERSION0_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(79).cast() }
    }
    #[doc = "0x50 - Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r2(&self) -> &DEFAULT_BOOT_VERSION0_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x51 - Default boot version thermometer counter, bits 47:24 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version1(&self) -> &DEFAULT_BOOT_VERSION1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(81).cast() }
    }
    #[doc = "0x52 - Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r1(&self) -> &DEFAULT_BOOT_VERSION1_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(82).cast() }
    }
    #[doc = "0x53 - Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r2(&self) -> &DEFAULT_BOOT_VERSION1_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(83).cast() }
    }
    #[doc = "0x54 - Stores information about external flash device(s). (ECC)  

 Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
    #[inline(always)]
    pub const fn flash_devinfo(&self) -> &FLASH_DEVINFO {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x55 - Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
    #[inline(always)]
    pub const fn flash_partition_slot_size(&self) -> &FLASH_PARTITION_SLOT_SIZE {
        unsafe { &*(self as *const Self).cast::<u8>().add(85).cast() }
    }
    #[doc = "0x56 - Pin configuration for LED status, used by USB bootloader. (ECC)  
 Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
    #[inline(always)]
    pub const fn bootsel_led_cfg(&self) -> &BOOTSEL_LED_CFG {
        unsafe { &*(self as *const Self).cast::<u8>().add(86).cast() }
    }
    #[doc = "0x57 - Optional PLL configuration for BOOTSEL mode. (ECC)  

 This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.  

 If no configuration is given, the crystal is assumed to be 12 MHz.  

 The PLL frequency can be calculated as:  

 PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)  

 Conversely the crystal frequency can be calculated as:  

 XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV  

 (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_pll_cfg(&self) -> &BOOTSEL_PLL_CFG {
        unsafe { &*(self as *const Self).cast::<u8>().add(87).cast() }
    }
    #[doc = "0x58 - Non-default crystal oscillator configuration for the USB bootloader. (ECC)  

 These values may also be used by user code configuring the crystal oscillator.  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_xosc_cfg(&self) -> &BOOTSEL_XOSC_CFG {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x59 - USB boot specific feature flags (RBIT-3)"]
    #[inline(always)]
    pub const fn usb_boot_flags(&self) -> &USB_BOOT_FLAGS {
        unsafe { &*(self as *const Self).cast::<u8>().add(89).cast() }
    }
    #[doc = "0x5a - Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r1(&self) -> &USB_BOOT_FLAGS_R1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(90).cast() }
    }
    #[doc = "0x5b - Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r2(&self) -> &USB_BOOT_FLAGS_R2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(91).cast() }
    }
    #[doc = "0x5c - Row index of the USB_WHITE_LABEL structure within OTP (ECC)  

 The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC).  

 The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value.  

 The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte.  

 In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom.  

 Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero.  

 Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used.  

 The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> &USB_WHITE_LABEL_ADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5e - OTP start row for the OTP boot image. (ECC)  

 If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.  

 This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
    #[inline(always)]
    pub const fn otpboot_src(&self) -> &OTPBOOT_SRC {
        unsafe { &*(self as *const Self).cast::<u8>().add(94).cast() }
    }
    #[doc = "0x5f - Length in rows of the OTP boot image. (ECC)  

 OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
    #[inline(always)]
    pub const fn otpboot_len(&self) -> &OTPBOOT_LEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(95).cast() }
    }
    #[doc = "0x60 - Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst0(&self) -> &OTPBOOT_DST0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x61 - Bits 31:16 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst1(&self) -> &OTPBOOT_DST1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(97).cast() }
    }
    #[doc = "0x80 - Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_0(&self) -> &BOOTKEY0_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x81 - Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_1(&self) -> &BOOTKEY0_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(129).cast() }
    }
    #[doc = "0x82 - Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_2(&self) -> &BOOTKEY0_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(130).cast() }
    }
    #[doc = "0x83 - Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_3(&self) -> &BOOTKEY0_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(131).cast() }
    }
    #[doc = "0x84 - Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_4(&self) -> &BOOTKEY0_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x85 - Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_5(&self) -> &BOOTKEY0_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(133).cast() }
    }
    #[doc = "0x86 - Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_6(&self) -> &BOOTKEY0_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(134).cast() }
    }
    #[doc = "0x87 - Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_7(&self) -> &BOOTKEY0_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(135).cast() }
    }
    #[doc = "0x88 - Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_8(&self) -> &BOOTKEY0_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x89 - Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_9(&self) -> &BOOTKEY0_9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(137).cast() }
    }
    #[doc = "0x8a - Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_10(&self) -> &BOOTKEY0_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_11(&self) -> &BOOTKEY0_11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0x8c - Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_12(&self) -> &BOOTKEY0_12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8d - Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_13(&self) -> &BOOTKEY0_13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(141).cast() }
    }
    #[doc = "0x8e - Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_14(&self) -> &BOOTKEY0_14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(142).cast() }
    }
    #[doc = "0x8f - Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_15(&self) -> &BOOTKEY0_15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(143).cast() }
    }
    #[doc = "0x90 - Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_0(&self) -> &BOOTKEY1_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x91 - Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_1(&self) -> &BOOTKEY1_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(145).cast() }
    }
    #[doc = "0x92 - Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_2(&self) -> &BOOTKEY1_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(146).cast() }
    }
    #[doc = "0x93 - Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_3(&self) -> &BOOTKEY1_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(147).cast() }
    }
    #[doc = "0x94 - Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_4(&self) -> &BOOTKEY1_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x95 - Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_5(&self) -> &BOOTKEY1_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(149).cast() }
    }
    #[doc = "0x96 - Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_6(&self) -> &BOOTKEY1_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(150).cast() }
    }
    #[doc = "0x97 - Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_7(&self) -> &BOOTKEY1_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(151).cast() }
    }
    #[doc = "0x98 - Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_8(&self) -> &BOOTKEY1_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x99 - Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_9(&self) -> &BOOTKEY1_9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(153).cast() }
    }
    #[doc = "0x9a - Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_10(&self) -> &BOOTKEY1_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(154).cast() }
    }
    #[doc = "0x9b - Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_11(&self) -> &BOOTKEY1_11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(155).cast() }
    }
    #[doc = "0x9c - Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_12(&self) -> &BOOTKEY1_12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9d - Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_13(&self) -> &BOOTKEY1_13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(157).cast() }
    }
    #[doc = "0x9e - Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_14(&self) -> &BOOTKEY1_14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(158).cast() }
    }
    #[doc = "0x9f - Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_15(&self) -> &BOOTKEY1_15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(159).cast() }
    }
    #[doc = "0xa0 - Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_0(&self) -> &BOOTKEY2_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa1 - Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_1(&self) -> &BOOTKEY2_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(161).cast() }
    }
    #[doc = "0xa2 - Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_2(&self) -> &BOOTKEY2_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(162).cast() }
    }
    #[doc = "0xa3 - Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_3(&self) -> &BOOTKEY2_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(163).cast() }
    }
    #[doc = "0xa4 - Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_4(&self) -> &BOOTKEY2_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa5 - Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_5(&self) -> &BOOTKEY2_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(165).cast() }
    }
    #[doc = "0xa6 - Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_6(&self) -> &BOOTKEY2_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(166).cast() }
    }
    #[doc = "0xa7 - Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_7(&self) -> &BOOTKEY2_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(167).cast() }
    }
    #[doc = "0xa8 - Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_8(&self) -> &BOOTKEY2_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa9 - Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_9(&self) -> &BOOTKEY2_9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(169).cast() }
    }
    #[doc = "0xaa - Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_10(&self) -> &BOOTKEY2_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(170).cast() }
    }
    #[doc = "0xab - Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_11(&self) -> &BOOTKEY2_11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(171).cast() }
    }
    #[doc = "0xac - Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_12(&self) -> &BOOTKEY2_12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xad - Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_13(&self) -> &BOOTKEY2_13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(173).cast() }
    }
    #[doc = "0xae - Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_14(&self) -> &BOOTKEY2_14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(174).cast() }
    }
    #[doc = "0xaf - Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_15(&self) -> &BOOTKEY2_15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(175).cast() }
    }
    #[doc = "0xb0 - Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_0(&self) -> &BOOTKEY3_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb1 - Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_1(&self) -> &BOOTKEY3_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(177).cast() }
    }
    #[doc = "0xb2 - Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_2(&self) -> &BOOTKEY3_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(178).cast() }
    }
    #[doc = "0xb3 - Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_3(&self) -> &BOOTKEY3_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(179).cast() }
    }
    #[doc = "0xb4 - Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_4(&self) -> &BOOTKEY3_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb5 - Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_5(&self) -> &BOOTKEY3_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(181).cast() }
    }
    #[doc = "0xb6 - Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_6(&self) -> &BOOTKEY3_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(182).cast() }
    }
    #[doc = "0xb7 - Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_7(&self) -> &BOOTKEY3_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(183).cast() }
    }
    #[doc = "0xb8 - Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_8(&self) -> &BOOTKEY3_8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb9 - Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_9(&self) -> &BOOTKEY3_9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(185).cast() }
    }
    #[doc = "0xba - Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_10(&self) -> &BOOTKEY3_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(186).cast() }
    }
    #[doc = "0xbb - Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_11(&self) -> &BOOTKEY3_11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(187).cast() }
    }
    #[doc = "0xbc - Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_12(&self) -> &BOOTKEY3_12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbd - Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_13(&self) -> &BOOTKEY3_13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(189).cast() }
    }
    #[doc = "0xbe - Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_14(&self) -> &BOOTKEY3_14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(190).cast() }
    }
    #[doc = "0xbf - Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_15(&self) -> &BOOTKEY3_15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(191).cast() }
    }
    #[doc = "0xf48 - Bits 15:0 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_0(&self) -> &KEY1_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3912).cast() }
    }
    #[doc = "0xf49 - Bits 31:16 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_1(&self) -> &KEY1_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3913).cast() }
    }
    #[doc = "0xf4a - Bits 47:32 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_2(&self) -> &KEY1_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3914).cast() }
    }
    #[doc = "0xf4b - Bits 63:48 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_3(&self) -> &KEY1_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3915).cast() }
    }
    #[doc = "0xf4c - Bits 79:64 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_4(&self) -> &KEY1_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3916).cast() }
    }
    #[doc = "0xf4d - Bits 95:80 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_5(&self) -> &KEY1_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3917).cast() }
    }
    #[doc = "0xf4e - Bits 111:96 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_6(&self) -> &KEY1_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3918).cast() }
    }
    #[doc = "0xf4f - Bits 127:112 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_7(&self) -> &KEY1_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3919).cast() }
    }
    #[doc = "0xf50 - Bits 15:0 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_0(&self) -> &KEY2_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3920).cast() }
    }
    #[doc = "0xf51 - Bits 31:16 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_1(&self) -> &KEY2_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3921).cast() }
    }
    #[doc = "0xf52 - Bits 47:32 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_2(&self) -> &KEY2_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3922).cast() }
    }
    #[doc = "0xf53 - Bits 63:48 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_3(&self) -> &KEY2_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3923).cast() }
    }
    #[doc = "0xf54 - Bits 79:64 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_4(&self) -> &KEY2_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3924).cast() }
    }
    #[doc = "0xf55 - Bits 95:80 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_5(&self) -> &KEY2_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3925).cast() }
    }
    #[doc = "0xf56 - Bits 111:96 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_6(&self) -> &KEY2_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3926).cast() }
    }
    #[doc = "0xf57 - Bits 127:112 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_7(&self) -> &KEY2_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3927).cast() }
    }
    #[doc = "0xf58 - Bits 15:0 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_0(&self) -> &KEY3_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3928).cast() }
    }
    #[doc = "0xf59 - Bits 31:16 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_1(&self) -> &KEY3_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3929).cast() }
    }
    #[doc = "0xf5a - Bits 47:32 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_2(&self) -> &KEY3_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3930).cast() }
    }
    #[doc = "0xf5b - Bits 63:48 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_3(&self) -> &KEY3_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3931).cast() }
    }
    #[doc = "0xf5c - Bits 79:64 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_4(&self) -> &KEY3_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3932).cast() }
    }
    #[doc = "0xf5d - Bits 95:80 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_5(&self) -> &KEY3_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3933).cast() }
    }
    #[doc = "0xf5e - Bits 111:96 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_6(&self) -> &KEY3_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3934).cast() }
    }
    #[doc = "0xf5f - Bits 127:112 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_7(&self) -> &KEY3_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3935).cast() }
    }
    #[doc = "0xf60 - Bits 15:0 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_0(&self) -> &KEY4_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3936).cast() }
    }
    #[doc = "0xf61 - Bits 31:16 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_1(&self) -> &KEY4_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3937).cast() }
    }
    #[doc = "0xf62 - Bits 47:32 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_2(&self) -> &KEY4_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3938).cast() }
    }
    #[doc = "0xf63 - Bits 63:48 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_3(&self) -> &KEY4_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3939).cast() }
    }
    #[doc = "0xf64 - Bits 79:64 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_4(&self) -> &KEY4_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3940).cast() }
    }
    #[doc = "0xf65 - Bits 95:80 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_5(&self) -> &KEY4_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3941).cast() }
    }
    #[doc = "0xf66 - Bits 111:96 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_6(&self) -> &KEY4_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3942).cast() }
    }
    #[doc = "0xf67 - Bits 127:112 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_7(&self) -> &KEY4_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3943).cast() }
    }
    #[doc = "0xf68 - Bits 15:0 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_0(&self) -> &KEY5_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3944).cast() }
    }
    #[doc = "0xf69 - Bits 31:16 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_1(&self) -> &KEY5_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3945).cast() }
    }
    #[doc = "0xf6a - Bits 47:32 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_2(&self) -> &KEY5_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3946).cast() }
    }
    #[doc = "0xf6b - Bits 63:48 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_3(&self) -> &KEY5_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3947).cast() }
    }
    #[doc = "0xf6c - Bits 79:64 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_4(&self) -> &KEY5_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3948).cast() }
    }
    #[doc = "0xf6d - Bits 95:80 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_5(&self) -> &KEY5_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3949).cast() }
    }
    #[doc = "0xf6e - Bits 111:96 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_6(&self) -> &KEY5_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3950).cast() }
    }
    #[doc = "0xf6f - Bits 127:112 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_7(&self) -> &KEY5_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3951).cast() }
    }
    #[doc = "0xf70 - Bits 15:0 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_0(&self) -> &KEY6_0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3952).cast() }
    }
    #[doc = "0xf71 - Bits 31:16 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_1(&self) -> &KEY6_1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3953).cast() }
    }
    #[doc = "0xf72 - Bits 47:32 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_2(&self) -> &KEY6_2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3954).cast() }
    }
    #[doc = "0xf73 - Bits 63:48 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_3(&self) -> &KEY6_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3955).cast() }
    }
    #[doc = "0xf74 - Bits 79:64 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_4(&self) -> &KEY6_4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3956).cast() }
    }
    #[doc = "0xf75 - Bits 95:80 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_5(&self) -> &KEY6_5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3957).cast() }
    }
    #[doc = "0xf76 - Bits 111:96 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_6(&self) -> &KEY6_6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3958).cast() }
    }
    #[doc = "0xf77 - Bits 127:112 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_7(&self) -> &KEY6_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3959).cast() }
    }
    #[doc = "0xf79 - Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key1_valid(&self) -> &KEY1_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3961).cast() }
    }
    #[doc = "0xf7a - Valid flag for key 2. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key2_valid(&self) -> &KEY2_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3962).cast() }
    }
    #[doc = "0xf7b - Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key3_valid(&self) -> &KEY3_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3963).cast() }
    }
    #[doc = "0xf7c - Valid flag for key 4. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key4_valid(&self) -> &KEY4_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3964).cast() }
    }
    #[doc = "0xf7d - Valid flag for key 5. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key5_valid(&self) -> &KEY5_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3965).cast() }
    }
    #[doc = "0xf7e - Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key6_valid(&self) -> &KEY6_VALID {
        unsafe { &*(self as *const Self).cast::<u8>().add(3966).cast() }
    }
    #[doc = "0xf80 - Lock configuration LSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page0_lock0(&self) -> &PAGE0_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3968).cast() }
    }
    #[doc = "0xf81 - Lock configuration MSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page0_lock1(&self) -> &PAGE0_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3969).cast() }
    }
    #[doc = "0xf82 - Lock configuration LSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page1_lock0(&self) -> &PAGE1_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3970).cast() }
    }
    #[doc = "0xf83 - Lock configuration MSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page1_lock1(&self) -> &PAGE1_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3971).cast() }
    }
    #[doc = "0xf84 - Lock configuration LSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page2_lock0(&self) -> &PAGE2_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3972).cast() }
    }
    #[doc = "0xf85 - Lock configuration MSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page2_lock1(&self) -> &PAGE2_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3973).cast() }
    }
    #[doc = "0xf86 - Lock configuration LSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page3_lock0(&self) -> &PAGE3_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3974).cast() }
    }
    #[doc = "0xf87 - Lock configuration MSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page3_lock1(&self) -> &PAGE3_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3975).cast() }
    }
    #[doc = "0xf88 - Lock configuration LSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page4_lock0(&self) -> &PAGE4_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3976).cast() }
    }
    #[doc = "0xf89 - Lock configuration MSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page4_lock1(&self) -> &PAGE4_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3977).cast() }
    }
    #[doc = "0xf8a - Lock configuration LSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page5_lock0(&self) -> &PAGE5_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3978).cast() }
    }
    #[doc = "0xf8b - Lock configuration MSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page5_lock1(&self) -> &PAGE5_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3979).cast() }
    }
    #[doc = "0xf8c - Lock configuration LSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page6_lock0(&self) -> &PAGE6_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3980).cast() }
    }
    #[doc = "0xf8d - Lock configuration MSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page6_lock1(&self) -> &PAGE6_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3981).cast() }
    }
    #[doc = "0xf8e - Lock configuration LSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page7_lock0(&self) -> &PAGE7_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3982).cast() }
    }
    #[doc = "0xf8f - Lock configuration MSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page7_lock1(&self) -> &PAGE7_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3983).cast() }
    }
    #[doc = "0xf90 - Lock configuration LSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page8_lock0(&self) -> &PAGE8_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3984).cast() }
    }
    #[doc = "0xf91 - Lock configuration MSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page8_lock1(&self) -> &PAGE8_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3985).cast() }
    }
    #[doc = "0xf92 - Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page9_lock0(&self) -> &PAGE9_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3986).cast() }
    }
    #[doc = "0xf93 - Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page9_lock1(&self) -> &PAGE9_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3987).cast() }
    }
    #[doc = "0xf94 - Lock configuration LSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page10_lock0(&self) -> &PAGE10_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3988).cast() }
    }
    #[doc = "0xf95 - Lock configuration MSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page10_lock1(&self) -> &PAGE10_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3989).cast() }
    }
    #[doc = "0xf96 - Lock configuration LSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page11_lock0(&self) -> &PAGE11_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3990).cast() }
    }
    #[doc = "0xf97 - Lock configuration MSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page11_lock1(&self) -> &PAGE11_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3991).cast() }
    }
    #[doc = "0xf98 - Lock configuration LSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page12_lock0(&self) -> &PAGE12_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3992).cast() }
    }
    #[doc = "0xf99 - Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page12_lock1(&self) -> &PAGE12_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3993).cast() }
    }
    #[doc = "0xf9a - Lock configuration LSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page13_lock0(&self) -> &PAGE13_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3994).cast() }
    }
    #[doc = "0xf9b - Lock configuration MSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page13_lock1(&self) -> &PAGE13_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3995).cast() }
    }
    #[doc = "0xf9c - Lock configuration LSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page14_lock0(&self) -> &PAGE14_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3996).cast() }
    }
    #[doc = "0xf9d - Lock configuration MSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page14_lock1(&self) -> &PAGE14_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3997).cast() }
    }
    #[doc = "0xf9e - Lock configuration LSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page15_lock0(&self) -> &PAGE15_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3998).cast() }
    }
    #[doc = "0xf9f - Lock configuration MSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page15_lock1(&self) -> &PAGE15_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(3999).cast() }
    }
    #[doc = "0xfa0 - Lock configuration LSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page16_lock0(&self) -> &PAGE16_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4000).cast() }
    }
    #[doc = "0xfa1 - Lock configuration MSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page16_lock1(&self) -> &PAGE16_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4001).cast() }
    }
    #[doc = "0xfa2 - Lock configuration LSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page17_lock0(&self) -> &PAGE17_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4002).cast() }
    }
    #[doc = "0xfa3 - Lock configuration MSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page17_lock1(&self) -> &PAGE17_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4003).cast() }
    }
    #[doc = "0xfa4 - Lock configuration LSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page18_lock0(&self) -> &PAGE18_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4004).cast() }
    }
    #[doc = "0xfa5 - Lock configuration MSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page18_lock1(&self) -> &PAGE18_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4005).cast() }
    }
    #[doc = "0xfa6 - Lock configuration LSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page19_lock0(&self) -> &PAGE19_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4006).cast() }
    }
    #[doc = "0xfa7 - Lock configuration MSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page19_lock1(&self) -> &PAGE19_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4007).cast() }
    }
    #[doc = "0xfa8 - Lock configuration LSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page20_lock0(&self) -> &PAGE20_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4008).cast() }
    }
    #[doc = "0xfa9 - Lock configuration MSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page20_lock1(&self) -> &PAGE20_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4009).cast() }
    }
    #[doc = "0xfaa - Lock configuration LSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page21_lock0(&self) -> &PAGE21_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4010).cast() }
    }
    #[doc = "0xfab - Lock configuration MSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page21_lock1(&self) -> &PAGE21_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4011).cast() }
    }
    #[doc = "0xfac - Lock configuration LSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page22_lock0(&self) -> &PAGE22_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4012).cast() }
    }
    #[doc = "0xfad - Lock configuration MSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page22_lock1(&self) -> &PAGE22_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4013).cast() }
    }
    #[doc = "0xfae - Lock configuration LSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page23_lock0(&self) -> &PAGE23_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4014).cast() }
    }
    #[doc = "0xfaf - Lock configuration MSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page23_lock1(&self) -> &PAGE23_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4015).cast() }
    }
    #[doc = "0xfb0 - Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page24_lock0(&self) -> &PAGE24_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4016).cast() }
    }
    #[doc = "0xfb1 - Lock configuration MSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page24_lock1(&self) -> &PAGE24_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4017).cast() }
    }
    #[doc = "0xfb2 - Lock configuration LSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page25_lock0(&self) -> &PAGE25_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4018).cast() }
    }
    #[doc = "0xfb3 - Lock configuration MSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page25_lock1(&self) -> &PAGE25_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4019).cast() }
    }
    #[doc = "0xfb4 - Lock configuration LSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page26_lock0(&self) -> &PAGE26_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4020).cast() }
    }
    #[doc = "0xfb5 - Lock configuration MSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page26_lock1(&self) -> &PAGE26_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4021).cast() }
    }
    #[doc = "0xfb6 - Lock configuration LSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page27_lock0(&self) -> &PAGE27_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4022).cast() }
    }
    #[doc = "0xfb7 - Lock configuration MSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page27_lock1(&self) -> &PAGE27_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4023).cast() }
    }
    #[doc = "0xfb8 - Lock configuration LSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page28_lock0(&self) -> &PAGE28_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4024).cast() }
    }
    #[doc = "0xfb9 - Lock configuration MSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page28_lock1(&self) -> &PAGE28_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4025).cast() }
    }
    #[doc = "0xfba - Lock configuration LSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page29_lock0(&self) -> &PAGE29_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4026).cast() }
    }
    #[doc = "0xfbb - Lock configuration MSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page29_lock1(&self) -> &PAGE29_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4027).cast() }
    }
    #[doc = "0xfbc - Lock configuration LSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page30_lock0(&self) -> &PAGE30_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4028).cast() }
    }
    #[doc = "0xfbd - Lock configuration MSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page30_lock1(&self) -> &PAGE30_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4029).cast() }
    }
    #[doc = "0xfbe - Lock configuration LSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page31_lock0(&self) -> &PAGE31_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4030).cast() }
    }
    #[doc = "0xfbf - Lock configuration MSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page31_lock1(&self) -> &PAGE31_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4031).cast() }
    }
    #[doc = "0xfc0 - Lock configuration LSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page32_lock0(&self) -> &PAGE32_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4032).cast() }
    }
    #[doc = "0xfc1 - Lock configuration MSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page32_lock1(&self) -> &PAGE32_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4033).cast() }
    }
    #[doc = "0xfc2 - Lock configuration LSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page33_lock0(&self) -> &PAGE33_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4034).cast() }
    }
    #[doc = "0xfc3 - Lock configuration MSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page33_lock1(&self) -> &PAGE33_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4035).cast() }
    }
    #[doc = "0xfc4 - Lock configuration LSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page34_lock0(&self) -> &PAGE34_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4036).cast() }
    }
    #[doc = "0xfc5 - Lock configuration MSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page34_lock1(&self) -> &PAGE34_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4037).cast() }
    }
    #[doc = "0xfc6 - Lock configuration LSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page35_lock0(&self) -> &PAGE35_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4038).cast() }
    }
    #[doc = "0xfc7 - Lock configuration MSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page35_lock1(&self) -> &PAGE35_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4039).cast() }
    }
    #[doc = "0xfc8 - Lock configuration LSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page36_lock0(&self) -> &PAGE36_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4040).cast() }
    }
    #[doc = "0xfc9 - Lock configuration MSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page36_lock1(&self) -> &PAGE36_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4041).cast() }
    }
    #[doc = "0xfca - Lock configuration LSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page37_lock0(&self) -> &PAGE37_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4042).cast() }
    }
    #[doc = "0xfcb - Lock configuration MSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page37_lock1(&self) -> &PAGE37_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4043).cast() }
    }
    #[doc = "0xfcc - Lock configuration LSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page38_lock0(&self) -> &PAGE38_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4044).cast() }
    }
    #[doc = "0xfcd - Lock configuration MSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page38_lock1(&self) -> &PAGE38_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4045).cast() }
    }
    #[doc = "0xfce - Lock configuration LSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page39_lock0(&self) -> &PAGE39_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4046).cast() }
    }
    #[doc = "0xfcf - Lock configuration MSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page39_lock1(&self) -> &PAGE39_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4047).cast() }
    }
    #[doc = "0xfd0 - Lock configuration LSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page40_lock0(&self) -> &PAGE40_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4048).cast() }
    }
    #[doc = "0xfd1 - Lock configuration MSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page40_lock1(&self) -> &PAGE40_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4049).cast() }
    }
    #[doc = "0xfd2 - Lock configuration LSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page41_lock0(&self) -> &PAGE41_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4050).cast() }
    }
    #[doc = "0xfd3 - Lock configuration MSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page41_lock1(&self) -> &PAGE41_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4051).cast() }
    }
    #[doc = "0xfd4 - Lock configuration LSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page42_lock0(&self) -> &PAGE42_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4052).cast() }
    }
    #[doc = "0xfd5 - Lock configuration MSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page42_lock1(&self) -> &PAGE42_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4053).cast() }
    }
    #[doc = "0xfd6 - Lock configuration LSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page43_lock0(&self) -> &PAGE43_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4054).cast() }
    }
    #[doc = "0xfd7 - Lock configuration MSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page43_lock1(&self) -> &PAGE43_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4055).cast() }
    }
    #[doc = "0xfd8 - Lock configuration LSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page44_lock0(&self) -> &PAGE44_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4056).cast() }
    }
    #[doc = "0xfd9 - Lock configuration MSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page44_lock1(&self) -> &PAGE44_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4057).cast() }
    }
    #[doc = "0xfda - Lock configuration LSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page45_lock0(&self) -> &PAGE45_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4058).cast() }
    }
    #[doc = "0xfdb - Lock configuration MSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page45_lock1(&self) -> &PAGE45_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4059).cast() }
    }
    #[doc = "0xfdc - Lock configuration LSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page46_lock0(&self) -> &PAGE46_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4060).cast() }
    }
    #[doc = "0xfdd - Lock configuration MSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page46_lock1(&self) -> &PAGE46_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4061).cast() }
    }
    #[doc = "0xfde - Lock configuration LSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page47_lock0(&self) -> &PAGE47_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4062).cast() }
    }
    #[doc = "0xfdf - Lock configuration MSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page47_lock1(&self) -> &PAGE47_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4063).cast() }
    }
    #[doc = "0xfe0 - Lock configuration LSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page48_lock0(&self) -> &PAGE48_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4064).cast() }
    }
    #[doc = "0xfe1 - Lock configuration MSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page48_lock1(&self) -> &PAGE48_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4065).cast() }
    }
    #[doc = "0xfe2 - Lock configuration LSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page49_lock0(&self) -> &PAGE49_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4066).cast() }
    }
    #[doc = "0xfe3 - Lock configuration MSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page49_lock1(&self) -> &PAGE49_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4067).cast() }
    }
    #[doc = "0xfe4 - Lock configuration LSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page50_lock0(&self) -> &PAGE50_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4068).cast() }
    }
    #[doc = "0xfe5 - Lock configuration MSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page50_lock1(&self) -> &PAGE50_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4069).cast() }
    }
    #[doc = "0xfe6 - Lock configuration LSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page51_lock0(&self) -> &PAGE51_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4070).cast() }
    }
    #[doc = "0xfe7 - Lock configuration MSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page51_lock1(&self) -> &PAGE51_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4071).cast() }
    }
    #[doc = "0xfe8 - Lock configuration LSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page52_lock0(&self) -> &PAGE52_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4072).cast() }
    }
    #[doc = "0xfe9 - Lock configuration MSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page52_lock1(&self) -> &PAGE52_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4073).cast() }
    }
    #[doc = "0xfea - Lock configuration LSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page53_lock0(&self) -> &PAGE53_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4074).cast() }
    }
    #[doc = "0xfeb - Lock configuration MSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page53_lock1(&self) -> &PAGE53_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4075).cast() }
    }
    #[doc = "0xfec - Lock configuration LSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page54_lock0(&self) -> &PAGE54_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4076).cast() }
    }
    #[doc = "0xfed - Lock configuration MSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page54_lock1(&self) -> &PAGE54_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4077).cast() }
    }
    #[doc = "0xfee - Lock configuration LSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page55_lock0(&self) -> &PAGE55_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4078).cast() }
    }
    #[doc = "0xfef - Lock configuration MSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page55_lock1(&self) -> &PAGE55_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4079).cast() }
    }
    #[doc = "0xff0 - Lock configuration LSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page56_lock0(&self) -> &PAGE56_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4080).cast() }
    }
    #[doc = "0xff1 - Lock configuration MSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page56_lock1(&self) -> &PAGE56_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4081).cast() }
    }
    #[doc = "0xff2 - Lock configuration LSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page57_lock0(&self) -> &PAGE57_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4082).cast() }
    }
    #[doc = "0xff3 - Lock configuration MSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page57_lock1(&self) -> &PAGE57_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4083).cast() }
    }
    #[doc = "0xff4 - Lock configuration LSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page58_lock0(&self) -> &PAGE58_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4084).cast() }
    }
    #[doc = "0xff5 - Lock configuration MSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page58_lock1(&self) -> &PAGE58_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4085).cast() }
    }
    #[doc = "0xff6 - Lock configuration LSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page59_lock0(&self) -> &PAGE59_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4086).cast() }
    }
    #[doc = "0xff7 - Lock configuration MSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page59_lock1(&self) -> &PAGE59_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4087).cast() }
    }
    #[doc = "0xff8 - Lock configuration LSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page60_lock0(&self) -> &PAGE60_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4088).cast() }
    }
    #[doc = "0xff9 - Lock configuration MSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page60_lock1(&self) -> &PAGE60_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4089).cast() }
    }
    #[doc = "0xffa - Lock configuration LSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page61_lock0(&self) -> &PAGE61_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4090).cast() }
    }
    #[doc = "0xffb - Lock configuration MSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page61_lock1(&self) -> &PAGE61_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4091).cast() }
    }
    #[doc = "0xffc - Lock configuration LSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page62_lock0(&self) -> &PAGE62_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4092).cast() }
    }
    #[doc = "0xffd - Lock configuration MSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page62_lock1(&self) -> &PAGE62_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4093).cast() }
    }
    #[doc = "0xffe - Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page63_lock0(&self) -> &PAGE63_LOCK0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4094).cast() }
    }
    #[doc = "0xfff - Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page63_lock1(&self) -> &PAGE63_LOCK1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4095).cast() }
    }
}
#[doc = "CHIPID_DEVICE0 (r) register accessor: Lower 16 bits of test lot/device number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_device0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid_device0`]
module"]
pub type CHIPID_DEVICE0 = crate::Reg<chipid_device0::CHIPID_DEVICE0_SPEC>;
#[doc = "Lower 16 bits of test lot/device number (ECC)"]
pub mod chipid_device0;
#[doc = "CHIPID_DEVICE1 (r) register accessor: Upper 16 bits of test lot/device number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_device1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid_device1`]
module"]
pub type CHIPID_DEVICE1 = crate::Reg<chipid_device1::CHIPID_DEVICE1_SPEC>;
#[doc = "Upper 16 bits of test lot/device number (ECC)"]
pub mod chipid_device1;
#[doc = "CHIPID_WAFER0 (r) register accessor: Lower 16 bits of wafer lot (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_wafer0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid_wafer0`]
module"]
pub type CHIPID_WAFER0 = crate::Reg<chipid_wafer0::CHIPID_WAFER0_SPEC>;
#[doc = "Lower 16 bits of wafer lot (ECC)"]
pub mod chipid_wafer0;
#[doc = "CHIPID_WAFER1 (r) register accessor: Upper 16 bits of wafer lot (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_wafer1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@chipid_wafer1`]
module"]
pub type CHIPID_WAFER1 = crate::Reg<chipid_wafer1::CHIPID_WAFER1_SPEC>;
#[doc = "Upper 16 bits of wafer lot (ECC)"]
pub mod chipid_wafer1;
#[doc = "RANDID0 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid0`]
module"]
pub type RANDID0 = crate::Reg<randid0::RANDID0_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid0;
#[doc = "RANDID1 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid1`]
module"]
pub type RANDID1 = crate::Reg<randid1::RANDID1_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid1;
#[doc = "RANDID2 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid2`]
module"]
pub type RANDID2 = crate::Reg<randid2::RANDID2_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid2;
#[doc = "RANDID3 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid3`]
module"]
pub type RANDID3 = crate::Reg<randid3::RANDID3_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid3;
#[doc = "RANDID4 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid4`]
module"]
pub type RANDID4 = crate::Reg<randid4::RANDID4_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid4;
#[doc = "RANDID5 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid5`]
module"]
pub type RANDID5 = crate::Reg<randid5::RANDID5_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid5;
#[doc = "RANDID6 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid6`]
module"]
pub type RANDID6 = crate::Reg<randid6::RANDID6_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid6;
#[doc = "RANDID7 (r) register accessor: 16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@randid7`]
module"]
pub type RANDID7 = crate::Reg<randid7::RANDID7_SPEC>;
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)"]
pub mod randid7;
#[doc = "ROSC_CALIB (r) register accessor: Ring oscillator calibration constant measured during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`rosc_calib::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@rosc_calib`]
module"]
pub type ROSC_CALIB = crate::Reg<rosc_calib::ROSC_CALIB_SPEC>;
#[doc = "Ring oscillator calibration constant measured during manufacturing (ECC)"]
pub mod rosc_calib;
#[doc = "LPOSC_CALIB (r) register accessor: Low-power oscillator calibration constant measured during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`lposc_calib::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lposc_calib`]
module"]
pub type LPOSC_CALIB = crate::Reg<lposc_calib::LPOSC_CALIB_SPEC>;
#[doc = "Low-power oscillator calibration constant measured during manufacturing (ECC)"]
pub mod lposc_calib;
#[doc = "INFO_CRC0 (r) register accessor: Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@info_crc0`]
module"]
pub type INFO_CRC0 = crate::Reg<info_crc0::INFO_CRC0_SPEC>;
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
pub mod info_crc0;
#[doc = "INFO_CRC1 (r) register accessor: Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)  

You can [`read`](crate::Reg::read) this register and get [`info_crc1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@info_crc1`]
module"]
pub type INFO_CRC1 = crate::Reg<info_crc1::INFO_CRC1_SPEC>;
#[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
pub mod info_crc1;
#[doc = "CRIT0 (r) register accessor: Page 0 critical boot flags (RBIT-8)  

You can [`read`](crate::Reg::read) this register and get [`crit0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0`]
module"]
pub type CRIT0 = crate::Reg<crit0::CRIT0_SPEC>;
#[doc = "Page 0 critical boot flags (RBIT-8)"]
pub mod crit0;
#[doc = "CRIT0_R1 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r1`]
module"]
pub type CRIT0_R1 = crate::Reg<crit0_r1::CRIT0_R1_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r1;
#[doc = "CRIT0_R2 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r2`]
module"]
pub type CRIT0_R2 = crate::Reg<crit0_r2::CRIT0_R2_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r2;
#[doc = "CRIT0_R3 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r3`]
module"]
pub type CRIT0_R3 = crate::Reg<crit0_r3::CRIT0_R3_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r3;
#[doc = "CRIT0_R4 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r4`]
module"]
pub type CRIT0_R4 = crate::Reg<crit0_r4::CRIT0_R4_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r4;
#[doc = "CRIT0_R5 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r5`]
module"]
pub type CRIT0_R5 = crate::Reg<crit0_r5::CRIT0_R5_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r5;
#[doc = "CRIT0_R6 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r6`]
module"]
pub type CRIT0_R6 = crate::Reg<crit0_r6::CRIT0_R6_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r6;
#[doc = "CRIT0_R7 (r) register accessor: Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit0_r7`]
module"]
pub type CRIT0_R7 = crate::Reg<crit0_r7::CRIT0_R7_SPEC>;
#[doc = "Redundant copy of CRIT0"]
pub mod crit0_r7;
#[doc = "CRIT1 (r) register accessor: Page 1 critical boot flags (RBIT-8)  

You can [`read`](crate::Reg::read) this register and get [`crit1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1`]
module"]
pub type CRIT1 = crate::Reg<crit1::CRIT1_SPEC>;
#[doc = "Page 1 critical boot flags (RBIT-8)"]
pub mod crit1;
#[doc = "CRIT1_R1 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r1`]
module"]
pub type CRIT1_R1 = crate::Reg<crit1_r1::CRIT1_R1_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r1;
#[doc = "CRIT1_R2 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r2`]
module"]
pub type CRIT1_R2 = crate::Reg<crit1_r2::CRIT1_R2_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r2;
#[doc = "CRIT1_R3 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r3`]
module"]
pub type CRIT1_R3 = crate::Reg<crit1_r3::CRIT1_R3_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r3;
#[doc = "CRIT1_R4 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r4`]
module"]
pub type CRIT1_R4 = crate::Reg<crit1_r4::CRIT1_R4_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r4;
#[doc = "CRIT1_R5 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r5`]
module"]
pub type CRIT1_R5 = crate::Reg<crit1_r5::CRIT1_R5_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r5;
#[doc = "CRIT1_R6 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r6`]
module"]
pub type CRIT1_R6 = crate::Reg<crit1_r6::CRIT1_R6_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r6;
#[doc = "CRIT1_R7 (r) register accessor: Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@crit1_r7`]
module"]
pub type CRIT1_R7 = crate::Reg<crit1_r7::CRIT1_R7_SPEC>;
#[doc = "Redundant copy of CRIT1"]
pub mod crit1_r7;
#[doc = "BOOT_FLAGS0 (r) register accessor: Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags0`]
module"]
pub type BOOT_FLAGS0 = crate::Reg<boot_flags0::BOOT_FLAGS0_SPEC>;
#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
pub mod boot_flags0;
#[doc = "BOOT_FLAGS0_R1 (r) register accessor: Redundant copy of BOOT_FLAGS0  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags0_r1`]
module"]
pub type BOOT_FLAGS0_R1 = crate::Reg<boot_flags0_r1::BOOT_FLAGS0_R1_SPEC>;
#[doc = "Redundant copy of BOOT_FLAGS0"]
pub mod boot_flags0_r1;
#[doc = "BOOT_FLAGS0_R2 (r) register accessor: Redundant copy of BOOT_FLAGS0  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags0_r2`]
module"]
pub type BOOT_FLAGS0_R2 = crate::Reg<boot_flags0_r2::BOOT_FLAGS0_R2_SPEC>;
#[doc = "Redundant copy of BOOT_FLAGS0"]
pub mod boot_flags0_r2;
#[doc = "BOOT_FLAGS1 (r) register accessor: Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags1`]
module"]
pub type BOOT_FLAGS1 = crate::Reg<boot_flags1::BOOT_FLAGS1_SPEC>;
#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
pub mod boot_flags1;
#[doc = "BOOT_FLAGS1_R1 (r) register accessor: Redundant copy of BOOT_FLAGS1  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags1_r1`]
module"]
pub type BOOT_FLAGS1_R1 = crate::Reg<boot_flags1_r1::BOOT_FLAGS1_R1_SPEC>;
#[doc = "Redundant copy of BOOT_FLAGS1"]
pub mod boot_flags1_r1;
#[doc = "BOOT_FLAGS1_R2 (r) register accessor: Redundant copy of BOOT_FLAGS1  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@boot_flags1_r2`]
module"]
pub type BOOT_FLAGS1_R2 = crate::Reg<boot_flags1_r2::BOOT_FLAGS1_R2_SPEC>;
#[doc = "Redundant copy of BOOT_FLAGS1"]
pub mod boot_flags1_r2;
#[doc = "DEFAULT_BOOT_VERSION0 (r) register accessor: Default boot version thermometer counter, bits 23:0 (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version0`]
module"]
pub type DEFAULT_BOOT_VERSION0 = crate::Reg<default_boot_version0::DEFAULT_BOOT_VERSION0_SPEC>;
#[doc = "Default boot version thermometer counter, bits 23:0 (RBIT-3)"]
pub mod default_boot_version0;
#[doc = "DEFAULT_BOOT_VERSION0_R1 (r) register accessor: Redundant copy of DEFAULT_BOOT_VERSION0  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version0_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version0_r1`]
module"]
pub type DEFAULT_BOOT_VERSION0_R1 =
    crate::Reg<default_boot_version0_r1::DEFAULT_BOOT_VERSION0_R1_SPEC>;
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
pub mod default_boot_version0_r1;
#[doc = "DEFAULT_BOOT_VERSION0_R2 (r) register accessor: Redundant copy of DEFAULT_BOOT_VERSION0  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version0_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version0_r2`]
module"]
pub type DEFAULT_BOOT_VERSION0_R2 =
    crate::Reg<default_boot_version0_r2::DEFAULT_BOOT_VERSION0_R2_SPEC>;
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
pub mod default_boot_version0_r2;
#[doc = "DEFAULT_BOOT_VERSION1 (r) register accessor: Default boot version thermometer counter, bits 47:24 (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version1`]
module"]
pub type DEFAULT_BOOT_VERSION1 = crate::Reg<default_boot_version1::DEFAULT_BOOT_VERSION1_SPEC>;
#[doc = "Default boot version thermometer counter, bits 47:24 (RBIT-3)"]
pub mod default_boot_version1;
#[doc = "DEFAULT_BOOT_VERSION1_R1 (r) register accessor: Redundant copy of DEFAULT_BOOT_VERSION1  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version1_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version1_r1`]
module"]
pub type DEFAULT_BOOT_VERSION1_R1 =
    crate::Reg<default_boot_version1_r1::DEFAULT_BOOT_VERSION1_R1_SPEC>;
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
pub mod default_boot_version1_r1;
#[doc = "DEFAULT_BOOT_VERSION1_R2 (r) register accessor: Redundant copy of DEFAULT_BOOT_VERSION1  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version1_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@default_boot_version1_r2`]
module"]
pub type DEFAULT_BOOT_VERSION1_R2 =
    crate::Reg<default_boot_version1_r2::DEFAULT_BOOT_VERSION1_R2_SPEC>;
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
pub mod default_boot_version1_r2;
#[doc = "FLASH_DEVINFO (r) register accessor: Stores information about external flash device(s). (ECC)  

 Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set.  

You can [`read`](crate::Reg::read) this register and get [`flash_devinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flash_devinfo`]
module"]
pub type FLASH_DEVINFO = crate::Reg<flash_devinfo::FLASH_DEVINFO_SPEC>;
#[doc = "Stores information about external flash device(s). (ECC)  

 Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
pub mod flash_devinfo;
#[doc = "FLASH_PARTITION_SLOT_SIZE (r) register accessor: Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)  

You can [`read`](crate::Reg::read) this register and get [`flash_partition_slot_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@flash_partition_slot_size`]
module"]
pub type FLASH_PARTITION_SLOT_SIZE =
    crate::Reg<flash_partition_slot_size::FLASH_PARTITION_SLOT_SIZE_SPEC>;
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
pub mod flash_partition_slot_size;
#[doc = "BOOTSEL_LED_CFG (r) register accessor: Pin configuration for LED status, used by USB bootloader. (ECC)  
 Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_led_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_led_cfg`]
module"]
pub type BOOTSEL_LED_CFG = crate::Reg<bootsel_led_cfg::BOOTSEL_LED_CFG_SPEC>;
#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC)  
 Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
pub mod bootsel_led_cfg;
#[doc = "BOOTSEL_PLL_CFG (r) register accessor: Optional PLL configuration for BOOTSEL mode. (ECC)  

 This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.  

 If no configuration is given, the crystal is assumed to be 12 MHz.  

 The PLL frequency can be calculated as:  

 PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)  

 Conversely the crystal frequency can be calculated as:  

 XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV  

 (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_pll_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_pll_cfg`]
module"]
pub type BOOTSEL_PLL_CFG = crate::Reg<bootsel_pll_cfg::BOOTSEL_PLL_CFG_SPEC>;
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC)  

 This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.  

 If no configuration is given, the crystal is assumed to be 12 MHz.  

 The PLL frequency can be calculated as:  

 PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)  

 Conversely the crystal frequency can be calculated as:  

 XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV  

 (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
pub mod bootsel_pll_cfg;
#[doc = "BOOTSEL_XOSC_CFG (r) register accessor: Non-default crystal oscillator configuration for the USB bootloader. (ECC)  

 These values may also be used by user code configuring the crystal oscillator.  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_xosc_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootsel_xosc_cfg`]
module"]
pub type BOOTSEL_XOSC_CFG = crate::Reg<bootsel_xosc_cfg::BOOTSEL_XOSC_CFG_SPEC>;
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC)  

 These values may also be used by user code configuring the crystal oscillator.  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
pub mod bootsel_xosc_cfg;
#[doc = "USB_BOOT_FLAGS (r) register accessor: USB boot specific feature flags (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`usb_boot_flags::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_boot_flags`]
module"]
pub type USB_BOOT_FLAGS = crate::Reg<usb_boot_flags::USB_BOOT_FLAGS_SPEC>;
#[doc = "USB boot specific feature flags (RBIT-3)"]
pub mod usb_boot_flags;
#[doc = "USB_BOOT_FLAGS_R1 (r) register accessor: Redundant copy of USB_BOOT_FLAGS  

You can [`read`](crate::Reg::read) this register and get [`usb_boot_flags_r1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_boot_flags_r1`]
module"]
pub type USB_BOOT_FLAGS_R1 = crate::Reg<usb_boot_flags_r1::USB_BOOT_FLAGS_R1_SPEC>;
#[doc = "Redundant copy of USB_BOOT_FLAGS"]
pub mod usb_boot_flags_r1;
#[doc = "USB_BOOT_FLAGS_R2 (r) register accessor: Redundant copy of USB_BOOT_FLAGS  

You can [`read`](crate::Reg::read) this register and get [`usb_boot_flags_r2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_boot_flags_r2`]
module"]
pub type USB_BOOT_FLAGS_R2 = crate::Reg<usb_boot_flags_r2::USB_BOOT_FLAGS_R2_SPEC>;
#[doc = "Redundant copy of USB_BOOT_FLAGS"]
pub mod usb_boot_flags_r2;
#[doc = "USB_WHITE_LABEL_ADDR (r) register accessor: Row index of the USB_WHITE_LABEL structure within OTP (ECC)  

 The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC).  

 The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value.  

 The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte.  

 In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom.  

 Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero.  

 Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used.  

 The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):  

You can [`read`](crate::Reg::read) this register and get [`usb_white_label_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@usb_white_label_addr`]
module"]
pub type USB_WHITE_LABEL_ADDR = crate::Reg<usb_white_label_addr::USB_WHITE_LABEL_ADDR_SPEC>;
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC)  

 The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC).  

 The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value.  

 The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte.  

 In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom.  

 Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero.  

 Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used.  

 The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
pub mod usb_white_label_addr;
#[doc = "OTPBOOT_SRC (r) register accessor: OTP start row for the OTP boot image. (ECC)  

 If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.  

 This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_src`]
module"]
pub type OTPBOOT_SRC = crate::Reg<otpboot_src::OTPBOOT_SRC_SPEC>;
#[doc = "OTP start row for the OTP boot image. (ECC)  

 If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.  

 This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
pub mod otpboot_src;
#[doc = "OTPBOOT_LEN (r) register accessor: Length in rows of the OTP boot image. (ECC)  

 OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).  

You can [`read`](crate::Reg::read) this register and get [`otpboot_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_len`]
module"]
pub type OTPBOOT_LEN = crate::Reg<otpboot_len::OTPBOOT_LEN_SPEC>;
#[doc = "Length in rows of the OTP boot image. (ECC)  

 OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
pub mod otpboot_len;
#[doc = "OTPBOOT_DST0 (r) register accessor: Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_dst0`]
module"]
pub type OTPBOOT_DST0 = crate::Reg<otpboot_dst0::OTPBOOT_DST0_SPEC>;
#[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
pub mod otpboot_dst0;
#[doc = "OTPBOOT_DST1 (r) register accessor: Bits 31:16 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@otpboot_dst1`]
module"]
pub type OTPBOOT_DST1 = crate::Reg<otpboot_dst1::OTPBOOT_DST1_SPEC>;
#[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
pub mod otpboot_dst1;
#[doc = "BOOTKEY0_0 (r) register accessor: Bits 15:0 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_0`]
module"]
pub type BOOTKEY0_0 = crate::Reg<bootkey0_0::BOOTKEY0_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_0;
#[doc = "BOOTKEY0_1 (r) register accessor: Bits 31:16 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_1`]
module"]
pub type BOOTKEY0_1 = crate::Reg<bootkey0_1::BOOTKEY0_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_1;
#[doc = "BOOTKEY0_2 (r) register accessor: Bits 47:32 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_2`]
module"]
pub type BOOTKEY0_2 = crate::Reg<bootkey0_2::BOOTKEY0_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_2;
#[doc = "BOOTKEY0_3 (r) register accessor: Bits 63:48 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_3`]
module"]
pub type BOOTKEY0_3 = crate::Reg<bootkey0_3::BOOTKEY0_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_3;
#[doc = "BOOTKEY0_4 (r) register accessor: Bits 79:64 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_4`]
module"]
pub type BOOTKEY0_4 = crate::Reg<bootkey0_4::BOOTKEY0_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_4;
#[doc = "BOOTKEY0_5 (r) register accessor: Bits 95:80 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_5`]
module"]
pub type BOOTKEY0_5 = crate::Reg<bootkey0_5::BOOTKEY0_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_5;
#[doc = "BOOTKEY0_6 (r) register accessor: Bits 111:96 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_6`]
module"]
pub type BOOTKEY0_6 = crate::Reg<bootkey0_6::BOOTKEY0_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_6;
#[doc = "BOOTKEY0_7 (r) register accessor: Bits 127:112 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_7`]
module"]
pub type BOOTKEY0_7 = crate::Reg<bootkey0_7::BOOTKEY0_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_7;
#[doc = "BOOTKEY0_8 (r) register accessor: Bits 143:128 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_8`]
module"]
pub type BOOTKEY0_8 = crate::Reg<bootkey0_8::BOOTKEY0_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_8;
#[doc = "BOOTKEY0_9 (r) register accessor: Bits 159:144 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_9`]
module"]
pub type BOOTKEY0_9 = crate::Reg<bootkey0_9::BOOTKEY0_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_9;
#[doc = "BOOTKEY0_10 (r) register accessor: Bits 175:160 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_10`]
module"]
pub type BOOTKEY0_10 = crate::Reg<bootkey0_10::BOOTKEY0_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_10;
#[doc = "BOOTKEY0_11 (r) register accessor: Bits 191:176 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_11`]
module"]
pub type BOOTKEY0_11 = crate::Reg<bootkey0_11::BOOTKEY0_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_11;
#[doc = "BOOTKEY0_12 (r) register accessor: Bits 207:192 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_12`]
module"]
pub type BOOTKEY0_12 = crate::Reg<bootkey0_12::BOOTKEY0_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_12;
#[doc = "BOOTKEY0_13 (r) register accessor: Bits 223:208 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_13`]
module"]
pub type BOOTKEY0_13 = crate::Reg<bootkey0_13::BOOTKEY0_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_13;
#[doc = "BOOTKEY0_14 (r) register accessor: Bits 239:224 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_14`]
module"]
pub type BOOTKEY0_14 = crate::Reg<bootkey0_14::BOOTKEY0_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_14;
#[doc = "BOOTKEY0_15 (r) register accessor: Bits 255:240 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey0_15`]
module"]
pub type BOOTKEY0_15 = crate::Reg<bootkey0_15::BOOTKEY0_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
pub mod bootkey0_15;
#[doc = "BOOTKEY1_0 (r) register accessor: Bits 15:0 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_0`]
module"]
pub type BOOTKEY1_0 = crate::Reg<bootkey1_0::BOOTKEY1_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_0;
#[doc = "BOOTKEY1_1 (r) register accessor: Bits 31:16 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_1`]
module"]
pub type BOOTKEY1_1 = crate::Reg<bootkey1_1::BOOTKEY1_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_1;
#[doc = "BOOTKEY1_2 (r) register accessor: Bits 47:32 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_2`]
module"]
pub type BOOTKEY1_2 = crate::Reg<bootkey1_2::BOOTKEY1_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_2;
#[doc = "BOOTKEY1_3 (r) register accessor: Bits 63:48 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_3`]
module"]
pub type BOOTKEY1_3 = crate::Reg<bootkey1_3::BOOTKEY1_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_3;
#[doc = "BOOTKEY1_4 (r) register accessor: Bits 79:64 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_4`]
module"]
pub type BOOTKEY1_4 = crate::Reg<bootkey1_4::BOOTKEY1_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_4;
#[doc = "BOOTKEY1_5 (r) register accessor: Bits 95:80 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_5`]
module"]
pub type BOOTKEY1_5 = crate::Reg<bootkey1_5::BOOTKEY1_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_5;
#[doc = "BOOTKEY1_6 (r) register accessor: Bits 111:96 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_6`]
module"]
pub type BOOTKEY1_6 = crate::Reg<bootkey1_6::BOOTKEY1_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_6;
#[doc = "BOOTKEY1_7 (r) register accessor: Bits 127:112 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_7`]
module"]
pub type BOOTKEY1_7 = crate::Reg<bootkey1_7::BOOTKEY1_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_7;
#[doc = "BOOTKEY1_8 (r) register accessor: Bits 143:128 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_8`]
module"]
pub type BOOTKEY1_8 = crate::Reg<bootkey1_8::BOOTKEY1_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_8;
#[doc = "BOOTKEY1_9 (r) register accessor: Bits 159:144 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_9`]
module"]
pub type BOOTKEY1_9 = crate::Reg<bootkey1_9::BOOTKEY1_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_9;
#[doc = "BOOTKEY1_10 (r) register accessor: Bits 175:160 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_10`]
module"]
pub type BOOTKEY1_10 = crate::Reg<bootkey1_10::BOOTKEY1_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_10;
#[doc = "BOOTKEY1_11 (r) register accessor: Bits 191:176 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_11`]
module"]
pub type BOOTKEY1_11 = crate::Reg<bootkey1_11::BOOTKEY1_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_11;
#[doc = "BOOTKEY1_12 (r) register accessor: Bits 207:192 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_12`]
module"]
pub type BOOTKEY1_12 = crate::Reg<bootkey1_12::BOOTKEY1_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_12;
#[doc = "BOOTKEY1_13 (r) register accessor: Bits 223:208 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_13`]
module"]
pub type BOOTKEY1_13 = crate::Reg<bootkey1_13::BOOTKEY1_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_13;
#[doc = "BOOTKEY1_14 (r) register accessor: Bits 239:224 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_14`]
module"]
pub type BOOTKEY1_14 = crate::Reg<bootkey1_14::BOOTKEY1_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_14;
#[doc = "BOOTKEY1_15 (r) register accessor: Bits 255:240 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey1_15`]
module"]
pub type BOOTKEY1_15 = crate::Reg<bootkey1_15::BOOTKEY1_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
pub mod bootkey1_15;
#[doc = "BOOTKEY2_0 (r) register accessor: Bits 15:0 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_0`]
module"]
pub type BOOTKEY2_0 = crate::Reg<bootkey2_0::BOOTKEY2_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_0;
#[doc = "BOOTKEY2_1 (r) register accessor: Bits 31:16 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_1`]
module"]
pub type BOOTKEY2_1 = crate::Reg<bootkey2_1::BOOTKEY2_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_1;
#[doc = "BOOTKEY2_2 (r) register accessor: Bits 47:32 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_2`]
module"]
pub type BOOTKEY2_2 = crate::Reg<bootkey2_2::BOOTKEY2_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_2;
#[doc = "BOOTKEY2_3 (r) register accessor: Bits 63:48 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_3`]
module"]
pub type BOOTKEY2_3 = crate::Reg<bootkey2_3::BOOTKEY2_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_3;
#[doc = "BOOTKEY2_4 (r) register accessor: Bits 79:64 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_4`]
module"]
pub type BOOTKEY2_4 = crate::Reg<bootkey2_4::BOOTKEY2_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_4;
#[doc = "BOOTKEY2_5 (r) register accessor: Bits 95:80 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_5`]
module"]
pub type BOOTKEY2_5 = crate::Reg<bootkey2_5::BOOTKEY2_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_5;
#[doc = "BOOTKEY2_6 (r) register accessor: Bits 111:96 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_6`]
module"]
pub type BOOTKEY2_6 = crate::Reg<bootkey2_6::BOOTKEY2_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_6;
#[doc = "BOOTKEY2_7 (r) register accessor: Bits 127:112 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_7`]
module"]
pub type BOOTKEY2_7 = crate::Reg<bootkey2_7::BOOTKEY2_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_7;
#[doc = "BOOTKEY2_8 (r) register accessor: Bits 143:128 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_8`]
module"]
pub type BOOTKEY2_8 = crate::Reg<bootkey2_8::BOOTKEY2_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_8;
#[doc = "BOOTKEY2_9 (r) register accessor: Bits 159:144 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_9`]
module"]
pub type BOOTKEY2_9 = crate::Reg<bootkey2_9::BOOTKEY2_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_9;
#[doc = "BOOTKEY2_10 (r) register accessor: Bits 175:160 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_10`]
module"]
pub type BOOTKEY2_10 = crate::Reg<bootkey2_10::BOOTKEY2_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_10;
#[doc = "BOOTKEY2_11 (r) register accessor: Bits 191:176 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_11`]
module"]
pub type BOOTKEY2_11 = crate::Reg<bootkey2_11::BOOTKEY2_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_11;
#[doc = "BOOTKEY2_12 (r) register accessor: Bits 207:192 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_12`]
module"]
pub type BOOTKEY2_12 = crate::Reg<bootkey2_12::BOOTKEY2_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_12;
#[doc = "BOOTKEY2_13 (r) register accessor: Bits 223:208 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_13`]
module"]
pub type BOOTKEY2_13 = crate::Reg<bootkey2_13::BOOTKEY2_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_13;
#[doc = "BOOTKEY2_14 (r) register accessor: Bits 239:224 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_14`]
module"]
pub type BOOTKEY2_14 = crate::Reg<bootkey2_14::BOOTKEY2_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_14;
#[doc = "BOOTKEY2_15 (r) register accessor: Bits 255:240 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey2_15`]
module"]
pub type BOOTKEY2_15 = crate::Reg<bootkey2_15::BOOTKEY2_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
pub mod bootkey2_15;
#[doc = "BOOTKEY3_0 (r) register accessor: Bits 15:0 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_0`]
module"]
pub type BOOTKEY3_0 = crate::Reg<bootkey3_0::BOOTKEY3_0_SPEC>;
#[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_0;
#[doc = "BOOTKEY3_1 (r) register accessor: Bits 31:16 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_1`]
module"]
pub type BOOTKEY3_1 = crate::Reg<bootkey3_1::BOOTKEY3_1_SPEC>;
#[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_1;
#[doc = "BOOTKEY3_2 (r) register accessor: Bits 47:32 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_2`]
module"]
pub type BOOTKEY3_2 = crate::Reg<bootkey3_2::BOOTKEY3_2_SPEC>;
#[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_2;
#[doc = "BOOTKEY3_3 (r) register accessor: Bits 63:48 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_3`]
module"]
pub type BOOTKEY3_3 = crate::Reg<bootkey3_3::BOOTKEY3_3_SPEC>;
#[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_3;
#[doc = "BOOTKEY3_4 (r) register accessor: Bits 79:64 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_4`]
module"]
pub type BOOTKEY3_4 = crate::Reg<bootkey3_4::BOOTKEY3_4_SPEC>;
#[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_4;
#[doc = "BOOTKEY3_5 (r) register accessor: Bits 95:80 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_5`]
module"]
pub type BOOTKEY3_5 = crate::Reg<bootkey3_5::BOOTKEY3_5_SPEC>;
#[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_5;
#[doc = "BOOTKEY3_6 (r) register accessor: Bits 111:96 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_6`]
module"]
pub type BOOTKEY3_6 = crate::Reg<bootkey3_6::BOOTKEY3_6_SPEC>;
#[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_6;
#[doc = "BOOTKEY3_7 (r) register accessor: Bits 127:112 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_7`]
module"]
pub type BOOTKEY3_7 = crate::Reg<bootkey3_7::BOOTKEY3_7_SPEC>;
#[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_7;
#[doc = "BOOTKEY3_8 (r) register accessor: Bits 143:128 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_8`]
module"]
pub type BOOTKEY3_8 = crate::Reg<bootkey3_8::BOOTKEY3_8_SPEC>;
#[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_8;
#[doc = "BOOTKEY3_9 (r) register accessor: Bits 159:144 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_9`]
module"]
pub type BOOTKEY3_9 = crate::Reg<bootkey3_9::BOOTKEY3_9_SPEC>;
#[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_9;
#[doc = "BOOTKEY3_10 (r) register accessor: Bits 175:160 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_10`]
module"]
pub type BOOTKEY3_10 = crate::Reg<bootkey3_10::BOOTKEY3_10_SPEC>;
#[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_10;
#[doc = "BOOTKEY3_11 (r) register accessor: Bits 191:176 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_11`]
module"]
pub type BOOTKEY3_11 = crate::Reg<bootkey3_11::BOOTKEY3_11_SPEC>;
#[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_11;
#[doc = "BOOTKEY3_12 (r) register accessor: Bits 207:192 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_12`]
module"]
pub type BOOTKEY3_12 = crate::Reg<bootkey3_12::BOOTKEY3_12_SPEC>;
#[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_12;
#[doc = "BOOTKEY3_13 (r) register accessor: Bits 223:208 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_13`]
module"]
pub type BOOTKEY3_13 = crate::Reg<bootkey3_13::BOOTKEY3_13_SPEC>;
#[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_13;
#[doc = "BOOTKEY3_14 (r) register accessor: Bits 239:224 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_14`]
module"]
pub type BOOTKEY3_14 = crate::Reg<bootkey3_14::BOOTKEY3_14_SPEC>;
#[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_14;
#[doc = "BOOTKEY3_15 (r) register accessor: Bits 255:240 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootkey3_15`]
module"]
pub type BOOTKEY3_15 = crate::Reg<bootkey3_15::BOOTKEY3_15_SPEC>;
#[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
pub mod bootkey3_15;
#[doc = "KEY1_0 (r) register accessor: Bits 15:0 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_0`]
module"]
pub type KEY1_0 = crate::Reg<key1_0::KEY1_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
pub mod key1_0;
#[doc = "KEY1_1 (r) register accessor: Bits 31:16 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_1`]
module"]
pub type KEY1_1 = crate::Reg<key1_1::KEY1_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
pub mod key1_1;
#[doc = "KEY1_2 (r) register accessor: Bits 47:32 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_2`]
module"]
pub type KEY1_2 = crate::Reg<key1_2::KEY1_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
pub mod key1_2;
#[doc = "KEY1_3 (r) register accessor: Bits 63:48 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_3`]
module"]
pub type KEY1_3 = crate::Reg<key1_3::KEY1_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
pub mod key1_3;
#[doc = "KEY1_4 (r) register accessor: Bits 79:64 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_4`]
module"]
pub type KEY1_4 = crate::Reg<key1_4::KEY1_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
pub mod key1_4;
#[doc = "KEY1_5 (r) register accessor: Bits 95:80 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_5`]
module"]
pub type KEY1_5 = crate::Reg<key1_5::KEY1_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
pub mod key1_5;
#[doc = "KEY1_6 (r) register accessor: Bits 111:96 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_6`]
module"]
pub type KEY1_6 = crate::Reg<key1_6::KEY1_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
pub mod key1_6;
#[doc = "KEY1_7 (r) register accessor: Bits 127:112 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_7`]
module"]
pub type KEY1_7 = crate::Reg<key1_7::KEY1_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
pub mod key1_7;
#[doc = "KEY2_0 (r) register accessor: Bits 15:0 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_0`]
module"]
pub type KEY2_0 = crate::Reg<key2_0::KEY2_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
pub mod key2_0;
#[doc = "KEY2_1 (r) register accessor: Bits 31:16 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_1`]
module"]
pub type KEY2_1 = crate::Reg<key2_1::KEY2_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
pub mod key2_1;
#[doc = "KEY2_2 (r) register accessor: Bits 47:32 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_2`]
module"]
pub type KEY2_2 = crate::Reg<key2_2::KEY2_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
pub mod key2_2;
#[doc = "KEY2_3 (r) register accessor: Bits 63:48 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_3`]
module"]
pub type KEY2_3 = crate::Reg<key2_3::KEY2_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
pub mod key2_3;
#[doc = "KEY2_4 (r) register accessor: Bits 79:64 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_4`]
module"]
pub type KEY2_4 = crate::Reg<key2_4::KEY2_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
pub mod key2_4;
#[doc = "KEY2_5 (r) register accessor: Bits 95:80 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_5`]
module"]
pub type KEY2_5 = crate::Reg<key2_5::KEY2_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
pub mod key2_5;
#[doc = "KEY2_6 (r) register accessor: Bits 111:96 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_6`]
module"]
pub type KEY2_6 = crate::Reg<key2_6::KEY2_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
pub mod key2_6;
#[doc = "KEY2_7 (r) register accessor: Bits 127:112 of OTP access key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key2_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_7`]
module"]
pub type KEY2_7 = crate::Reg<key2_7::KEY2_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
pub mod key2_7;
#[doc = "KEY3_0 (r) register accessor: Bits 15:0 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_0`]
module"]
pub type KEY3_0 = crate::Reg<key3_0::KEY3_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
pub mod key3_0;
#[doc = "KEY3_1 (r) register accessor: Bits 31:16 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_1`]
module"]
pub type KEY3_1 = crate::Reg<key3_1::KEY3_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
pub mod key3_1;
#[doc = "KEY3_2 (r) register accessor: Bits 47:32 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_2`]
module"]
pub type KEY3_2 = crate::Reg<key3_2::KEY3_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
pub mod key3_2;
#[doc = "KEY3_3 (r) register accessor: Bits 63:48 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_3`]
module"]
pub type KEY3_3 = crate::Reg<key3_3::KEY3_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
pub mod key3_3;
#[doc = "KEY3_4 (r) register accessor: Bits 79:64 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_4`]
module"]
pub type KEY3_4 = crate::Reg<key3_4::KEY3_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
pub mod key3_4;
#[doc = "KEY3_5 (r) register accessor: Bits 95:80 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_5`]
module"]
pub type KEY3_5 = crate::Reg<key3_5::KEY3_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
pub mod key3_5;
#[doc = "KEY3_6 (r) register accessor: Bits 111:96 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_6`]
module"]
pub type KEY3_6 = crate::Reg<key3_6::KEY3_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
pub mod key3_6;
#[doc = "KEY3_7 (r) register accessor: Bits 127:112 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_7`]
module"]
pub type KEY3_7 = crate::Reg<key3_7::KEY3_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
pub mod key3_7;
#[doc = "KEY4_0 (r) register accessor: Bits 15:0 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_0`]
module"]
pub type KEY4_0 = crate::Reg<key4_0::KEY4_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
pub mod key4_0;
#[doc = "KEY4_1 (r) register accessor: Bits 31:16 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_1`]
module"]
pub type KEY4_1 = crate::Reg<key4_1::KEY4_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
pub mod key4_1;
#[doc = "KEY4_2 (r) register accessor: Bits 47:32 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_2`]
module"]
pub type KEY4_2 = crate::Reg<key4_2::KEY4_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
pub mod key4_2;
#[doc = "KEY4_3 (r) register accessor: Bits 63:48 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_3`]
module"]
pub type KEY4_3 = crate::Reg<key4_3::KEY4_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
pub mod key4_3;
#[doc = "KEY4_4 (r) register accessor: Bits 79:64 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_4`]
module"]
pub type KEY4_4 = crate::Reg<key4_4::KEY4_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
pub mod key4_4;
#[doc = "KEY4_5 (r) register accessor: Bits 95:80 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_5`]
module"]
pub type KEY4_5 = crate::Reg<key4_5::KEY4_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
pub mod key4_5;
#[doc = "KEY4_6 (r) register accessor: Bits 111:96 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_6`]
module"]
pub type KEY4_6 = crate::Reg<key4_6::KEY4_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
pub mod key4_6;
#[doc = "KEY4_7 (r) register accessor: Bits 127:112 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_7`]
module"]
pub type KEY4_7 = crate::Reg<key4_7::KEY4_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
pub mod key4_7;
#[doc = "KEY5_0 (r) register accessor: Bits 15:0 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_0`]
module"]
pub type KEY5_0 = crate::Reg<key5_0::KEY5_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
pub mod key5_0;
#[doc = "KEY5_1 (r) register accessor: Bits 31:16 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_1`]
module"]
pub type KEY5_1 = crate::Reg<key5_1::KEY5_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
pub mod key5_1;
#[doc = "KEY5_2 (r) register accessor: Bits 47:32 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_2`]
module"]
pub type KEY5_2 = crate::Reg<key5_2::KEY5_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
pub mod key5_2;
#[doc = "KEY5_3 (r) register accessor: Bits 63:48 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_3`]
module"]
pub type KEY5_3 = crate::Reg<key5_3::KEY5_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
pub mod key5_3;
#[doc = "KEY5_4 (r) register accessor: Bits 79:64 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_4`]
module"]
pub type KEY5_4 = crate::Reg<key5_4::KEY5_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
pub mod key5_4;
#[doc = "KEY5_5 (r) register accessor: Bits 95:80 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_5`]
module"]
pub type KEY5_5 = crate::Reg<key5_5::KEY5_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
pub mod key5_5;
#[doc = "KEY5_6 (r) register accessor: Bits 111:96 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_6`]
module"]
pub type KEY5_6 = crate::Reg<key5_6::KEY5_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
pub mod key5_6;
#[doc = "KEY5_7 (r) register accessor: Bits 127:112 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_7`]
module"]
pub type KEY5_7 = crate::Reg<key5_7::KEY5_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
pub mod key5_7;
#[doc = "KEY6_0 (r) register accessor: Bits 15:0 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_0`]
module"]
pub type KEY6_0 = crate::Reg<key6_0::KEY6_0_SPEC>;
#[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
pub mod key6_0;
#[doc = "KEY6_1 (r) register accessor: Bits 31:16 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_1`]
module"]
pub type KEY6_1 = crate::Reg<key6_1::KEY6_1_SPEC>;
#[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
pub mod key6_1;
#[doc = "KEY6_2 (r) register accessor: Bits 47:32 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_2`]
module"]
pub type KEY6_2 = crate::Reg<key6_2::KEY6_2_SPEC>;
#[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
pub mod key6_2;
#[doc = "KEY6_3 (r) register accessor: Bits 63:48 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_3`]
module"]
pub type KEY6_3 = crate::Reg<key6_3::KEY6_3_SPEC>;
#[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
pub mod key6_3;
#[doc = "KEY6_4 (r) register accessor: Bits 79:64 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_4`]
module"]
pub type KEY6_4 = crate::Reg<key6_4::KEY6_4_SPEC>;
#[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
pub mod key6_4;
#[doc = "KEY6_5 (r) register accessor: Bits 95:80 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_5`]
module"]
pub type KEY6_5 = crate::Reg<key6_5::KEY6_5_SPEC>;
#[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
pub mod key6_5;
#[doc = "KEY6_6 (r) register accessor: Bits 111:96 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_6`]
module"]
pub type KEY6_6 = crate::Reg<key6_6::KEY6_6_SPEC>;
#[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
pub mod key6_6;
#[doc = "KEY6_7 (r) register accessor: Bits 127:112 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_7`]
module"]
pub type KEY6_7 = crate::Reg<key6_7::KEY6_7_SPEC>;
#[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
pub mod key6_7;
#[doc = "KEY1_VALID (r) register accessor: Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key1_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key1_valid`]
module"]
pub type KEY1_VALID = crate::Reg<key1_valid::KEY1_VALID_SPEC>;
#[doc = "Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key1_valid;
#[doc = "KEY2_VALID (r) register accessor: Valid flag for key 2. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key2_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key2_valid`]
module"]
pub type KEY2_VALID = crate::Reg<key2_valid::KEY2_VALID_SPEC>;
#[doc = "Valid flag for key 2. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key2_valid;
#[doc = "KEY3_VALID (r) register accessor: Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key3_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key3_valid`]
module"]
pub type KEY3_VALID = crate::Reg<key3_valid::KEY3_VALID_SPEC>;
#[doc = "Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key3_valid;
#[doc = "KEY4_VALID (r) register accessor: Valid flag for key 4. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key4_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key4_valid`]
module"]
pub type KEY4_VALID = crate::Reg<key4_valid::KEY4_VALID_SPEC>;
#[doc = "Valid flag for key 4. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key4_valid;
#[doc = "KEY5_VALID (r) register accessor: Valid flag for key 5. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key5_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key5_valid`]
module"]
pub type KEY5_VALID = crate::Reg<key5_valid::KEY5_VALID_SPEC>;
#[doc = "Valid flag for key 5. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key5_valid;
#[doc = "KEY6_VALID (r) register accessor: Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages.  

You can [`read`](crate::Reg::read) this register and get [`key6_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@key6_valid`]
module"]
pub type KEY6_VALID = crate::Reg<key6_valid::KEY6_VALID_SPEC>;
#[doc = "Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
pub mod key6_valid;
#[doc = "PAGE0_LOCK0 (r) register accessor: Lock configuration LSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page0_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page0_lock0`]
module"]
pub type PAGE0_LOCK0 = crate::Reg<page0_lock0::PAGE0_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page0_lock0;
#[doc = "PAGE0_LOCK1 (r) register accessor: Lock configuration MSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page0_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page0_lock1`]
module"]
pub type PAGE0_LOCK1 = crate::Reg<page0_lock1::PAGE0_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page0_lock1;
#[doc = "PAGE1_LOCK0 (r) register accessor: Lock configuration LSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page1_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page1_lock0`]
module"]
pub type PAGE1_LOCK0 = crate::Reg<page1_lock0::PAGE1_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page1_lock0;
#[doc = "PAGE1_LOCK1 (r) register accessor: Lock configuration MSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page1_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page1_lock1`]
module"]
pub type PAGE1_LOCK1 = crate::Reg<page1_lock1::PAGE1_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page1_lock1;
#[doc = "PAGE2_LOCK0 (r) register accessor: Lock configuration LSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page2_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page2_lock0`]
module"]
pub type PAGE2_LOCK0 = crate::Reg<page2_lock0::PAGE2_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page2_lock0;
#[doc = "PAGE2_LOCK1 (r) register accessor: Lock configuration MSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page2_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page2_lock1`]
module"]
pub type PAGE2_LOCK1 = crate::Reg<page2_lock1::PAGE2_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page2_lock1;
#[doc = "PAGE3_LOCK0 (r) register accessor: Lock configuration LSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page3_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page3_lock0`]
module"]
pub type PAGE3_LOCK0 = crate::Reg<page3_lock0::PAGE3_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page3_lock0;
#[doc = "PAGE3_LOCK1 (r) register accessor: Lock configuration MSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page3_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page3_lock1`]
module"]
pub type PAGE3_LOCK1 = crate::Reg<page3_lock1::PAGE3_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page3_lock1;
#[doc = "PAGE4_LOCK0 (r) register accessor: Lock configuration LSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page4_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page4_lock0`]
module"]
pub type PAGE4_LOCK0 = crate::Reg<page4_lock0::PAGE4_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page4_lock0;
#[doc = "PAGE4_LOCK1 (r) register accessor: Lock configuration MSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page4_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page4_lock1`]
module"]
pub type PAGE4_LOCK1 = crate::Reg<page4_lock1::PAGE4_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page4_lock1;
#[doc = "PAGE5_LOCK0 (r) register accessor: Lock configuration LSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page5_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page5_lock0`]
module"]
pub type PAGE5_LOCK0 = crate::Reg<page5_lock0::PAGE5_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page5_lock0;
#[doc = "PAGE5_LOCK1 (r) register accessor: Lock configuration MSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page5_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page5_lock1`]
module"]
pub type PAGE5_LOCK1 = crate::Reg<page5_lock1::PAGE5_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page5_lock1;
#[doc = "PAGE6_LOCK0 (r) register accessor: Lock configuration LSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page6_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page6_lock0`]
module"]
pub type PAGE6_LOCK0 = crate::Reg<page6_lock0::PAGE6_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page6_lock0;
#[doc = "PAGE6_LOCK1 (r) register accessor: Lock configuration MSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page6_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page6_lock1`]
module"]
pub type PAGE6_LOCK1 = crate::Reg<page6_lock1::PAGE6_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page6_lock1;
#[doc = "PAGE7_LOCK0 (r) register accessor: Lock configuration LSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page7_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page7_lock0`]
module"]
pub type PAGE7_LOCK0 = crate::Reg<page7_lock0::PAGE7_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page7_lock0;
#[doc = "PAGE7_LOCK1 (r) register accessor: Lock configuration MSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page7_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page7_lock1`]
module"]
pub type PAGE7_LOCK1 = crate::Reg<page7_lock1::PAGE7_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page7_lock1;
#[doc = "PAGE8_LOCK0 (r) register accessor: Lock configuration LSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page8_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page8_lock0`]
module"]
pub type PAGE8_LOCK0 = crate::Reg<page8_lock0::PAGE8_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page8_lock0;
#[doc = "PAGE8_LOCK1 (r) register accessor: Lock configuration MSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page8_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page8_lock1`]
module"]
pub type PAGE8_LOCK1 = crate::Reg<page8_lock1::PAGE8_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page8_lock1;
#[doc = "PAGE9_LOCK0 (r) register accessor: Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page9_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page9_lock0`]
module"]
pub type PAGE9_LOCK0 = crate::Reg<page9_lock0::PAGE9_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page9_lock0;
#[doc = "PAGE9_LOCK1 (r) register accessor: Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page9_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page9_lock1`]
module"]
pub type PAGE9_LOCK1 = crate::Reg<page9_lock1::PAGE9_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page9_lock1;
#[doc = "PAGE10_LOCK0 (r) register accessor: Lock configuration LSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page10_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page10_lock0`]
module"]
pub type PAGE10_LOCK0 = crate::Reg<page10_lock0::PAGE10_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page10_lock0;
#[doc = "PAGE10_LOCK1 (r) register accessor: Lock configuration MSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page10_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page10_lock1`]
module"]
pub type PAGE10_LOCK1 = crate::Reg<page10_lock1::PAGE10_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page10_lock1;
#[doc = "PAGE11_LOCK0 (r) register accessor: Lock configuration LSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page11_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page11_lock0`]
module"]
pub type PAGE11_LOCK0 = crate::Reg<page11_lock0::PAGE11_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page11_lock0;
#[doc = "PAGE11_LOCK1 (r) register accessor: Lock configuration MSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page11_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page11_lock1`]
module"]
pub type PAGE11_LOCK1 = crate::Reg<page11_lock1::PAGE11_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page11_lock1;
#[doc = "PAGE12_LOCK0 (r) register accessor: Lock configuration LSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page12_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page12_lock0`]
module"]
pub type PAGE12_LOCK0 = crate::Reg<page12_lock0::PAGE12_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page12_lock0;
#[doc = "PAGE12_LOCK1 (r) register accessor: Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page12_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page12_lock1`]
module"]
pub type PAGE12_LOCK1 = crate::Reg<page12_lock1::PAGE12_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page12_lock1;
#[doc = "PAGE13_LOCK0 (r) register accessor: Lock configuration LSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page13_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page13_lock0`]
module"]
pub type PAGE13_LOCK0 = crate::Reg<page13_lock0::PAGE13_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page13_lock0;
#[doc = "PAGE13_LOCK1 (r) register accessor: Lock configuration MSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page13_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page13_lock1`]
module"]
pub type PAGE13_LOCK1 = crate::Reg<page13_lock1::PAGE13_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page13_lock1;
#[doc = "PAGE14_LOCK0 (r) register accessor: Lock configuration LSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page14_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page14_lock0`]
module"]
pub type PAGE14_LOCK0 = crate::Reg<page14_lock0::PAGE14_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page14_lock0;
#[doc = "PAGE14_LOCK1 (r) register accessor: Lock configuration MSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page14_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page14_lock1`]
module"]
pub type PAGE14_LOCK1 = crate::Reg<page14_lock1::PAGE14_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page14_lock1;
#[doc = "PAGE15_LOCK0 (r) register accessor: Lock configuration LSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page15_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page15_lock0`]
module"]
pub type PAGE15_LOCK0 = crate::Reg<page15_lock0::PAGE15_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page15_lock0;
#[doc = "PAGE15_LOCK1 (r) register accessor: Lock configuration MSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page15_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page15_lock1`]
module"]
pub type PAGE15_LOCK1 = crate::Reg<page15_lock1::PAGE15_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page15_lock1;
#[doc = "PAGE16_LOCK0 (r) register accessor: Lock configuration LSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page16_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page16_lock0`]
module"]
pub type PAGE16_LOCK0 = crate::Reg<page16_lock0::PAGE16_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page16_lock0;
#[doc = "PAGE16_LOCK1 (r) register accessor: Lock configuration MSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page16_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page16_lock1`]
module"]
pub type PAGE16_LOCK1 = crate::Reg<page16_lock1::PAGE16_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page16_lock1;
#[doc = "PAGE17_LOCK0 (r) register accessor: Lock configuration LSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page17_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page17_lock0`]
module"]
pub type PAGE17_LOCK0 = crate::Reg<page17_lock0::PAGE17_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page17_lock0;
#[doc = "PAGE17_LOCK1 (r) register accessor: Lock configuration MSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page17_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page17_lock1`]
module"]
pub type PAGE17_LOCK1 = crate::Reg<page17_lock1::PAGE17_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page17_lock1;
#[doc = "PAGE18_LOCK0 (r) register accessor: Lock configuration LSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page18_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page18_lock0`]
module"]
pub type PAGE18_LOCK0 = crate::Reg<page18_lock0::PAGE18_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page18_lock0;
#[doc = "PAGE18_LOCK1 (r) register accessor: Lock configuration MSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page18_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page18_lock1`]
module"]
pub type PAGE18_LOCK1 = crate::Reg<page18_lock1::PAGE18_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page18_lock1;
#[doc = "PAGE19_LOCK0 (r) register accessor: Lock configuration LSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page19_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page19_lock0`]
module"]
pub type PAGE19_LOCK0 = crate::Reg<page19_lock0::PAGE19_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page19_lock0;
#[doc = "PAGE19_LOCK1 (r) register accessor: Lock configuration MSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page19_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page19_lock1`]
module"]
pub type PAGE19_LOCK1 = crate::Reg<page19_lock1::PAGE19_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page19_lock1;
#[doc = "PAGE20_LOCK0 (r) register accessor: Lock configuration LSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page20_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page20_lock0`]
module"]
pub type PAGE20_LOCK0 = crate::Reg<page20_lock0::PAGE20_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page20_lock0;
#[doc = "PAGE20_LOCK1 (r) register accessor: Lock configuration MSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page20_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page20_lock1`]
module"]
pub type PAGE20_LOCK1 = crate::Reg<page20_lock1::PAGE20_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page20_lock1;
#[doc = "PAGE21_LOCK0 (r) register accessor: Lock configuration LSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page21_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page21_lock0`]
module"]
pub type PAGE21_LOCK0 = crate::Reg<page21_lock0::PAGE21_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page21_lock0;
#[doc = "PAGE21_LOCK1 (r) register accessor: Lock configuration MSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page21_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page21_lock1`]
module"]
pub type PAGE21_LOCK1 = crate::Reg<page21_lock1::PAGE21_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page21_lock1;
#[doc = "PAGE22_LOCK0 (r) register accessor: Lock configuration LSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page22_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page22_lock0`]
module"]
pub type PAGE22_LOCK0 = crate::Reg<page22_lock0::PAGE22_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page22_lock0;
#[doc = "PAGE22_LOCK1 (r) register accessor: Lock configuration MSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page22_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page22_lock1`]
module"]
pub type PAGE22_LOCK1 = crate::Reg<page22_lock1::PAGE22_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page22_lock1;
#[doc = "PAGE23_LOCK0 (r) register accessor: Lock configuration LSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page23_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page23_lock0`]
module"]
pub type PAGE23_LOCK0 = crate::Reg<page23_lock0::PAGE23_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page23_lock0;
#[doc = "PAGE23_LOCK1 (r) register accessor: Lock configuration MSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page23_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page23_lock1`]
module"]
pub type PAGE23_LOCK1 = crate::Reg<page23_lock1::PAGE23_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page23_lock1;
#[doc = "PAGE24_LOCK0 (r) register accessor: Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page24_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page24_lock0`]
module"]
pub type PAGE24_LOCK0 = crate::Reg<page24_lock0::PAGE24_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page24_lock0;
#[doc = "PAGE24_LOCK1 (r) register accessor: Lock configuration MSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page24_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page24_lock1`]
module"]
pub type PAGE24_LOCK1 = crate::Reg<page24_lock1::PAGE24_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page24_lock1;
#[doc = "PAGE25_LOCK0 (r) register accessor: Lock configuration LSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page25_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page25_lock0`]
module"]
pub type PAGE25_LOCK0 = crate::Reg<page25_lock0::PAGE25_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page25_lock0;
#[doc = "PAGE25_LOCK1 (r) register accessor: Lock configuration MSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page25_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page25_lock1`]
module"]
pub type PAGE25_LOCK1 = crate::Reg<page25_lock1::PAGE25_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page25_lock1;
#[doc = "PAGE26_LOCK0 (r) register accessor: Lock configuration LSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page26_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page26_lock0`]
module"]
pub type PAGE26_LOCK0 = crate::Reg<page26_lock0::PAGE26_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page26_lock0;
#[doc = "PAGE26_LOCK1 (r) register accessor: Lock configuration MSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page26_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page26_lock1`]
module"]
pub type PAGE26_LOCK1 = crate::Reg<page26_lock1::PAGE26_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page26_lock1;
#[doc = "PAGE27_LOCK0 (r) register accessor: Lock configuration LSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page27_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page27_lock0`]
module"]
pub type PAGE27_LOCK0 = crate::Reg<page27_lock0::PAGE27_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page27_lock0;
#[doc = "PAGE27_LOCK1 (r) register accessor: Lock configuration MSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page27_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page27_lock1`]
module"]
pub type PAGE27_LOCK1 = crate::Reg<page27_lock1::PAGE27_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page27_lock1;
#[doc = "PAGE28_LOCK0 (r) register accessor: Lock configuration LSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page28_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page28_lock0`]
module"]
pub type PAGE28_LOCK0 = crate::Reg<page28_lock0::PAGE28_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page28_lock0;
#[doc = "PAGE28_LOCK1 (r) register accessor: Lock configuration MSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page28_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page28_lock1`]
module"]
pub type PAGE28_LOCK1 = crate::Reg<page28_lock1::PAGE28_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page28_lock1;
#[doc = "PAGE29_LOCK0 (r) register accessor: Lock configuration LSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page29_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page29_lock0`]
module"]
pub type PAGE29_LOCK0 = crate::Reg<page29_lock0::PAGE29_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page29_lock0;
#[doc = "PAGE29_LOCK1 (r) register accessor: Lock configuration MSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page29_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page29_lock1`]
module"]
pub type PAGE29_LOCK1 = crate::Reg<page29_lock1::PAGE29_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page29_lock1;
#[doc = "PAGE30_LOCK0 (r) register accessor: Lock configuration LSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page30_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page30_lock0`]
module"]
pub type PAGE30_LOCK0 = crate::Reg<page30_lock0::PAGE30_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page30_lock0;
#[doc = "PAGE30_LOCK1 (r) register accessor: Lock configuration MSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page30_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page30_lock1`]
module"]
pub type PAGE30_LOCK1 = crate::Reg<page30_lock1::PAGE30_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page30_lock1;
#[doc = "PAGE31_LOCK0 (r) register accessor: Lock configuration LSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page31_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page31_lock0`]
module"]
pub type PAGE31_LOCK0 = crate::Reg<page31_lock0::PAGE31_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page31_lock0;
#[doc = "PAGE31_LOCK1 (r) register accessor: Lock configuration MSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page31_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page31_lock1`]
module"]
pub type PAGE31_LOCK1 = crate::Reg<page31_lock1::PAGE31_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page31_lock1;
#[doc = "PAGE32_LOCK0 (r) register accessor: Lock configuration LSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page32_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page32_lock0`]
module"]
pub type PAGE32_LOCK0 = crate::Reg<page32_lock0::PAGE32_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page32_lock0;
#[doc = "PAGE32_LOCK1 (r) register accessor: Lock configuration MSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page32_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page32_lock1`]
module"]
pub type PAGE32_LOCK1 = crate::Reg<page32_lock1::PAGE32_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page32_lock1;
#[doc = "PAGE33_LOCK0 (r) register accessor: Lock configuration LSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page33_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page33_lock0`]
module"]
pub type PAGE33_LOCK0 = crate::Reg<page33_lock0::PAGE33_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page33_lock0;
#[doc = "PAGE33_LOCK1 (r) register accessor: Lock configuration MSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page33_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page33_lock1`]
module"]
pub type PAGE33_LOCK1 = crate::Reg<page33_lock1::PAGE33_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page33_lock1;
#[doc = "PAGE34_LOCK0 (r) register accessor: Lock configuration LSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page34_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page34_lock0`]
module"]
pub type PAGE34_LOCK0 = crate::Reg<page34_lock0::PAGE34_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page34_lock0;
#[doc = "PAGE34_LOCK1 (r) register accessor: Lock configuration MSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page34_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page34_lock1`]
module"]
pub type PAGE34_LOCK1 = crate::Reg<page34_lock1::PAGE34_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page34_lock1;
#[doc = "PAGE35_LOCK0 (r) register accessor: Lock configuration LSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page35_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page35_lock0`]
module"]
pub type PAGE35_LOCK0 = crate::Reg<page35_lock0::PAGE35_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page35_lock0;
#[doc = "PAGE35_LOCK1 (r) register accessor: Lock configuration MSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page35_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page35_lock1`]
module"]
pub type PAGE35_LOCK1 = crate::Reg<page35_lock1::PAGE35_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page35_lock1;
#[doc = "PAGE36_LOCK0 (r) register accessor: Lock configuration LSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page36_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page36_lock0`]
module"]
pub type PAGE36_LOCK0 = crate::Reg<page36_lock0::PAGE36_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page36_lock0;
#[doc = "PAGE36_LOCK1 (r) register accessor: Lock configuration MSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page36_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page36_lock1`]
module"]
pub type PAGE36_LOCK1 = crate::Reg<page36_lock1::PAGE36_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page36_lock1;
#[doc = "PAGE37_LOCK0 (r) register accessor: Lock configuration LSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page37_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page37_lock0`]
module"]
pub type PAGE37_LOCK0 = crate::Reg<page37_lock0::PAGE37_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page37_lock0;
#[doc = "PAGE37_LOCK1 (r) register accessor: Lock configuration MSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page37_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page37_lock1`]
module"]
pub type PAGE37_LOCK1 = crate::Reg<page37_lock1::PAGE37_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page37_lock1;
#[doc = "PAGE38_LOCK0 (r) register accessor: Lock configuration LSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page38_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page38_lock0`]
module"]
pub type PAGE38_LOCK0 = crate::Reg<page38_lock0::PAGE38_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page38_lock0;
#[doc = "PAGE38_LOCK1 (r) register accessor: Lock configuration MSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page38_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page38_lock1`]
module"]
pub type PAGE38_LOCK1 = crate::Reg<page38_lock1::PAGE38_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page38_lock1;
#[doc = "PAGE39_LOCK0 (r) register accessor: Lock configuration LSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page39_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page39_lock0`]
module"]
pub type PAGE39_LOCK0 = crate::Reg<page39_lock0::PAGE39_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page39_lock0;
#[doc = "PAGE39_LOCK1 (r) register accessor: Lock configuration MSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page39_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page39_lock1`]
module"]
pub type PAGE39_LOCK1 = crate::Reg<page39_lock1::PAGE39_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page39_lock1;
#[doc = "PAGE40_LOCK0 (r) register accessor: Lock configuration LSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page40_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page40_lock0`]
module"]
pub type PAGE40_LOCK0 = crate::Reg<page40_lock0::PAGE40_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page40_lock0;
#[doc = "PAGE40_LOCK1 (r) register accessor: Lock configuration MSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page40_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page40_lock1`]
module"]
pub type PAGE40_LOCK1 = crate::Reg<page40_lock1::PAGE40_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page40_lock1;
#[doc = "PAGE41_LOCK0 (r) register accessor: Lock configuration LSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page41_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page41_lock0`]
module"]
pub type PAGE41_LOCK0 = crate::Reg<page41_lock0::PAGE41_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page41_lock0;
#[doc = "PAGE41_LOCK1 (r) register accessor: Lock configuration MSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page41_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page41_lock1`]
module"]
pub type PAGE41_LOCK1 = crate::Reg<page41_lock1::PAGE41_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page41_lock1;
#[doc = "PAGE42_LOCK0 (r) register accessor: Lock configuration LSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page42_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page42_lock0`]
module"]
pub type PAGE42_LOCK0 = crate::Reg<page42_lock0::PAGE42_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page42_lock0;
#[doc = "PAGE42_LOCK1 (r) register accessor: Lock configuration MSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page42_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page42_lock1`]
module"]
pub type PAGE42_LOCK1 = crate::Reg<page42_lock1::PAGE42_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page42_lock1;
#[doc = "PAGE43_LOCK0 (r) register accessor: Lock configuration LSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page43_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page43_lock0`]
module"]
pub type PAGE43_LOCK0 = crate::Reg<page43_lock0::PAGE43_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page43_lock0;
#[doc = "PAGE43_LOCK1 (r) register accessor: Lock configuration MSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page43_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page43_lock1`]
module"]
pub type PAGE43_LOCK1 = crate::Reg<page43_lock1::PAGE43_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page43_lock1;
#[doc = "PAGE44_LOCK0 (r) register accessor: Lock configuration LSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page44_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page44_lock0`]
module"]
pub type PAGE44_LOCK0 = crate::Reg<page44_lock0::PAGE44_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page44_lock0;
#[doc = "PAGE44_LOCK1 (r) register accessor: Lock configuration MSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page44_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page44_lock1`]
module"]
pub type PAGE44_LOCK1 = crate::Reg<page44_lock1::PAGE44_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page44_lock1;
#[doc = "PAGE45_LOCK0 (r) register accessor: Lock configuration LSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page45_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page45_lock0`]
module"]
pub type PAGE45_LOCK0 = crate::Reg<page45_lock0::PAGE45_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page45_lock0;
#[doc = "PAGE45_LOCK1 (r) register accessor: Lock configuration MSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page45_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page45_lock1`]
module"]
pub type PAGE45_LOCK1 = crate::Reg<page45_lock1::PAGE45_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page45_lock1;
#[doc = "PAGE46_LOCK0 (r) register accessor: Lock configuration LSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page46_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page46_lock0`]
module"]
pub type PAGE46_LOCK0 = crate::Reg<page46_lock0::PAGE46_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page46_lock0;
#[doc = "PAGE46_LOCK1 (r) register accessor: Lock configuration MSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page46_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page46_lock1`]
module"]
pub type PAGE46_LOCK1 = crate::Reg<page46_lock1::PAGE46_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page46_lock1;
#[doc = "PAGE47_LOCK0 (r) register accessor: Lock configuration LSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page47_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page47_lock0`]
module"]
pub type PAGE47_LOCK0 = crate::Reg<page47_lock0::PAGE47_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page47_lock0;
#[doc = "PAGE47_LOCK1 (r) register accessor: Lock configuration MSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page47_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page47_lock1`]
module"]
pub type PAGE47_LOCK1 = crate::Reg<page47_lock1::PAGE47_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page47_lock1;
#[doc = "PAGE48_LOCK0 (r) register accessor: Lock configuration LSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page48_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page48_lock0`]
module"]
pub type PAGE48_LOCK0 = crate::Reg<page48_lock0::PAGE48_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page48_lock0;
#[doc = "PAGE48_LOCK1 (r) register accessor: Lock configuration MSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page48_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page48_lock1`]
module"]
pub type PAGE48_LOCK1 = crate::Reg<page48_lock1::PAGE48_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page48_lock1;
#[doc = "PAGE49_LOCK0 (r) register accessor: Lock configuration LSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page49_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page49_lock0`]
module"]
pub type PAGE49_LOCK0 = crate::Reg<page49_lock0::PAGE49_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page49_lock0;
#[doc = "PAGE49_LOCK1 (r) register accessor: Lock configuration MSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page49_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page49_lock1`]
module"]
pub type PAGE49_LOCK1 = crate::Reg<page49_lock1::PAGE49_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page49_lock1;
#[doc = "PAGE50_LOCK0 (r) register accessor: Lock configuration LSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page50_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page50_lock0`]
module"]
pub type PAGE50_LOCK0 = crate::Reg<page50_lock0::PAGE50_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page50_lock0;
#[doc = "PAGE50_LOCK1 (r) register accessor: Lock configuration MSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page50_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page50_lock1`]
module"]
pub type PAGE50_LOCK1 = crate::Reg<page50_lock1::PAGE50_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page50_lock1;
#[doc = "PAGE51_LOCK0 (r) register accessor: Lock configuration LSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page51_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page51_lock0`]
module"]
pub type PAGE51_LOCK0 = crate::Reg<page51_lock0::PAGE51_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page51_lock0;
#[doc = "PAGE51_LOCK1 (r) register accessor: Lock configuration MSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page51_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page51_lock1`]
module"]
pub type PAGE51_LOCK1 = crate::Reg<page51_lock1::PAGE51_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page51_lock1;
#[doc = "PAGE52_LOCK0 (r) register accessor: Lock configuration LSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page52_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page52_lock0`]
module"]
pub type PAGE52_LOCK0 = crate::Reg<page52_lock0::PAGE52_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page52_lock0;
#[doc = "PAGE52_LOCK1 (r) register accessor: Lock configuration MSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page52_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page52_lock1`]
module"]
pub type PAGE52_LOCK1 = crate::Reg<page52_lock1::PAGE52_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page52_lock1;
#[doc = "PAGE53_LOCK0 (r) register accessor: Lock configuration LSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page53_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page53_lock0`]
module"]
pub type PAGE53_LOCK0 = crate::Reg<page53_lock0::PAGE53_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page53_lock0;
#[doc = "PAGE53_LOCK1 (r) register accessor: Lock configuration MSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page53_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page53_lock1`]
module"]
pub type PAGE53_LOCK1 = crate::Reg<page53_lock1::PAGE53_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page53_lock1;
#[doc = "PAGE54_LOCK0 (r) register accessor: Lock configuration LSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page54_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page54_lock0`]
module"]
pub type PAGE54_LOCK0 = crate::Reg<page54_lock0::PAGE54_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page54_lock0;
#[doc = "PAGE54_LOCK1 (r) register accessor: Lock configuration MSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page54_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page54_lock1`]
module"]
pub type PAGE54_LOCK1 = crate::Reg<page54_lock1::PAGE54_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page54_lock1;
#[doc = "PAGE55_LOCK0 (r) register accessor: Lock configuration LSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page55_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page55_lock0`]
module"]
pub type PAGE55_LOCK0 = crate::Reg<page55_lock0::PAGE55_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page55_lock0;
#[doc = "PAGE55_LOCK1 (r) register accessor: Lock configuration MSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page55_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page55_lock1`]
module"]
pub type PAGE55_LOCK1 = crate::Reg<page55_lock1::PAGE55_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page55_lock1;
#[doc = "PAGE56_LOCK0 (r) register accessor: Lock configuration LSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page56_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page56_lock0`]
module"]
pub type PAGE56_LOCK0 = crate::Reg<page56_lock0::PAGE56_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page56_lock0;
#[doc = "PAGE56_LOCK1 (r) register accessor: Lock configuration MSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page56_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page56_lock1`]
module"]
pub type PAGE56_LOCK1 = crate::Reg<page56_lock1::PAGE56_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page56_lock1;
#[doc = "PAGE57_LOCK0 (r) register accessor: Lock configuration LSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page57_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page57_lock0`]
module"]
pub type PAGE57_LOCK0 = crate::Reg<page57_lock0::PAGE57_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page57_lock0;
#[doc = "PAGE57_LOCK1 (r) register accessor: Lock configuration MSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page57_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page57_lock1`]
module"]
pub type PAGE57_LOCK1 = crate::Reg<page57_lock1::PAGE57_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page57_lock1;
#[doc = "PAGE58_LOCK0 (r) register accessor: Lock configuration LSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page58_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page58_lock0`]
module"]
pub type PAGE58_LOCK0 = crate::Reg<page58_lock0::PAGE58_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page58_lock0;
#[doc = "PAGE58_LOCK1 (r) register accessor: Lock configuration MSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page58_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page58_lock1`]
module"]
pub type PAGE58_LOCK1 = crate::Reg<page58_lock1::PAGE58_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page58_lock1;
#[doc = "PAGE59_LOCK0 (r) register accessor: Lock configuration LSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page59_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page59_lock0`]
module"]
pub type PAGE59_LOCK0 = crate::Reg<page59_lock0::PAGE59_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page59_lock0;
#[doc = "PAGE59_LOCK1 (r) register accessor: Lock configuration MSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page59_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page59_lock1`]
module"]
pub type PAGE59_LOCK1 = crate::Reg<page59_lock1::PAGE59_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page59_lock1;
#[doc = "PAGE60_LOCK0 (r) register accessor: Lock configuration LSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page60_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page60_lock0`]
module"]
pub type PAGE60_LOCK0 = crate::Reg<page60_lock0::PAGE60_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page60_lock0;
#[doc = "PAGE60_LOCK1 (r) register accessor: Lock configuration MSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page60_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page60_lock1`]
module"]
pub type PAGE60_LOCK1 = crate::Reg<page60_lock1::PAGE60_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page60_lock1;
#[doc = "PAGE61_LOCK0 (r) register accessor: Lock configuration LSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page61_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page61_lock0`]
module"]
pub type PAGE61_LOCK0 = crate::Reg<page61_lock0::PAGE61_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page61_lock0;
#[doc = "PAGE61_LOCK1 (r) register accessor: Lock configuration MSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page61_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page61_lock1`]
module"]
pub type PAGE61_LOCK1 = crate::Reg<page61_lock1::PAGE61_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page61_lock1;
#[doc = "PAGE62_LOCK0 (r) register accessor: Lock configuration LSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page62_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page62_lock0`]
module"]
pub type PAGE62_LOCK0 = crate::Reg<page62_lock0::PAGE62_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page62_lock0;
#[doc = "PAGE62_LOCK1 (r) register accessor: Lock configuration MSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page62_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page62_lock1`]
module"]
pub type PAGE62_LOCK1 = crate::Reg<page62_lock1::PAGE62_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page62_lock1;
#[doc = "PAGE63_LOCK0 (r) register accessor: Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page63_lock0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page63_lock0`]
module"]
pub type PAGE63_LOCK0 = crate::Reg<page63_lock0::PAGE63_LOCK0_SPEC>;
#[doc = "Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page63_lock0;
#[doc = "PAGE63_LOCK1 (r) register accessor: Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page63_lock1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@page63_lock1`]
module"]
pub type PAGE63_LOCK1 = crate::Reg<page63_lock1::PAGE63_LOCK1_SPEC>;
#[doc = "Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently.  

 This OTP location is always readable, and is write-protected by its own permissions."]
pub mod page63_lock1;
