#[doc = "Register `RANDID7` reader"]
pub type R = crate::R<RANDID7_SPEC>;
#[doc = "Register `RANDID7` writer"]
pub type W = crate::W<RANDID7_SPEC>;
#[doc = "Field `RANDID7` reader - "]
pub type RANDID7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn randid7(&self) -> RANDID7_R {
        RANDID7_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 127:112 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID7_SPEC;
impl crate::RegisterSpec for RANDID7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid7::R`](R) reader structure"]
impl crate::Readable for RANDID7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid7::W`](W) writer structure"]
impl crate::Writable for RANDID7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDID7 to value 0"]
impl crate::Resettable for RANDID7_SPEC {
    const RESET_VALUE: u32 = 0;
}
