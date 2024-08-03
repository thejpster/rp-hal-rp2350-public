#[doc = "Register `ABSTRACTAUTO` reader"]
pub type R = crate::R<ABSTRACTAUTO_SPEC>;
#[doc = "Register `ABSTRACTAUTO` writer"]
pub type W = crate::W<ABSTRACTAUTO_SPEC>;
#[doc = "Field `AUTOEXEDDATA` reader - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again.  

 Hazard3 implements only the least-significant bit of this field."]
pub type AUTOEXEDDATA_R = crate::FieldReader<u16>;
#[doc = "Field `AUTOEXEDDATA` writer - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again.  

 Hazard3 implements only the least-significant bit of this field."]
pub type AUTOEXEDDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AUTOEXECPROGBUF` reader - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again.  

 Hazard3 implements only the two least-significant bits of this field (for the two-entry progbuf)"]
pub type AUTOEXECPROGBUF_R = crate::FieldReader<u32>;
#[doc = "Field `AUTOEXECPROGBUF` writer - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again.  

 Hazard3 implements only the two least-significant bits of this field (for the two-entry progbuf)"]
pub type AUTOEXECPROGBUF_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:11 - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again.  

 Hazard3 implements only the least-significant bit of this field."]
    #[inline(always)]
    pub fn autoexeddata(&self) -> AUTOEXEDDATA_R {
        AUTOEXEDDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 15:31 - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again.  

 Hazard3 implements only the two least-significant bits of this field (for the two-entry progbuf)"]
    #[inline(always)]
    pub fn autoexecprogbuf(&self) -> AUTOEXECPROGBUF_R {
        AUTOEXECPROGBUF_R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - When a bit in this field is 1, read or write accesses to the corresponding data word cause the command in command to be executed again.  

 Hazard3 implements only the least-significant bit of this field."]
    #[inline(always)]
    #[must_use]
    pub fn autoexeddata(&mut self) -> AUTOEXEDDATA_W<ABSTRACTAUTO_SPEC> {
        AUTOEXEDDATA_W::new(self, 0)
    }
    #[doc = "Bits 15:31 - When a bit in this field is 1, read or write accesses to the corresponding progbuf word cause the command in command to be executed again.  

 Hazard3 implements only the two least-significant bits of this field (for the two-entry progbuf)"]
    #[inline(always)]
    #[must_use]
    pub fn autoexecprogbuf(&mut self) -> AUTOEXECPROGBUF_W<ABSTRACTAUTO_SPEC> {
        AUTOEXECPROGBUF_W::new(self, 15)
    }
}
#[doc = "Abstract Command Autoexec. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

You can [`read`](crate::Reg::read) this register and get [`abstractauto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractauto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABSTRACTAUTO_SPEC;
impl crate::RegisterSpec for ABSTRACTAUTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abstractauto::R`](R) reader structure"]
impl crate::Readable for ABSTRACTAUTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`abstractauto::W`](W) writer structure"]
impl crate::Writable for ABSTRACTAUTO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABSTRACTAUTO to value 0"]
impl crate::Resettable for ABSTRACTAUTO_SPEC {
    const RESET_VALUE: u32 = 0;
}
