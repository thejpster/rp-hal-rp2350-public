#[doc = "Register `PMPADDR10` reader"]
pub type R = crate::R<PMPADDR10_SPEC>;
#[doc = "Field `PMPADDR10` reader - "]
pub type PMPADDR10_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr10(&self) -> PMPADDR10_R {
        PMPADDR10_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 10. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0xd0000000` through `0xdfffffff`, which contains the core-local peripherals (SIO). This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR10_SPEC;
impl crate::RegisterSpec for PMPADDR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr10::R`](R) reader structure"]
impl crate::Readable for PMPADDR10_SPEC {}
#[doc = "`reset()` method sets PMPADDR10 to value 0x35ff_ffff"]
impl crate::Resettable for PMPADDR10_SPEC {
    const RESET_VALUE: u32 = 0x35ff_ffff;
}
