#[doc = "Register `RANDID5` reader"]
pub type R = crate::R<RANDID5_SPEC>;
#[doc = "Register `RANDID5` writer"]
pub type W = crate::W<RANDID5_SPEC>;
#[doc = "Field `RANDID5` reader - "]
pub type RANDID5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid5(&self) -> RANDID5_R {
        RANDID5_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 95:80 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID5_SPEC;
impl crate::RegisterSpec for RANDID5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid5::R`](R) reader structure"]
impl crate::Readable for RANDID5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid5::W`](W) writer structure"]
impl crate::Writable for RANDID5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RANDID5 to value 0"]
impl crate::Resettable for RANDID5_SPEC {
    const RESET_VALUE: u16 = 0;
}
