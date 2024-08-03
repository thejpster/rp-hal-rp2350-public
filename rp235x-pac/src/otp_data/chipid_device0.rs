#[doc = "Register `CHIPID_DEVICE0` reader"]
pub type R = crate::R<CHIPID_DEVICE0_SPEC>;
#[doc = "Field `CHIPID_DEVICE0` reader - "]
pub type CHIPID_DEVICE0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid_device0(&self) -> CHIPID_DEVICE0_R {
        CHIPID_DEVICE0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Lower 16 bits of test lot/device number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid_device0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID_DEVICE0_SPEC;
impl crate::RegisterSpec for CHIPID_DEVICE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid_device0::R`](R) reader structure"]
impl crate::Readable for CHIPID_DEVICE0_SPEC {}
#[doc = "`reset()` method sets CHIPID_DEVICE0 to value 0"]
impl crate::Resettable for CHIPID_DEVICE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
