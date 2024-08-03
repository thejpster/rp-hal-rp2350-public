#[doc = "Register `KEY1_1` reader"]
pub type R = crate::R<KEY1_1_SPEC>;
#[doc = "Field `KEY1_1` reader - "]
pub type KEY1_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key1_1(&self) -> KEY1_1_R {
        KEY1_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 31:16 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_1_SPEC;
impl crate::RegisterSpec for KEY1_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_1::R`](R) reader structure"]
impl crate::Readable for KEY1_1_SPEC {}
#[doc = "`reset()` method sets KEY1_1 to value 0"]
impl crate::Resettable for KEY1_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
