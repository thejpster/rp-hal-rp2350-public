#[doc = "Register `XOSC_FREQ_KHZ_INT` reader"]
pub type R = crate::R<XOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Register `XOSC_FREQ_KHZ_INT` writer"]
pub type W = crate::W<XOSC_FREQ_KHZ_INT_SPEC>;
#[doc = "Field `XOSC_FREQ_KHZ_INT` reader - Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
pub type XOSC_FREQ_KHZ_INT_R = crate::FieldReader<u16>;
#[doc = "Field `XOSC_FREQ_KHZ_INT` writer - Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
pub type XOSC_FREQ_KHZ_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub fn xosc_freq_khz_int(&self) -> XOSC_FREQ_KHZ_INT_R {
        XOSC_FREQ_KHZ_INT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    #[must_use]
    pub fn xosc_freq_khz_int(&mut self) -> XOSC_FREQ_KHZ_INT_W<XOSC_FREQ_KHZ_INT_SPEC> {
        XOSC_FREQ_KHZ_INT_W::new(self, 0)
    }
}
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the XOSC.  

You can [`read`](crate::Reg::read) this register and get [`xosc_freq_khz_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc_freq_khz_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOSC_FREQ_KHZ_INT_SPEC;
impl crate::RegisterSpec for XOSC_FREQ_KHZ_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc_freq_khz_int::R`](R) reader structure"]
impl crate::Readable for XOSC_FREQ_KHZ_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xosc_freq_khz_int::W`](W) writer structure"]
impl crate::Writable for XOSC_FREQ_KHZ_INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XOSC_FREQ_KHZ_INT to value 0x2ee0"]
impl crate::Resettable for XOSC_FREQ_KHZ_INT_SPEC {
    const RESET_VALUE: u32 = 0x2ee0;
}
