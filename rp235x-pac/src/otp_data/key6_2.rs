#[doc = "Register `KEY6_2` reader"]
pub type R = crate::R<KEY6_2_SPEC>;
#[doc = "Field `KEY6_2` reader - "]
pub type KEY6_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key6_2(&self) -> KEY6_2_R {
        KEY6_2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 47:32 of OTP access key 6 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key6_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY6_2_SPEC;
impl crate::RegisterSpec for KEY6_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6_2::R`](R) reader structure"]
impl crate::Readable for KEY6_2_SPEC {}
#[doc = "`reset()` method sets KEY6_2 to value 0"]
impl crate::Resettable for KEY6_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
