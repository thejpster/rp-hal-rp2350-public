#[doc = "Register `OTPBOOT_DST0` reader"]
pub type R = crate::R<OTPBOOT_DST0_SPEC>;
#[doc = "Field `OTPBOOT_DST0` reader - "]
pub type OTPBOOT_DST0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_dst0(&self) -> OTPBOOT_DST0_R {
        OTPBOOT_DST0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)  

 This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_DST0_SPEC;
impl crate::RegisterSpec for OTPBOOT_DST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpboot_dst0::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_DST0_SPEC {}
#[doc = "`reset()` method sets OTPBOOT_DST0 to value 0"]
impl crate::Resettable for OTPBOOT_DST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
