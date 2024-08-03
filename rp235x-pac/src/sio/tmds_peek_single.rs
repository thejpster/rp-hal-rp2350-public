#[doc = "Register `TMDS_PEEK_SINGLE` reader"]
pub type R = crate::R<TMDS_PEEK_SINGLE_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TMDS_PEEK_SINGLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols).  

 The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling.  

You can [`read`](crate::Reg::read) this register and get [`tmds_peek_single::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct TMDS_PEEK_SINGLE_SPEC;
impl crate::RegisterSpec for TMDS_PEEK_SINGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_peek_single::R`](R) reader structure"]
impl crate::Readable for TMDS_PEEK_SINGLE_SPEC {}
#[doc = "`reset()` method sets TMDS_PEEK_SINGLE to value 0"]
impl crate::Resettable for TMDS_PEEK_SINGLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
