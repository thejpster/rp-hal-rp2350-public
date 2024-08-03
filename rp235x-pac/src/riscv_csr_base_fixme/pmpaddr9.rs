#[doc = "Register `PMPADDR9` reader"]
pub type R = crate::R<PMPADDR9_SPEC>;
#[doc = "Field `PMPADDR9` reader - "]
pub type PMPADDR9_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr9(&self) -> PMPADDR9_R {
        PMPADDR9_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 9. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x40000000` through `0x5fffffff`, which contains the system peripherals. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR9_SPEC;
impl crate::RegisterSpec for PMPADDR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr9::R`](R) reader structure"]
impl crate::Readable for PMPADDR9_SPEC {}
#[doc = "`reset()` method sets PMPADDR9 to value 0x13ff_ffff"]
impl crate::Resettable for PMPADDR9_SPEC {
    const RESET_VALUE: u32 = 0x13ff_ffff;
}
