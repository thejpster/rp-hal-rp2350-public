#[doc = "Register `RANDID0` reader"]
pub type R = crate::R<RANDID0_SPEC>;
#[doc = "Register `RANDID0` writer"]
pub type W = crate::W<RANDID0_SPEC>;
#[doc = "Field `RANDID0` reader - "]
pub type RANDID0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn randid0(&self) -> RANDID0_R {
        RANDID0_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0.  

You can [`read`](crate::Reg::read) this register and get [`randid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID0_SPEC;
impl crate::RegisterSpec for RANDID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid0::R`](R) reader structure"]
impl crate::Readable for RANDID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid0::W`](W) writer structure"]
impl crate::Writable for RANDID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDID0 to value 0"]
impl crate::Resettable for RANDID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
