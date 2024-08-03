#[doc = "Register `ITM_PIDR4` reader"]
pub type R = crate::R<ITM_PIDR4_SPEC>;
#[doc = "Field `DES_2` reader - See CoreSight Architecture Specification"]
pub type DES_2_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - See CoreSight Architecture Specification"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_PIDR4_SPEC;
impl crate::RegisterSpec for ITM_PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_pidr4::R`](R) reader structure"]
impl crate::Readable for ITM_PIDR4_SPEC {}
#[doc = "`reset()` method sets ITM_PIDR4 to value 0x04"]
impl crate::Resettable for ITM_PIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
