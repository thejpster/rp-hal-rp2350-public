#[doc = "Register `ID_DFR0` reader"]
pub type R = crate::R<ID_DFR0_SPEC>;
#[doc = "Field `MPROFDBG` reader - Indicates the supported M-profile debug architecture"]
pub type MPROFDBG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 20:23 - Indicates the supported M-profile debug architecture"]
    #[inline(always)]
    pub fn mprofdbg(&self) -> MPROFDBG_R {
        MPROFDBG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Provides top level information about the debug system  

You can [`read`](crate::Reg::read) this register and get [`id_dfr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_DFR0_SPEC;
impl crate::RegisterSpec for ID_DFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_dfr0::R`](R) reader structure"]
impl crate::Readable for ID_DFR0_SPEC {}
#[doc = "`reset()` method sets ID_DFR0 to value 0x0020_0000"]
impl crate::Resettable for ID_DFR0_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
