#[doc = "Register `RANDID4` reader"]
pub type R = crate::R<RANDID4_SPEC>;
#[doc = "Register `RANDID4` writer"]
pub type W = crate::W<RANDID4_SPEC>;
#[doc = "Field `RANDID4` reader - "]
pub type RANDID4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid4(&self) -> RANDID4_R {
        RANDID4_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 79:64 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID4_SPEC;
impl crate::RegisterSpec for RANDID4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid4::R`](R) reader structure"]
impl crate::Readable for RANDID4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid4::W`](W) writer structure"]
impl crate::Writable for RANDID4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RANDID4 to value 0"]
impl crate::Resettable for RANDID4_SPEC {
    const RESET_VALUE: u16 = 0;
}
