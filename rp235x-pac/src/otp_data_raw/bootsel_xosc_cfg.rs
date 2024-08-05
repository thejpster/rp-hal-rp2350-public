#[doc = "Register `BOOTSEL_XOSC_CFG` reader"]
pub type R = crate::R<BOOTSEL_XOSC_CFG_SPEC>;
#[doc = "Register `BOOTSEL_XOSC_CFG` writer"]
pub type W = crate::W<BOOTSEL_XOSC_CFG_SPEC>;
#[doc = "Field `STARTUP` reader - Value of the XOSC_STARTUP register"]
pub type STARTUP_R = crate::FieldReader<u16>;
#[doc = "Value of the XOSC_CTRL_FREQ_RANGE register.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: `0`"]
    _1_15MHZ = 0,
    #[doc = "1: `1`"]
    _10_30MHZ = 1,
    #[doc = "2: `10`"]
    _25_60MHZ = 2,
    #[doc = "3: `11`"]
    _40_100MHZ = 3,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RANGE_A {
    type Ux = u8;
}
impl crate::IsEnum for RANGE_A {}
#[doc = "Field `RANGE` reader - Value of the XOSC_CTRL_FREQ_RANGE register."]
pub type RANGE_R = crate::FieldReader<RANGE_A>;
impl RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RANGE_A {
        match self.bits {
            0 => RANGE_A::_1_15MHZ,
            1 => RANGE_A::_10_30MHZ,
            2 => RANGE_A::_25_60MHZ,
            3 => RANGE_A::_40_100MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_1_15mhz(&self) -> bool {
        *self == RANGE_A::_1_15MHZ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_10_30mhz(&self) -> bool {
        *self == RANGE_A::_10_30MHZ
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_25_60mhz(&self) -> bool {
        *self == RANGE_A::_25_60MHZ
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_40_100mhz(&self) -> bool {
        *self == RANGE_A::_40_100MHZ
    }
}
impl R {
    #[doc = "Bits 0:13 - Value of the XOSC_STARTUP register"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(self.bits & 0x3fff)
    }
    #[doc = "Bits 14:15 - Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {}
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_xosc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootsel_xosc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTSEL_XOSC_CFG_SPEC;
impl crate::RegisterSpec for BOOTSEL_XOSC_CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootsel_xosc_cfg::R`](R) reader structure"]
impl crate::Readable for BOOTSEL_XOSC_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootsel_xosc_cfg::W`](W) writer structure"]
impl crate::Writable for BOOTSEL_XOSC_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BOOTSEL_XOSC_CFG to value 0"]
impl crate::Resettable for BOOTSEL_XOSC_CFG_SPEC {
    const RESET_VALUE: u16 = 0;
}
