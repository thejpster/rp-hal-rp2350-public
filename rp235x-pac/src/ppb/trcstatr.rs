#[doc = "Register `TRCSTATR` reader"]
pub type R = crate::R<TRCSTATR_SPEC>;
#[doc = "Field `IDLE` reader - Indicates that the trace unit is inactive"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `PMSTABLE` reader - Indicates whether the ETM-Teal registers are stable and can be read"]
pub type PMSTABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that the trace unit is inactive"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the ETM-Teal registers are stable and can be read"]
    #[inline(always)]
    pub fn pmstable(&self) -> PMSTABLE_R {
        PMSTABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The TRCSTATR indicates the ETM-Teal status  

You can [`read`](crate::Reg::read) this register and get [`trcstatr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCSTATR_SPEC;
impl crate::RegisterSpec for TRCSTATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcstatr::R`](R) reader structure"]
impl crate::Readable for TRCSTATR_SPEC {}
#[doc = "`reset()` method sets TRCSTATR to value 0"]
impl crate::Resettable for TRCSTATR_SPEC {
    const RESET_VALUE: u32 = 0;
}
