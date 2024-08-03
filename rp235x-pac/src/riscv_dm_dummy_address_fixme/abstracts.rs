#[doc = "Register `ABSTRACTS` reader"]
pub type R = crate::R<ABSTRACTS_SPEC>;
#[doc = "Register `ABSTRACTS` writer"]
pub type W = crate::W<ABSTRACTS_SPEC>;
#[doc = "Field `DATACOUNT` reader - Number of data registers that are implemented as part of the abstract command interface."]
pub type DATACOUNT_R = crate::FieldReader;
#[doc = "Field `CMDERR` reader - Gets set if an abstract command fails. The bits in this field remain set until they are cleared by writing 1 to them. No abstract command is started until the value is reset to 0.  

 This field only contains a valid value if busy is 0.  

 0 (none): No error.  

 1 (busy): An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0.  

 2 (not supported): The requested command is not supported, regardless of whether the hart is running or not.  

 3 (exception): An exception occurred while executing the command (e.g. while executing the Program Buffer).  

 4 (halt/resume): The abstract command couldn’t execute because the hart wasn’t in the required state (running/halted), or unavailable.  

 5 (bus): The abstract command failed due to a bus error (e.g. alignment, access size, or timeout).  

 7 (other): The command failed for another reason.  

 Note: Hazard3 does not set values 5 or 7. Load/store instructions in the program buffer raise an exception when they encounter a bus fault, setting cmderr=3."]
pub type CMDERR_R = crate::FieldReader;
#[doc = "Field `CMDERR` writer - Gets set if an abstract command fails. The bits in this field remain set until they are cleared by writing 1 to them. No abstract command is started until the value is reset to 0.  

 This field only contains a valid value if busy is 0.  

 0 (none): No error.  

 1 (busy): An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0.  

 2 (not supported): The requested command is not supported, regardless of whether the hart is running or not.  

 3 (exception): An exception occurred while executing the command (e.g. while executing the Program Buffer).  

 4 (halt/resume): The abstract command couldn’t execute because the hart wasn’t in the required state (running/halted), or unavailable.  

 5 (bus): The abstract command failed due to a bus error (e.g. alignment, access size, or timeout).  

 7 (other): The command failed for another reason.  

 Note: Hazard3 does not set values 5 or 7. Load/store instructions in the program buffer raise an exception when they encounter a bus fault, setting cmderr=3."]
pub type CMDERR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSY` reader - 1: An abstract command is currently being executed.  

 This bit is set as soon as command is written, and is not cleared until that command has completed."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - 1: An abstract command is currently being executed.  

 This bit is set as soon as command is written, and is not cleared until that command has completed."]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGBUFSIZE` reader - Size of the Program Buffer, in 32-bit words."]
pub type PROGBUFSIZE_R = crate::FieldReader;
#[doc = "Field `PROGBUFSIZE` writer - Size of the Program Buffer, in 32-bit words."]
pub type PROGBUFSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Number of data registers that are implemented as part of the abstract command interface."]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Gets set if an abstract command fails. The bits in this field remain set until they are cleared by writing 1 to them. No abstract command is started until the value is reset to 0.  

 This field only contains a valid value if busy is 0.  

 0 (none): No error.  

 1 (busy): An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0.  

 2 (not supported): The requested command is not supported, regardless of whether the hart is running or not.  

 3 (exception): An exception occurred while executing the command (e.g. while executing the Program Buffer).  

 4 (halt/resume): The abstract command couldn’t execute because the hart wasn’t in the required state (running/halted), or unavailable.  

 5 (bus): The abstract command failed due to a bus error (e.g. alignment, access size, or timeout).  

 7 (other): The command failed for another reason.  

 Note: Hazard3 does not set values 5 or 7. Load/store instructions in the program buffer raise an exception when they encounter a bus fault, setting cmderr=3."]
    #[inline(always)]
    pub fn cmderr(&self) -> CMDERR_R {
        CMDERR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - 1: An abstract command is currently being executed.  

 This bit is set as soon as command is written, and is not cleared until that command has completed."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Size of the Program Buffer, in 32-bit words."]
    #[inline(always)]
    pub fn progbufsize(&self) -> PROGBUFSIZE_R {
        PROGBUFSIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Gets set if an abstract command fails. The bits in this field remain set until they are cleared by writing 1 to them. No abstract command is started until the value is reset to 0.  

 This field only contains a valid value if busy is 0.  

 0 (none): No error.  

 1 (busy): An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0.  

 2 (not supported): The requested command is not supported, regardless of whether the hart is running or not.  

 3 (exception): An exception occurred while executing the command (e.g. while executing the Program Buffer).  

 4 (halt/resume): The abstract command couldn’t execute because the hart wasn’t in the required state (running/halted), or unavailable.  

 5 (bus): The abstract command failed due to a bus error (e.g. alignment, access size, or timeout).  

 7 (other): The command failed for another reason.  

 Note: Hazard3 does not set values 5 or 7. Load/store instructions in the program buffer raise an exception when they encounter a bus fault, setting cmderr=3."]
    #[inline(always)]
    #[must_use]
    pub fn cmderr(&mut self) -> CMDERR_W<ABSTRACTS_SPEC> {
        CMDERR_W::new(self, 8)
    }
    #[doc = "Bit 12 - 1: An abstract command is currently being executed.  

 This bit is set as soon as command is written, and is not cleared until that command has completed."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<ABSTRACTS_SPEC> {
        BUSY_W::new(self, 12)
    }
    #[doc = "Bits 24:28 - Size of the Program Buffer, in 32-bit words."]
    #[inline(always)]
    #[must_use]
    pub fn progbufsize(&mut self) -> PROGBUFSIZE_W<ABSTRACTS_SPEC> {
        PROGBUFSIZE_W::new(self, 24)
    }
}
#[doc = "Abstract Control and Status. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

You can [`read`](crate::Reg::read) this register and get [`abstracts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstracts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABSTRACTS_SPEC;
impl crate::RegisterSpec for ABSTRACTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abstracts::R`](R) reader structure"]
impl crate::Readable for ABSTRACTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`abstracts::W`](W) writer structure"]
impl crate::Writable for ABSTRACTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0700;
}
#[doc = "`reset()` method sets ABSTRACTS to value 0x0200_0001"]
impl crate::Resettable for ABSTRACTS_SPEC {
    const RESET_VALUE: u32 = 0x0200_0001;
}
