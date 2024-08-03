#[doc = "Register `BOOTKEY1_8` reader"]
pub type R = crate::R<BOOTKEY1_8_SPEC>;
#[doc = "Field `BOOTKEY1_8` reader - "]
pub type BOOTKEY1_8_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey1_8(&self) -> BOOTKEY1_8_R {
        BOOTKEY1_8_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY1_8_SPEC;
impl crate::RegisterSpec for BOOTKEY1_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey1_8::R`](R) reader structure"]
impl crate::Readable for BOOTKEY1_8_SPEC {}
#[doc = "`reset()` method sets BOOTKEY1_8 to value 0"]
impl crate::Resettable for BOOTKEY1_8_SPEC {
    const RESET_VALUE: u32 = 0;
}
