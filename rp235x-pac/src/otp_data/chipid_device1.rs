#[doc = "Register `CHIPID_DEVICE1` reader"]
pub type R = crate::R<CHIPID_DEVICE1_SPEC>;
#[doc = "Field `CHIPID_DEVICE1` reader - "]
pub type CHIPID_DEVICE1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid_device1(&self) -> CHIPID_DEVICE1_R {
        CHIPID_DEVICE1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Upper 16 bits of test lot/device number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_device1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID_DEVICE1_SPEC;
impl crate::RegisterSpec for CHIPID_DEVICE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid_device1::R`](R) reader structure"]
impl crate::Readable for CHIPID_DEVICE1_SPEC {}
#[doc = "`reset()` method sets CHIPID_DEVICE1 to value 0"]
impl crate::Resettable for CHIPID_DEVICE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
