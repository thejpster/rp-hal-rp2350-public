#[doc = "Register `RANDID2` reader"]
pub type R = crate::R<RANDID2_SPEC>;
#[doc = "Register `RANDID2` writer"]
pub type W = crate::W<RANDID2_SPEC>;
#[doc = "Field `RANDID2` reader - "]
pub type RANDID2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid2(&self) -> RANDID2_R {
        RANDID2_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 47:32 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID2_SPEC;
impl crate::RegisterSpec for RANDID2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid2::R`](R) reader structure"]
impl crate::Readable for RANDID2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid2::W`](W) writer structure"]
impl crate::Writable for RANDID2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RANDID2 to value 0"]
impl crate::Resettable for RANDID2_SPEC {
    const RESET_VALUE: u16 = 0;
}
