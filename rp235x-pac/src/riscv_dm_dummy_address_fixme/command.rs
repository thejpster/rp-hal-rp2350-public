#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<COMMAND_SPEC>;
#[doc = "Field `REGNO` writer - Number of the register to access.  

 On Hazard3 this must be in the range 0x1000 through 0x101f inclusive, referring to GPRs x0 through x31."]
pub type REGNO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE` writer - When transfer is set:  

 0: Copy data from the specified register into data0.  

 1: Copy data from data0 into the specified register."]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER` writer - 0: Don’t do the operation specified by write.  

 1: Do the operation specified by write.  

 This bit can be used to just execute the Program Buffer without having to worry about placing valid values into aarsize or regno."]
pub type TRANSFER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTEXEC` writer - 0: No effect.  

 1: Execute the program in the Program Buffer exactly once after performing the transfer, if any."]
pub type POSTEXEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AARPOSTINCREMENT` writer - On Hazard3 this field must be 0 (no post-increment of regno)"]
pub type AARPOSTINCREMENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AARSIZE` writer - On Hazard3 this field must be 2 (32-bit register access)"]
pub type AARSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMDTYPE` writer - On Hazard3 this field must be 0 (Access Register)"]
pub type CMDTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:15 - Number of the register to access.  

 On Hazard3 this must be in the range 0x1000 through 0x101f inclusive, referring to GPRs x0 through x31."]
    #[inline(always)]
    #[must_use]
    pub fn regno(&mut self) -> REGNO_W<COMMAND_SPEC> {
        REGNO_W::new(self, 0)
    }
    #[doc = "Bit 16 - When transfer is set:  

 0: Copy data from the specified register into data0.  

 1: Copy data from data0 into the specified register."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<COMMAND_SPEC> {
        WRITE_W::new(self, 16)
    }
    #[doc = "Bit 17 - 0: Don’t do the operation specified by write.  

 1: Do the operation specified by write.  

 This bit can be used to just execute the Program Buffer without having to worry about placing valid values into aarsize or regno."]
    #[inline(always)]
    #[must_use]
    pub fn transfer(&mut self) -> TRANSFER_W<COMMAND_SPEC> {
        TRANSFER_W::new(self, 17)
    }
    #[doc = "Bit 18 - 0: No effect.  

 1: Execute the program in the Program Buffer exactly once after performing the transfer, if any."]
    #[inline(always)]
    #[must_use]
    pub fn postexec(&mut self) -> POSTEXEC_W<COMMAND_SPEC> {
        POSTEXEC_W::new(self, 18)
    }
    #[doc = "Bit 19 - On Hazard3 this field must be 0 (no post-increment of regno)"]
    #[inline(always)]
    #[must_use]
    pub fn aarpostincrement(&mut self) -> AARPOSTINCREMENT_W<COMMAND_SPEC> {
        AARPOSTINCREMENT_W::new(self, 19)
    }
    #[doc = "Bits 20:22 - On Hazard3 this field must be 2 (32-bit register access)"]
    #[inline(always)]
    #[must_use]
    pub fn aarsize(&mut self) -> AARSIZE_W<COMMAND_SPEC> {
        AARSIZE_W::new(self, 20)
    }
    #[doc = "Bits 24:31 - On Hazard3 this field must be 0 (Access Register)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtype(&mut self) -> CMDTYPE_W<COMMAND_SPEC> {
        CMDTYPE_W::new(self, 24)
    }
}
#[doc = "Writes to this register cause the corresponding abstract command to be executed.  

 Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

 If cmderr is non-zero, writes to this register are ignored.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: u32 = 0;
}
