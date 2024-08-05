#[doc = "Register `BOOT3` reader"]
pub type R = crate::R<BOOT3_SPEC>;
#[doc = "Register `BOOT3` writer"]
pub type W = crate::W<BOOT3_SPEC>;
#[doc = "Field `BOOT3` reader - "]
pub type BOOT3_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT3` writer - "]
pub type BOOT3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot3(&self) -> BOOT3_R {
        BOOT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn boot3(&mut self) -> BOOT3_W<BOOT3_SPEC> {
        BOOT3_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT3_SPEC;
impl crate::RegisterSpec for BOOT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot3::R`](R) reader structure"]
impl crate::Readable for BOOT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot3::W`](W) writer structure"]
impl crate::Writable for BOOT3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT3 to value 0"]
impl crate::Resettable for BOOT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
