#[doc = "Register `RANDID6` reader"]
pub type R = crate::R<RANDID6_SPEC>;
#[doc = "Register `RANDID6` writer"]
pub type W = crate::W<RANDID6_SPEC>;
#[doc = "Field `RANDID6` reader - "]
pub type RANDID6_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid6(&self) -> RANDID6_R {
        RANDID6_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 111:96 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID6_SPEC;
impl crate::RegisterSpec for RANDID6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid6::R`](R) reader structure"]
impl crate::Readable for RANDID6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid6::W`](W) writer structure"]
impl crate::Writable for RANDID6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RANDID6 to value 0"]
impl crate::Resettable for RANDID6_SPEC {
    const RESET_VALUE: u16 = 0;
}
