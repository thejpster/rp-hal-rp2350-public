#[doc = "Register `TRCIDR12` reader"]
pub type R = crate::R<TRCIDR12_SPEC>;
#[doc = "Register `TRCIDR12` writer"]
pub type W = crate::W<TRCIDR12_SPEC>;
#[doc = "Field `NUMCONDKEY` reader - reads as `ImpDef"]
pub type NUMCONDKEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcondkey(&self) -> NUMCONDKEY_R {
        NUMCONDKEY_R::new(self.bits)
    }
}
impl W {}
#[doc = "TRCIDR12  

You can [`read`](crate::Reg::read) this register and get [`trcidr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcidr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR12_SPEC;
impl crate::RegisterSpec for TRCIDR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr12::R`](R) reader structure"]
impl crate::Readable for TRCIDR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trcidr12::W`](W) writer structure"]
impl crate::Writable for TRCIDR12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCIDR12 to value 0x01"]
impl crate::Resettable for TRCIDR12_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
