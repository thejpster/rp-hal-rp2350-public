#[doc = "Register `CTICHINSTATUS` reader"]
pub type R = crate::R<CTICHINSTATUS_SPEC>;
#[doc = "Field `CTICHOUTSTATUS` reader - Shows the status of the ctichout outputs. There is one bit of the field for each channel output"]
pub type CTICHOUTSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Shows the status of the ctichout outputs. There is one bit of the field for each channel output"]
    #[inline(always)]
    pub fn ctichoutstatus(&self) -> CTICHOUTSTATUS_R {
        CTICHOUTSTATUS_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "CTI Channel In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctichinstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTICHINSTATUS_SPEC;
impl crate::RegisterSpec for CTICHINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctichinstatus::R`](R) reader structure"]
impl crate::Readable for CTICHINSTATUS_SPEC {}
#[doc = "`reset()` method sets CTICHINSTATUS to value 0"]
impl crate::Resettable for CTICHINSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
