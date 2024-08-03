#[doc = "Register `ROSC_CALIB` reader"]
pub type R = crate::R<ROSC_CALIB_SPEC>;
#[doc = "Field `ROSC_CALIB` reader - "]
pub type ROSC_CALIB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rosc_calib(&self) -> ROSC_CALIB_R {
        ROSC_CALIB_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Ring oscillator calibration constant measured during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`rosc_calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROSC_CALIB_SPEC;
impl crate::RegisterSpec for ROSC_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosc_calib::R`](R) reader structure"]
impl crate::Readable for ROSC_CALIB_SPEC {}
#[doc = "`reset()` method sets ROSC_CALIB to value 0"]
impl crate::Resettable for ROSC_CALIB_SPEC {
    const RESET_VALUE: u32 = 0;
}
