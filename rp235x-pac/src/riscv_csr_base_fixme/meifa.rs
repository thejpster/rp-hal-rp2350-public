#[doc = "Register `MEIFA` reader"]
pub type R = crate::R<MEIFA_SPEC>;
#[doc = "Register `MEIFA` writer"]
pub type W = crate::W<MEIFA_SPEC>;
#[doc = "Field `INDEX` writer - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WINDOW` reader - 16-bit read/write window into the external interrupt force array"]
pub type WINDOW_R = crate::FieldReader<u16>;
#[doc = "Field `WINDOW` writer - 16-bit read/write window into the external interrupt force array"]
pub type WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt force array"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write-only, self-clearing field (no value is stored) used to control which window of the array appears in `window`."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<MEIFA_SPEC> {
        INDEX_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 16-bit read/write window into the external interrupt force array"]
    #[inline(always)]
    #[must_use]
    pub fn window(&mut self) -> WINDOW_W<MEIFA_SPEC> {
        WINDOW_W::new(self, 16)
    }
}
#[doc = "External interrupt force array  

 Contains a read-write bit for every interrupt request. Writing a 1 to a bit in the interrupt force array causes the corresponding bit to become pending in `meipa`. Software can use this feature to manually trigger a particular interrupt.  

 There are no restrictions on using `meifa` inside of an interrupt. The more useful case here is to schedule some lower-priority handler from within a high-priority interrupt, so that it will execute before the core returns to the foreground code. Implementers may wish to reserve some external IRQs with their external inputs tied to 0 for this purpose.  

 Bits can be cleared by software, and are cleared automatically by hardware upon a read of `meinext` which returns the corresponding IRQ number in `meinext.irq` with `mienext.noirq` clear (no matter whether `meinext.update` is written).  

 `meifa` implements the same array window indexing scheme as `meiea` and `meipa`.  

You can [`read`](crate::Reg::read) this register and get [`meifa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meifa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEIFA_SPEC;
impl crate::RegisterSpec for MEIFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meifa::R`](R) reader structure"]
impl crate::Readable for MEIFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meifa::W`](W) writer structure"]
impl crate::Writable for MEIFA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEIFA to value 0"]
impl crate::Resettable for MEIFA_SPEC {
    const RESET_VALUE: u32 = 0;
}
