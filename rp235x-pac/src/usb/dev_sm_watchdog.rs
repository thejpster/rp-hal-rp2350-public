#[doc = "Register `DEV_SM_WATCHDOG` reader"]
pub type R = crate::R<DEV_SM_WATCHDOG_SPEC>;
#[doc = "Register `DEV_SM_WATCHDOG` writer"]
pub type W = crate::W<DEV_SM_WATCHDOG_SPEC>;
#[doc = "Field `LIMIT` reader - "]
pub type LIMIT_R = crate::FieldReader<u32>;
#[doc = "Field `LIMIT` writer - "]
pub type LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `ENABLE` reader - "]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - "]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Set to 1 to forcibly reset the device state machine on watchdog expiry"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Set to 1 to forcibly reset the device state machine on watchdog expiry"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRED` reader - "]
pub type FIRED_R = crate::BitReader;
#[doc = "Field `FIRED` writer - "]
pub type FIRED_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set to 1 to forcibly reset the device state machine on watchdog expiry"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fired(&self) -> FIRED_R {
        FIRED_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<DEV_SM_WATCHDOG_SPEC> {
        LIMIT_W::new(self, 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DEV_SM_WATCHDOG_SPEC> {
        ENABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set to 1 to forcibly reset the device state machine on watchdog expiry"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<DEV_SM_WATCHDOG_SPEC> {
        RESET_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn fired(&mut self) -> FIRED_W<DEV_SM_WATCHDOG_SPEC> {
        FIRED_W::new(self, 20)
    }
}
#[doc = "Watchdog that forces the device state machine to idle and raises an interrupt if the device stays in a state that isn't idle for the configured limit. The counter is reset on every state transition. Set limit while enable is low and then set the enable.  

You can [`read`](crate::Reg::read) this register and get [`dev_sm_watchdog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_sm_watchdog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEV_SM_WATCHDOG_SPEC;
impl crate::RegisterSpec for DEV_SM_WATCHDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_sm_watchdog::R`](R) reader structure"]
impl crate::Readable for DEV_SM_WATCHDOG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dev_sm_watchdog::W`](W) writer structure"]
impl crate::Writable for DEV_SM_WATCHDOG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_0000;
}
#[doc = "`reset()` method sets DEV_SM_WATCHDOG to value 0"]
impl crate::Resettable for DEV_SM_WATCHDOG_SPEC {
    const RESET_VALUE: u32 = 0;
}
