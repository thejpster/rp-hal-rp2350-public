#[doc = "Register `OTPBOOT_SRC` reader"]
pub type R = crate::R<OTPBOOT_SRC_SPEC>;
#[doc = "Register `OTPBOOT_SRC` writer"]
pub type W = crate::W<OTPBOOT_SRC_SPEC>;
#[doc = "Field `OTPBOOT_SRC` reader - "]
pub type OTPBOOT_SRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_src(&self) -> OTPBOOT_SRC_R {
        OTPBOOT_SRC_R::new(self.bits)
    }
}
impl W {}
#[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_SRC_SPEC;
impl crate::RegisterSpec for OTPBOOT_SRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`otpboot_src::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otpboot_src::W`](W) writer structure"]
impl crate::Writable for OTPBOOT_SRC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets OTPBOOT_SRC to value 0"]
impl crate::Resettable for OTPBOOT_SRC_SPEC {
    const RESET_VALUE: u16 = 0;
}
