#[doc = "Register `TIMER` reader"]
pub type R = crate::R<TIMER_SPEC>;
#[doc = "Register `TIMER` writer"]
pub type W = crate::W<TIMER_SPEC>;
#[doc = "Field `NONSEC_WRITE` reader - Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
pub type NONSEC_WRITE_R = crate::BitReader;
#[doc = "Field `NONSEC_WRITE` writer - Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
pub type NONSEC_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN` reader - Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
pub type RUN_R = crate::BitReader;
#[doc = "Field `RUN` writer - Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
pub type RUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time."]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_ENAB` reader - Enables the alarm. The alarm must be disabled while writing the alarm time."]
pub type ALARM_ENAB_R = crate::BitReader;
#[doc = "Field `ALARM_ENAB` writer - Enables the alarm. The alarm must be disabled while writing the alarm time."]
pub type ALARM_ENAB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUP_ON_ALARM` reader - Alarm wakes the chip from low power mode"]
pub type PWRUP_ON_ALARM_R = crate::BitReader;
#[doc = "Field `PWRUP_ON_ALARM` writer - Alarm wakes the chip from low power mode"]
pub type PWRUP_ON_ALARM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM` reader - Alarm has fired. Write to 1 to clear the alarm."]
pub type ALARM_R = crate::BitReader;
#[doc = "Field `ALARM` writer - Alarm has fired. Write to 1 to clear the alarm."]
pub type ALARM_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `USE_LPOSC` writer - Switch to lposc as the source of the 1kHz timer tick"]
pub type USE_LPOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_XOSC` writer - switch to xosc as the source of the 1kHz timer tick"]
pub type USE_XOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_GPIO_1KHZ` writer - switch to gpio as the source of the 1kHz timer tick"]
pub type USE_GPIO_1KHZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_GPIO_1HZ` reader - Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
pub type USE_GPIO_1HZ_R = crate::BitReader;
#[doc = "Field `USE_GPIO_1HZ` writer - Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
pub type USE_GPIO_1HZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USING_XOSC` reader - Timer is running from xosc"]
pub type USING_XOSC_R = crate::BitReader;
#[doc = "Field `USING_LPOSC` reader - Timer is running from lposc"]
pub type USING_LPOSC_R = crate::BitReader;
#[doc = "Field `USING_GPIO_1KHZ` reader - Timer is running from a 1khz gpio source"]
pub type USING_GPIO_1KHZ_R = crate::BitReader;
#[doc = "Field `USING_GPIO_1HZ` reader - Timer is synchronised to a 1hz gpio source"]
pub type USING_GPIO_1HZ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
    #[inline(always)]
    pub fn nonsec_write(&self) -> NONSEC_WRITE_R {
        NONSEC_WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the alarm. The alarm must be disabled while writing the alarm time."]
    #[inline(always)]
    pub fn alarm_enab(&self) -> ALARM_ENAB_R {
        ALARM_ENAB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alarm wakes the chip from low power mode"]
    #[inline(always)]
    pub fn pwrup_on_alarm(&self) -> PWRUP_ON_ALARM_R {
        PWRUP_ON_ALARM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alarm has fired. Write to 1 to clear the alarm."]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
    #[inline(always)]
    pub fn use_gpio_1hz(&self) -> USE_GPIO_1HZ_R {
        USE_GPIO_1HZ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer is running from xosc"]
    #[inline(always)]
    pub fn using_xosc(&self) -> USING_XOSC_R {
        USING_XOSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer is running from lposc"]
    #[inline(always)]
    pub fn using_lposc(&self) -> USING_LPOSC_R {
        USING_LPOSC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer is running from a 1khz gpio source"]
    #[inline(always)]
    pub fn using_gpio_1khz(&self) -> USING_GPIO_1KHZ_R {
        USING_GPIO_1KHZ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer is synchronised to a 1hz gpio source"]
    #[inline(always)]
    pub fn using_gpio_1hz(&self) -> USING_GPIO_1HZ_R {
        USING_GPIO_1HZ_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn nonsec_write(&mut self) -> NONSEC_WRITE_W<TIMER_SPEC> {
        NONSEC_WRITE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RUN_W<TIMER_SPEC> {
        RUN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<TIMER_SPEC> {
        CLEAR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Enables the alarm. The alarm must be disabled while writing the alarm time."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_enab(&mut self) -> ALARM_ENAB_W<TIMER_SPEC> {
        ALARM_ENAB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Alarm wakes the chip from low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_on_alarm(&mut self) -> PWRUP_ON_ALARM_W<TIMER_SPEC> {
        PWRUP_ON_ALARM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Alarm has fired. Write to 1 to clear the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn alarm(&mut self) -> ALARM_W<TIMER_SPEC> {
        ALARM_W::new(self, 6)
    }
    #[doc = "Bit 8 - Switch to lposc as the source of the 1kHz timer tick"]
    #[inline(always)]
    #[must_use]
    pub fn use_lposc(&mut self) -> USE_LPOSC_W<TIMER_SPEC> {
        USE_LPOSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - switch to xosc as the source of the 1kHz timer tick"]
    #[inline(always)]
    #[must_use]
    pub fn use_xosc(&mut self) -> USE_XOSC_W<TIMER_SPEC> {
        USE_XOSC_W::new(self, 9)
    }
    #[doc = "Bit 10 - switch to gpio as the source of the 1kHz timer tick"]
    #[inline(always)]
    #[must_use]
    pub fn use_gpio_1khz(&mut self) -> USE_GPIO_1KHZ_W<TIMER_SPEC> {
        USE_GPIO_1KHZ_W::new(self, 10)
    }
    #[doc = "Bit 13 - Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
    #[inline(always)]
    #[must_use]
    pub fn use_gpio_1hz(&mut self) -> USE_GPIO_1HZ_W<TIMER_SPEC> {
        USE_GPIO_1HZ_W::new(self, 13)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer::R`](R) reader structure"]
impl crate::Readable for TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer::W`](W) writer structure"]
impl crate::Writable for TIMER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x40;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TIMER_SPEC {
    const RESET_VALUE: u32 = 0;
}
