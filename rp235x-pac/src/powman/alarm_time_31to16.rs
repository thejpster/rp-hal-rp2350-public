#[doc = "Register `ALARM_TIME_31TO16` reader"]
pub type R = crate::R<ALARM_TIME_31TO16_SPEC>;
#[doc = "Register `ALARM_TIME_31TO16` writer"]
pub type W = crate::W<ALARM_TIME_31TO16_SPEC>;
#[doc = "Field `ALARM_TIME_31TO16` reader - This field must only be written when POWMAN_ALARM_ENAB=0"]
pub type ALARM_TIME_31TO16_R = crate::FieldReader<u16>;
#[doc = "Field `ALARM_TIME_31TO16` writer - This field must only be written when POWMAN_ALARM_ENAB=0"]
pub type ALARM_TIME_31TO16_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub fn alarm_time_31to16(&self) -> ALARM_TIME_31TO16_R {
        ALARM_TIME_31TO16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_time_31to16(&mut self) -> ALARM_TIME_31TO16_W<ALARM_TIME_31TO16_SPEC> {
        ALARM_TIME_31TO16_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`alarm_time_31to16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm_time_31to16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM_TIME_31TO16_SPEC;
impl crate::RegisterSpec for ALARM_TIME_31TO16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm_time_31to16::R`](R) reader structure"]
impl crate::Readable for ALARM_TIME_31TO16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm_time_31to16::W`](W) writer structure"]
impl crate::Writable for ALARM_TIME_31TO16_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM_TIME_31TO16 to value 0"]
impl crate::Resettable for ALARM_TIME_31TO16_SPEC {
    const RESET_VALUE: u32 = 0;
}
