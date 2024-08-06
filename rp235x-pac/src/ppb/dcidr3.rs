#[doc = "Register `DCIDR3` reader"]
pub type R = crate::R<DCIDR3_SPEC>;
#[doc = "Register `DCIDR3` writer"]
pub type W = crate::W<DCIDR3_SPEC>;
#[doc = "Field `PRMBL_3` reader - See CoreSight Architecture Specification"]
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the SCS  

You can [`read`](crate::Reg::read) this register and get [`dcidr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcidr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCIDR3_SPEC;
impl crate::RegisterSpec for DCIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcidr3::R`](R) reader structure"]
impl crate::Readable for DCIDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcidr3::W`](W) writer structure"]
impl crate::Writable for DCIDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCIDR3 to value 0xb1"]
impl crate::Resettable for DCIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
