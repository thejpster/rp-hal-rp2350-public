#[doc = "Register `RISCV_SOFTIRQ` reader"]
pub type R = crate::R<RISCV_SOFTIRQ_SPEC>;
#[doc = "Register `RISCV_SOFTIRQ` writer"]
pub type W = crate::W<RISCV_SOFTIRQ_SPEC>;
#[doc = "Field `CORE0_SET` reader - Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
pub type CORE0_SET_R = crate::BitReader;
#[doc = "Field `CORE0_SET` writer - Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
pub type CORE0_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_SET` reader - Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
pub type CORE1_SET_R = crate::BitReader;
#[doc = "Field `CORE1_SET` writer - Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
pub type CORE1_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_CLR` reader - Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
pub type CORE0_CLR_R = crate::BitReader;
#[doc = "Field `CORE0_CLR` writer - Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
pub type CORE0_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_CLR` reader - Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
pub type CORE1_CLR_R = crate::BitReader;
#[doc = "Field `CORE1_CLR` writer - Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
pub type CORE1_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn core0_set(&self) -> CORE0_SET_R {
        CORE0_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn core1_set(&self) -> CORE1_SET_R {
        CORE1_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn core0_clr(&self) -> CORE0_CLR_R {
        CORE0_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn core1_clr(&self) -> CORE1_CLR_R {
        CORE1_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    #[must_use]
    pub fn core0_set(&mut self) -> CORE0_SET_W<RISCV_SOFTIRQ_SPEC> {
        CORE0_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    #[must_use]
    pub fn core1_set(&mut self) -> CORE1_SET_W<RISCV_SOFTIRQ_SPEC> {
        CORE1_SET_W::new(self, 1)
    }
    #[doc = "Bit 8 - Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    #[must_use]
    pub fn core0_clr(&mut self) -> CORE0_CLR_W<RISCV_SOFTIRQ_SPEC> {
        CORE0_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    #[must_use]
    pub fn core1_clr(&mut self) -> CORE1_CLR_W<RISCV_SOFTIRQ_SPEC> {
        CORE1_CLR_W::new(self, 9)
    }
}
#[doc = "Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect.  

You can [`read`](crate::Reg::read) this register and get [`riscv_softirq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`riscv_softirq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_SOFTIRQ_SPEC;
impl crate::RegisterSpec for RISCV_SOFTIRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_softirq::R`](R) reader structure"]
impl crate::Readable for RISCV_SOFTIRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`riscv_softirq::W`](W) writer structure"]
impl crate::Writable for RISCV_SOFTIRQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RISCV_SOFTIRQ to value 0"]
impl crate::Resettable for RISCV_SOFTIRQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
