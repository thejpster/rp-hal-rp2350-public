#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `JTAG_TRSTN` reader - Resets the JTAG module. Active low."]
pub type JTAG_TRSTN_R = crate::BitReader;
#[doc = "Field `JTAG_TRSTN` writer - Resets the JTAG module. Active low."]
pub type JTAG_TRSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_FUNCSEL` reader - Multiplexes the JTAG ports onto GPIO0-3"]
pub type JTAG_FUNCSEL_R = crate::BitReader;
#[doc = "Field `JTAG_FUNCSEL` writer - Multiplexes the JTAG ports onto GPIO0-3"]
pub type JTAG_FUNCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWMAN_DBGMODE` reader - This prevents the power manager from powering down and resetting the switched-core power domain. It is intended for DFT and for debugging the power manager after the chip has booted. It cannot be used to force initial power on because it simultaneously deasserts the reset."]
pub type POWMAN_DBGMODE_R = crate::BitReader;
#[doc = "Field `POWMAN_DBGMODE` writer - This prevents the power manager from powering down and resetting the switched-core power domain. It is intended for DFT and for debugging the power manager after the chip has booted. It cannot be used to force initial power on because it simultaneously deasserts the reset."]
pub type POWMAN_DBGMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWMAN_DFT_PWRON` reader - Holds the power switches on for all domains. This is intended to keep the power on for DFT and debug, rather than for switching the power on. The power switches are not sequenced and the sudden demand for current could cause the always-on power domain to brown out. This register is in the always-on domain therefore chaos could ensue. It is recommended to use the DBG_POW_OVRD controls instead."]
pub type POWMAN_DFT_PWRON_R = crate::BitReader;
#[doc = "Field `POWMAN_DFT_PWRON` writer - Holds the power switches on for all domains. This is intended to keep the power on for DFT and debug, rather than for switching the power on. The power switches are not sequenced and the sudden demand for current could cause the always-on power domain to brown out. This register is in the always-on domain therefore chaos could ensue. It is recommended to use the DBG_POW_OVRD controls instead."]
pub type POWMAN_DFT_PWRON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWMAN_DFT_ISO_OFF` reader - Holds the isolation gates between power domains in the open state. This is intended to hold the gates open for DFT and power manager debug. It is not intended to force the isolation gates open. Use the overrides in DBG_POW_OVRD to force the isolation gates open or closed."]
pub type POWMAN_DFT_ISO_OFF_R = crate::BitReader;
#[doc = "Field `POWMAN_DFT_ISO_OFF` writer - Holds the isolation gates between power domains in the open state. This is intended to hold the gates open for DFT and power manager debug. It is not intended to force the isolation gates open. Use the overrides in DBG_POW_OVRD to force the isolation gates open or closed."]
pub type POWMAN_DFT_ISO_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOSC_STABLE_FRCE` reader - Allows the chip to start-up even though the Low Power Oscillator (LPOSC) is failing to set its stable flag. Initial power sequencing is clocked by LPOSC at around 32kHz but does not start until the LPOSC declares itself to be stable. If the LPOSC is otherwise working correctly the chip will boot when this bit is set. If the LPOSC is not working then DBG_FRCE_GPIO_LPCK must be set and an external clock provided."]
pub type LPOSC_STABLE_FRCE_R = crate::BitReader;
#[doc = "Field `LPOSC_STABLE_FRCE` writer - Allows the chip to start-up even though the Low Power Oscillator (LPOSC) is failing to set its stable flag. Initial power sequencing is clocked by LPOSC at around 32kHz but does not start until the LPOSC declares itself to be stable. If the LPOSC is otherwise working correctly the chip will boot when this bit is set. If the LPOSC is not working then DBG_FRCE_GPIO_LPCK must be set and an external clock provided."]
pub type LPOSC_STABLE_FRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_FRCE_GPIO_LPCK` reader - Allows chip start-up when the Low Power Oscillator (LPOSC) is inoperative or malfunctioning and also allows the initial power sequencing rate to be adjusted. Write to 1 to force the LPOSC output to be driven from a GPIO (gpio20 on 80-pin package, gpio34 on the 60-pin package). If the LPOSC is inoperative or malfunctioning it may also be necessary to set the LPOSC_STABLE_FRCE bit in this register. The user must provide a clock on the GPIO. For normal operation use a clock running at around 32kHz. Adjusting the frequency will speed up or slow down the initial power-up sequence."]
pub type DBG_FRCE_GPIO_LPCK_R = crate::BitReader;
#[doc = "Field `DBG_FRCE_GPIO_LPCK` writer - Allows chip start-up when the Low Power Oscillator (LPOSC) is inoperative or malfunctioning and also allows the initial power sequencing rate to be adjusted. Write to 1 to force the LPOSC output to be driven from a GPIO (gpio20 on 80-pin package, gpio34 on the 60-pin package). If the LPOSC is inoperative or malfunctioning it may also be necessary to set the LPOSC_STABLE_FRCE bit in this register. The user must provide a clock on the GPIO. For normal operation use a clock running at around 32kHz. Adjusting the frequency will speed up or slow down the initial power-up sequence."]
pub type DBG_FRCE_GPIO_LPCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE` reader - Unused"]
pub type SPARE_R = crate::BitReader;
#[doc = "Field `SPARE` writer - Unused"]
pub type SPARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESCUE_RESTART` reader - Allows debug of boot problems by restarting the chip with minimal boot code execution. Write to 1 to put the chip in reset then write to 0 to restart the chip with the rescue flag set. The rescue flag is in the POWMAN_CHIP_RESET register and is read by boot code. The rescue flag is cleared by writing 0 to POWMAN_CHIP_RESET_RESCUE_FLAG or by resetting the chip by any means other than RESCUE_RESTART."]
pub type RESCUE_RESTART_R = crate::BitReader;
#[doc = "Field `RESCUE_RESTART` writer - Allows debug of boot problems by restarting the chip with minimal boot code execution. Write to 1 to put the chip in reset then write to 0 to restart the chip with the rescue flag set. The rescue flag is in the POWMAN_CHIP_RESET register and is read by boot code. The rescue flag is cleared by writing 0 to POWMAN_CHIP_RESET_RESCUE_FLAG or by resetting the chip by any means other than RESCUE_RESTART."]
pub type RESCUE_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Resets the JTAG module. Active low."]
    #[inline(always)]
    pub fn jtag_trstn(&self) -> JTAG_TRSTN_R {
        JTAG_TRSTN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multiplexes the JTAG ports onto GPIO0-3"]
    #[inline(always)]
    pub fn jtag_funcsel(&self) -> JTAG_FUNCSEL_R {
        JTAG_FUNCSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This prevents the power manager from powering down and resetting the switched-core power domain. It is intended for DFT and for debugging the power manager after the chip has booted. It cannot be used to force initial power on because it simultaneously deasserts the reset."]
    #[inline(always)]
    pub fn powman_dbgmode(&self) -> POWMAN_DBGMODE_R {
        POWMAN_DBGMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Holds the power switches on for all domains. This is intended to keep the power on for DFT and debug, rather than for switching the power on. The power switches are not sequenced and the sudden demand for current could cause the always-on power domain to brown out. This register is in the always-on domain therefore chaos could ensue. It is recommended to use the DBG_POW_OVRD controls instead."]
    #[inline(always)]
    pub fn powman_dft_pwron(&self) -> POWMAN_DFT_PWRON_R {
        POWMAN_DFT_PWRON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Holds the isolation gates between power domains in the open state. This is intended to hold the gates open for DFT and power manager debug. It is not intended to force the isolation gates open. Use the overrides in DBG_POW_OVRD to force the isolation gates open or closed."]
    #[inline(always)]
    pub fn powman_dft_iso_off(&self) -> POWMAN_DFT_ISO_OFF_R {
        POWMAN_DFT_ISO_OFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Allows the chip to start-up even though the Low Power Oscillator (LPOSC) is failing to set its stable flag. Initial power sequencing is clocked by LPOSC at around 32kHz but does not start until the LPOSC declares itself to be stable. If the LPOSC is otherwise working correctly the chip will boot when this bit is set. If the LPOSC is not working then DBG_FRCE_GPIO_LPCK must be set and an external clock provided."]
    #[inline(always)]
    pub fn lposc_stable_frce(&self) -> LPOSC_STABLE_FRCE_R {
        LPOSC_STABLE_FRCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Allows chip start-up when the Low Power Oscillator (LPOSC) is inoperative or malfunctioning and also allows the initial power sequencing rate to be adjusted. Write to 1 to force the LPOSC output to be driven from a GPIO (gpio20 on 80-pin package, gpio34 on the 60-pin package). If the LPOSC is inoperative or malfunctioning it may also be necessary to set the LPOSC_STABLE_FRCE bit in this register. The user must provide a clock on the GPIO. For normal operation use a clock running at around 32kHz. Adjusting the frequency will speed up or slow down the initial power-up sequence."]
    #[inline(always)]
    pub fn dbg_frce_gpio_lpck(&self) -> DBG_FRCE_GPIO_LPCK_R {
        DBG_FRCE_GPIO_LPCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 30 - Unused"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Allows debug of boot problems by restarting the chip with minimal boot code execution. Write to 1 to put the chip in reset then write to 0 to restart the chip with the rescue flag set. The rescue flag is in the POWMAN_CHIP_RESET register and is read by boot code. The rescue flag is cleared by writing 0 to POWMAN_CHIP_RESET_RESCUE_FLAG or by resetting the chip by any means other than RESCUE_RESTART."]
    #[inline(always)]
    pub fn rescue_restart(&self) -> RESCUE_RESTART_R {
        RESCUE_RESTART_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resets the JTAG module. Active low."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_trstn(&mut self) -> JTAG_TRSTN_W<CTRL_SPEC> {
        JTAG_TRSTN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Multiplexes the JTAG ports onto GPIO0-3"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_funcsel(&mut self) -> JTAG_FUNCSEL_W<CTRL_SPEC> {
        JTAG_FUNCSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - This prevents the power manager from powering down and resetting the switched-core power domain. It is intended for DFT and for debugging the power manager after the chip has booted. It cannot be used to force initial power on because it simultaneously deasserts the reset."]
    #[inline(always)]
    #[must_use]
    pub fn powman_dbgmode(&mut self) -> POWMAN_DBGMODE_W<CTRL_SPEC> {
        POWMAN_DBGMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Holds the power switches on for all domains. This is intended to keep the power on for DFT and debug, rather than for switching the power on. The power switches are not sequenced and the sudden demand for current could cause the always-on power domain to brown out. This register is in the always-on domain therefore chaos could ensue. It is recommended to use the DBG_POW_OVRD controls instead."]
    #[inline(always)]
    #[must_use]
    pub fn powman_dft_pwron(&mut self) -> POWMAN_DFT_PWRON_W<CTRL_SPEC> {
        POWMAN_DFT_PWRON_W::new(self, 3)
    }
    #[doc = "Bit 4 - Holds the isolation gates between power domains in the open state. This is intended to hold the gates open for DFT and power manager debug. It is not intended to force the isolation gates open. Use the overrides in DBG_POW_OVRD to force the isolation gates open or closed."]
    #[inline(always)]
    #[must_use]
    pub fn powman_dft_iso_off(&mut self) -> POWMAN_DFT_ISO_OFF_W<CTRL_SPEC> {
        POWMAN_DFT_ISO_OFF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Allows the chip to start-up even though the Low Power Oscillator (LPOSC) is failing to set its stable flag. Initial power sequencing is clocked by LPOSC at around 32kHz but does not start until the LPOSC declares itself to be stable. If the LPOSC is otherwise working correctly the chip will boot when this bit is set. If the LPOSC is not working then DBG_FRCE_GPIO_LPCK must be set and an external clock provided."]
    #[inline(always)]
    #[must_use]
    pub fn lposc_stable_frce(&mut self) -> LPOSC_STABLE_FRCE_W<CTRL_SPEC> {
        LPOSC_STABLE_FRCE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Allows chip start-up when the Low Power Oscillator (LPOSC) is inoperative or malfunctioning and also allows the initial power sequencing rate to be adjusted. Write to 1 to force the LPOSC output to be driven from a GPIO (gpio20 on 80-pin package, gpio34 on the 60-pin package). If the LPOSC is inoperative or malfunctioning it may also be necessary to set the LPOSC_STABLE_FRCE bit in this register. The user must provide a clock on the GPIO. For normal operation use a clock running at around 32kHz. Adjusting the frequency will speed up or slow down the initial power-up sequence."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_frce_gpio_lpck(&mut self) -> DBG_FRCE_GPIO_LPCK_W<CTRL_SPEC> {
        DBG_FRCE_GPIO_LPCK_W::new(self, 6)
    }
    #[doc = "Bit 30 - Unused"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<CTRL_SPEC> {
        SPARE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Allows debug of boot problems by restarting the chip with minimal boot code execution. Write to 1 to put the chip in reset then write to 0 to restart the chip with the rescue flag set. The rescue flag is in the POWMAN_CHIP_RESET register and is read by boot code. The rescue flag is cleared by writing 0 to POWMAN_CHIP_RESET_RESCUE_FLAG or by resetting the chip by any means other than RESCUE_RESTART."]
    #[inline(always)]
    #[must_use]
    pub fn rescue_restart(&mut self) -> RESCUE_RESTART_W<CTRL_SPEC> {
        RESCUE_RESTART_W::new(self, 31)
    }
}
#[doc = "This register is primarily used for DFT but can also be used to overcome some power up problems. However, it should not be used to force power up of domains. Use DBG_POW_OVRD for that.  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
