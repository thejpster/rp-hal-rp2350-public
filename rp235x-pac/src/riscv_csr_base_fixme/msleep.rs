#[doc = "Register `MSLEEP` reader"]
pub type R = crate::R<MSLEEP_SPEC>;
#[doc = "Register `MSLEEP` writer"]
pub type W = crate::W<MSLEEP_SPEC>;
#[doc = "Field `DEEPSLEEP` reader - Deassert the processor clock enable when entering the sleep state. If a clock gate is instantiated, this allows most of the processor (everything except the power state machine and the interrupt and halt input registers) to be clock gated whilst asleep, which may reduce the sleep current. This adds one cycle to the wakeup latency."]
pub type DEEPSLEEP_R = crate::BitReader;
#[doc = "Field `DEEPSLEEP` writer - Deassert the processor clock enable when entering the sleep state. If a clock gate is instantiated, this allows most of the processor (everything except the power state machine and the interrupt and halt input registers) to be clock gated whilst asleep, which may reduce the sleep current. This adds one cycle to the wakeup latency."]
pub type DEEPSLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWERDOWN` reader - Release the external power request when going to sleep. The function of this is platform-defined -- it may do nothing, it may do something simple like clock-gating the fabric, or it may be tied to some complex system-level power controller.  

 When waking, the processor reasserts its external power-up request, and will not fetch any instructions until the request is acknowledged. This may add considerable latency to the wakeup."]
pub type POWERDOWN_R = crate::BitReader;
#[doc = "Field `POWERDOWN` writer - Release the external power request when going to sleep. The function of this is platform-defined -- it may do nothing, it may do something simple like clock-gating the fabric, or it may be tied to some complex system-level power controller.  

 When waking, the processor reasserts its external power-up request, and will not fetch any instructions until the request is acknowledged. This may add considerable latency to the wakeup."]
pub type POWERDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPONBLOCK` reader - Enter the deep sleep state configured by msleep.deepsleep/msleep.powerdown on a `h3.block` instruction, as well as a standard `wfi`. If this bit is clear, a `h3.block` is always implemented as a simple pipeline stall."]
pub type SLEEPONBLOCK_R = crate::BitReader;
#[doc = "Field `SLEEPONBLOCK` writer - Enter the deep sleep state configured by msleep.deepsleep/msleep.powerdown on a `h3.block` instruction, as well as a standard `wfi`. If this bit is clear, a `h3.block` is always implemented as a simple pipeline stall."]
pub type SLEEPONBLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Deassert the processor clock enable when entering the sleep state. If a clock gate is instantiated, this allows most of the processor (everything except the power state machine and the interrupt and halt input registers) to be clock gated whilst asleep, which may reduce the sleep current. This adds one cycle to the wakeup latency."]
    #[inline(always)]
    pub fn deepsleep(&self) -> DEEPSLEEP_R {
        DEEPSLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Release the external power request when going to sleep. The function of this is platform-defined -- it may do nothing, it may do something simple like clock-gating the fabric, or it may be tied to some complex system-level power controller.  

 When waking, the processor reasserts its external power-up request, and will not fetch any instructions until the request is acknowledged. This may add considerable latency to the wakeup."]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enter the deep sleep state configured by msleep.deepsleep/msleep.powerdown on a `h3.block` instruction, as well as a standard `wfi`. If this bit is clear, a `h3.block` is always implemented as a simple pipeline stall."]
    #[inline(always)]
    pub fn sleeponblock(&self) -> SLEEPONBLOCK_R {
        SLEEPONBLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deassert the processor clock enable when entering the sleep state. If a clock gate is instantiated, this allows most of the processor (everything except the power state machine and the interrupt and halt input registers) to be clock gated whilst asleep, which may reduce the sleep current. This adds one cycle to the wakeup latency."]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep(&mut self) -> DEEPSLEEP_W<MSLEEP_SPEC> {
        DEEPSLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Release the external power request when going to sleep. The function of this is platform-defined -- it may do nothing, it may do something simple like clock-gating the fabric, or it may be tied to some complex system-level power controller.  

 When waking, the processor reasserts its external power-up request, and will not fetch any instructions until the request is acknowledged. This may add considerable latency to the wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> POWERDOWN_W<MSLEEP_SPEC> {
        POWERDOWN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enter the deep sleep state configured by msleep.deepsleep/msleep.powerdown on a `h3.block` instruction, as well as a standard `wfi`. If this bit is clear, a `h3.block` is always implemented as a simple pipeline stall."]
    #[inline(always)]
    #[must_use]
    pub fn sleeponblock(&mut self) -> SLEEPONBLOCK_W<MSLEEP_SPEC> {
        SLEEPONBLOCK_W::new(self, 2)
    }
}
#[doc = "M-mode sleep control register  

You can [`read`](crate::Reg::read) this register and get [`msleep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSLEEP_SPEC;
impl crate::RegisterSpec for MSLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msleep::R`](R) reader structure"]
impl crate::Readable for MSLEEP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msleep::W`](W) writer structure"]
impl crate::Writable for MSLEEP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSLEEP to value 0"]
impl crate::Resettable for MSLEEP_SPEC {
    const RESET_VALUE: u32 = 0;
}
