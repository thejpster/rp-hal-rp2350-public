#[doc = "Register `CTICHINSTATUS` reader"]
pub type R = crate::R<CTICHINSTATUS_SPEC>;
#[doc = "Register `CTICHINSTATUS` writer"]
pub type W = crate::W<CTICHINSTATUS_SPEC>;
#[doc = "Field `CTICHOUTSTATUS` reader - Shows the status of the ctichout outputs. There is one bit of the field for each channel output"]
pub type CTICHOUTSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Shows the status of the ctichout outputs. There is one bit of the field for each channel output"]
    #[inline(always)]
    pub fn ctichoutstatus(&self) -> CTICHOUTSTATUS_R {
        CTICHOUTSTATUS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "CTI Channel In Status Register  

You can [`read`](crate::Reg::read) this register and get [`ctichinstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctichinstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTICHINSTATUS_SPEC;
impl crate::RegisterSpec for CTICHINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctichinstatus::R`](R) reader structure"]
impl crate::Readable for CTICHINSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctichinstatus::W`](W) writer structure"]
impl crate::Writable for CTICHINSTATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTICHINSTATUS to value 0"]
impl crate::Resettable for CTICHINSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
