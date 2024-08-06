#[doc = "Register `TRCIDR11` reader"]
pub type R = crate::R<TRCIDR11_SPEC>;
#[doc = "Register `TRCIDR11` writer"]
pub type W = crate::W<TRCIDR11_SPEC>;
#[doc = "Field `NUMP1SPC` reader - reads as `ImpDef"]
pub type NUMP1SPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nump1spc(&self) -> NUMP1SPC_R {
        NUMP1SPC_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR11  

You can [`read`](crate::Reg::read) this register and get [`trcidr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR11_SPEC;
impl crate::RegisterSpec for TRCIDR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr11::R`](R) reader structure"]
impl crate::Readable for TRCIDR11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr11::W`](W) writer structure"]
impl crate::Writable for TRCIDR11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR11 to value 0"]
impl crate::Resettable for TRCIDR11_SPEC {
    const RESET_VALUE: u32 = 0;
}
