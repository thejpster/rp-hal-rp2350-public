#[doc = "Register `RNG_DEBUG_EN_INPUT` reader"]
pub type R = crate::R<RNG_DEBUG_EN_INPUT_SPEC>;
#[doc = "Register `RNG_DEBUG_EN_INPUT` writer"]
pub type W = crate::W<RNG_DEBUG_EN_INPUT_SPEC>;
#[doc = "Field `RNG_DEBUG_EN` reader - * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
pub type RNG_DEBUG_EN_R = crate::BitReader;
#[doc = "Field `RNG_DEBUG_EN` writer - * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
pub type RNG_DEBUG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
    #[inline(always)]
    pub fn rng_debug_en(&self) -> RNG_DEBUG_EN_R {
        RNG_DEBUG_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - * 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rng_debug_en(&mut self) -> RNG_DEBUG_EN_W<RNG_DEBUG_EN_INPUT_SPEC> {
        RNG_DEBUG_EN_W::new(self, 0)
    }
}
#[doc = "Enable the RNG debug mode  

You can [`read`](crate::Reg::read) this register and get [`rng_debug_en_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_debug_en_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_DEBUG_EN_INPUT_SPEC;
impl crate::RegisterSpec for RNG_DEBUG_EN_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_debug_en_input::R`](R) reader structure"]
impl crate::Readable for RNG_DEBUG_EN_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_debug_en_input::W`](W) writer structure"]
impl crate::Writable for RNG_DEBUG_EN_INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_DEBUG_EN_INPUT to value 0"]
impl crate::Resettable for RNG_DEBUG_EN_INPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
