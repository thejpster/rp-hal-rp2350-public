#[doc = "Register `SBPI_RDATA_2` reader"]
pub type R = crate::R<SBPI_RDATA_2_SPEC>;
#[doc = "Register `SBPI_RDATA_2` writer"]
pub type W = crate::W<SBPI_RDATA_2_SPEC>;
#[doc = "Field `SBPI_RDATA_2` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SBPI_RDATA_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sbpi_rdata_2(&self) -> SBPI_RDATA_2_R {
        SBPI_RDATA_2_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read payload bytes 11..8. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_rdata_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_RDATA_2_SPEC;
impl crate::RegisterSpec for SBPI_RDATA_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_rdata_2::R`](R) reader structure"]
impl crate::Readable for SBPI_RDATA_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbpi_rdata_2::W`](W) writer structure"]
impl crate::Writable for SBPI_RDATA_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBPI_RDATA_2 to value 0"]
impl crate::Resettable for SBPI_RDATA_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
