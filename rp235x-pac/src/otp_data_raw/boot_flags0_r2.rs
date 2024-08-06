#[doc = "Register `BOOT_FLAGS0_R2` reader"]
pub type R = crate::R<BOOT_FLAGS0_R2_SPEC>;
#[doc = "Register `BOOT_FLAGS0_R2` writer"]
pub type W = crate::W<BOOT_FLAGS0_R2_SPEC>;
#[doc = "Field `BOOT_FLAGS0_R2` reader - "]
pub type BOOT_FLAGS0_R2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn boot_flags0_r2(&self) -> BOOT_FLAGS0_R2_R {
        BOOT_FLAGS0_R2_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Redundant copy of BOOT_FLAGS0  

You can [`read`](crate::Reg::read) this register and get [`boot_flags0_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_flags0_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_FLAGS0_R2_SPEC;
impl crate::RegisterSpec for BOOT_FLAGS0_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_flags0_r2::R`](R) reader structure"]
impl crate::Readable for BOOT_FLAGS0_R2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_flags0_r2::W`](W) writer structure"]
impl crate::Writable for BOOT_FLAGS0_R2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_FLAGS0_R2 to value 0"]
impl crate::Resettable for BOOT_FLAGS0_R2_SPEC {
    const RESET_VALUE: u32 = 0;
}
