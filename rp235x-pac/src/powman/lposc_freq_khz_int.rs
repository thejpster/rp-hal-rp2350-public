#[doc = "Register `LPOSC_FREQ_KHZ_INT` reader"]
pub type R = crate::R<LPOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Register `LPOSC_FREQ_KHZ_INT` writer"]
pub type W = crate::W<LPOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Field `LPOSC_FREQ_KHZ_INT` reader - Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
pub type LPOSC_FREQ_KHZ_INT_R = crate::FieldReader;
#[doc = "Field `LPOSC_FREQ_KHZ_INT` writer - Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
pub type LPOSC_FREQ_KHZ_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub fn lposc_freq_khz_int(&self) -> LPOSC_FREQ_KHZ_INT_R {
        LPOSC_FREQ_KHZ_INT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lposc_freq_khz_int(&mut self) -> LPOSC_FREQ_KHZ_INT_W<LPOSC_FREQ_KHZ_INT_SPEC> {
        LPOSC_FREQ_KHZ_INT_W::new(self, 0)
    }
}
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC.  

You can [`read`](crate::Reg::read) this register and get [`lposc_freq_khz_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lposc_freq_khz_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPOSC_FREQ_KHZ_INT_SPEC;
impl crate::RegisterSpec for LPOSC_FREQ_KHZ_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lposc_freq_khz_int::R`](R) reader structure"]
impl crate::Readable for LPOSC_FREQ_KHZ_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lposc_freq_khz_int::W`](W) writer structure"]
impl crate::Writable for LPOSC_FREQ_KHZ_INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPOSC_FREQ_KHZ_INT to value 0x20"]
impl crate::Resettable for LPOSC_FREQ_KHZ_INT_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
