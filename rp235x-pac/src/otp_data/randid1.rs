#[doc = "Register `RANDID1` reader"]
pub type R = crate::R<RANDID1_SPEC>;
#[doc = "Field `RANDID1` reader - "]
pub type RANDID1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid1(&self) -> RANDID1_R {
        RANDID1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID1_SPEC;
impl crate::RegisterSpec for RANDID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid1::R`](R) reader structure"]
impl crate::Readable for RANDID1_SPEC {}
#[doc = "`reset()` method sets RANDID1 to value 0"]
impl crate::Resettable for RANDID1_SPEC {
    const RESET_VALUE: u32 = 0;
}
