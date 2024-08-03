#[doc = "Register `BOOTKEY0_1` reader"]
pub type R = crate::R<BOOTKEY0_1_SPEC>;
#[doc = "Field `BOOTKEY0_1` reader - "]
pub type BOOTKEY0_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey0_1(&self) -> BOOTKEY0_1_R {
        BOOTKEY0_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY0_1_SPEC;
impl crate::RegisterSpec for BOOTKEY0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey0_1::R`](R) reader structure"]
impl crate::Readable for BOOTKEY0_1_SPEC {}
#[doc = "`reset()` method sets BOOTKEY0_1 to value 0"]
impl crate::Resettable for BOOTKEY0_1_SPEC {
    const RESET_VALUE: u32 = 0;
}