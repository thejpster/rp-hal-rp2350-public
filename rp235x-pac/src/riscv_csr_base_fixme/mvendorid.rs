#[doc = "Register `MVENDORID` reader"]
pub type R = crate::R<MVENDORID_SPEC>;
#[doc = "Field `OFFSET` reader - "]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `BANK` reader - "]
pub type BANK_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[doc = "Vendor ID  

You can [`read`](crate::Reg::read) this register and get [`mvendorid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MVENDORID_SPEC;
impl crate::RegisterSpec for MVENDORID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvendorid::R`](R) reader structure"]
impl crate::Readable for MVENDORID_SPEC {}
#[doc = "`reset()` method sets MVENDORID to value 0"]
impl crate::Resettable for MVENDORID_SPEC {
    const RESET_VALUE: u32 = 0;
}
