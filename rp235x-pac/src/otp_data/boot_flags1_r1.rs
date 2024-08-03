#[doc = "Register `BOOT_FLAGS1_R1` reader"]
pub type R = crate::R<BOOT_FLAGS1_R1_SPEC>;
#[doc = "Field `BOOT_FLAGS1_R1` reader - "]
pub type BOOT_FLAGS1_R1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn boot_flags1_r1(&self) -> BOOT_FLAGS1_R1_R {
        BOOT_FLAGS1_R1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS1  

You can [`read`](crate::Reg::read) this register and get [`boot_flags1_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS1_R1_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS1_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags1_r1::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS1_R1_SPEC {}
#[doc = "`reset()` method sets BOOT_FLAGS1_R1 to value 0"]
impl crate::Resettable for BOOT_FLAGS1_R1_SPEC {
    const RESET_VALUE: u32 = 0;
}
