#[doc = "Register `SET_TIME_15TO0` reader"]
pub type R = crate::R<SET_TIME_15TO0_SPEC>;
#[doc = "Register `SET_TIME_15TO0` writer"]
pub type W = crate::W<SET_TIME_15TO0_SPEC>;
#[doc = "Field `SET_TIME_15TO0` reader - For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
pub type SET_TIME_15TO0_R = crate::FieldReader<u16>;
#[doc = "Field `SET_TIME_15TO0` writer - For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
pub type SET_TIME_15TO0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_time_15to0(&self) -> SET_TIME_15TO0_R {
        SET_TIME_15TO0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    #[must_use]
    pub fn set_time_15to0(&mut self) -> SET_TIME_15TO0_W<SET_TIME_15TO0_SPEC> {
        SET_TIME_15TO0_W::new(self, 0)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`set_time_15to0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_time_15to0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_TIME_15TO0_SPEC;
impl crate::RegisterSpec for SET_TIME_15TO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set_time_15to0::R`](R) reader structure"]
impl crate::Readable for SET_TIME_15TO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set_time_15to0::W`](W) writer structure"]
impl crate::Writable for SET_TIME_15TO0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_TIME_15TO0 to value 0"]
impl crate::Resettable for SET_TIME_15TO0_SPEC {
    const RESET_VALUE: u32 = 0;
}
