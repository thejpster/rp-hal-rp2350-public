#[doc = "Register `STIR` reader"]
pub type R = crate::R<STIR_SPEC>;
#[doc = "Register `STIR` writer"]
pub type W = crate::W<STIR_SPEC>;
#[doc = "Field `INTID` reader - Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
pub type INTID_R = crate::FieldReader<u16>;
#[doc = "Field `INTID` writer - Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
pub type INTID_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
    #[inline(always)]
    #[must_use]
    pub fn intid(&mut self) -> INTID_W<STIR_SPEC> {
        INTID_W::new(self, 0)
    }
}
#[doc = "Provides a mechanism for software to generate an interrupt  

You can [`read`](crate::Reg::read) this register and get [`stir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STIR_SPEC;
impl crate::RegisterSpec for STIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stir::R`](R) reader structure"]
impl crate::Readable for STIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stir::W`](W) writer structure"]
impl crate::Writable for STIR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIR to value 0"]
impl crate::Resettable for STIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
