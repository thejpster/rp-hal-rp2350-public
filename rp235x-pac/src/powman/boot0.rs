#[doc = "Register `BOOT0` reader"]
pub type R = crate::R<BOOT0_SPEC>;
#[doc = "Register `BOOT0` writer"]
pub type W = crate::W<BOOT0_SPEC>;
#[doc = "Field `BOOT0` reader - "]
pub type BOOT0_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT0` writer - "]
pub type BOOT0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn boot0(&mut self) -> BOOT0_W<BOOT0_SPEC> {
        BOOT0_W::new(self, 0)
    }
}
#[doc = "Scratch register. Information persists in low power mode  

You can [`read`](crate::Reg::read) this register and get [`boot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT0_SPEC;
impl crate::RegisterSpec for BOOT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot0::R`](R) reader structure"]
impl crate::Readable for BOOT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot0::W`](W) writer structure"]
impl crate::Writable for BOOT0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT0 to value 0"]
impl crate::Resettable for BOOT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
