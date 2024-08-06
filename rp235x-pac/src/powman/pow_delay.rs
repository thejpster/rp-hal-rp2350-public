#[doc = "Register `POW_DELAY` reader"]
pub type R = crate::R<POW_DELAY_SPEC>;
#[doc = "Register `POW_DELAY` writer"]
pub type W = crate::W<POW_DELAY_SPEC>;
#[doc = "Field `SWCORE_STEP` reader - timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
pub type SWCORE_STEP_R = crate::FieldReader;
#[doc = "Field `SWCORE_STEP` writer - timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
pub type SWCORE_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XIP_STEP` reader - timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
pub type XIP_STEP_R = crate::FieldReader;
#[doc = "Field `XIP_STEP` writer - timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
pub type XIP_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRAM_STEP` reader - timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
pub type SRAM_STEP_R = crate::FieldReader;
#[doc = "Field `SRAM_STEP` writer - timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
pub type SRAM_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn swcore_step(&self) -> SWCORE_STEP_R {
        SWCORE_STEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn xip_step(&self) -> XIP_STEP_R {
        XIP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn sram_step(&self) -> SRAM_STEP_R {
        SRAM_STEP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    #[must_use]
    pub fn swcore_step(&mut self) -> SWCORE_STEP_W<POW_DELAY_SPEC> {
        SWCORE_STEP_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    #[must_use]
    pub fn xip_step(&mut self) -> XIP_STEP_W<POW_DELAY_SPEC> {
        XIP_STEP_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
    #[inline(always)]
    #[must_use]
    pub fn sram_step(&mut self) -> SRAM_STEP_W<POW_DELAY_SPEC> {
        SRAM_STEP_W::new(self, 8)
    }
}
#[doc = "power state machine delays  

You can [`read`](crate::Reg::read) this register and get [`pow_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pow_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POW_DELAY_SPEC;
impl crate::RegisterSpec for POW_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pow_delay::R`](R) reader structure"]
impl crate::Readable for POW_DELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pow_delay::W`](W) writer structure"]
impl crate::Writable for POW_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POW_DELAY to value 0x2011"]
impl crate::Resettable for POW_DELAY_SPEC {
    const RESET_VALUE: u32 = 0x2011;
}
