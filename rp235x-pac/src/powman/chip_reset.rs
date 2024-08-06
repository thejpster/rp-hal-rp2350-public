#[doc = "Register `CHIP_RESET` reader"]
pub type R = crate::R<CHIP_RESET_SPEC>;
#[doc = "Register `CHIP_RESET` writer"]
pub type W = crate::W<CHIP_RESET_SPEC>;
#[doc = "Field `DOUBLE_TAP` reader - This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
pub type DOUBLE_TAP_R = crate::BitReader;
#[doc = "Field `DOUBLE_TAP` writer - This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
pub type DOUBLE_TAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESCUE_FLAG` reader - This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
pub type RESCUE_FLAG_R = crate::BitReader;
#[doc = "Field `RESCUE_FLAG` writer - This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
pub type RESCUE_FLAG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HAD_POR` reader - Last reset was from the power-on reset This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_POR_R = crate::BitReader;
#[doc = "Field `HAD_BOR` reader - Last reset was from the brown-out detection block This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_BOR_R = crate::BitReader;
#[doc = "Field `HAD_RUN_LOW` reader - Last reset was from the RUN pin This resets: double_tap flag no DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_RUN_LOW_R = crate::BitReader;
#[doc = "Field `HAD_DP_RESET_REQ` reader - Last reset was an reset request from the arm debugger This resets: double_tap flag no DP no RPAP no rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_DP_RESET_REQ_R = crate::BitReader;
#[doc = "Field `HAD_RESCUE` reader - Last reset was a rescue reset from the debugger This resets: double_tap flag no DP no RPAP no rescue_flag no, it sets this flag timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_RESCUE_R = crate::BitReader;
#[doc = "Field `HAD_WATCHDOG_RESET_POWMAN_ASYNC` reader - Last reset was a watchdog timeout which was configured to reset the power manager asynchronously This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_WATCHDOG_RESET_POWMAN_ASYNC_R = crate::BitReader;
#[doc = "Field `HAD_WATCHDOG_RESET_POWMAN` reader - Last reset was a watchdog timeout which was configured to reset the power manager This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
pub type HAD_WATCHDOG_RESET_POWMAN_R = crate::BitReader;
#[doc = "Field `HAD_WATCHDOG_RESET_SWCORE` reader - Last reset was a watchdog timeout which was configured to reset the switched-core This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
pub type HAD_WATCHDOG_RESET_SWCORE_R = crate::BitReader;
#[doc = "Field `HAD_SWCORE_PD` reader - Last reset was a switched core powerdown This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
pub type HAD_SWCORE_PD_R = crate::BitReader;
#[doc = "Field `HAD_GLITCH_DETECT` reader - Last reset was due to a power supply glitch This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
pub type HAD_GLITCH_DETECT_R = crate::BitReader;
#[doc = "Field `HAD_HZD_SYS_RESET_REQ` reader - Last reset was a system reset from the hazard debugger This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
pub type HAD_HZD_SYS_RESET_REQ_R = crate::BitReader;
#[doc = "Field `HAD_WATCHDOG_RESET_RSM` reader - Last reset was a watchdog timeout which was configured to reset the power-on state machine This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
pub type HAD_WATCHDOG_RESET_RSM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
    #[inline(always)]
    pub fn double_tap(&self) -> DOUBLE_TAP_R {
        DOUBLE_TAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
    #[inline(always)]
    pub fn rescue_flag(&self) -> RESCUE_FLAG_R {
        RESCUE_FLAG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Last reset was from the power-on reset This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_por(&self) -> HAD_POR_R {
        HAD_POR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Last reset was from the brown-out detection block This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_bor(&self) -> HAD_BOR_R {
        HAD_BOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Last reset was from the RUN pin This resets: double_tap flag no DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_run_low(&self) -> HAD_RUN_LOW_R {
        HAD_RUN_LOW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Last reset was an reset request from the arm debugger This resets: double_tap flag no DP no RPAP no rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_dp_reset_req(&self) -> HAD_DP_RESET_REQ_R {
        HAD_DP_RESET_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Last reset was a rescue reset from the debugger This resets: double_tap flag no DP no RPAP no rescue_flag no, it sets this flag timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_rescue(&self) -> HAD_RESCUE_R {
        HAD_RESCUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Last reset was a watchdog timeout which was configured to reset the power manager asynchronously This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_watchdog_reset_powman_async(&self) -> HAD_WATCHDOG_RESET_POWMAN_ASYNC_R {
        HAD_WATCHDOG_RESET_POWMAN_ASYNC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Last reset was a watchdog timeout which was configured to reset the power manager This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_watchdog_reset_powman(&self) -> HAD_WATCHDOG_RESET_POWMAN_R {
        HAD_WATCHDOG_RESET_POWMAN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Last reset was a watchdog timeout which was configured to reset the switched-core This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_watchdog_reset_swcore(&self) -> HAD_WATCHDOG_RESET_SWCORE_R {
        HAD_WATCHDOG_RESET_SWCORE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Last reset was a switched core powerdown This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn had_swcore_pd(&self) -> HAD_SWCORE_PD_R {
        HAD_SWCORE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Last reset was due to a power supply glitch This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn had_glitch_detect(&self) -> HAD_GLITCH_DETECT_R {
        HAD_GLITCH_DETECT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Last reset was a system reset from the hazard debugger This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn had_hzd_sys_reset_req(&self) -> HAD_HZD_SYS_RESET_REQ_R {
        HAD_HZD_SYS_RESET_REQ_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Last reset was a watchdog timeout which was configured to reset the power-on state machine This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn had_watchdog_reset_rsm(&self) -> HAD_WATCHDOG_RESET_RSM_R {
        HAD_WATCHDOG_RESET_RSM_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
    #[inline(always)]
    #[must_use]
    pub fn double_tap(&mut self) -> DOUBLE_TAP_W<CHIP_RESET_SPEC> {
        DOUBLE_TAP_W::new(self, 0)
    }
    #[doc = "Bit 4 - This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
    #[inline(always)]
    #[must_use]
    pub fn rescue_flag(&mut self) -> RESCUE_FLAG_W<CHIP_RESET_SPEC> {
        RESCUE_FLAG_W::new(self, 4)
    }
}
#[doc = "Chip reset control and status  

You can [`read`](crate::Reg::read) this register and get [`chip_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIP_RESET_SPEC;
impl crate::RegisterSpec for CHIP_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_reset::R`](R) reader structure"]
impl crate::Readable for CHIP_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chip_reset::W`](W) writer structure"]
impl crate::Writable for CHIP_RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets CHIP_RESET to value 0"]
impl crate::Resettable for CHIP_RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
