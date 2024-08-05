#[doc = "Register `TRCIDR13` reader"]
pub type R = crate::R<TRCIDR13_SPEC>;
#[doc = "Register `TRCIDR13` writer"]
pub type W = crate::W<TRCIDR13_SPEC>;
#[doc = "Field `NUMCONDSPC` reader - reads as `ImpDef"]
pub type NUMCONDSPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcondspc(&self) -> NUMCONDSPC_R {
        NUMCONDSPC_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR13  

You can [`read`](crate::Reg::read) this register and get [`trcidr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR13_SPEC;
impl crate::RegisterSpec for TRCIDR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr13::R`](R) reader structure"]
impl crate::Readable for TRCIDR13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr13::W`](W) writer structure"]
impl crate::Writable for TRCIDR13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR13 to value 0"]
impl crate::Resettable for TRCIDR13_SPEC {
    const RESET_VALUE: u32 = 0;
}
