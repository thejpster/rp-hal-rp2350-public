#[doc = "Register `OTPBOOT_LEN` reader"]
pub type R = crate::R<OTPBOOT_LEN_SPEC>;
#[doc = "Register `OTPBOOT_LEN` writer"]
pub type W = crate::W<OTPBOOT_LEN_SPEC>;
#[doc = "Field `OTPBOOT_LEN` reader - "]
pub type OTPBOOT_LEN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_len(&self) -> OTPBOOT_LEN_R {
        OTPBOOT_LEN_R::new(self.bits)
    }
}
impl W {}
#[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).  

You can [`read`](crate::Reg::read) this register and get [`otpboot_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_LEN_SPEC;
impl crate::RegisterSpec for OTPBOOT_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`otpboot_len::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otpboot_len::W`](W) writer structure"]
impl crate::Writable for OTPBOOT_LEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets OTPBOOT_LEN to value 0"]
impl crate::Resettable for OTPBOOT_LEN_SPEC {
    const RESET_VALUE: u16 = 0;
}
