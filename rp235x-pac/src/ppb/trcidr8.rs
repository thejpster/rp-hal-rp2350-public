#[doc = "Register `TRCIDR8` reader"]
pub type R = crate::R<TRCIDR8_SPEC>;
#[doc = "Register `TRCIDR8` writer"]
pub type W = crate::W<TRCIDR8_SPEC>;
#[doc = "Field `MAXSPEC` reader - reads as `ImpDef"]
pub type MAXSPEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn maxspec(&self) -> MAXSPEC_R {
        MAXSPEC_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR8  

You can [`read`](crate::Reg::read) this register and get [`trcidr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR8_SPEC;
impl crate::RegisterSpec for TRCIDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr8::R`](R) reader structure"]
impl crate::Readable for TRCIDR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr8::W`](W) writer structure"]
impl crate::Writable for TRCIDR8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR8 to value 0"]
impl crate::Resettable for TRCIDR8_SPEC {
    const RESET_VALUE: u32 = 0;
}
