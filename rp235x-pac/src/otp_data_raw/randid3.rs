#[doc = "Register `RANDID3` reader"]
pub type R = crate::R<RANDID3_SPEC>;
#[doc = "Register `RANDID3` writer"]
pub type W = crate::W<RANDID3_SPEC>;
#[doc = "Field `RANDID3` reader - "]
pub type RANDID3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn randid3(&self) -> RANDID3_R {
        RANDID3_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Bits 63:48 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`randid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID3_SPEC;
impl crate::RegisterSpec for RANDID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid3::R`](R) reader structure"]
impl crate::Readable for RANDID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`randid3::W`](W) writer structure"]
impl crate::Writable for RANDID3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANDID3 to value 0"]
impl crate::Resettable for RANDID3_SPEC {
    const RESET_VALUE: u32 = 0;
}
