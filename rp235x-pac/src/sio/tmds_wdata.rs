#[doc = "Register `TMDS_WDATA` writer"]
pub type W = crate::W<TMDS_WDATA_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TMDS_WDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Write-only access to the TMDS colour data register.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmds_wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_WDATA_SPEC;
impl crate::RegisterSpec for TMDS_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tmds_wdata::W`](W) writer structure"]
impl crate::Writable for TMDS_WDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDS_WDATA to value 0"]
impl crate::Resettable for TMDS_WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
