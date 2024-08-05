#[doc = "Register `STREAM` reader"]
pub type R = crate::R<STREAM_SPEC>;
#[doc = "Register `STREAM` writer"]
pub type W = crate::W<STREAM_SPEC>;
#[doc = "Field `STREAM` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type STREAM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn stream(&self) -> STREAM_R {
        STREAM_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)  

You can [`read`](crate::Reg::read) this register and get [`stream::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stream::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_SPEC;
impl crate::RegisterSpec for STREAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream::R`](R) reader structure"]
impl crate::Readable for STREAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stream::W`](W) writer structure"]
impl crate::Writable for STREAM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STREAM to value 0"]
impl crate::Resettable for STREAM_SPEC {
    const RESET_VALUE: u32 = 0;
}
