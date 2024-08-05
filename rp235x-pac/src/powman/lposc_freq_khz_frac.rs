#[doc = "Register `LPOSC_FREQ_KHZ_FRAC` reader"]
pub type R = crate::R<LPOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Register `LPOSC_FREQ_KHZ_FRAC` writer"]
pub type W = crate::W<LPOSC_FREQ_KHZ_FRAC_SPEC>;
#[doc = "Field `LPOSC_FREQ_KHZ_FRAC` reader - Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
pub type LPOSC_FREQ_KHZ_FRAC_R = crate::FieldReader<u16>;
#[doc = "Field `LPOSC_FREQ_KHZ_FRAC` writer - Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
pub type LPOSC_FREQ_KHZ_FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub fn lposc_freq_khz_frac(&self) -> LPOSC_FREQ_KHZ_FRAC_R {
        LPOSC_FREQ_KHZ_FRAC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lposc_freq_khz_frac(&mut self) -> LPOSC_FREQ_KHZ_FRAC_W<LPOSC_FREQ_KHZ_FRAC_SPEC> {
        LPOSC_FREQ_KHZ_FRAC_W::new(self, 0)
    }
}
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC.  

You can [`read`](crate::Reg::read) this register and get [`lposc_freq_khz_frac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_freq_khz_frac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOSC_FREQ_KHZ_FRAC_SPEC;
impl crate::RegisterSpec for LPOSC_FREQ_KHZ_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposc_freq_khz_frac::R`](R) reader structure"]
impl crate::Readable for LPOSC_FREQ_KHZ_FRAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lposc_freq_khz_frac::W`](W) writer structure"]
impl crate::Writable for LPOSC_FREQ_KHZ_FRAC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOSC_FREQ_KHZ_FRAC to value 0xc49c"]
impl crate::Resettable for LPOSC_FREQ_KHZ_FRAC_SPEC {
    const RESET_VALUE: u32 = 0xc49c;
}
