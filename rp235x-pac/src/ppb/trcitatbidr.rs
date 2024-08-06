#[doc = "Register `TRCITATBIDR` reader"]
pub type R = crate::R<TRCITATBIDR_SPEC>;
#[doc = "Register `TRCITATBIDR` writer"]
pub type W = crate::W<TRCITATBIDR_SPEC>;
#[doc = "Field `ID` reader - Trace ID"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - Trace ID"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<TRCITATBIDR_SPEC> {
        ID_W::new(self, 0)
    }
}
#[doc = "Trace Integration ATB Identification Register  

You can [`read`](crate::Reg::read) this register and get [`trcitatbidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitatbidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCITATBIDR_SPEC;
impl crate::RegisterSpec for TRCITATBIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitatbidr::R`](R) reader structure"]
impl crate::Readable for TRCITATBIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcitatbidr::W`](W) writer structure"]
impl crate::Writable for TRCITATBIDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCITATBIDR to value 0"]
impl crate::Resettable for TRCITATBIDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
