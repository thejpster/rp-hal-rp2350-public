#[doc = "Register `ROSC_CALIB` reader"]
pub type R = crate::R<ROSC_CALIB_SPEC>;
#[doc = "Register `ROSC_CALIB` writer"]
pub type W = crate::W<ROSC_CALIB_SPEC>;
#[doc = "Field `ROSC_CALIB` reader - "]
pub type ROSC_CALIB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rosc_calib(&self) -> ROSC_CALIB_R {
        ROSC_CALIB_R::new(self.bits)
    }
}
impl W {}
#[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state.  

You can [`read`](crate::Reg::read) this register and get [`rosc_calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosc_calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROSC_CALIB_SPEC;
impl crate::RegisterSpec for ROSC_CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rosc_calib::R`](R) reader structure"]
impl crate::Readable for ROSC_CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rosc_calib::W`](W) writer structure"]
impl crate::Writable for ROSC_CALIB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ROSC_CALIB to value 0"]
impl crate::Resettable for ROSC_CALIB_SPEC {
    const RESET_VALUE: u16 = 0;
}
