#[doc = "Register `AUXCTRL` reader"]
pub type R = crate::R<AUXCTRL_SPEC>;
#[doc = "Register `AUXCTRL` writer"]
pub type W = crate::W<AUXCTRL_SPEC>;
#[doc = "Field `AUXCTRL` reader - * Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
pub type AUXCTRL_R = crate::FieldReader;
#[doc = "Field `AUXCTRL` writer - * Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
pub type AUXCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - * Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
    #[inline(always)]
    pub fn auxctrl(&self) -> AUXCTRL_R {
        AUXCTRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - * Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
    #[inline(always)]
    #[must_use]
    pub fn auxctrl(&mut self) -> AUXCTRL_W<AUXCTRL_SPEC> {
        AUXCTRL_W::new(self, 0)
    }
}
#[doc = "Auxiliary system control register  

You can [`read`](crate::Reg::read) this register and get [`auxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUXCTRL_SPEC;
impl crate::RegisterSpec for AUXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxctrl::R`](R) reader structure"]
impl crate::Readable for AUXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`auxctrl::W`](W) writer structure"]
impl crate::Writable for AUXCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXCTRL to value 0"]
impl crate::Resettable for AUXCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
