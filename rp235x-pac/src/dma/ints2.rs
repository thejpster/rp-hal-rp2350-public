#[doc = "Register `INTS2` reader"]
pub type R = crate::R<INTS2_SPEC>;
#[doc = "Register `INTS2` writer"]
pub type W = crate::W<INTS2_SPEC>;
#[doc = "Field `INTS2` reader - Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
pub type INTS2_R = crate::FieldReader<u16>;
#[doc = "Field `INTS2` writer - Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
pub type INTS2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn ints2(&self) -> INTS2_R {
        INTS2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    #[must_use]
    pub fn ints2(&mut self) -> INTS2_W<INTS2_SPEC> {
        INTS2_W::new(self, 0)
    }
}
#[doc = "Interrupt Status for IRQ 2  

You can [`read`](crate::Reg::read) this register and get [`ints2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS2_SPEC;
impl crate::RegisterSpec for INTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints2::R`](R) reader structure"]
impl crate::Readable for INTS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints2::W`](W) writer structure"]
impl crate::Writable for INTS2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INTS2 to value 0"]
impl crate::Resettable for INTS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
