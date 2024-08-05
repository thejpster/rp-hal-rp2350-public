#[doc = "Register `BOOT2` reader"]
pub type R = crate::R<BOOT2_SPEC>;
#[doc = "Register `BOOT2` writer"]
pub type W = crate::W<BOOT2_SPEC>;
#[doc = "Field `BOOT2` reader - "]
pub type BOOT2_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT2` writer - "]
pub type BOOT2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn boot2(&mut self) -> BOOT2_W<BOOT2_SPEC> {
        BOOT2_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT2_SPEC;
impl crate::RegisterSpec for BOOT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot2::R`](R) reader structure"]
impl crate::Readable for BOOT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot2::W`](W) writer structure"]
impl crate::Writable for BOOT2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT2 to value 0"]
impl crate::Resettable for BOOT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
