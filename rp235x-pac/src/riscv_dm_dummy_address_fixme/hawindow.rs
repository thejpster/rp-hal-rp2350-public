#[doc = "Register `HAWINDOW` reader"]
pub type R = crate::R<HAWINDOW_SPEC>;
#[doc = "Register `HAWINDOW` writer"]
pub type W = crate::W<HAWINDOW_SPEC>;
#[doc = "Field `MASKDATA` reader - "]
pub type MASKDATA_R = crate::FieldReader;
#[doc = "Field `MASKDATA` writer - "]
pub type MASKDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn maskdata(&self) -> MASKDATA_R {
        MASKDATA_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn maskdata(&mut self) -> MASKDATA_W<HAWINDOW_SPEC> {
        MASKDATA_W::new(self, 0)
    }
}
#[doc = "This register provides R/W access to a 32-bit portion of the hart array mask register. The position of the window is determined by hawindowsel. I.e. bit 0 refers to hart hawindowsel ∗ 32, while bit 31 refers to hart hawindowsel ∗ 32 + 31.  

 On RP2350 only the two least-significant bits of this register are implemented, since there are only two cores. This is still useful to run/halt/reset both cores exactly simultaneously.  

You can [`read`](crate::Reg::read) this register and get [`hawindow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAWINDOW_SPEC;
impl crate::RegisterSpec for HAWINDOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hawindow::R`](R) reader structure"]
impl crate::Readable for HAWINDOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hawindow::W`](W) writer structure"]
impl crate::Writable for HAWINDOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAWINDOW to value 0"]
impl crate::Resettable for HAWINDOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
