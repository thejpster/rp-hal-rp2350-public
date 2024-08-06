#[doc = "Register `RANDID1` reader"]
pub type R = crate::R<RANDID1_SPEC>;
#[doc = "Register `RANDID1` writer"]
pub type W = crate::W<RANDID1_SPEC>;
#[doc = "Field `RANDID1` reader - "]
pub type RANDID1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid1(&self) -> RANDID1_R {
        RANDID1_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 31:16 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID1_SPEC;
impl crate::RegisterSpec for RANDID1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid1::R`](R) reader structure"]
impl crate::Readable for RANDID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid1::W`](W) writer structure"]
impl crate::Writable for RANDID1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RANDID1 to value 0"]
impl crate::Resettable for RANDID1_SPEC {
    const RESET_VALUE: u16 = 0;
}
