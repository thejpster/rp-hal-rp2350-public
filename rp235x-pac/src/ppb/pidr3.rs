#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<PIDR3_SPEC>;
#[doc = "Field `CMOD` reader - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. ARM recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. ARM recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Periperal ID3  

You can [`read`](crate::Reg::read) this register and get [`pidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
