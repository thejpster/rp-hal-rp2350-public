#[doc = "Register `RNG_BIST_CNTR_0` reader"]
pub type R = crate::R<RNG_BIST_CNTR_0_SPEC>;
#[doc = "Field `ROSC_CNTR_VAL` reader - Reflects the results of RNG BIST counter."]
pub type ROSC_CNTR_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> ROSC_CNTR_VAL_R {
        ROSC_CNTR_VAL_R::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Collected BIST results.  

You can [`read`](crate::Reg::read) this register and get [`rng_bist_cntr_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_BIST_CNTR_0_SPEC;
impl crate::RegisterSpec for RNG_BIST_CNTR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_bist_cntr_0::R`](R) reader structure"]
impl crate::Readable for RNG_BIST_CNTR_0_SPEC {}
#[doc = "`reset()` method sets RNG_BIST_CNTR_0 to value 0"]
impl crate::Resettable for RNG_BIST_CNTR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
