#[doc = "Register `GITREF_RP2350` reader"]
pub type R = crate::R<GITREF_RP2350_SPEC>;
#[doc = "Register `GITREF_RP2350` writer"]
pub type W = crate::W<GITREF_RP2350_SPEC>;
#[doc = "Field `GITREF_RP2350` reader - "]
pub type GITREF_RP2350_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gitref_rp2350(&self) -> GITREF_RP2350_R {
        GITREF_RP2350_R::new(self.bits)
    }
}
impl W {}
#[doc = "Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::Reg::read) this register and get [`gitref_rp2350::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gitref_rp2350::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GITREF_RP2350_SPEC;
impl crate::RegisterSpec for GITREF_RP2350_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gitref_rp2350::R`](R) reader structure"]
impl crate::Readable for GITREF_RP2350_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gitref_rp2350::W`](W) writer structure"]
impl crate::Writable for GITREF_RP2350_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GITREF_RP2350 to value 0"]
impl crate::Resettable for GITREF_RP2350_SPEC {
    const RESET_VALUE: u32 = 0;
}
