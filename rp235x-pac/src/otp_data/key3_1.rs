#[doc = "Register `KEY3_1` reader"]
pub type R = crate::R<KEY3_1_SPEC>;
#[doc = "Field `KEY3_1` reader - "]
pub type KEY3_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key3_1(&self) -> KEY3_1_R {
        KEY3_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 31:16 of OTP access key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key3_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY3_1_SPEC;
impl crate::RegisterSpec for KEY3_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_1::R`](R) reader structure"]
impl crate::Readable for KEY3_1_SPEC {}
#[doc = "`reset()` method sets KEY3_1 to value 0"]
impl crate::Resettable for KEY3_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
