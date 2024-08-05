#[doc = "Register `CTITRIGOUTSTATUS` reader"]
pub type R = crate::R<CTITRIGOUTSTATUS_SPEC>;
#[doc = "Register `CTITRIGOUTSTATUS` writer"]
pub type W = crate::W<CTITRIGOUTSTATUS_SPEC>;
#[doc = "Field `TRIGOUTSTATUS` reader - Shows the status of the ctitrigout outputs. There is one bit of the field for each trigger output."]
pub type TRIGOUTSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Shows the status of the ctitrigout outputs. There is one bit of the field for each trigger output."]
    #[inline(always)]
    pub fn trigoutstatus(&self) -> TRIGOUTSTATUS_R {
        TRIGOUTSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "CTI Trigger In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctitrigoutstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctitrigoutstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTITRIGOUTSTATUS_SPEC;
impl crate::RegisterSpec for CTITRIGOUTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctitrigoutstatus::R`](R) reader structure"]
impl crate::Readable for CTITRIGOUTSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctitrigoutstatus::W`](W) writer structure"]
impl crate::Writable for CTITRIGOUTSTATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTITRIGOUTSTATUS to value 0"]
impl crate::Resettable for CTITRIGOUTSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
