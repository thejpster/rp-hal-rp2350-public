#[doc = "Register `TMDS_POP_DOUBLE_L0` reader"]
pub type R = crate::R<TMDS_POP_DOUBLE_L0_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TMDS_POP_DOUBLE_L0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.  

 The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct TMDS_POP_DOUBLE_L0_SPEC;
impl crate::RegisterSpec for TMDS_POP_DOUBLE_L0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_pop_double_l0::R`](R) reader structure"]
impl crate::Readable for TMDS_POP_DOUBLE_L0_SPEC {}
#[doc = "`reset()` method sets TMDS_POP_DOUBLE_L0 to value 0"]
impl crate::Resettable for TMDS_POP_DOUBLE_L0_SPEC {
    const RESET_VALUE: u32 = 0;
}
