#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HFSR_SPEC>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HFSR_SPEC>;
#[doc = "Field `VECTTBL` reader - Indicates when a fault has occurred because of a vector table read error on exception processing"]
pub type VECTTBL_R = crate::BitReader;
#[doc = "Field `VECTTBL` writer - Indicates when a fault has occurred because of a vector table read error on exception processing"]
pub type VECTTBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCED` reader - Indicates that a fault with configurable priority has been escalated to a HardFault exception, because it could not be made active, because of priority, or because it was disabled"]
pub type FORCED_R = crate::BitReader;
#[doc = "Field `FORCED` writer - Indicates that a fault with configurable priority has been escalated to a HardFault exception, because it could not be made active, because of priority, or because it was disabled"]
pub type FORCED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGEVT` reader - Indicates when a Debug event has occurred"]
pub type DEBUGEVT_R = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - Indicates when a Debug event has occurred"]
pub type DEBUGEVT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Indicates when a fault has occurred because of a vector table read error on exception processing"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates that a fault with configurable priority has been escalated to a HardFault exception, because it could not be made active, because of priority, or because it was disabled"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates when a Debug event has occurred"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates when a fault has occurred because of a vector table read error on exception processing"]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VECTTBL_W<HFSR_SPEC> {
        VECTTBL_W::new(self, 1)
    }
    #[doc = "Bit 30 - Indicates that a fault with configurable priority has been escalated to a HardFault exception, because it could not be made active, because of priority, or because it was disabled"]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> FORCED_W<HFSR_SPEC> {
        FORCED_W::new(self, 30)
    }
    #[doc = "Bit 31 - Indicates when a Debug event has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DEBUGEVT_W<HFSR_SPEC> {
        DEBUGEVT_W::new(self, 31)
    }
}
#[doc = "Shows the cause of any HardFaults  

You can [`read`](crate::Reg::read) this register and get [`hfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
