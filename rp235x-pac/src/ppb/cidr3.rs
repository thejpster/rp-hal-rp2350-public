#[doc = "Register `CIDR3` reader"]
pub type R = crate::R<CIDR3_SPEC>;
#[doc = "Field `PRMBL_3` reader - Preamble\\[3\\]. Contains bits\\[31:24\\]
of the component identification code."]
pub type PRMBL_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble\\[3\\]. Contains bits\\[31:24\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Component ID3  

You can [`read`](crate::Reg::read) this register and get [`cidr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR3_SPEC;
impl crate::RegisterSpec for CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr3::R`](R) reader structure"]
impl crate::Readable for CIDR3_SPEC {}
#[doc = "`reset()` method sets CIDR3 to value 0xb1"]
impl crate::Resettable for CIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
