#[doc = "Register `CTITRIGOUTSTATUS` reader"]
pub type R = crate::R<CTITRIGOUTSTATUS_SPEC>;
#[doc = "Field `TRIGOUTSTATUS` reader - Shows the status of the ctitrigout outputs. There is one bit of the field for each trigger output."]
pub type TRIGOUTSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Shows the status of the ctitrigout outputs. There is one bit of the field for each trigger output."]
    #[inline(always)]
    pub fn trigoutstatus(&self) -> TRIGOUTSTATUS_R {
        TRIGOUTSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CTI Trigger In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctitrigoutstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTITRIGOUTSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGOUTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitrigoutstatus::R`](R) reader structure"]
impl crate::Readable for CTITRIGOUTSTATUS_SPEC {}
#[doc = "`reset()` method sets CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for CTITRIGOUTSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
