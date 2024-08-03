#[doc = "Register `PMPADDR15` reader"]
pub type R = crate::R<PMPADDR15_SPEC>;
#[doc = "Field `PMPADDR15` reader - "]
pub type PMPADDR15_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr15(&self) -> PMPADDR15_R {
        PMPADDR15_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 15. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR15_SPEC;
impl crate::RegisterSpec for PMPADDR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr15::R`](R) reader structure"]
impl crate::Readable for PMPADDR15_SPEC {}
#[doc = "`reset()` method sets PMPADDR15 to value 0"]
impl crate::Resettable for PMPADDR15_SPEC {
    const RESET_VALUE: u32 = 0;
}
