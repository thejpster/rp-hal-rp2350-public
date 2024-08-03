#[doc = "Register `BOOTSEL_PLL_CFG` reader"]
pub type R = crate::R<BOOTSEL_PLL_CFG_SPEC>;
#[doc = "Field `FBDIV` reader - PLL feedback divisor, in the range 16..320 inclusive."]
pub type FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `POSTDIV1` reader - PLL post-divide 1 divisor, in the range 1..7 inclusive."]
pub type POSTDIV1_R = crate::FieldReader;
#[doc = "Field `POSTDIV2` reader - PLL post-divide 2 divisor, in the range 1..7 inclusive."]
pub type POSTDIV2_R = crate::FieldReader;
#[doc = "Field `REFDIV` reader - PLL reference divisor, minus one.  

 Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
pub type REFDIV_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - PLL feedback divisor, in the range 16..320 inclusive."]
    #[inline(always)]
    pub fn fbdiv(&self) -> FBDIV_R {
        FBDIV_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - PLL post-divide 1 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PLL post-divide 2 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - PLL reference divisor, minus one.  

 Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC)  

 This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.  

 If no configuration is given, the crystal is assumed to be 12 MHz.  

 The PLL frequency can be calculated as:  

 PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)  

 Conversely the crystal frequency can be calculated as:  

 XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV  

 (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)  

 Valid if and only if BOOTSEL_CFG_NONDEFAULT_CLOCK_CFG bit is set. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed.  

You can [`read`](crate::Reg::read) this register and get [`bootsel_pll_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTSEL_PLL_CFG_SPEC;
impl crate::RegisterSpec for BOOTSEL_PLL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootsel_pll_cfg::R`](R) reader structure"]
impl crate::Readable for BOOTSEL_PLL_CFG_SPEC {}
#[doc = "`reset()` method sets BOOTSEL_PLL_CFG to value 0"]
impl crate::Resettable for BOOTSEL_PLL_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
