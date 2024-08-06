#[doc = "Register `TRCIDR10` reader"]
pub type R = crate::R<TRCIDR10_SPEC>;
#[doc = "Register `TRCIDR10` writer"]
pub type W = crate::W<TRCIDR10_SPEC>;
#[doc = "Field `NUMP1KEY` reader - reads as `ImpDef"]
pub type NUMP1KEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nump1key(&self) -> NUMP1KEY_R {
        NUMP1KEY_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR10  

You can [`read`](crate::Reg::read) this register and get [`trcidr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR10_SPEC;
impl crate::RegisterSpec for TRCIDR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr10::R`](R) reader structure"]
impl crate::Readable for TRCIDR10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr10::W`](W) writer structure"]
impl crate::Writable for TRCIDR10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR10 to value 0"]
impl crate::Resettable for TRCIDR10_SPEC {
    const RESET_VALUE: u32 = 0;
}
