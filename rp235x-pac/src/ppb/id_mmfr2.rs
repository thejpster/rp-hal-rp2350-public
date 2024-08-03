#[doc = "Register `ID_MMFR2` reader"]
pub type R = crate::R<ID_MMFR2_SPEC>;
#[doc = "Field `WFISTALL` reader - Indicates the support for Wait For Interrupt (WFI) stalling"]
pub type WFISTALL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 24:27 - Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub fn wfistall(&self) -> WFISTALL_R {
        WFISTALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_MMFR2_SPEC;
impl crate::RegisterSpec for ID_MMFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr2::R`](R) reader structure"]
impl crate::Readable for ID_MMFR2_SPEC {}
#[doc = "`reset()` method sets ID_MMFR2 to value 0x0100_0000"]
impl crate::Resettable for ID_MMFR2_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
