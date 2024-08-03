#[doc = "Register `TRCIDR8` reader"]
pub type R = crate::R<TRCIDR8_SPEC>;
#[doc = "Field `MAXSPEC` reader - reads as `ImpDef"]
pub type MAXSPEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reads as `ImpDef"]
    #[inline(always)]
    pub fn maxspec(&self) -> MAXSPEC_R {
        MAXSPEC_R::new(self.bits)
    }
}
#[doc = "TRCIDR8  

You can [`read`](crate::Reg::read) this register and get [`trcidr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCIDR8_SPEC;
impl crate::RegisterSpec for TRCIDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcidr8::R`](R) reader structure"]
impl crate::Readable for TRCIDR8_SPEC {}
#[doc = "`reset()` method sets TRCIDR8 to value 0"]
impl crate::Resettable for TRCIDR8_SPEC {
    const RESET_VALUE: u32 = 0;
}
