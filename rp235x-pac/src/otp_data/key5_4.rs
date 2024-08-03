#[doc = "Register `KEY5_4` reader"]
pub type R = crate::R<KEY5_4_SPEC>;
#[doc = "Field `KEY5_4` reader - "]
pub type KEY5_4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key5_4(&self) -> KEY5_4_R {
        KEY5_4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 79:64 of OTP access key 5 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key5_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY5_4_SPEC;
impl crate::RegisterSpec for KEY5_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_4::R`](R) reader structure"]
impl crate::Readable for KEY5_4_SPEC {}
#[doc = "`reset()` method sets KEY5_4 to value 0"]
impl crate::Resettable for KEY5_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
