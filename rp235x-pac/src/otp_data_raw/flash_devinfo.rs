#[doc = "Register `FLASH_DEVINFO` reader"]
pub type R = crate::R<FLASH_DEVINFO_SPEC>;
#[doc = "Register `FLASH_DEVINFO` writer"]
pub type W = crate::W<FLASH_DEVINFO_SPEC>;
#[doc = "Field `CS1_GPIO` reader - Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
pub type CS1_GPIO_R = crate::FieldReader;
#[doc = "Field `D8H_ERASE_SUPPORTED` reader - If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
pub type D8H_ERASE_SUPPORTED_R = crate::BitReader;
#[doc = "The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_SIZE_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    _8K = 1,
    #[doc = "2: `10`"]
    _16K = 2,
    #[doc = "3: `11`"]
    _32K = 3,
    #[doc = "4: `100`"]
    _64K = 4,
    #[doc = "5: `101`"]
    _128K = 5,
    #[doc = "6: `110`"]
    _256K = 6,
    #[doc = "7: `111`"]
    _512K = 7,
    #[doc = "8: `1000`"]
    _1M = 8,
    #[doc = "9: `1001`"]
    _2M = 9,
    #[doc = "10: `1010`"]
    _4M = 10,
    #[doc = "11: `1011`"]
    _8M = 11,
    #[doc = "12: `1100`"]
    _16M = 12,
}
impl From<CS0_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS0_SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for CS0_SIZE_A {}
#[doc = "Field `CS0_SIZE` reader - The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
pub type CS0_SIZE_R = crate::FieldReader<CS0_SIZE_A>;
impl CS0_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CS0_SIZE_A> {
        match self.bits {
            0 => Some(CS0_SIZE_A::NONE),
            1 => Some(CS0_SIZE_A::_8K),
            2 => Some(CS0_SIZE_A::_16K),
            3 => Some(CS0_SIZE_A::_32K),
            4 => Some(CS0_SIZE_A::_64K),
            5 => Some(CS0_SIZE_A::_128K),
            6 => Some(CS0_SIZE_A::_256K),
            7 => Some(CS0_SIZE_A::_512K),
            8 => Some(CS0_SIZE_A::_1M),
            9 => Some(CS0_SIZE_A::_2M),
            10 => Some(CS0_SIZE_A::_4M),
            11 => Some(CS0_SIZE_A::_8M),
            12 => Some(CS0_SIZE_A::_16M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CS0_SIZE_A::NONE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == CS0_SIZE_A::_8K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == CS0_SIZE_A::_16K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == CS0_SIZE_A::_32K
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == CS0_SIZE_A::_64K
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == CS0_SIZE_A::_128K
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == CS0_SIZE_A::_256K
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == CS0_SIZE_A::_512K
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == CS0_SIZE_A::_1M
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == CS0_SIZE_A::_2M
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_4m(&self) -> bool {
        *self == CS0_SIZE_A::_4M
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == CS0_SIZE_A::_8M
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == CS0_SIZE_A::_16M
    }
}
#[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS1_SIZE_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    _8K = 1,
    #[doc = "2: `10`"]
    _16K = 2,
    #[doc = "3: `11`"]
    _32K = 3,
    #[doc = "4: `100`"]
    _64K = 4,
    #[doc = "5: `101`"]
    _128K = 5,
    #[doc = "6: `110`"]
    _256K = 6,
    #[doc = "7: `111`"]
    _512K = 7,
    #[doc = "8: `1000`"]
    _1M = 8,
    #[doc = "9: `1001`"]
    _2M = 9,
    #[doc = "10: `1010`"]
    _4M = 10,
    #[doc = "11: `1011`"]
    _8M = 11,
    #[doc = "12: `1100`"]
    _16M = 12,
}
impl From<CS1_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CS1_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS1_SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for CS1_SIZE_A {}
#[doc = "Field `CS1_SIZE` reader - The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
pub type CS1_SIZE_R = crate::FieldReader<CS1_SIZE_A>;
impl CS1_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CS1_SIZE_A> {
        match self.bits {
            0 => Some(CS1_SIZE_A::NONE),
            1 => Some(CS1_SIZE_A::_8K),
            2 => Some(CS1_SIZE_A::_16K),
            3 => Some(CS1_SIZE_A::_32K),
            4 => Some(CS1_SIZE_A::_64K),
            5 => Some(CS1_SIZE_A::_128K),
            6 => Some(CS1_SIZE_A::_256K),
            7 => Some(CS1_SIZE_A::_512K),
            8 => Some(CS1_SIZE_A::_1M),
            9 => Some(CS1_SIZE_A::_2M),
            10 => Some(CS1_SIZE_A::_4M),
            11 => Some(CS1_SIZE_A::_8M),
            12 => Some(CS1_SIZE_A::_16M),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CS1_SIZE_A::NONE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == CS1_SIZE_A::_8K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == CS1_SIZE_A::_16K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == CS1_SIZE_A::_32K
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == CS1_SIZE_A::_64K
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == CS1_SIZE_A::_128K
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == CS1_SIZE_A::_256K
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == CS1_SIZE_A::_512K
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == CS1_SIZE_A::_1M
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == CS1_SIZE_A::_2M
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_4m(&self) -> bool {
        *self == CS1_SIZE_A::_4M
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == CS1_SIZE_A::_8M
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == CS1_SIZE_A::_16M
    }
}
impl R {
    #[doc = "Bits 0:5 - Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
    #[inline(always)]
    pub fn cs1_gpio(&self) -> CS1_GPIO_R {
        CS1_GPIO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
    #[inline(always)]
    pub fn d8h_erase_supported(&self) -> D8H_ERASE_SUPPORTED_R {
        D8H_ERASE_SUPPORTED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
    #[inline(always)]
    pub fn cs0_size(&self) -> CS0_SIZE_R {
        CS0_SIZE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB &lt;&lt; CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub fn cs1_size(&self) -> CS1_SIZE_R {
        CS1_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set.  

You can [`read`](crate::Reg::read) this register and get [`flash_devinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_devinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_DEVINFO_SPEC;
impl crate::RegisterSpec for FLASH_DEVINFO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`flash_devinfo::R`](R) reader structure"]
impl crate::Readable for FLASH_DEVINFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_devinfo::W`](W) writer structure"]
impl crate::Writable for FLASH_DEVINFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FLASH_DEVINFO to value 0"]
impl crate::Resettable for FLASH_DEVINFO_SPEC {
    const RESET_VALUE: u16 = 0;
}
