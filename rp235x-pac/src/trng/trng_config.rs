#[doc = "Register `TRNG_CONFIG` reader"]
pub type R = crate::R<TRNG_CONFIG_SPEC>;
#[doc = "Register `TRNG_CONFIG` writer"]
pub type W = crate::W<TRNG_CONFIG_SPEC>;
#[doc = "Field `RND_SRC_SEL` reader - Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
pub type RND_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `RND_SRC_SEL` writer - Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
pub type RND_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
    #[inline(always)]
    pub fn rnd_src_sel(&self) -> RND_SRC_SEL_R {
        RND_SRC_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
    #[inline(always)]
    #[must_use]
    pub fn rnd_src_sel(&mut self) -> RND_SRC_SEL_W<TRNG_CONFIG_SPEC> {
        RND_SRC_SEL_W::new(self, 0)
    }
}
#[doc = "Selecting the inverter-chain length.  

You can [`read`](crate::Reg::read) this register and get [`trng_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRNG_CONFIG_SPEC;
impl crate::RegisterSpec for TRNG_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trng_config::R`](R) reader structure"]
impl crate::Readable for TRNG_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trng_config::W`](W) writer structure"]
impl crate::Writable for TRNG_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNG_CONFIG to value 0"]
impl crate::Resettable for TRNG_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
