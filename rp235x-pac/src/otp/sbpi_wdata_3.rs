#[doc = "Register `SBPI_WDATA_3` reader"]
pub type R = crate::R<SBPI_WDATA_3_SPEC>;
#[doc = "Register `SBPI_WDATA_3` writer"]
pub type W = crate::W<SBPI_WDATA_3_SPEC>;
#[doc = "Field `SBPI_WDATA_3` reader - "]
pub type SBPI_WDATA_3_R = crate::FieldReader<u32>;
#[doc = "Field `SBPI_WDATA_3` writer - "]
pub type SBPI_WDATA_3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sbpi_wdata_3(&self) -> SBPI_WDATA_3_R {
        SBPI_WDATA_3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sbpi_wdata_3(&mut self) -> SBPI_WDATA_3_W<SBPI_WDATA_3_SPEC> {
        SBPI_WDATA_3_W::new(self, 0)
    }
}
#[doc = "SBPI write payload bytes 15..12  

You can [`read`](crate::Reg::read) this register and get [`sbpi_wdata_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbpi_wdata_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_WDATA_3_SPEC;
impl crate::RegisterSpec for SBPI_WDATA_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_wdata_3::R`](R) reader structure"]
impl crate::Readable for SBPI_WDATA_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbpi_wdata_3::W`](W) writer structure"]
impl crate::Writable for SBPI_WDATA_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBPI_WDATA_3 to value 0"]
impl crate::Resettable for SBPI_WDATA_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
