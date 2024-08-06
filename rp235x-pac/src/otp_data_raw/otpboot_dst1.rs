#[doc = "Register `OTPBOOT_DST1` reader"]
pub type R = crate::R<OTPBOOT_DST1_SPEC>;
#[doc = "Register `OTPBOOT_DST1` writer"]
pub type W = crate::W<OTPBOOT_DST1_SPEC>;
#[doc = "Field `OTPBOOT_DST1` reader - "]
pub type OTPBOOT_DST1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn otpboot_dst1(&self) -> OTPBOOT_DST1_R {
        OTPBOOT_DST1_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.  

You can [`read`](crate::Reg::read) this register and get [`otpboot_dst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpboot_dst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBOOT_DST1_SPEC;
impl crate::RegisterSpec for OTPBOOT_DST1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`otpboot_dst1::R`](R) reader structure"]
impl crate::Readable for OTPBOOT_DST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otpboot_dst1::W`](W) writer structure"]
impl crate::Writable for OTPBOOT_DST1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets OTPBOOT_DST1 to value 0"]
impl crate::Resettable for OTPBOOT_DST1_SPEC {
    const RESET_VALUE: u16 = 0;
}
