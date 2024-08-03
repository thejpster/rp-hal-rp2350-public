#[doc = "Register `TRNG_SW_RESET` reader"]
pub type R = crate::R<TRNG_SW_RESET_SPEC>;
#[doc = "Register `TRNG_SW_RESET` writer"]
pub type W = crate::W<TRNG_SW_RESET_SPEC>;
#[doc = "Field `TRNG_SW_RESET` reader - Writing 1'b1 to this register causes an internal RNG reset."]
pub type TRNG_SW_RESET_R = crate::BitReader;
#[doc = "Field `TRNG_SW_RESET` writer - Writing 1'b1 to this register causes an internal RNG reset."]
pub type TRNG_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing 1'b1 to this register causes an internal RNG reset."]
    #[inline(always)]
    pub fn trng_sw_reset(&self) -> TRNG_SW_RESET_R {
        TRNG_SW_RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1'b1 to this register causes an internal RNG reset."]
    #[inline(always)]
    #[must_use]
    pub fn trng_sw_reset(&mut self) -> TRNG_SW_RESET_W<TRNG_SW_RESET_SPEC> {
        TRNG_SW_RESET_W::new(self, 0)
    }
}
#[doc = "Generate internal SW reset within the RNG block.  

You can [`read`](crate::Reg::read) this register and get [`trng_sw_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_sw_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRNG_SW_RESET_SPEC;
impl crate::RegisterSpec for TRNG_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_sw_reset::R`](R) reader structure"]
impl crate::Readable for TRNG_SW_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trng_sw_reset::W`](W) writer structure"]
impl crate::Writable for TRNG_SW_RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_SW_RESET to value 0"]
impl crate::Resettable for TRNG_SW_RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
