#[doc = "Register `INTS3` reader"]
pub type R = crate::R<INTS3_SPEC>;
#[doc = "Register `INTS3` writer"]
pub type W = crate::W<INTS3_SPEC>;
#[doc = "Field `INTS3` reader - Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
pub type INTS3_R = crate::FieldReader<u16>;
#[doc = "Field `INTS3` writer - Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
pub type INTS3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn ints3(&self) -> INTS3_R {
        INTS3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    #[must_use]
    pub fn ints3(&mut self) -> INTS3_W<INTS3_SPEC> {
        INTS3_W::new(self, 0)
    }
}
#[doc = "Interrupt Status for IRQ 3  

You can [`read`](crate::Reg::read) this register and get [`ints3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ints3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS3_SPEC;
impl crate::RegisterSpec for INTS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints3::R`](R) reader structure"]
impl crate::Readable for INTS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints3::W`](W) writer structure"]
impl crate::Writable for INTS3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets INTS3 to value 0"]
impl crate::Resettable for INTS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
