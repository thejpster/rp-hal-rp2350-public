#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<PIDR2_SPEC>;
#[doc = "Register `PIDR2` writer"]
pub type W = crate::W<PIDR2_SPEC>;
#[doc = "Field `DES_1` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type DES_1_R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
pub type JEDEC_R = crate::BitReader;
#[doc = "Field `REVISION` reader - This device is at r1p0"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Always 1. Indicates that the JEDEC-assigned designer ID is used."]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This device is at r1p0"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "CoreSight Peripheral ID2  

You can [`read`](crate::Reg::read) this register and get [`pidr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pidr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pidr2::W`](W) writer structure"]
impl crate::Writable for PIDR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR2 to value 0x0b"]
impl crate::Resettable for PIDR2_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
