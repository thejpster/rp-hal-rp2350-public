#[doc = "Register `TMDS_PEEK_DOUBLE_L2` reader"]
pub type R = crate::R<TMDS_PEEK_DOUBLE_L2_SPEC>;
#[doc = "Register `TMDS_PEEK_DOUBLE_L2` writer"]
pub type W = crate::W<TMDS_PEEK_DOUBLE_L2_SPEC>;
#[doc = "Field `TMDS_PEEK_DOUBLE_L2` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TMDS_PEEK_DOUBLE_L2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmds_peek_double_l2(&self) -> TMDS_PEEK_DOUBLE_L2_R {
        TMDS_PEEK_DOUBLE_L2_R::new(self.bits)
    }
}
impl W {}
#[doc = "Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_double_l2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_peek_double_l2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_PEEK_DOUBLE_L2_SPEC;
impl crate::RegisterSpec for TMDS_PEEK_DOUBLE_L2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_peek_double_l2::R`](R) reader structure"]
impl crate::Readable for TMDS_PEEK_DOUBLE_L2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmds_peek_double_l2::W`](W) writer structure"]
impl crate::Writable for TMDS_PEEK_DOUBLE_L2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_PEEK_DOUBLE_L2 to value 0"]
impl crate::Resettable for TMDS_PEEK_DOUBLE_L2_SPEC {
    const RESET_VALUE: u32 = 0;
}
