#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<SHCSR_SPEC>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<SHCSR_SPEC>;
#[doc = "Field `MEMFAULTACT` reader - `IAAMO the active state of the MemManage exception `FTSSS"]
pub type MEMFAULTACT_R = crate::BitReader;
#[doc = "Field `MEMFAULTACT` writer - `IAAMO the active state of the MemManage exception `FTSSS"]
pub type MEMFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTACT` reader - `IAAMO the active state of the BusFault exception"]
pub type BUSFAULTACT_R = crate::BitReader;
#[doc = "Field `BUSFAULTACT` writer - `IAAMO the active state of the BusFault exception"]
pub type BUSFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDFAULTACT` reader - Indicates and allows limited modification of the active state of the HardFault exception `FTSSS"]
pub type HARDFAULTACT_R = crate::BitReader;
#[doc = "Field `HARDFAULTACT` writer - Indicates and allows limited modification of the active state of the HardFault exception `FTSSS"]
pub type HARDFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTACT` reader - `IAAMO the active state of the UsageFault exception `FTSSS"]
pub type USGFAULTACT_R = crate::BitReader;
#[doc = "Field `USGFAULTACT` writer - `IAAMO the active state of the UsageFault exception `FTSSS"]
pub type USGFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECUREFAULTACT` reader - `IAAMO the active state of the SecureFault exception"]
pub type SECUREFAULTACT_R = crate::BitReader;
#[doc = "Field `SECUREFAULTACT` writer - `IAAMO the active state of the SecureFault exception"]
pub type SECUREFAULTACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIACT` reader - `IAAMO the active state of the NMI exception"]
pub type NMIACT_R = crate::BitReader;
#[doc = "Field `NMIACT` writer - `IAAMO the active state of the NMI exception"]
pub type NMIACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLACT` reader - `IAAMO the active state of the SVCall exception `FTSSS"]
pub type SVCALLACT_R = crate::BitReader;
#[doc = "Field `SVCALLACT` writer - `IAAMO the active state of the SVCall exception `FTSSS"]
pub type SVCALLACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONITORACT` reader - `IAAMO the active state of the DebugMonitor exception"]
pub type MONITORACT_R = crate::BitReader;
#[doc = "Field `MONITORACT` writer - `IAAMO the active state of the DebugMonitor exception"]
pub type MONITORACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVACT` reader - `IAAMO the active state of the PendSV exception `FTSSS"]
pub type PENDSVACT_R = crate::BitReader;
#[doc = "Field `PENDSVACT` writer - `IAAMO the active state of the PendSV exception `FTSSS"]
pub type PENDSVACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTICKACT` reader - `IAAMO the active state of the SysTick exception `FTSSS"]
pub type SYSTICKACT_R = crate::BitReader;
#[doc = "Field `SYSTICKACT` writer - `IAAMO the active state of the SysTick exception `FTSSS"]
pub type SYSTICKACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTPENDED` reader - The UsageFault exception is banked between Security states, `IAAMO the pending state of the UsageFault exception `FTSSS"]
pub type USGFAULTPENDED_R = crate::BitReader;
#[doc = "Field `USGFAULTPENDED` writer - The UsageFault exception is banked between Security states, `IAAMO the pending state of the UsageFault exception `FTSSS"]
pub type USGFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTPENDED` reader - `IAAMO the pending state of the MemManage exception `FTSSS"]
pub type MEMFAULTPENDED_R = crate::BitReader;
#[doc = "Field `MEMFAULTPENDED` writer - `IAAMO the pending state of the MemManage exception `FTSSS"]
pub type MEMFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTPENDED` reader - `IAAMO the pending state of the BusFault exception"]
pub type BUSFAULTPENDED_R = crate::BitReader;
#[doc = "Field `BUSFAULTPENDED` writer - `IAAMO the pending state of the BusFault exception"]
pub type BUSFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVCALLPENDED` reader - `IAAMO the pending state of the SVCall exception `FTSSS"]
pub type SVCALLPENDED_R = crate::BitReader;
#[doc = "Field `SVCALLPENDED` writer - `IAAMO the pending state of the SVCall exception `FTSSS"]
pub type SVCALLPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMFAULTENA` reader - `DW the MemManage exception is enabled `FTSSS"]
pub type MEMFAULTENA_R = crate::BitReader;
#[doc = "Field `MEMFAULTENA` writer - `DW the MemManage exception is enabled `FTSSS"]
pub type MEMFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFAULTENA` reader - `DW the BusFault exception is enabled"]
pub type BUSFAULTENA_R = crate::BitReader;
#[doc = "Field `BUSFAULTENA` writer - `DW the BusFault exception is enabled"]
pub type BUSFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USGFAULTENA` reader - `DW the UsageFault exception is enabled `FTSSS"]
pub type USGFAULTENA_R = crate::BitReader;
#[doc = "Field `USGFAULTENA` writer - `DW the UsageFault exception is enabled `FTSSS"]
pub type USGFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECUREFAULTENA` reader - `DW the SecureFault exception is enabled"]
pub type SECUREFAULTENA_R = crate::BitReader;
#[doc = "Field `SECUREFAULTENA` writer - `DW the SecureFault exception is enabled"]
pub type SECUREFAULTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECUREFAULTPENDED` reader - `IAAMO the pending state of the SecureFault exception"]
pub type SECUREFAULTPENDED_R = crate::BitReader;
#[doc = "Field `SECUREFAULTPENDED` writer - `IAAMO the pending state of the SecureFault exception"]
pub type SECUREFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARDFAULTPENDED` reader - `IAAMO the pending state of the HardFault exception `CTTSSS"]
pub type HARDFAULTPENDED_R = crate::BitReader;
#[doc = "Field `HARDFAULTPENDED` writer - `IAAMO the pending state of the HardFault exception `CTTSSS"]
pub type HARDFAULTPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `IAAMO the active state of the MemManage exception `FTSSS"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `IAAMO the active state of the BusFault exception"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates and allows limited modification of the active state of the HardFault exception `FTSSS"]
    #[inline(always)]
    pub fn hardfaultact(&self) -> HARDFAULTACT_R {
        HARDFAULTACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `IAAMO the active state of the UsageFault exception `FTSSS"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `IAAMO the active state of the SecureFault exception"]
    #[inline(always)]
    pub fn securefaultact(&self) -> SECUREFAULTACT_R {
        SECUREFAULTACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `IAAMO the active state of the NMI exception"]
    #[inline(always)]
    pub fn nmiact(&self) -> NMIACT_R {
        NMIACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - `IAAMO the active state of the SVCall exception `FTSSS"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `IAAMO the active state of the DebugMonitor exception"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - `IAAMO the active state of the PendSV exception `FTSSS"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `IAAMO the active state of the SysTick exception `FTSSS"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The UsageFault exception is banked between Security states, `IAAMO the pending state of the UsageFault exception `FTSSS"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `IAAMO the pending state of the MemManage exception `FTSSS"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `IAAMO the pending state of the BusFault exception"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `IAAMO the pending state of the SVCall exception `FTSSS"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - `DW the MemManage exception is enabled `FTSSS"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - `DW the BusFault exception is enabled"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - `DW the UsageFault exception is enabled `FTSSS"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - `DW the SecureFault exception is enabled"]
    #[inline(always)]
    pub fn securefaultena(&self) -> SECUREFAULTENA_R {
        SECUREFAULTENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - `IAAMO the pending state of the SecureFault exception"]
    #[inline(always)]
    pub fn securefaultpended(&self) -> SECUREFAULTPENDED_R {
        SECUREFAULTPENDED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - `IAAMO the pending state of the HardFault exception `CTTSSS"]
    #[inline(always)]
    pub fn hardfaultpended(&self) -> HARDFAULTPENDED_R {
        HARDFAULTPENDED_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `IAAMO the active state of the MemManage exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W<SHCSR_SPEC> {
        MEMFAULTACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - `IAAMO the active state of the BusFault exception"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W<SHCSR_SPEC> {
        BUSFAULTACT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates and allows limited modification of the active state of the HardFault exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn hardfaultact(&mut self) -> HARDFAULTACT_W<SHCSR_SPEC> {
        HARDFAULTACT_W::new(self, 2)
    }
    #[doc = "Bit 3 - `IAAMO the active state of the UsageFault exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W<SHCSR_SPEC> {
        USGFAULTACT_W::new(self, 3)
    }
    #[doc = "Bit 4 - `IAAMO the active state of the SecureFault exception"]
    #[inline(always)]
    #[must_use]
    pub fn securefaultact(&mut self) -> SECUREFAULTACT_W<SHCSR_SPEC> {
        SECUREFAULTACT_W::new(self, 4)
    }
    #[doc = "Bit 5 - `IAAMO the active state of the NMI exception"]
    #[inline(always)]
    #[must_use]
    pub fn nmiact(&mut self) -> NMIACT_W<SHCSR_SPEC> {
        NMIACT_W::new(self, 5)
    }
    #[doc = "Bit 7 - `IAAMO the active state of the SVCall exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn svcallact(&mut self) -> SVCALLACT_W<SHCSR_SPEC> {
        SVCALLACT_W::new(self, 7)
    }
    #[doc = "Bit 8 - `IAAMO the active state of the DebugMonitor exception"]
    #[inline(always)]
    #[must_use]
    pub fn monitoract(&mut self) -> MONITORACT_W<SHCSR_SPEC> {
        MONITORACT_W::new(self, 8)
    }
    #[doc = "Bit 10 - `IAAMO the active state of the PendSV exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvact(&mut self) -> PENDSVACT_W<SHCSR_SPEC> {
        PENDSVACT_W::new(self, 10)
    }
    #[doc = "Bit 11 - `IAAMO the active state of the SysTick exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn systickact(&mut self) -> SYSTICKACT_W<SHCSR_SPEC> {
        SYSTICKACT_W::new(self, 11)
    }
    #[doc = "Bit 12 - The UsageFault exception is banked between Security states, `IAAMO the pending state of the UsageFault exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W<SHCSR_SPEC> {
        USGFAULTPENDED_W::new(self, 12)
    }
    #[doc = "Bit 13 - `IAAMO the pending state of the MemManage exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W<SHCSR_SPEC> {
        MEMFAULTPENDED_W::new(self, 13)
    }
    #[doc = "Bit 14 - `IAAMO the pending state of the BusFault exception"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W<SHCSR_SPEC> {
        BUSFAULTPENDED_W::new(self, 14)
    }
    #[doc = "Bit 15 - `IAAMO the pending state of the SVCall exception `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<SHCSR_SPEC> {
        SVCALLPENDED_W::new(self, 15)
    }
    #[doc = "Bit 16 - `DW the MemManage exception is enabled `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W<SHCSR_SPEC> {
        MEMFAULTENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - `DW the BusFault exception is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W<SHCSR_SPEC> {
        BUSFAULTENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - `DW the UsageFault exception is enabled `FTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W<SHCSR_SPEC> {
        USGFAULTENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - `DW the SecureFault exception is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn securefaultena(&mut self) -> SECUREFAULTENA_W<SHCSR_SPEC> {
        SECUREFAULTENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - `IAAMO the pending state of the SecureFault exception"]
    #[inline(always)]
    #[must_use]
    pub fn securefaultpended(&mut self) -> SECUREFAULTPENDED_W<SHCSR_SPEC> {
        SECUREFAULTPENDED_W::new(self, 20)
    }
    #[doc = "Bit 21 - `IAAMO the pending state of the HardFault exception `CTTSSS"]
    #[inline(always)]
    #[must_use]
    pub fn hardfaultpended(&mut self) -> HARDFAULTPENDED_W<SHCSR_SPEC> {
        HARDFAULTPENDED_W::new(self, 21)
    }
}
#[doc = "Provides access to the active and pending status of system exceptions  

You can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
