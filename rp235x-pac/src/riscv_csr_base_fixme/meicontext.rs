#[doc = "Register `MEICONTEXT` reader"]
pub type R = crate::R<MEICONTEXT_SPEC>;
#[doc = "Register `MEICONTEXT` writer"]
pub type W = crate::W<MEICONTEXT_SPEC>;
#[doc = "Field `MRETEIRQ` reader - If 1, enable restore of the preemption priority stack on `mret`. This bit is set on entering the external interrupt vector, cleared by `mret`, and cleared upon taking any trap other than an external interrupt.  

 Provided `meicontext` is saved on entry to the external interrupt vector (before enabling preemption), is restored before exiting, and the standard software/timer IRQs are prevented from preempting (e.g. by using `clearts`), this flag allows the hardware to safely manage the preemption priority stack even when an external interrupt handler may take exceptions."]
pub type MRETEIRQ_R = crate::BitReader;
#[doc = "Field `MRETEIRQ` writer - If 1, enable restore of the preemption priority stack on `mret`. This bit is set on entering the external interrupt vector, cleared by `mret`, and cleared upon taking any trap other than an external interrupt.  

 Provided `meicontext` is saved on entry to the external interrupt vector (before enabling preemption), is restored before exiting, and the standard software/timer IRQs are prevented from preempting (e.g. by using `clearts`), this flag allows the hardware to safely manage the preemption priority stack even when an external interrupt handler may take exceptions."]
pub type MRETEIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTS` reader - Write-1 self-clearing field. Writing 1 will clear `mie.mtie` and `mie.msie`, and present their prior values in the `mtiesave` and `msiesave` of this register. This makes it safe to re-enable IRQs (via `mstatus.mie`) without the possibility of being preempted by the standard timer and soft interrupt handlers, which may not be aware of Hazard3's interrupt hardware.  

 The clear due to `clearts` takes precedence over the set due to `mtiesave`/`msiesave`, although it would be unusual for software to write both on the same cycle."]
pub type CLEARTS_R = crate::BitReader;
#[doc = "Field `CLEARTS` writer - Write-1 self-clearing field. Writing 1 will clear `mie.mtie` and `mie.msie`, and present their prior values in the `mtiesave` and `msiesave` of this register. This makes it safe to re-enable IRQs (via `mstatus.mie`) without the possibility of being preempted by the standard timer and soft interrupt handlers, which may not be aware of Hazard3's interrupt hardware.  

 The clear due to `clearts` takes precedence over the set due to `mtiesave`/`msiesave`, although it would be unusual for software to write both on the same cycle."]
pub type CLEARTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIESAVE` reader - Reads as the current value of `mie.msie`, if `clearts` is set by the same CSR access instruction. Otherwise reads as 0. Writes are ORed into `mie.msie`."]
pub type MSIESAVE_R = crate::BitReader;
#[doc = "Field `MTIESAVE` reader - Reads as the current value of `mie.mtie`, if `clearts` is set by the same CSR access instruction. Otherwise reads as 0. Writes are ORed into `mie.mtie`."]
pub type MTIESAVE_R = crate::BitReader;
#[doc = "Field `IRQ` reader - Current IRQ number (read/write). Set to `meinext.irq` when `meinext.update` is written. No hardware effect."]
pub type IRQ_R = crate::FieldReader<u16>;
#[doc = "Field `IRQ` writer - Current IRQ number (read/write). Set to `meinext.irq` when `meinext.update` is written. No hardware effect."]
pub type IRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NOIRQ` reader - Not in interrupt (read/write). Set to 1 at reset. Set to `meinext.noirq` when `meinext.update` is written. No hardware effect."]
pub type NOIRQ_R = crate::BitReader;
#[doc = "Field `NOIRQ` writer - Not in interrupt (read/write). Set to 1 at reset. Set to `meinext.noirq` when `meinext.update` is written. No hardware effect."]
pub type NOIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREEMPT` reader - Minimum interrupt priority to preempt the current interrupt. Interrupts with lower priority than `preempt` do not cause the core to transfer to an interrupt handler. Updated by hardware when when `meinext.update` is written, or when hardware enters the external interrupt vector.  

 If an interrupt is present in `meinext` when this field is updated, then `preempt` is set to one level greater than that interrupt's priority. Otherwise, `ppreempt` is set to one level greater than the maximum interrupt priority, disabling preemption."]
pub type PREEMPT_R = crate::FieldReader;
#[doc = "Field `PREEMPT` writer - Minimum interrupt priority to preempt the current interrupt. Interrupts with lower priority than `preempt` do not cause the core to transfer to an interrupt handler. Updated by hardware when when `meinext.update` is written, or when hardware enters the external interrupt vector.  

 If an interrupt is present in `meinext` when this field is updated, then `preempt` is set to one level greater than that interrupt's priority. Otherwise, `ppreempt` is set to one level greater than the maximum interrupt priority, disabling preemption."]
pub type PREEMPT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PPREEMPT` reader - Previous `preempt`. Set to `preempt` on priority save, restored to to `pppreempt` on priority restore.  

 IRQs of lower priority than `ppreempt` are not visible in `meinext`, so that a preemptee is not re-taken in the preempting frame."]
pub type PPREEMPT_R = crate::FieldReader;
#[doc = "Field `PPREEMPT` writer - Previous `preempt`. Set to `preempt` on priority save, restored to to `pppreempt` on priority restore.  

 IRQs of lower priority than `ppreempt` are not visible in `meinext`, so that a preemptee is not re-taken in the preempting frame."]
pub type PPREEMPT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPPREEMPT` reader - Previous `ppreempt`. Set to `ppreempt` on priority save, set to zero on priority restore. Has no hardware effect, but ensures that when `meicontext` is saved/restored correctly, `preempt` and `ppreempt` stack correctly through arbitrarily many preemption frames."]
pub type PPPREEMPT_R = crate::FieldReader;
#[doc = "Field `PPPREEMPT` writer - Previous `ppreempt`. Set to `ppreempt` on priority save, set to zero on priority restore. Has no hardware effect, but ensures that when `meicontext` is saved/restored correctly, `preempt` and `ppreempt` stack correctly through arbitrarily many preemption frames."]
pub type PPPREEMPT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - If 1, enable restore of the preemption priority stack on `mret`. This bit is set on entering the external interrupt vector, cleared by `mret`, and cleared upon taking any trap other than an external interrupt.  

 Provided `meicontext` is saved on entry to the external interrupt vector (before enabling preemption), is restored before exiting, and the standard software/timer IRQs are prevented from preempting (e.g. by using `clearts`), this flag allows the hardware to safely manage the preemption priority stack even when an external interrupt handler may take exceptions."]
    #[inline(always)]
    pub fn mreteirq(&self) -> MRETEIRQ_R {
        MRETEIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write-1 self-clearing field. Writing 1 will clear `mie.mtie` and `mie.msie`, and present their prior values in the `mtiesave` and `msiesave` of this register. This makes it safe to re-enable IRQs (via `mstatus.mie`) without the possibility of being preempted by the standard timer and soft interrupt handlers, which may not be aware of Hazard3's interrupt hardware.  

 The clear due to `clearts` takes precedence over the set due to `mtiesave`/`msiesave`, although it would be unusual for software to write both on the same cycle."]
    #[inline(always)]
    pub fn clearts(&self) -> CLEARTS_R {
        CLEARTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reads as the current value of `mie.msie`, if `clearts` is set by the same CSR access instruction. Otherwise reads as 0. Writes are ORed into `mie.msie`."]
    #[inline(always)]
    pub fn msiesave(&self) -> MSIESAVE_R {
        MSIESAVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reads as the current value of `mie.mtie`, if `clearts` is set by the same CSR access instruction. Otherwise reads as 0. Writes are ORed into `mie.mtie`."]
    #[inline(always)]
    pub fn mtiesave(&self) -> MTIESAVE_R {
        MTIESAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:12 - Current IRQ number (read/write). Set to `meinext.irq` when `meinext.update` is written. No hardware effect."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Not in interrupt (read/write). Set to 1 at reset. Set to `meinext.noirq` when `meinext.update` is written. No hardware effect."]
    #[inline(always)]
    pub fn noirq(&self) -> NOIRQ_R {
        NOIRQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Minimum interrupt priority to preempt the current interrupt. Interrupts with lower priority than `preempt` do not cause the core to transfer to an interrupt handler. Updated by hardware when when `meinext.update` is written, or when hardware enters the external interrupt vector.  

 If an interrupt is present in `meinext` when this field is updated, then `preempt` is set to one level greater than that interrupt's priority. Otherwise, `ppreempt` is set to one level greater than the maximum interrupt priority, disabling preemption."]
    #[inline(always)]
    pub fn preempt(&self) -> PREEMPT_R {
        PREEMPT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Previous `preempt`. Set to `preempt` on priority save, restored to to `pppreempt` on priority restore.  

 IRQs of lower priority than `ppreempt` are not visible in `meinext`, so that a preemptee is not re-taken in the preempting frame."]
    #[inline(always)]
    pub fn ppreempt(&self) -> PPREEMPT_R {
        PPREEMPT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Previous `ppreempt`. Set to `ppreempt` on priority save, set to zero on priority restore. Has no hardware effect, but ensures that when `meicontext` is saved/restored correctly, `preempt` and `ppreempt` stack correctly through arbitrarily many preemption frames."]
    #[inline(always)]
    pub fn pppreempt(&self) -> PPPREEMPT_R {
        PPPREEMPT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, enable restore of the preemption priority stack on `mret`. This bit is set on entering the external interrupt vector, cleared by `mret`, and cleared upon taking any trap other than an external interrupt.  

 Provided `meicontext` is saved on entry to the external interrupt vector (before enabling preemption), is restored before exiting, and the standard software/timer IRQs are prevented from preempting (e.g. by using `clearts`), this flag allows the hardware to safely manage the preemption priority stack even when an external interrupt handler may take exceptions."]
    #[inline(always)]
    #[must_use]
    pub fn mreteirq(&mut self) -> MRETEIRQ_W<MEICONTEXT_SPEC> {
        MRETEIRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write-1 self-clearing field. Writing 1 will clear `mie.mtie` and `mie.msie`, and present their prior values in the `mtiesave` and `msiesave` of this register. This makes it safe to re-enable IRQs (via `mstatus.mie`) without the possibility of being preempted by the standard timer and soft interrupt handlers, which may not be aware of Hazard3's interrupt hardware.  

 The clear due to `clearts` takes precedence over the set due to `mtiesave`/`msiesave`, although it would be unusual for software to write both on the same cycle."]
    #[inline(always)]
    #[must_use]
    pub fn clearts(&mut self) -> CLEARTS_W<MEICONTEXT_SPEC> {
        CLEARTS_W::new(self, 1)
    }
    #[doc = "Bits 4:12 - Current IRQ number (read/write). Set to `meinext.irq` when `meinext.update` is written. No hardware effect."]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<MEICONTEXT_SPEC> {
        IRQ_W::new(self, 4)
    }
    #[doc = "Bit 15 - Not in interrupt (read/write). Set to 1 at reset. Set to `meinext.noirq` when `meinext.update` is written. No hardware effect."]
    #[inline(always)]
    #[must_use]
    pub fn noirq(&mut self) -> NOIRQ_W<MEICONTEXT_SPEC> {
        NOIRQ_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Minimum interrupt priority to preempt the current interrupt. Interrupts with lower priority than `preempt` do not cause the core to transfer to an interrupt handler. Updated by hardware when when `meinext.update` is written, or when hardware enters the external interrupt vector.  

 If an interrupt is present in `meinext` when this field is updated, then `preempt` is set to one level greater than that interrupt's priority. Otherwise, `ppreempt` is set to one level greater than the maximum interrupt priority, disabling preemption."]
    #[inline(always)]
    #[must_use]
    pub fn preempt(&mut self) -> PREEMPT_W<MEICONTEXT_SPEC> {
        PREEMPT_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Previous `preempt`. Set to `preempt` on priority save, restored to to `pppreempt` on priority restore.  

 IRQs of lower priority than `ppreempt` are not visible in `meinext`, so that a preemptee is not re-taken in the preempting frame."]
    #[inline(always)]
    #[must_use]
    pub fn ppreempt(&mut self) -> PPREEMPT_W<MEICONTEXT_SPEC> {
        PPREEMPT_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Previous `ppreempt`. Set to `ppreempt` on priority save, set to zero on priority restore. Has no hardware effect, but ensures that when `meicontext` is saved/restored correctly, `preempt` and `ppreempt` stack correctly through arbitrarily many preemption frames."]
    #[inline(always)]
    #[must_use]
    pub fn pppreempt(&mut self) -> PPPREEMPT_W<MEICONTEXT_SPEC> {
        PPPREEMPT_W::new(self, 28)
    }
}
#[doc = "External interrupt context register  

 Configures the priority level for interrupt preemption, and helps software track which interrupt it is currently in. The latter is useful when a common interrupt service routine handles interrupt requests from multiple instances of the same peripheral.  

 A three-level stack of preemption priorities is maintained in the `preempt`, `ppreempt` and `pppreempt` fields. The priority stack is saved when hardware enters the external interrupt vector, and restored by an `mret` instruction if `meicontext.mreteirq` is set.  

 The top entry of the priority stack, `preempt`, is used by hardware to ensure that only higher-priority interrupts can preempt the current interrupt. The next entry, `ppreempt`, is used to avoid servicing interrupts which may already be in progress in a frame that was preempted. The third entry, `pppreempt`, has no hardware effect, but ensures that `preempt` and `ppreempt` can be correctly saved/restored across arbitary levels of preemption.  

You can [`read`](crate::Reg::read) this register and get [`meicontext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meicontext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEICONTEXT_SPEC;
impl crate::RegisterSpec for MEICONTEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meicontext::R`](R) reader structure"]
impl crate::Readable for MEICONTEXT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meicontext::W`](W) writer structure"]
impl crate::Writable for MEICONTEXT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEICONTEXT to value 0x8000"]
impl crate::Resettable for MEICONTEXT_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
