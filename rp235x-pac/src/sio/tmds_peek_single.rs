#[doc = "Register `TMDS_PEEK_SINGLE` reader"]
pub type R = crate::R<TMDS_PEEK_SINGLE_SPEC>;
#[doc = "Register `TMDS_PEEK_SINGLE` writer"]
pub type W = crate::W<TMDS_PEEK_SINGLE_SPEC>;
#[doc = "Field `TMDS_PEEK_SINGLE` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TMDS_PEEK_SINGLE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmds_peek_single(&self) -> TMDS_PEEK_SINGLE_R {
        TMDS_PEEK_SINGLE_R::new(self.bits)
    }
}
impl W {}
#[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols). The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_single::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_single::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_PEEK_SINGLE_SPEC;
impl crate::RegisterSpec for TMDS_PEEK_SINGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_peek_single::R`](R) reader structure"]
impl crate::Readable for TMDS_PEEK_SINGLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmds_peek_single::W`](W) writer structure"]
impl crate::Writable for TMDS_PEEK_SINGLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_PEEK_SINGLE to value 0"]
impl crate::Resettable for TMDS_PEEK_SINGLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
