#[doc = "Register `TMDS_PEEK_DOUBLE_L1` reader"]
pub type R = crate::R<TMDS_PEEK_DOUBLE_L1_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TMDS_PEEK_DOUBLE_L1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.  

 The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_double_l1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct TMDS_PEEK_DOUBLE_L1_SPEC;
impl crate::RegisterSpec for TMDS_PEEK_DOUBLE_L1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_peek_double_l1::R`](R) reader structure"]
impl crate::Readable for TMDS_PEEK_DOUBLE_L1_SPEC {}
#[doc = "`reset()` method sets TMDS_PEEK_DOUBLE_L1 to value 0"]
impl crate::Resettable for TMDS_PEEK_DOUBLE_L1_SPEC {
    const RESET_VALUE: u32 = 0;
}
