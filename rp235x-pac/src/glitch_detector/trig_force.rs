#[doc = "Register `TRIG_FORCE` reader"]
pub type R = crate::R<TRIG_FORCE_SPEC>;
#[doc = "Register `TRIG_FORCE` writer"]
pub type W = crate::W<TRIG_FORCE_SPEC>;
#[doc = "Field `TRIG_FORCE` writer - "]
pub type TRIG_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn trig_force(&mut self) -> TRIG_FORCE_W<TRIG_FORCE_SPEC> {
        TRIG_FORCE_W::new(self, 0)
    }
}
#[doc = "Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`trig_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIG_FORCE_SPEC;
impl crate::RegisterSpec for TRIG_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trig_force::R`](R) reader structure"]
impl crate::Readable for TRIG_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trig_force::W`](W) writer structure"]
impl crate::Writable for TRIG_FORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIG_FORCE to value 0"]
impl crate::Resettable for TRIG_FORCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
