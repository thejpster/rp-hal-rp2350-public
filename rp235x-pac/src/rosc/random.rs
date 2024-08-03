#[doc = "Register `RANDOM` reader"]
pub type R = crate::R<RANDOM_SPEC>;
#[doc = "Register `RANDOM` writer"]
pub type W = crate::W<RANDOM_SPEC>;
#[doc = "Field `SEED` reader - "]
pub type SEED_R = crate::FieldReader<u32>;
#[doc = "Field `SEED` writer - "]
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn seed(&mut self) -> SEED_W<RANDOM_SPEC> {
        SEED_W::new(self, 0)
    }
}
#[doc = "Loads a value to the LFSR randomiser  

You can [`read`](crate::Reg::read) this register and get [`random::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`random::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDOM_SPEC;
impl crate::RegisterSpec for RANDOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random::R`](R) reader structure"]
impl crate::Readable for RANDOM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`random::W`](W) writer structure"]
impl crate::Writable for RANDOM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDOM to value 0x3f04_b16d"]
impl crate::Resettable for RANDOM_SPEC {
    const RESET_VALUE: u32 = 0x3f04_b16d;
}
