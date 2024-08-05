#[doc = "Register `RNG_BIST_CNTR_0` reader"]
pub type R = crate::R<RNG_BIST_CNTR_0_SPEC>;
#[doc = "Register `RNG_BIST_CNTR_0` writer"]
pub type W = crate::W<RNG_BIST_CNTR_0_SPEC>;
#[doc = "Field `ROSC_CNTR_VAL` reader - Reflects the results of RNG BIST counter."]
pub type ROSC_CNTR_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> ROSC_CNTR_VAL_R {
        ROSC_CNTR_VAL_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {}
#[doc = "Collected BIST results.  

You can [`read`](crate::Reg::read) this register and get [`rng_bist_cntr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_bist_cntr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_BIST_CNTR_0_SPEC;
impl crate::RegisterSpec for RNG_BIST_CNTR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_bist_cntr_0::R`](R) reader structure"]
impl crate::Readable for RNG_BIST_CNTR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_bist_cntr_0::W`](W) writer structure"]
impl crate::Writable for RNG_BIST_CNTR_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_BIST_CNTR_0 to value 0"]
impl crate::Resettable for RNG_BIST_CNTR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
