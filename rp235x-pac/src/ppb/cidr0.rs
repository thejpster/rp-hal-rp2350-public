#[doc = "Register `CIDR0` reader"]
pub type R = crate::R<CIDR0_SPEC>;
#[doc = "Field `PRMBL_0` reader - Preamble\\[0\\]. Contains bits\\[7:0\\]
of the component identification code"]
pub type PRMBL_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble\\[0\\]. Contains bits\\[7:0\\]
of the component identification code"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CoreSight Component ID0  

You can [`read`](crate::Reg::read) this register and get [`cidr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr0::R`](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
