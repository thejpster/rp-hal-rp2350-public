#[doc = "Register `PMPADDR8` reader"]
pub type R = crate::R<PMPADDR8_SPEC>;
#[doc = "Field `PMPADDR8` reader - "]
pub type PMPADDR8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn pmpaddr8(&self) -> PMPADDR8_R {
        PMPADDR8_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Physical memory protection address for region 8. Note all PMP addresses are in units of four bytes.  

 Hardwired to the address range `0x00000000` through `0x0fffffff`, which contains the boot ROM. This range is made accessible to User mode by default. User mode access to this range can be disabled using one of the dynamically configurable PMP regions, or using the permission registers in ACCESSCTRL.  

You can [`read`](crate::Reg::read) this register and get [`pmpaddr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPADDR8_SPEC;
impl crate::RegisterSpec for PMPADDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpaddr8::R`](R) reader structure"]
impl crate::Readable for PMPADDR8_SPEC {}
#[doc = "`reset()` method sets PMPADDR8 to value 0x01ff_ffff"]
impl crate::Resettable for PMPADDR8_SPEC {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
