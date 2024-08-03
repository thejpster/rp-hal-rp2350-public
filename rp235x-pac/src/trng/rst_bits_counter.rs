#[doc = "Register `RST_BITS_COUNTER` reader"]
pub type R = crate::R<RST_BITS_COUNTER_SPEC>;
#[doc = "Register `RST_BITS_COUNTER` writer"]
pub type W = crate::W<RST_BITS_COUNTER_SPEC>;
#[doc = "Field `RST_BITS_COUNTER` reader - Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
pub type RST_BITS_COUNTER_R = crate::BitReader;
#[doc = "Field `RST_BITS_COUNTER` writer - Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
pub type RST_BITS_COUNTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
    #[inline(always)]
    pub fn rst_bits_counter(&self) -> RST_BITS_COUNTER_R {
        RST_BITS_COUNTER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
    #[inline(always)]
    #[must_use]
    pub fn rst_bits_counter(&mut self) -> RST_BITS_COUNTER_W<RST_BITS_COUNTER_SPEC> {
        RST_BITS_COUNTER_W::new(self, 0)
    }
}
#[doc = "Reset the counter of collected bits in the RNG.  

You can [`read`](crate::Reg::read) this register and get [`rst_bits_counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_bits_counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_BITS_COUNTER_SPEC;
impl crate::RegisterSpec for RST_BITS_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_bits_counter::R`](R) reader structure"]
impl crate::Readable for RST_BITS_COUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_bits_counter::W`](W) writer structure"]
impl crate::Writable for RST_BITS_COUNTER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_BITS_COUNTER to value 0"]
impl crate::Resettable for RST_BITS_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
