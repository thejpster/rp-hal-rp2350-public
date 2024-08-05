#[doc = "Register `ID_MMFR2` reader"]
pub type R = crate::R<ID_MMFR2_SPEC>;
#[doc = "Register `ID_MMFR2` writer"]
pub type W = crate::W<ID_MMFR2_SPEC>;
#[doc = "Field `WFISTALL` reader - Indicates the support for Wait For Interrupt (WFI) stalling"]
pub type WFISTALL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 24:27 - Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub fn wfistall(&self) -> WFISTALL_R {
        WFISTALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Provides information about the implemented memory model and memory management support  

You can [`read`](crate::Reg::read) this register and get [`id_mmfr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_mmfr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_MMFR2_SPEC;
impl crate::RegisterSpec for ID_MMFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_mmfr2::R`](R) reader structure"]
impl crate::Readable for ID_MMFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`id_mmfr2::W`](W) writer structure"]
impl crate::Writable for ID_MMFR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_MMFR2 to value 0x0100_0000"]
impl crate::Resettable for ID_MMFR2_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
