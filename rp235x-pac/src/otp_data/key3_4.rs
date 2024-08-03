#[doc = "Register `KEY3_4` reader"]
pub type R = crate::R<KEY3_4_SPEC>;
#[doc = "Field `KEY3_4` reader - "]
pub type KEY3_4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key3_4(&self) -> KEY3_4_R {
        KEY3_4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 79:64 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY3_4_SPEC;
impl crate::RegisterSpec for KEY3_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_4::R`](R) reader structure"]
impl crate::Readable for KEY3_4_SPEC {}
#[doc = "`reset()` method sets KEY3_4 to value 0"]
impl crate::Resettable for KEY3_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
