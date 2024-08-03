#[doc = "Register `CRIT0_R2` reader"]
pub type R = crate::R<CRIT0_R2_SPEC>;
#[doc = "Field `CRIT0_R2` reader - "]
pub type CRIT0_R2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn crit0_r2(&self) -> CRIT0_R2_R {
        CRIT0_R2_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Redundant copy of CRIT0  

You can [`read`](crate::Reg::read) this register and get [`crit0_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT0_R2_SPEC;
impl crate::RegisterSpec for CRIT0_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit0_r2::R`](R) reader structure"]
impl crate::Readable for CRIT0_R2_SPEC {}
#[doc = "`reset()` method sets CRIT0_R2 to value 0"]
impl crate::Resettable for CRIT0_R2_SPEC {
    const RESET_VALUE: u32 = 0;
}
