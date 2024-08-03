#[doc = "Register `LPOSC_CALIB` reader"]
pub type R = crate::R<LPOSC_CALIB_SPEC>;
#[doc = "Field `LPOSC_CALIB` reader - "]
pub type LPOSC_CALIB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lposc_calib(&self) -> LPOSC_CALIB_R {
        LPOSC_CALIB_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Low-power oscillator calibration constant measured during manufacturing (ECC)  

You can [`read`](crate::Reg::read) this register and get [`lposc_calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOSC_CALIB_SPEC;
impl crate::RegisterSpec for LPOSC_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposc_calib::R`](R) reader structure"]
impl crate::Readable for LPOSC_CALIB_SPEC {}
#[doc = "`reset()` method sets LPOSC_CALIB to value 0"]
impl crate::Resettable for LPOSC_CALIB_SPEC {
    const RESET_VALUE: u32 = 0;
}
