#[doc = "Register `CTITRIGINSTATUS` reader"]
pub type R = crate::R<CTITRIGINSTATUS_SPEC>;
#[doc = "Register `CTITRIGINSTATUS` writer"]
pub type W = crate::W<CTITRIGINSTATUS_SPEC>;
#[doc = "Field `TRIGINSTATUS` reader - Shows the status of the ctitrigin inputs. There is one bit of the field for each trigger input.Because the register provides a view of the raw ctitrigin inputs, the reset value is UNKNOWN."]
pub type TRIGINSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Shows the status of the ctitrigin inputs. There is one bit of the field for each trigger input.Because the register provides a view of the raw ctitrigin inputs, the reset value is UNKNOWN."]
    #[inline(always)]
    pub fn triginstatus(&self) -> TRIGINSTATUS_R {
        TRIGINSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "CTI Trigger to Channel Enable Registers  

You can [`read`](crate::Reg::read) this register and get [`ctitriginstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctitriginstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTITRIGINSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitriginstatus::R`](R) reader structure"]
impl crate::Readable for CTITRIGINSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctitriginstatus::W`](W) writer structure"]
impl crate::Writable for CTITRIGINSTATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTITRIGINSTATUS to value 0"]
impl crate::Resettable for CTITRIGINSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
