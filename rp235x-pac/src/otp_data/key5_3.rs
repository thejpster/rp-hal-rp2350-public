#[doc = "Register `KEY5_3` reader"]
pub type R = crate::R<KEY5_3_SPEC>;
#[doc = "Field `KEY5_3` reader - "]
pub type KEY5_3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key5_3(&self) -> KEY5_3_R {
        KEY5_3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 63:48 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY5_3_SPEC;
impl crate::RegisterSpec for KEY5_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_3::R`](R) reader structure"]
impl crate::Readable for KEY5_3_SPEC {}
#[doc = "`reset()` method sets KEY5_3 to value 0"]
impl crate::Resettable for KEY5_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
