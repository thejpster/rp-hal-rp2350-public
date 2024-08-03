#[doc = "Register `CTITRIGINSTATUS` reader"]
pub type R = crate::R<CTITRIGINSTATUS_SPEC>;
#[doc = "Field `TRIGINSTATUS` reader - Shows the status of the ctitrigin inputs. There is one bit of the field for each trigger input.Because the register provides a view of the raw ctitrigin inputs, the reset value is UNKNOWN."]
pub type TRIGINSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Shows the status of the ctitrigin inputs. There is one bit of the field for each trigger input.Because the register provides a view of the raw ctitrigin inputs, the reset value is UNKNOWN."]
    #[inline(always)]
    pub fn triginstatus(&self) -> TRIGINSTATUS_R {
        TRIGINSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctitriginstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTITRIGINSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitriginstatus::R`](R) reader structure"]
impl crate::Readable for CTITRIGINSTATUS_SPEC {}
#[doc = "`reset()` method sets CTITRIGINSTATUS to value 0"]
impl crate::Resettable for CTITRIGINSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
