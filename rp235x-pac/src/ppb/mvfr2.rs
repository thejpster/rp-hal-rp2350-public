#[doc = "Register `MVFR2` reader"]
pub type R = crate::R<MVFR2_SPEC>;
#[doc = "Field `FPMISC` reader - Indicates support for miscellaneous FP features"]
pub type FPMISC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Indicates support for miscellaneous FP features"]
    #[inline(always)]
    pub fn fpmisc(&self) -> FPMISC_R {
        FPMISC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Describes the features provided by the Floating-point Extension  

You can [`read`](crate::Reg::read) this register and get [`mvfr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVFR2_SPEC;
impl crate::RegisterSpec for MVFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr2::R`](R) reader structure"]
impl crate::Readable for MVFR2_SPEC {}
#[doc = "`reset()` method sets MVFR2 to value 0x60"]
impl crate::Resettable for MVFR2_SPEC {
    const RESET_VALUE: u32 = 0x60;
}
