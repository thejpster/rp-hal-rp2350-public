#[doc = "Register `KEY_VALID` reader"]
pub type R = crate::R<KEY_VALID_SPEC>;
#[doc = "Field `KEY_VALID` reader - "]
pub type KEY_VALID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn key_valid(&self) -> KEY_VALID_R {
        KEY_VALID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Which keys were valid (enrolled) at boot time  

You can [`read`](crate::Reg::read) this register and get [`key_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_VALID_SPEC;
impl crate::RegisterSpec for KEY_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_valid::R`](R) reader structure"]
impl crate::Readable for KEY_VALID_SPEC {}
#[doc = "`reset()` method sets KEY_VALID to value 0"]
impl crate::Resettable for KEY_VALID_SPEC {
    const RESET_VALUE: u32 = 0;
}
