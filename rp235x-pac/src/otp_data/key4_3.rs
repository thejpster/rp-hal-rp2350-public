#[doc = "Register `KEY4_3` reader"]
pub type R = crate::R<KEY4_3_SPEC>;
#[doc = "Field `KEY4_3` reader - "]
pub type KEY4_3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key4_3(&self) -> KEY4_3_R {
        KEY4_3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 63:48 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY4_3_SPEC;
impl crate::RegisterSpec for KEY4_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4_3::R`](R) reader structure"]
impl crate::Readable for KEY4_3_SPEC {}
#[doc = "`reset()` method sets KEY4_3 to value 0"]
impl crate::Resettable for KEY4_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
