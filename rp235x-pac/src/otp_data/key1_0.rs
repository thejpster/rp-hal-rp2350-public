#[doc = "Register `KEY1_0` reader"]
pub type R = crate::R<KEY1_0_SPEC>;
#[doc = "Field `KEY1_0` reader - "]
pub type KEY1_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key1_0(&self) -> KEY1_0_R {
        KEY1_0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 15:0 of OTP access key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key1_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY1_0_SPEC;
impl crate::RegisterSpec for KEY1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_0::R`](R) reader structure"]
impl crate::Readable for KEY1_0_SPEC {}
#[doc = "`reset()` method sets KEY1_0 to value 0"]
impl crate::Resettable for KEY1_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
