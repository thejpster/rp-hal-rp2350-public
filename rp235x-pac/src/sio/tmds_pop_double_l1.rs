#[doc = "Register `TMDS_POP_DOUBLE_L1` reader"]
pub type R = crate::R<TMDS_POP_DOUBLE_L1_SPEC>;
#[doc = "Register `TMDS_POP_DOUBLE_L1` writer"]
pub type W = crate::W<TMDS_POP_DOUBLE_L1_SPEC>;
#[doc = "Field `TMDS_POP_DOUBLE_L1` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TMDS_POP_DOUBLE_L1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmds_pop_double_l1(&self) -> TMDS_POP_DOUBLE_L1_R {
        TMDS_POP_DOUBLE_L1_R::new(self.bits)
    }
}
impl W {}
#[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_pop_double_l1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_POP_DOUBLE_L1_SPEC;
impl crate::RegisterSpec for TMDS_POP_DOUBLE_L1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_pop_double_l1::R`](R) reader structure"]
impl crate::Readable for TMDS_POP_DOUBLE_L1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmds_pop_double_l1::W`](W) writer structure"]
impl crate::Writable for TMDS_POP_DOUBLE_L1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_POP_DOUBLE_L1 to value 0"]
impl crate::Resettable for TMDS_POP_DOUBLE_L1_SPEC {
    const RESET_VALUE: u32 = 0;
}
