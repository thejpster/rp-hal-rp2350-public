#[doc = "Register `HAWINDOWSEL` reader"]
pub type R = crate::R<HAWINDOWSEL_SPEC>;
#[doc = "Register `HAWINDOWSEL` writer"]
pub type W = crate::W<HAWINDOWSEL_SPEC>;
#[doc = "Field `HAWINDOWSEL` reader - On Hazard3 this register is entirely hardwired to 0."]
pub type HAWINDOWSEL_R = crate::FieldReader<u16>;
#[doc = "Field `HAWINDOWSEL` writer - On Hazard3 this register is entirely hardwired to 0."]
pub type HAWINDOWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - On Hazard3 this register is entirely hardwired to 0."]
    #[inline(always)]
    pub fn hawindowsel(&self) -> HAWINDOWSEL_R {
        HAWINDOWSEL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - On Hazard3 this register is entirely hardwired to 0."]
    #[inline(always)]
    #[must_use]
    pub fn hawindowsel(&mut self) -> HAWINDOWSEL_W<HAWINDOWSEL_SPEC> {
        HAWINDOWSEL_W::new(self, 0)
    }
}
#[doc = "This register selects which of the 32-bit portion of the hart array mask register is accessible in hawindow.  

You can [`read`](crate::Reg::read) this register and get [`hawindowsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindowsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAWINDOWSEL_SPEC;
impl crate::RegisterSpec for HAWINDOWSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hawindowsel::R`](R) reader structure"]
impl crate::Readable for HAWINDOWSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hawindowsel::W`](W) writer structure"]
impl crate::Writable for HAWINDOWSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAWINDOWSEL to value 0"]
impl crate::Resettable for HAWINDOWSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
