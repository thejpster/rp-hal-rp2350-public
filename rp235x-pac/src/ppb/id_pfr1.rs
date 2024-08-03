#[doc = "Register `ID_PFR1` reader"]
pub type R = crate::R<ID_PFR1_SPEC>;
#[doc = "Field `SECURITY` reader - Identifies whether the Security Extension is implemented"]
pub type SECURITY_R = crate::FieldReader;
#[doc = "Field `MPROGMOD` reader - Identifies support for the M-Profile programmers' model support"]
pub type MPROGMOD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Identifies whether the Security Extension is implemented"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Identifies support for the M-Profile programmers' model support"]
    #[inline(always)]
    pub fn mprogmod(&self) -> MPROGMOD_R {
        MPROGMOD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Gives information about the programmers' model and Extensions support  

You can [`read`](crate::Reg::read) this register and get [`id_pfr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_PFR1_SPEC;
impl crate::RegisterSpec for ID_PFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_pfr1::R`](R) reader structure"]
impl crate::Readable for ID_PFR1_SPEC {}
#[doc = "`reset()` method sets ID_PFR1 to value 0x0520"]
impl crate::Resettable for ID_PFR1_SPEC {
    const RESET_VALUE: u32 = 0x0520;
}
