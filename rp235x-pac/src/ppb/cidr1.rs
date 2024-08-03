#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<CIDR1_SPEC>;
#[doc = "Field `PRMBL_1` reader - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code."]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble\\[1\\]. Contains bits\\[11:8\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\]
of the component identification code."]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "CoreSight Component ID1  

You can [`read`](crate::Reg::read) this register and get [`cidr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR1_SPEC;
impl crate::RegisterSpec for CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for CIDR1_SPEC {}
#[doc = "`reset()` method sets CIDR1 to value 0x90"]
impl crate::Resettable for CIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
