#[doc = "Register `PIDR4` reader"]
pub type R = crate::R<PIDR4_SPEC>;
#[doc = "Field `DES_2` reader - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
pub type DES_2_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - Always 0b0000. Indicates that the device only occupies 4KB of memory"]
pub type SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component."]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Always 0b0000. Indicates that the device only occupies 4KB of memory"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Periperal ID4  

You can [`read`](crate::Reg::read) this register and get [`pidr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr4::R`](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for PIDR4_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
