#[doc = "Register `ITM_PIDR2` reader"]
pub type R = crate::R<ITM_PIDR2_SPEC>;
#[doc = "Register `ITM_PIDR2` writer"]
pub type W = crate::W<ITM_PIDR2_SPEC>;
#[doc = "Field `DES_1` reader - See CoreSight Architecture Specification"]
pub type DES_1_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - See CoreSight Architecture Specification"]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - See CoreSight Architecture Specification"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the ITM  

You can [`read`](crate::Reg::read) this register and get [`itm_pidr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itm_pidr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITM_PIDR2_SPEC;
impl crate::RegisterSpec for ITM_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itm_pidr2::R`](R) reader structure"]
impl crate::Readable for ITM_PIDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itm_pidr2::W`](W) writer structure"]
impl crate::Writable for ITM_PIDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITM_PIDR2 to value 0x0b"]
impl crate::Resettable for ITM_PIDR2_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
