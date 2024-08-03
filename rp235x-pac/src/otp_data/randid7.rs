#[doc = "Register `RANDID7` reader"]
pub type R = crate::R<RANDID7_SPEC>;
#[doc = "Field `RANDID7` reader - "]
pub type RANDID7_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid7(&self) -> RANDID7_R {
        RANDID7_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID7_SPEC;
impl crate::RegisterSpec for RANDID7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid7::R`](R) reader structure"]
impl crate::Readable for RANDID7_SPEC {}
#[doc = "`reset()` method sets RANDID7 to value 0"]
impl crate::Resettable for RANDID7_SPEC {
    const RESET_VALUE: u32 = 0;
}
