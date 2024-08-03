#[doc = "Register `BOOTKEY3_2` reader"]
pub type R = crate::R<BOOTKEY3_2_SPEC>;
#[doc = "Field `BOOTKEY3_2` reader - "]
pub type BOOTKEY3_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey3_2(&self) -> BOOTKEY3_2_R {
        BOOTKEY3_2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_2_SPEC;
impl crate::RegisterSpec for BOOTKEY3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey3_2::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_2_SPEC {}
#[doc = "`reset()` method sets BOOTKEY3_2 to value 0"]
impl crate::Resettable for BOOTKEY3_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
