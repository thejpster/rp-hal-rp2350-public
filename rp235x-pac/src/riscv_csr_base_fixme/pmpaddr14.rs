#[doc = "Register `PMPADDR14` reader"]
pub type R = crate::R<PMPADDR14_SPEC>;
#[doc = "Field `PMPADDR14` reader - "]
pub type PMPADDR14_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr14(&self) -> PMPADDR14_R {
        PMPADDR14_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 14. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR14_SPEC;
impl crate::RegisterSpec for PMPADDR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr14::R`](R) reader structure"]
impl crate::Readable for PMPADDR14_SPEC {}
#[doc = "`reset()` method sets PMPADDR14 to value 0"]
impl crate::Resettable for PMPADDR14_SPEC {
    const RESET_VALUE: u32 = 0;
}
