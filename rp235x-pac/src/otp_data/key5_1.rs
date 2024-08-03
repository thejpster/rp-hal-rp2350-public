#[doc = "Register `KEY5_1` reader"]
pub type R = crate::R<KEY5_1_SPEC>;
#[doc = "Field `KEY5_1` reader - "]
pub type KEY5_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key5_1(&self) -> KEY5_1_R {
        KEY5_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 31:16 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY5_1_SPEC;
impl crate::RegisterSpec for KEY5_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_1::R`](R) reader structure"]
impl crate::Readable for KEY5_1_SPEC {}
#[doc = "`reset()` method sets KEY5_1 to value 0"]
impl crate::Resettable for KEY5_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
