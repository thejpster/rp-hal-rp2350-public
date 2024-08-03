#[doc = "Register `CRIT1_R5` reader"]
pub type R = crate::R<CRIT1_R5_SPEC>;
#[doc = "Field `CRIT1_R5` reader - "]
pub type CRIT1_R5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn crit1_r5(&self) -> CRIT1_R5_R {
        CRIT1_R5_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Redundant copy of CRIT1  

You can [`read`](crate::Reg::read) this register and get [`crit1_r5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRIT1_R5_SPEC;
impl crate::RegisterSpec for CRIT1_R5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crit1_r5::R`](R) reader structure"]
impl crate::Readable for CRIT1_R5_SPEC {}
#[doc = "`reset()` method sets CRIT1_R5 to value 0"]
impl crate::Resettable for CRIT1_R5_SPEC {
    const RESET_VALUE: u32 = 0;
}