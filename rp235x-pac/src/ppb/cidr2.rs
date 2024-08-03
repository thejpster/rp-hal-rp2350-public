#[doc = "Register `CIDR2` reader"]
pub type R = crate::R<CIDR2_SPEC>;
#[doc = "Field `PRMBL_2` reader - Preamble\\[2\\]. Contains bits\\[23:16\\]
of the component identification code."]
pub type PRMBL_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble\\[2\\]. Contains bits\\[23:16\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Component ID2  

You can [`read`](crate::Reg::read) this register and get [`cidr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR2_SPEC;
impl crate::RegisterSpec for CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr2::R`](R) reader structure"]
impl crate::Readable for CIDR2_SPEC {}
#[doc = "`reset()` method sets CIDR2 to value 0x05"]
impl crate::Resettable for CIDR2_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
