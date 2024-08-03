#[doc = "Register `RANDID3` reader"]
pub type R = crate::R<RANDID3_SPEC>;
#[doc = "Field `RANDID3` reader - "]
pub type RANDID3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid3(&self) -> RANDID3_R {
        RANDID3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "16 bits of random data sampled from the TRNG during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID3_SPEC;
impl crate::RegisterSpec for RANDID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid3::R`](R) reader structure"]
impl crate::Readable for RANDID3_SPEC {}
#[doc = "`reset()` method sets RANDID3 to value 0"]
impl crate::Resettable for RANDID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
