#[doc = "Register `SLEEPCTRL` reader"]
pub type R = crate::R<SLEEPCTRL_SPEC>;
#[doc = "Register `SLEEPCTRL` writer"]
pub type W = crate::W<SLEEPCTRL_SPEC>;
#[doc = "Field `LIGHT_SLEEP` reader - By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
pub type LIGHT_SLEEP_R = crate::BitReader;
#[doc = "Field `LIGHT_SLEEP` writer - By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
pub type LIGHT_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WICENREQ` reader - Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
pub type WICENREQ_R = crate::BitReader;
#[doc = "Field `WICENREQ` writer - Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
pub type WICENREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WICENACK` reader - Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK."]
pub type WICENACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
    #[inline(always)]
    pub fn light_sleep(&self) -> LIGHT_SLEEP_R {
        LIGHT_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
    #[inline(always)]
    pub fn wicenreq(&self) -> WICENREQ_R {
        WICENREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK."]
    #[inline(always)]
    pub fn wicenack(&self) -> WICENACK_R {
        WICENACK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
    #[inline(always)]
    #[must_use]
    pub fn light_sleep(&mut self) -> LIGHT_SLEEP_W<SLEEPCTRL_SPEC> {
        LIGHT_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
    #[inline(always)]
    #[must_use]
    pub fn wicenreq(&mut self) -> WICENREQ_W<SLEEPCTRL_SPEC> {
        WICENREQ_W::new(self, 1)
    }
}
#[doc = "Nonstandard sleep control register  

You can [`read`](crate::Reg::read) this register and get [`sleepctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEPCTRL_SPEC;
impl crate::RegisterSpec for SLEEPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepctrl::R`](R) reader structure"]
impl crate::Readable for SLEEPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleepctrl::W`](W) writer structure"]
impl crate::Writable for SLEEPCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCTRL to value 0x02"]
impl crate::Resettable for SLEEPCTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
