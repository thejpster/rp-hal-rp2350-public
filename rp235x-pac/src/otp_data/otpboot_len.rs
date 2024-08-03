#[doc = "Register `OTPBOOT_LEN` reader"]
pub type R = crate::R<OTPBOOT_LEN_SPEC>;
#[doc = "Field `OTPBOOT_LEN` reader - "]
pub type OTPBOOT_LEN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_len(&self) -> OTPBOOT_LEN_R {
        OTPBOOT_LEN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Length in rows of the OTP boot image. (ECC)  

 OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).  

You can [`read`](crate::Reg::read) this register and get [`otpboot_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_LEN_SPEC;
impl crate::RegisterSpec for OTPBOOT_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpboot_len::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_LEN_SPEC {}
#[doc = "`reset()` method sets OTPBOOT_LEN to value 0"]
impl crate::Resettable for OTPBOOT_LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
