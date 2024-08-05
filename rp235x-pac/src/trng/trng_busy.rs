#[doc = "Register `TRNG_BUSY` reader"]
pub type R = crate::R<TRNG_BUSY_SPEC>;
#[doc = "Register `TRNG_BUSY` writer"]
pub type W = crate::W<TRNG_BUSY_SPEC>;
#[doc = "Field `TRNG_BUSY` reader - Reflects rng_busy status."]
pub type TRNG_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reflects rng_busy status."]
    #[inline(always)]
    pub fn trng_busy(&self) -> TRNG_BUSY_R {
        TRNG_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "RNG Busy indication.  

You can [`read`](crate::Reg::read) this register and get [`trng_busy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_busy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRNG_BUSY_SPEC;
impl crate::RegisterSpec for TRNG_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_busy::R`](R) reader structure"]
impl crate::Readable for TRNG_BUSY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trng_busy::W`](W) writer structure"]
impl crate::Writable for TRNG_BUSY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_BUSY to value 0"]
impl crate::Resettable for TRNG_BUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
