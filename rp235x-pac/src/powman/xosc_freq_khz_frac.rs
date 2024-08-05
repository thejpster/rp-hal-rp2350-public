#[doc = "Register `XOSC_FREQ_KHZ_FRAC` reader"]
pub type R = crate::R<XOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Register `XOSC_FREQ_KHZ_FRAC` writer"]
pub type W = crate::W<XOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Field `XOSC_FREQ_KHZ_FRAC` reader - Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
pub type XOSC_FREQ_KHZ_FRAC_R = crate::FieldReader<u16>;
#[doc = "Field `XOSC_FREQ_KHZ_FRAC` writer - Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
pub type XOSC_FREQ_KHZ_FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub fn xosc_freq_khz_frac(&self) -> XOSC_FREQ_KHZ_FRAC_R {
        XOSC_FREQ_KHZ_FRAC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_freq_khz_frac(&mut self) -> XOSC_FREQ_KHZ_FRAC_W<XOSC_FREQ_KHZ_FRAC_SPEC> {
        XOSC_FREQ_KHZ_FRAC_W::new(self, 0)
    }
}
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC.  

You can [`read`](crate::Reg::read) this register and get [`xosc_freq_khz_frac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_freq_khz_frac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSC_FREQ_KHZ_FRAC_SPEC;
impl crate::RegisterSpec for XOSC_FREQ_KHZ_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc_freq_khz_frac::R`](R) reader structure"]
impl crate::Readable for XOSC_FREQ_KHZ_FRAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xosc_freq_khz_frac::W`](W) writer structure"]
impl crate::Writable for XOSC_FREQ_KHZ_FRAC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XOSC_FREQ_KHZ_FRAC to value 0"]
impl crate::Resettable for XOSC_FREQ_KHZ_FRAC_SPEC {
    const RESET_VALUE: u32 = 0;
}
