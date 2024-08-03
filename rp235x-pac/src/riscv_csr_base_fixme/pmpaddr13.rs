#[doc = "Register `PMPADDR13` reader"]
pub type R = crate::R<PMPADDR13_SPEC>;
#[doc = "Field `PMPADDR13` reader - "]
pub type PMPADDR13_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr13(&self) -> PMPADDR13_R {
        PMPADDR13_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 13. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR13_SPEC;
impl crate::RegisterSpec for PMPADDR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr13::R`](R) reader structure"]
impl crate::Readable for PMPADDR13_SPEC {}
#[doc = "`reset()` method sets PMPADDR13 to value 0"]
impl crate::Resettable for PMPADDR13_SPEC {
    const RESET_VALUE: u32 = 0;
}
