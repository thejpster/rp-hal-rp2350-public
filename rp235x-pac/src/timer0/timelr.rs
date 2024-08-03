#[doc = "Register `TIMELR` reader"]
pub type R = crate::R<TIMELR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TIMELR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Read from bits 31:0 of time  

You can [`read`](crate::Reg::read) this register and get [`timelr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct TIMELR_SPEC;
impl crate::RegisterSpec for TIMELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timelr::R`](R) reader structure"]
impl crate::Readable for TIMELR_SPEC {}
#[doc = "`reset()` method sets TIMELR to value 0"]
impl crate::Resettable for TIMELR_SPEC {
    const RESET_VALUE: u32 = 0;
}
