#[doc = "Register `PMPADDR11` reader"]
pub type R = crate::R<PMPADDR11_SPEC>;
#[doc = "Field `PMPADDR11` reader - "]
pub type PMPADDR11_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr11(&self) -> PMPADDR11_R {
        PMPADDR11_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 11. Note all PMP addresses are in units of four bytes.  

 Hardwired to all-zeroes. This region is not implemented.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR11_SPEC;
impl crate::RegisterSpec for PMPADDR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr11::R`](R) reader structure"]
impl crate::Readable for PMPADDR11_SPEC {}
#[doc = "`reset()` method sets PMPADDR11 to value 0"]
impl crate::Resettable for PMPADDR11_SPEC {
    const RESET_VALUE: u32 = 0;
}
