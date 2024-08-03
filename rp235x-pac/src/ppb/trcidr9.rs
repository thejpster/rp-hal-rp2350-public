#[doc = "Register `TRCIDR9` reader"]
pub type R = crate::R<TRCIDR9_SPEC>;
#[doc = "Field `NUMP0KEY` reader - reads as `ImpDef"]
pub type NUMP0KEY_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn nump0key(&self) -> NUMP0KEY_R {
        NUMP0KEY_R::new(self.bits)
    }
}
#[doc = "TRCIDR9  

You can [`read`](crate::Reg::read) this register and get [`trcidr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR9_SPEC;
impl crate::RegisterSpec for TRCIDR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr9::R`](R) reader structure"]
impl crate::Readable for TRCIDR9_SPEC {}
#[doc = "`reset()` method sets TRCIDR9 to value 0"]
impl crate::Resettable for TRCIDR9_SPEC {
    const RESET_VALUE: u32 = 0;
}
