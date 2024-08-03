#[doc = "Register `ITM_ITCTRL` reader"]
pub type R = crate::R<ITM_ITCTRL_SPEC>;
#[doc = "Register `ITM_ITCTRL` writer"]
pub type W = crate::W<ITM_ITCTRL_SPEC>;
#[doc = "Field `IME` reader - Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
pub type IME_R = crate::BitReader;
#[doc = "Field `IME` writer - Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
pub type IME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
    #[inline(always)]
    pub fn ime(&self) -> IME_R {
        IME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration mode enable bit - The possible values are: 0 - The trace unit is not in integration mode. 1 - The trace unit is in integration mode. This mode enables: A debug agent to perform topology detection. SoC test software to perform integration testing."]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> IME_W<ITM_ITCTRL_SPEC> {
        IME_W::new(self, 0)
    }
}
#[doc = "Integration Mode Control Register  

You can [`read`](crate::Reg::read) this register and get [`itm_itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_ITCTRL_SPEC;
impl crate::RegisterSpec for ITM_ITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_itctrl::R`](R) reader structure"]
impl crate::Readable for ITM_ITCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_itctrl::W`](W) writer structure"]
impl crate::Writable for ITM_ITCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_ITCTRL to value 0"]
impl crate::Resettable for ITM_ITCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
