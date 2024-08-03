#[doc = "Register `DEFAULT_BOOT_VERSION0_R1` reader"]
pub type R = crate::R<DEFAULT_BOOT_VERSION0_R1_SPEC>;
#[doc = "Field `DEFAULT_BOOT_VERSION0_R1` reader - "]
pub type DEFAULT_BOOT_VERSION0_R1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn default_boot_version0_r1(&self) -> DEFAULT_BOOT_VERSION0_R1_R {
        DEFAULT_BOOT_VERSION0_R1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION0  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version0_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEFAULT_BOOT_VERSION0_R1_SPEC;
impl crate::RegisterSpec for DEFAULT_BOOT_VERSION0_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`default_boot_version0_r1::R`](R) reader structure"]
impl crate::Readable for DEFAULT_BOOT_VERSION0_R1_SPEC {}
#[doc = "`reset()` method sets DEFAULT_BOOT_VERSION0_R1 to value 0"]
impl crate::Resettable for DEFAULT_BOOT_VERSION0_R1_SPEC {
    const RESET_VALUE: u32 = 0;
}
