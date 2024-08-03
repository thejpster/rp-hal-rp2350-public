#[doc = "Register `TRCPDSR` reader"]
pub type R = crate::R<TRCPDSR_SPEC>;
#[doc = "Field `POWER` reader - Power status bit:"]
pub type POWER_R = crate::BitReader;
#[doc = "Field `STICKYPD` reader - Sticky powerdown status bit. Indicates whether the trace register state is valid:"]
pub type STICKYPD_R = crate::BitReader;
#[doc = "Field `OSLK` reader - OS Lock status bit:"]
pub type OSLK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power status bit:"]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky powerdown status bit. Indicates whether the trace register state is valid:"]
    #[inline(always)]
    pub fn stickypd(&self) -> STICKYPD_R {
        STICKYPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - OS Lock status bit:"]
    #[inline(always)]
    pub fn oslk(&self) -> OSLK_R {
        OSLK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Returns the following information about the trace unit: - OS Lock status. - Core power domain status. - Power interruption status  

You can [`read`](crate::Reg::read) this register and get [`trcpdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCPDSR_SPEC;
impl crate::RegisterSpec for TRCPDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpdsr::R`](R) reader structure"]
impl crate::Readable for TRCPDSR_SPEC {}
#[doc = "`reset()` method sets TRCPDSR to value 0x03"]
impl crate::Resettable for TRCPDSR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
