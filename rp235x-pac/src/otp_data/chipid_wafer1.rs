#[doc = "Register `CHIPID_WAFER1` reader"]
pub type R = crate::R<CHIPID_WAFER1_SPEC>;
#[doc = "Field `CHIPID_WAFER1` reader - "]
pub type CHIPID_WAFER1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid_wafer1(&self) -> CHIPID_WAFER1_R {
        CHIPID_WAFER1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Upper 16 bits of wafer lot (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_wafer1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID_WAFER1_SPEC;
impl crate::RegisterSpec for CHIPID_WAFER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid_wafer1::R`](R) reader structure"]
impl crate::Readable for CHIPID_WAFER1_SPEC {}
#[doc = "`reset()` method sets CHIPID_WAFER1 to value 0"]
impl crate::Resettable for CHIPID_WAFER1_SPEC {
    const RESET_VALUE: u32 = 0;
}
