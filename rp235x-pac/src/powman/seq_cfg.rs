#[doc = "Register `SEQ_CFG` reader"]
pub type R = crate::R<SEQ_CFG_SPEC>;
#[doc = "Register `SEQ_CFG` writer"]
pub type W = crate::W<SEQ_CFG_SPEC>;
#[doc = "Field `HW_PWRUP_SRAM1` reader - Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
pub type HW_PWRUP_SRAM1_R = crate::BitReader;
#[doc = "Field `HW_PWRUP_SRAM1` writer - Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
pub type HW_PWRUP_SRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_PWRUP_SRAM0` reader - Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
pub type HW_PWRUP_SRAM0_R = crate::BitReader;
#[doc = "Field `HW_PWRUP_SRAM0` writer - Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
pub type HW_PWRUP_SRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_VREG_LP` reader - Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
pub type USE_VREG_LP_R = crate::BitReader;
#[doc = "Field `USE_VREG_LP` writer - Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
pub type USE_VREG_LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_VREG_HP` reader - Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
pub type USE_VREG_HP_R = crate::BitReader;
#[doc = "Field `USE_VREG_HP` writer - Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
pub type USE_VREG_HP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_BOD_LP` reader - Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
pub type USE_BOD_LP_R = crate::BitReader;
#[doc = "Field `USE_BOD_LP` writer - Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
pub type USE_BOD_LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_BOD_HP` reader - Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
pub type USE_BOD_HP_R = crate::BitReader;
#[doc = "Field `USE_BOD_HP` writer - Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
pub type USE_BOD_HP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_LPOSC_IN_LP` reader - Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
pub type RUN_LPOSC_IN_LP_R = crate::BitReader;
#[doc = "Field `RUN_LPOSC_IN_LP` writer - Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
pub type RUN_LPOSC_IN_LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_FAST_POWCK` reader - selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
pub type USE_FAST_POWCK_R = crate::BitReader;
#[doc = "Field `USE_FAST_POWCK` writer - selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
pub type USE_FAST_POWCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USING_VREG_LP` reader - Indicates the voltage regulator (VREG) mode 0 = VREG high power mode which is the default 1 = VREG low power mode"]
pub type USING_VREG_LP_R = crate::BitReader;
#[doc = "Field `USING_BOD_LP` reader - Indicates the brown-out detector (BOD) mode 0 = BOD high power mode which is the default 1 = BOD low power mode"]
pub type USING_BOD_LP_R = crate::BitReader;
#[doc = "Field `USING_FAST_POWCK` reader - 0 indicates the POWMAN clock is running from the low power oscillator (32kHz) 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)"]
pub type USING_FAST_POWCK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub fn hw_pwrup_sram1(&self) -> HW_PWRUP_SRAM1_R {
        HW_PWRUP_SRAM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub fn hw_pwrup_sram0(&self) -> HW_PWRUP_SRAM0_R {
        HW_PWRUP_SRAM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn use_vreg_lp(&self) -> USE_VREG_LP_R {
        USE_VREG_LP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub fn use_vreg_hp(&self) -> USE_VREG_HP_R {
        USE_VREG_HP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn use_bod_lp(&self) -> USE_BOD_LP_R {
        USE_BOD_LP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub fn use_bod_hp(&self) -> USE_BOD_HP_R {
        USE_BOD_HP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn run_lposc_in_lp(&self) -> RUN_LPOSC_IN_LP_R {
        RUN_LPOSC_IN_LP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
    #[inline(always)]
    pub fn use_fast_powck(&self) -> USE_FAST_POWCK_R {
        USE_FAST_POWCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates the voltage regulator (VREG) mode 0 = VREG high power mode which is the default 1 = VREG low power mode"]
    #[inline(always)]
    pub fn using_vreg_lp(&self) -> USING_VREG_LP_R {
        USING_VREG_LP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates the brown-out detector (BOD) mode 0 = BOD high power mode which is the default 1 = BOD low power mode"]
    #[inline(always)]
    pub fn using_bod_lp(&self) -> USING_BOD_LP_R {
        USING_BOD_LP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - 0 indicates the POWMAN clock is running from the low power oscillator (32kHz) 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)"]
    #[inline(always)]
    pub fn using_fast_powck(&self) -> USING_FAST_POWCK_R {
        USING_FAST_POWCK_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    #[must_use]
    pub fn hw_pwrup_sram1(&mut self) -> HW_PWRUP_SRAM1_W<SEQ_CFG_SPEC> {
        HW_PWRUP_SRAM1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    #[must_use]
    pub fn hw_pwrup_sram0(&mut self) -> HW_PWRUP_SRAM0_W<SEQ_CFG_SPEC> {
        HW_PWRUP_SRAM0_W::new(self, 1)
    }
    #[doc = "Bit 4 - Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    #[must_use]
    pub fn use_vreg_lp(&mut self) -> USE_VREG_LP_W<SEQ_CFG_SPEC> {
        USE_VREG_LP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    #[must_use]
    pub fn use_vreg_hp(&mut self) -> USE_VREG_HP_W<SEQ_CFG_SPEC> {
        USE_VREG_HP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    #[must_use]
    pub fn use_bod_lp(&mut self) -> USE_BOD_LP_W<SEQ_CFG_SPEC> {
        USE_BOD_LP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    #[must_use]
    pub fn use_bod_hp(&mut self) -> USE_BOD_HP_W<SEQ_CFG_SPEC> {
        USE_BOD_HP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    #[must_use]
    pub fn run_lposc_in_lp(&mut self) -> RUN_LPOSC_IN_LP_W<SEQ_CFG_SPEC> {
        RUN_LPOSC_IN_LP_W::new(self, 8)
    }
    #[doc = "Bit 12 - selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
    #[inline(always)]
    #[must_use]
    pub fn use_fast_powck(&mut self) -> USE_FAST_POWCK_W<SEQ_CFG_SPEC> {
        USE_FAST_POWCK_W::new(self, 12)
    }
}
#[doc = "For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1  

You can [`read`](crate::Reg::read) this register and get [`seq_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_CFG_SPEC;
impl crate::RegisterSpec for SEQ_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_cfg::R`](R) reader structure"]
impl crate::Readable for SEQ_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_cfg::W`](W) writer structure"]
impl crate::Writable for SEQ_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_CFG to value 0x0010_11f0"]
impl crate::Resettable for SEQ_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0010_11f0;
}
