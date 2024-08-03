#[doc = "Register `BOOTKEY2_4` reader"]
pub type R = crate::R<BOOTKEY2_4_SPEC>;
#[doc = "Field `BOOTKEY2_4` reader - "]
pub type BOOTKEY2_4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey2_4(&self) -> BOOTKEY2_4_R {
        BOOTKEY2_4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY2_4_SPEC;
impl crate::RegisterSpec for BOOTKEY2_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey2_4::R`](R) reader structure"]
impl crate::Readable for BOOTKEY2_4_SPEC {}
#[doc = "`reset()` method sets BOOTKEY2_4 to value 0"]
impl crate::Resettable for BOOTKEY2_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
