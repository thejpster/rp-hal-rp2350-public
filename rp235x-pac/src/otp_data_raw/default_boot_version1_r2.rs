#[doc = "Register `DEFAULT_BOOT_VERSION1_R2` reader"]
pub type R = crate::R<DEFAULT_BOOT_VERSION1_R2_SPEC>;
#[doc = "Register `DEFAULT_BOOT_VERSION1_R2` writer"]
pub type W = crate::W<DEFAULT_BOOT_VERSION1_R2_SPEC>;
#[doc = "Field `DEFAULT_BOOT_VERSION1_R2` reader - "]
pub type DEFAULT_BOOT_VERSION1_R2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn default_boot_version1_r2(&self) -> DEFAULT_BOOT_VERSION1_R2_R {
        DEFAULT_BOOT_VERSION1_R2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION1  

You can [`read`](crate::Reg::read) this register and get [`default_boot_version1_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`default_boot_version1_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEFAULT_BOOT_VERSION1_R2_SPEC;
impl crate::RegisterSpec for DEFAULT_BOOT_VERSION1_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`default_boot_version1_r2::R`](R) reader structure"]
impl crate::Readable for DEFAULT_BOOT_VERSION1_R2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`default_boot_version1_r2::W`](W) writer structure"]
impl crate::Writable for DEFAULT_BOOT_VERSION1_R2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEFAULT_BOOT_VERSION1_R2 to value 0"]
impl crate::Resettable for DEFAULT_BOOT_VERSION1_R2_SPEC {
    const RESET_VALUE: u32 = 0;
}
