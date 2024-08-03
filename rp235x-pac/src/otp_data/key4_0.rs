#[doc = "Register `KEY4_0` reader"]
pub type R = crate::R<KEY4_0_SPEC>;
#[doc = "Field `KEY4_0` reader - "]
pub type KEY4_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key4_0(&self) -> KEY4_0_R {
        KEY4_0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 15:0 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY4_0_SPEC;
impl crate::RegisterSpec for KEY4_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4_0::R`](R) reader structure"]
impl crate::Readable for KEY4_0_SPEC {}
#[doc = "`reset()` method sets KEY4_0 to value 0"]
impl crate::Resettable for KEY4_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
