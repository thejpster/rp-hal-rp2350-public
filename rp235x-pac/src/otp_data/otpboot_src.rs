#[doc = "Register `OTPBOOT_SRC` reader"]
pub type R = crate::R<OTPBOOT_SRC_SPEC>;
#[doc = "Field `OTPBOOT_SRC` reader - "]
pub type OTPBOOT_SRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_src(&self) -> OTPBOOT_SRC_R {
        OTPBOOT_SRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTP start row for the OTP boot image. (ECC)  

 If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.  

 This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_SRC_SPEC;
impl crate::RegisterSpec for OTPBOOT_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpboot_src::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_SRC_SPEC {}
#[doc = "`reset()` method sets OTPBOOT_SRC to value 0"]
impl crate::Resettable for OTPBOOT_SRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
