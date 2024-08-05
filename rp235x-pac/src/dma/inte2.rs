#[doc = "Register `INTE2` reader"]
pub type R = crate::R<INTE2_SPEC>;
#[doc = "Register `INTE2` writer"]
pub type W = crate::W<INTE2_SPEC>;
#[doc = "Field `INTE2` reader - Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
pub type INTE2_R = crate::FieldReader<u16>;
#[doc = "Field `INTE2` writer - Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
pub type INTE2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
    #[inline(always)]
    pub fn inte2(&self) -> INTE2_R {
        INTE2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
    #[inline(always)]
    #[must_use]
    pub fn inte2(&mut self) -> INTE2_W<INTE2_SPEC> {
        INTE2_W::new(self, 0)
    }
}
#[doc = "Interrupt Enables for IRQ 2  

You can [`read`](crate::Reg::read) this register and get [`inte2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE2_SPEC;
impl crate::RegisterSpec for INTE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte2::R`](R) reader structure"]
impl crate::Readable for INTE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte2::W`](W) writer structure"]
impl crate::Writable for INTE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE2 to value 0"]
impl crate::Resettable for INTE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
