#[doc = "Register `KEY4_6` reader"]
pub type R = crate::R<KEY4_6_SPEC>;
#[doc = "Field `KEY4_6` reader - "]
pub type KEY4_6_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key4_6(&self) -> KEY4_6_R {
        KEY4_6_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 111:96 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY4_6_SPEC;
impl crate::RegisterSpec for KEY4_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4_6::R`](R) reader structure"]
impl crate::Readable for KEY4_6_SPEC {}
#[doc = "`reset()` method sets KEY4_6 to value 0"]
impl crate::Resettable for KEY4_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
