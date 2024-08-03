#[doc = "Register `CHIPID_WAFER0` reader"]
pub type R = crate::R<CHIPID_WAFER0_SPEC>;
#[doc = "Field `CHIPID_WAFER0` reader - "]
pub type CHIPID_WAFER0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid_wafer0(&self) -> CHIPID_WAFER0_R {
        CHIPID_WAFER0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Lower 16 bits of wafer lot (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_wafer0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID_WAFER0_SPEC;
impl crate::RegisterSpec for CHIPID_WAFER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid_wafer0::R`](R) reader structure"]
impl crate::Readable for CHIPID_WAFER0_SPEC {}
#[doc = "`reset()` method sets CHIPID_WAFER0 to value 0"]
impl crate::Resettable for CHIPID_WAFER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
