#[doc = "Register `BOOTKEY3_4` reader"]
pub type R = crate::R<BOOTKEY3_4_SPEC>;
#[doc = "Field `BOOTKEY3_4` reader - "]
pub type BOOTKEY3_4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey3_4(&self) -> BOOTKEY3_4_R {
        BOOTKEY3_4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_4_SPEC;
impl crate::RegisterSpec for BOOTKEY3_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey3_4::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_4_SPEC {}
#[doc = "`reset()` method sets BOOTKEY3_4 to value 0"]
impl crate::Resettable for BOOTKEY3_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
