#[doc = "Register `LPOSC_CALIB` reader"]
pub type R = crate::R<LPOSC_CALIB_SPEC>;
#[doc = "Register `LPOSC_CALIB` writer"]
pub type W = crate::W<LPOSC_CALIB_SPEC>;
#[doc = "Field `LPOSC_CALIB` reader - "]
pub type LPOSC_CALIB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn lposc_calib(&self) -> LPOSC_CALIB_R {
        LPOSC_CALIB_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state.  

You can [`read`](crate::Reg::read) this register and get [`lposc_calib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_calib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOSC_CALIB_SPEC;
impl crate::RegisterSpec for LPOSC_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposc_calib::R`](R) reader structure"]
impl crate::Readable for LPOSC_CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lposc_calib::W`](W) writer structure"]
impl crate::Writable for LPOSC_CALIB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOSC_CALIB to value 0"]
impl crate::Resettable for LPOSC_CALIB_SPEC {
    const RESET_VALUE: u32 = 0;
}
