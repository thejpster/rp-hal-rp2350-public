#[doc = "Register `ITCHOUT` reader"]
pub type R = crate::R<ITCHOUT_SPEC>;
#[doc = "Register `ITCHOUT` writer"]
pub type W = crate::W<ITCHOUT_SPEC>;
#[doc = "Field `CTCHOUT` reader - Sets the value of the ctichout outputs"]
pub type CTCHOUT_R = crate::FieldReader;
#[doc = "Field `CTCHOUT` writer - Sets the value of the ctichout outputs"]
pub type CTCHOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the value of the ctichout outputs"]
    #[inline(always)]
    pub fn ctchout(&self) -> CTCHOUT_R {
        CTCHOUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the value of the ctichout outputs"]
    #[inline(always)]
    #[must_use]
    pub fn ctchout(&mut self) -> CTCHOUT_W<ITCHOUT_SPEC> {
        CTCHOUT_W::new(self, 0)
    }
}
#[doc = "Integration Test Channel Output register  

You can [`read`](crate::Reg::read) this register and get [`itchout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itchout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCHOUT_SPEC;
impl crate::RegisterSpec for ITCHOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itchout::R`](R) reader structure"]
impl crate::Readable for ITCHOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itchout::W`](W) writer structure"]
impl crate::Writable for ITCHOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCHOUT to value 0"]
impl crate::Resettable for ITCHOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
