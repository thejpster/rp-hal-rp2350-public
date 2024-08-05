#[doc = "Register `INTE3` reader"]
pub type R = crate::R<INTE3_SPEC>;
#[doc = "Register `INTE3` writer"]
pub type W = crate::W<INTE3_SPEC>;
#[doc = "Field `INTE3` reader - Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
pub type INTE3_R = crate::FieldReader<u16>;
#[doc = "Field `INTE3` writer - Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
pub type INTE3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
    #[inline(always)]
    pub fn inte3(&self) -> INTE3_R {
        INTE3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
    #[inline(always)]
    #[must_use]
    pub fn inte3(&mut self) -> INTE3_W<INTE3_SPEC> {
        INTE3_W::new(self, 0)
    }
}
#[doc = "Interrupt Enables for IRQ 3  

You can [`read`](crate::Reg::read) this register and get [`inte3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE3_SPEC;
impl crate::RegisterSpec for INTE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte3::R`](R) reader structure"]
impl crate::Readable for INTE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte3::W`](W) writer structure"]
impl crate::Writable for INTE3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE3 to value 0"]
impl crate::Resettable for INTE3_SPEC {
    const RESET_VALUE: u32 = 0;
}
