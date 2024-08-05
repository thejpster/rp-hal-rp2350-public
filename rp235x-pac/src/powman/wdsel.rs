#[doc = "Register `WDSEL` reader"]
pub type R = crate::R<WDSEL_SPEC>;
#[doc = "Register `WDSEL` writer"]
pub type W = crate::W<WDSEL_SPEC>;
#[doc = "Field `RESET_POWMAN_ASYNC` reader - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
pub type RESET_POWMAN_ASYNC_R = crate::BitReader;
#[doc = "Field `RESET_POWMAN_ASYNC` writer - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
pub type RESET_POWMAN_ASYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_POWMAN` reader - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
pub type RESET_POWMAN_R = crate::BitReader;
#[doc = "Field `RESET_POWMAN` writer - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
pub type RESET_POWMAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_SWCORE` reader - If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
pub type RESET_SWCORE_R = crate::BitReader;
#[doc = "Field `RESET_SWCORE` writer - If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
pub type RESET_SWCORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_RSM` reader - If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
pub type RESET_RSM_R = crate::BitReader;
#[doc = "Field `RESET_RSM` writer - If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
pub type RESET_RSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
    #[inline(always)]
    pub fn reset_powman_async(&self) -> RESET_POWMAN_ASYNC_R {
        RESET_POWMAN_ASYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
    #[inline(always)]
    pub fn reset_powman(&self) -> RESET_POWMAN_R {
        RESET_POWMAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
    #[inline(always)]
    pub fn reset_swcore(&self) -> RESET_SWCORE_R {
        RESET_SWCORE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
    #[inline(always)]
    pub fn reset_rsm(&self) -> RESET_RSM_R {
        RESET_RSM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
    #[inline(always)]
    #[must_use]
    pub fn reset_powman_async(&mut self) -> RESET_POWMAN_ASYNC_W<WDSEL_SPEC> {
        RESET_POWMAN_ASYNC_W::new(self, 0)
    }
    #[doc = "Bit 4 - If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
    #[inline(always)]
    #[must_use]
    pub fn reset_powman(&mut self) -> RESET_POWMAN_W<WDSEL_SPEC> {
        RESET_POWMAN_W::new(self, 4)
    }
    #[doc = "Bit 8 - If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
    #[inline(always)]
    #[must_use]
    pub fn reset_swcore(&mut self) -> RESET_SWCORE_W<WDSEL_SPEC> {
        RESET_SWCORE_W::new(self, 8)
    }
    #[doc = "Bit 12 - If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
    #[inline(always)]
    #[must_use]
    pub fn reset_rsm(&mut self) -> RESET_RSM_W<WDSEL_SPEC> {
        RESET_RSM_W::new(self, 12)
    }
}
#[doc = "Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect.  

You can [`read`](crate::Reg::read) this register and get [`wdsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDSEL_SPEC;
impl crate::RegisterSpec for WDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdsel::R`](R) reader structure"]
impl crate::Readable for WDSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdsel::W`](W) writer structure"]
impl crate::Writable for WDSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDSEL to value 0"]
impl crate::Resettable for WDSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
