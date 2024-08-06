#[doc = "Register `BOOT_FLAGS0` reader"]
pub type R = crate::R<BOOT_FLAGS0_SPEC>;
#[doc = "Register `BOOT_FLAGS0` writer"]
pub type W = crate::W<BOOT_FLAGS0_SPEC>;
#[doc = "Field `DISABLE_BOOTSEL_EXEC2` reader - "]
pub type DISABLE_BOOTSEL_EXEC2_R = crate::BitReader;
#[doc = "Field `ENABLE_BOOTSEL_LED` reader - Enable bootloader activity LED. If set, bootsel_led_cfg is assumed to be valid"]
pub type ENABLE_BOOTSEL_LED_R = crate::BitReader;
#[doc = "Field `ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG` reader - Enable loading of the non-default XOSC and PLL configuration before entering BOOTSEL mode. Ensure that BOOTSEL_XOSC_CFG and BOOTSEL_PLL_CFG are correctly programmed before setting this bit. If this bit is set, user software may use the contents of BOOTSEL_PLL_CFG to calculated the expected XOSC frequency based on the fixed USB boot frequency of 48 MHz."]
pub type ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG_R = crate::BitReader;
#[doc = "Field `FLASH_IO_VOLTAGE_1V8` reader - If 1, configure the QSPI pads for 1.8 V operation when accessing flash for the first time from the bootrom, using the VOLTAGE_SELECT register for the QSPI pads bank. This slightly improves the input timing of the pads at low voltages, but does not affect their output characteristics. If 0, leave VOLTAGE_SELECT in its reset state (suitable for operation at and above 2.5 V)"]
pub type FLASH_IO_VOLTAGE_1V8_R = crate::BitReader;
#[doc = "Field `FAST_SIGCHECK_ROSC_DIV` reader - Enable quartering of ROSC divisor during signature check, to reduce secure boot time"]
pub type FAST_SIGCHECK_ROSC_DIV_R = crate::BitReader;
#[doc = "Field `FLASH_DEVINFO_ENABLE` reader - Mark FLASH_DEVINFO as containing valid, ECC'd data which describes external flash devices."]
pub type FLASH_DEVINFO_ENABLE_R = crate::BitReader;
#[doc = "Field `OVERRIDE_FLASH_PARTITION_SLOT_SIZE` reader - Override the limit for default flash metadata scanning. The value is specified in FLASH_PARTITION_SLOT_SIZE. Make sure FLASH_PARTITION_SLOT_SIZE is valid before setting this bit"]
pub type OVERRIDE_FLASH_PARTITION_SLOT_SIZE_R = crate::BitReader;
#[doc = "Field `SINGLE_FLASH_BINARY` reader - Restrict flash boot path to use of a single binary at the start of flash"]
pub type SINGLE_FLASH_BINARY_R = crate::BitReader;
#[doc = "Field `DISABLE_AUTO_SWITCH_ARCH` reader - Disable auto-switch of CPU architecture on boot when the (only) binary to be booted is for the other Arm/RISC-V architecture and both architectures are enabled"]
pub type DISABLE_AUTO_SWITCH_ARCH_R = crate::BitReader;
#[doc = "Field `SECURE_PARTITION_TABLE` reader - Require a partition table to be signed"]
pub type SECURE_PARTITION_TABLE_R = crate::BitReader;
#[doc = "Field `HASHED_PARTITION_TABLE` reader - Require a partition table to be hashed (if not signed)"]
pub type HASHED_PARTITION_TABLE_R = crate::BitReader;
#[doc = "Field `ROLLBACK_REQUIRED` reader - Require binaries to have a rollback version. Set automatically the first time a binary with a rollback version is booted."]
pub type ROLLBACK_REQUIRED_R = crate::BitReader;
#[doc = "Field `DISABLE_FLASH_BOOT` reader - "]
pub type DISABLE_FLASH_BOOT_R = crate::BitReader;
#[doc = "Field `DISABLE_OTP_BOOT` reader - Takes precedence over ENABLE_OTP_BOOT."]
pub type DISABLE_OTP_BOOT_R = crate::BitReader;
#[doc = "Field `ENABLE_OTP_BOOT` reader - Enable OTP boot. A number of OTP rows specified by OTPBOOT_LEN will be loaded, starting from OTPBOOT_SRC, into the SRAM location specified by OTPBOOT_DST1 and OTPBOOT_DST0. The loaded program image is stored with ECC, 16 bits per row, and must contain a valid IMAGE_DEF. Do not set this bit without first programming an image into OTP and configuring OTPBOOT_LEN, OTPBOOT_SRC, OTPBOOT_DST0 and OTPBOOT_DST1. Note that OTPBOOT_LEN and OTPBOOT_SRC must be even numbers of OTP rows. Equivalently, the image must be a multiple of 32 bits in size, and must start at a 32-bit-aligned address in the ECC read data address window."]
pub type ENABLE_OTP_BOOT_R = crate::BitReader;
#[doc = "Field `DISABLE_POWER_SCRATCH` reader - "]
pub type DISABLE_POWER_SCRATCH_R = crate::BitReader;
#[doc = "Field `DISABLE_WATCHDOG_SCRATCH` reader - "]
pub type DISABLE_WATCHDOG_SCRATCH_R = crate::BitReader;
#[doc = "Field `DISABLE_BOOTSEL_USB_MSD_IFC` reader - "]
pub type DISABLE_BOOTSEL_USB_MSD_IFC_R = crate::BitReader;
#[doc = "Field `DISABLE_BOOTSEL_USB_PICOBOOT_IFC` reader - "]
pub type DISABLE_BOOTSEL_USB_PICOBOOT_IFC_R = crate::BitReader;
#[doc = "Field `DISABLE_BOOTSEL_UART_BOOT` reader - "]
pub type DISABLE_BOOTSEL_UART_BOOT_R = crate::BitReader;
#[doc = "Field `DISABLE_XIP_ACCESS_ON_SRAM_ENTRY` reader - Disable all access to XIP after entering an SRAM binary. Note that this will cause bootrom APIs that access XIP to fail, including APIs that interact with the partition table."]
pub type DISABLE_XIP_ACCESS_ON_SRAM_ENTRY_R = crate::BitReader;
#[doc = "Field `DISABLE_SRAM_WINDOW_BOOT` reader - "]
pub type DISABLE_SRAM_WINDOW_BOOT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn disable_bootsel_exec2(&self) -> DISABLE_BOOTSEL_EXEC2_R {
        DISABLE_BOOTSEL_EXEC2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable bootloader activity LED. If set, bootsel_led_cfg is assumed to be valid"]
    #[inline(always)]
    pub fn enable_bootsel_led(&self) -> ENABLE_BOOTSEL_LED_R {
        ENABLE_BOOTSEL_LED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable loading of the non-default XOSC and PLL configuration before entering BOOTSEL mode. Ensure that BOOTSEL_XOSC_CFG and BOOTSEL_PLL_CFG are correctly programmed before setting this bit. If this bit is set, user software may use the contents of BOOTSEL_PLL_CFG to calculated the expected XOSC frequency based on the fixed USB boot frequency of 48 MHz."]
    #[inline(always)]
    pub fn enable_bootsel_non_default_pll_xosc_cfg(
        &self,
    ) -> ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG_R {
        ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, configure the QSPI pads for 1.8 V operation when accessing flash for the first time from the bootrom, using the VOLTAGE_SELECT register for the QSPI pads bank. This slightly improves the input timing of the pads at low voltages, but does not affect their output characteristics. If 0, leave VOLTAGE_SELECT in its reset state (suitable for operation at and above 2.5 V)"]
    #[inline(always)]
    pub fn flash_io_voltage_1v8(&self) -> FLASH_IO_VOLTAGE_1V8_R {
        FLASH_IO_VOLTAGE_1V8_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable quartering of ROSC divisor during signature check, to reduce secure boot time"]
    #[inline(always)]
    pub fn fast_sigcheck_rosc_div(&self) -> FAST_SIGCHECK_ROSC_DIV_R {
        FAST_SIGCHECK_ROSC_DIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mark FLASH_DEVINFO as containing valid, ECC'd data which describes external flash devices."]
    #[inline(always)]
    pub fn flash_devinfo_enable(&self) -> FLASH_DEVINFO_ENABLE_R {
        FLASH_DEVINFO_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override the limit for default flash metadata scanning. The value is specified in FLASH_PARTITION_SLOT_SIZE. Make sure FLASH_PARTITION_SLOT_SIZE is valid before setting this bit"]
    #[inline(always)]
    pub fn override_flash_partition_slot_size(&self) -> OVERRIDE_FLASH_PARTITION_SLOT_SIZE_R {
        OVERRIDE_FLASH_PARTITION_SLOT_SIZE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Restrict flash boot path to use of a single binary at the start of flash"]
    #[inline(always)]
    pub fn single_flash_binary(&self) -> SINGLE_FLASH_BINARY_R {
        SINGLE_FLASH_BINARY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable auto-switch of CPU architecture on boot when the (only) binary to be booted is for the other Arm/RISC-V architecture and both architectures are enabled"]
    #[inline(always)]
    pub fn disable_auto_switch_arch(&self) -> DISABLE_AUTO_SWITCH_ARCH_R {
        DISABLE_AUTO_SWITCH_ARCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Require a partition table to be signed"]
    #[inline(always)]
    pub fn secure_partition_table(&self) -> SECURE_PARTITION_TABLE_R {
        SECURE_PARTITION_TABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Require a partition table to be hashed (if not signed)"]
    #[inline(always)]
    pub fn hashed_partition_table(&self) -> HASHED_PARTITION_TABLE_R {
        HASHED_PARTITION_TABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Require binaries to have a rollback version. Set automatically the first time a binary with a rollback version is booted."]
    #[inline(always)]
    pub fn rollback_required(&self) -> ROLLBACK_REQUIRED_R {
        ROLLBACK_REQUIRED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn disable_flash_boot(&self) -> DISABLE_FLASH_BOOT_R {
        DISABLE_FLASH_BOOT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Takes precedence over ENABLE_OTP_BOOT."]
    #[inline(always)]
    pub fn disable_otp_boot(&self) -> DISABLE_OTP_BOOT_R {
        DISABLE_OTP_BOOT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable OTP boot. A number of OTP rows specified by OTPBOOT_LEN will be loaded, starting from OTPBOOT_SRC, into the SRAM location specified by OTPBOOT_DST1 and OTPBOOT_DST0. The loaded program image is stored with ECC, 16 bits per row, and must contain a valid IMAGE_DEF. Do not set this bit without first programming an image into OTP and configuring OTPBOOT_LEN, OTPBOOT_SRC, OTPBOOT_DST0 and OTPBOOT_DST1. Note that OTPBOOT_LEN and OTPBOOT_SRC must be even numbers of OTP rows. Equivalently, the image must be a multiple of 32 bits in size, and must start at a 32-bit-aligned address in the ECC read data address window."]
    #[inline(always)]
    pub fn enable_otp_boot(&self) -> ENABLE_OTP_BOOT_R {
        ENABLE_OTP_BOOT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn disable_power_scratch(&self) -> DISABLE_POWER_SCRATCH_R {
        DISABLE_POWER_SCRATCH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn disable_watchdog_scratch(&self) -> DISABLE_WATCHDOG_SCRATCH_R {
        DISABLE_WATCHDOG_SCRATCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn disable_bootsel_usb_msd_ifc(&self) -> DISABLE_BOOTSEL_USB_MSD_IFC_R {
        DISABLE_BOOTSEL_USB_MSD_IFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn disable_bootsel_usb_picoboot_ifc(&self) -> DISABLE_BOOTSEL_USB_PICOBOOT_IFC_R {
        DISABLE_BOOTSEL_USB_PICOBOOT_IFC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disable_bootsel_uart_boot(&self) -> DISABLE_BOOTSEL_UART_BOOT_R {
        DISABLE_BOOTSEL_UART_BOOT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable all access to XIP after entering an SRAM binary. Note that this will cause bootrom APIs that access XIP to fail, including APIs that interact with the partition table."]
    #[inline(always)]
    pub fn disable_xip_access_on_sram_entry(&self) -> DISABLE_XIP_ACCESS_ON_SRAM_ENTRY_R {
        DISABLE_XIP_ACCESS_ON_SRAM_ENTRY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn disable_sram_window_boot(&self) -> DISABLE_SRAM_WINDOW_BOOT_R {
        DISABLE_SRAM_WINDOW_BOOT_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {}
#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_flags0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS0_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags0::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_flags0::W`](W) writer structure"]
impl crate::Writable for BOOT_FLAGS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_FLAGS0 to value 0"]
impl crate::Resettable for BOOT_FLAGS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
