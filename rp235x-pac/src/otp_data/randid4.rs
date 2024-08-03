#[doc = "Register `RANDID4` reader"]
pub type R = crate::R<RANDID4_SPEC>;
#[doc = "Field `RANDID4` reader - "]
pub type RANDID4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid4(&self) -> RANDID4_R {
        RANDID4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID4_SPEC;
impl crate::RegisterSpec for RANDID4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid4::R`](R) reader structure"]
impl crate::Readable for RANDID4_SPEC {}
#[doc = "`reset()` method sets RANDID4 to value 0"]
impl crate::Resettable for RANDID4_SPEC {
    const RESET_VALUE: u32 = 0;
}
