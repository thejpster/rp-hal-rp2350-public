#[doc = "Register `TRCIDR12` reader"]
pub type R = crate::R<TRCIDR12_SPEC>;
#[doc = "Field `NUMCONDKEY` reader - reads as `ImpDef"]
pub type NUMCONDKEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn numcondkey(&self) -> NUMCONDKEY_R {
        NUMCONDKEY_R::new(self.bits)
    }
}
#[doc = "TRCIDR12  

You can [`read`](crate::Reg::read) this register and get [`trcidr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR12_SPEC;
impl crate::RegisterSpec for TRCIDR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr12::R`](R) reader structure"]
impl crate::Readable for TRCIDR12_SPEC {}
#[doc = "`reset()` method sets TRCIDR12 to value 0x01"]
impl crate::Resettable for TRCIDR12_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
