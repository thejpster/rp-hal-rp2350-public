#[doc = "Register `BOOTKEY2_5` reader"]
pub type R = crate::R<BOOTKEY2_5_SPEC>;
#[doc = "Field `BOOTKEY2_5` reader - "]
pub type BOOTKEY2_5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey2_5(&self) -> BOOTKEY2_5_R {
        BOOTKEY2_5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY2_5_SPEC;
impl crate::RegisterSpec for BOOTKEY2_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey2_5::R`](R) reader structure"]
impl crate::Readable for BOOTKEY2_5_SPEC {}
#[doc = "`reset()` method sets BOOTKEY2_5 to value 0"]
impl crate::Resettable for BOOTKEY2_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
