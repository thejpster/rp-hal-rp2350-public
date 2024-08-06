#[doc = "Register `EXT_TIME_REF` reader"]
pub type R = crate::R<EXT_TIME_REF_SPEC>;
#[doc = "Register `EXT_TIME_REF` writer"]
pub type W = crate::W<EXT_TIME_REF_SPEC>;
#[doc = "Field `SOURCE_SEL` reader - 0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
pub type SOURCE_SEL_R = crate::FieldReader;
#[doc = "Field `SOURCE_SEL` writer - 0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
pub type SOURCE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_LPCK` reader - Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
pub type DRIVE_LPCK_R = crate::BitReader;
#[doc = "Field `DRIVE_LPCK` writer - Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
pub type DRIVE_LPCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
    #[inline(always)]
    pub fn source_sel(&self) -> SOURCE_SEL_R {
        SOURCE_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn drive_lpck(&self) -> DRIVE_LPCK_R {
        DRIVE_LPCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
    #[inline(always)]
    #[must_use]
    pub fn source_sel(&mut self) -> SOURCE_SEL_W<EXT_TIME_REF_SPEC> {
        SOURCE_SEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    #[must_use]
    pub fn drive_lpck(&mut self) -> DRIVE_LPCK_W<EXT_TIME_REF_SPEC> {
        DRIVE_LPCK_W::new(self, 4)
    }
}
#[doc = "Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register.  

You can [`read`](crate::Reg::read) this register and get [`ext_time_ref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_time_ref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_TIME_REF_SPEC;
impl crate::RegisterSpec for EXT_TIME_REF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_time_ref::R`](R) reader structure"]
impl crate::Readable for EXT_TIME_REF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_time_ref::W`](W) writer structure"]
impl crate::Writable for EXT_TIME_REF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_TIME_REF to value 0"]
impl crate::Resettable for EXT_TIME_REF_SPEC {
    const RESET_VALUE: u32 = 0;
}
