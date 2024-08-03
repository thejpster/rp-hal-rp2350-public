#[doc = "Register `BOOT_FLAGS1_R2` reader"]
pub type R = crate::R<BOOT_FLAGS1_R2_SPEC>;
#[doc = "Field `BOOT_FLAGS1_R2` reader - "]
pub type BOOT_FLAGS1_R2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn boot_flags1_r2(&self) -> BOOT_FLAGS1_R2_R {
        BOOT_FLAGS1_R2_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS1  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS1_R2_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS1_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags1_r2::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS1_R2_SPEC {}
#[doc = "`reset()` method sets BOOT_FLAGS1_R2 to value 0"]
impl crate::Resettable for BOOT_FLAGS1_R2_SPEC {
    const RESET_VALUE: u32 = 0;
}
