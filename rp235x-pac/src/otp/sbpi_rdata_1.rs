#[doc = "Register `SBPI_RDATA_1` reader"]
pub type R = crate::R<SBPI_RDATA_1_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<SBPI_RDATA_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Read payload bytes 7..4. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct SBPI_RDATA_1_SPEC;
impl crate::RegisterSpec for SBPI_RDATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_rdata_1::R`](R) reader structure"]
impl crate::Readable for SBPI_RDATA_1_SPEC {}
#[doc = "`reset()` method sets SBPI_RDATA_1 to value 0"]
impl crate::Resettable for SBPI_RDATA_1_SPEC {
    const RESET_VALUE: u32 = 0;
}