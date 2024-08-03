#[doc = "Register `MEINEXT` reader"]
pub type R = crate::R<MEINEXT_SPEC>;
#[doc = "Register `MEINEXT` writer"]
pub type W = crate::W<MEINEXT_SPEC>;
#[doc = "Field `UPDATE` reader - Writing 1 (self-clearing) causes hardware to update `meicontext` according to the IRQ number and preemption priority of the interrupt indicated in `noirq`/`irq`. This should be done in a single atomic operation, i.e. `csrrsi a0, meinext, 0x1`."]
pub type UPDATE_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Writing 1 (self-clearing) causes hardware to update `meicontext` according to the IRQ number and preemption priority of the interrupt indicated in `noirq`/`irq`. This should be done in a single atomic operation, i.e. `csrrsi a0, meinext, 0x1`."]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ` reader - Index of the highest-priority active external interrupt. Zero when no external interrupts with sufficient priority are both pending and enabled."]
pub type IRQ_R = crate::FieldReader<u16>;
#[doc = "Field `NOIRQ` reader - Set when there is no external interrupt which is enabled, pending, and has priority greater than or equal to `meicontext.ppreempt`. Can be efficiently tested with a `bltz` or `bgez` instruction."]
pub type NOIRQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Writing 1 (self-clearing) causes hardware to update `meicontext` according to the IRQ number and preemption priority of the interrupt indicated in `noirq`/`irq`. This should be done in a single atomic operation, i.e. `csrrsi a0, meinext, 0x1`."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:10 - Index of the highest-priority active external interrupt. Zero when no external interrupts with sufficient priority are both pending and enabled."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - Set when there is no external interrupt which is enabled, pending, and has priority greater than or equal to `meicontext.ppreempt`. Can be efficiently tested with a `bltz` or `bgez` instruction."]
    #[inline(always)]
    pub fn noirq(&self) -> NOIRQ_R {
        NOIRQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 (self-clearing) causes hardware to update `meicontext` according to the IRQ number and preemption priority of the interrupt indicated in `noirq`/`irq`. This should be done in a single atomic operation, i.e. `csrrsi a0, meinext, 0x1`."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<MEINEXT_SPEC> {
        UPDATE_W::new(self, 0)
    }
}
#[doc = "Get next external interrupt  

 Contains the index of the highest-priority external interrupt which is both asserted in `meipa` and enabled in `meiea`, left-shifted by 2 so that it can be used to index an array of 32-bit function pointers. If there is no such interrupt, the MSB is set.  

 When multiple interrupts of the same priority are both pending and enabled, the lowest-numbered wins. Interrupts with priority less than `meicontext.ppreempt` -- the _previous_ preemption priority -- are treated as though they are not pending. This is to ensure that a preempting interrupt frame does not service interrupts which may be in progress in the frame that was preempted.  

You can [`read`](crate::Reg::read) this register and get [`meinext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meinext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEINEXT_SPEC;
impl crate::RegisterSpec for MEINEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meinext::R`](R) reader structure"]
impl crate::Readable for MEINEXT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meinext::W`](W) writer structure"]
impl crate::Writable for MEINEXT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEINEXT to value 0"]
impl crate::Resettable for MEINEXT_SPEC {
    const RESET_VALUE: u32 = 0;
}
