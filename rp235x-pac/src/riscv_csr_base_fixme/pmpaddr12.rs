#[doc = "Register `PMPADDR12` reader"]
pub type R = crate::R<PMPADDR12_SPEC>;
#[doc = "Field `PMPADDR12` reader - "]
pub type PMPADDR12_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr12(&self) -> PMPADDR12_R {
        PMPADDR12_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 12. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR12_SPEC;
impl crate::RegisterSpec for PMPADDR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr12::R`](R) reader structure"]
impl crate::Readable for PMPADDR12_SPEC {}
#[doc = "`reset()` method sets PMPADDR12 to value 0"]
impl crate::Resettable for PMPADDR12_SPEC {
    const RESET_VALUE: u32 = 0;
}
