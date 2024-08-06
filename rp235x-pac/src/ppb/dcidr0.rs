#[doc = "Register `DCIDR0` reader"]
pub type R = crate::R<DCIDR0_SPEC>;
#[doc = "Register `DCIDR0` writer"]
pub type W = crate::W<DCIDR0_SPEC>;
#[doc = "Field `PRMBL_0` reader - See CoreSight Architecture Specification"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCIDR0_SPEC;
impl crate::RegisterSpec for DCIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcidr0::R`](R) reader structure"]
impl crate::Readable for DCIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcidr0::W`](W) writer structure"]
impl crate::Writable for DCIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCIDR0 to value 0x0d"]
impl crate::Resettable for DCIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
