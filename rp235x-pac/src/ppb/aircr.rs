#[doc = "Register `AIRCR` reader"]
pub type R = crate::R<AIRCR_SPEC>;
#[doc = "Register `AIRCR` writer"]
pub type W = crate::W<AIRCR_SPEC>;
#[doc = "Field `VECTCLRACTIVE` reader - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_R = crate::BitReader;
#[doc = "Field `VECTCLRACTIVE` writer - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
pub type VECTCLRACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQ` reader - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_R = crate::BitReader;
#[doc = "Field `SYSRESETREQ` writer - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
pub type SYSRESETREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQS` reader - System reset request, Secure state only. 0 SYSRESETREQ functionality is available to both Security states. 1 SYSRESETREQ functionality is only available to Secure state."]
pub type SYSRESETREQS_R = crate::BitReader;
#[doc = "Field `SYSRESETREQS` writer - System reset request, Secure state only. 0 SYSRESETREQ functionality is available to both Security states. 1 SYSRESETREQ functionality is only available to Secure state."]
pub type SYSRESETREQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping field. This field determines the split of group priority from subpriority. See https://developer.arm.com/documentation/100235/0004/the-cortex-m33-peripherals/system-control-block/application-interrupt-and-reset-control-register?lang=en"]
pub type PRIGROUP_R = crate::FieldReader;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping field. This field determines the split of group priority from subpriority. See https://developer.arm.com/documentation/100235/0004/the-cortex-m33-peripherals/system-control-block/application-interrupt-and-reset-control-register?lang=en"]
pub type PRIGROUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BFHFNMINS` reader - BusFault, HardFault, and NMI Non-secure enable. 0 BusFault, HardFault, and NMI are Secure. 1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BFHFNMINS_R = crate::BitReader;
#[doc = "Field `BFHFNMINS` writer - BusFault, HardFault, and NMI Non-secure enable. 0 BusFault, HardFault, and NMI are Secure. 1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
pub type BFHFNMINS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIS` reader - Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. 0 Priority ranges of Secure and Non-secure exceptions are identical. 1 Non-secure exceptions are de-prioritized."]
pub type PRIS_R = crate::BitReader;
#[doc = "Field `PRIS` writer - Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. 0 Priority ranges of Secure and Non-secure exceptions are identical. 1 Non-secure exceptions are de-prioritized."]
pub type PRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIANESS` reader - Data endianness implemented: 0 = Little-endian."]
pub type ENDIANESS_R = crate::BitReader;
#[doc = "Field `VECTKEY` reader - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_R = crate::FieldReader<u16>;
#[doc = "Field `VECTKEY` writer - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
pub type VECTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    #[inline(always)]
    pub fn vectclractive(&self) -> VECTCLRACTIVE_R {
        VECTCLRACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SYSRESETREQ_R {
        SYSRESETREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System reset request, Secure state only. 0 SYSRESETREQ functionality is available to both Security states. 1 SYSRESETREQ functionality is only available to Secure state."]
    #[inline(always)]
    pub fn sysresetreqs(&self) -> SYSRESETREQS_R {
        SYSRESETREQS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority. See https://developer.arm.com/documentation/100235/0004/the-cortex-m33-peripherals/system-control-block/application-interrupt-and-reset-control-register?lang=en"]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - BusFault, HardFault, and NMI Non-secure enable. 0 BusFault, HardFault, and NMI are Secure. 1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    pub fn bfhfnmins(&self) -> BFHFNMINS_R {
        BFHFNMINS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. 0 Priority ranges of Secure and Non-secure exceptions are identical. 1 Non-secure exceptions are de-prioritized."]
    #[inline(always)]
    pub fn pris(&self) -> PRIS_R {
        PRIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data endianness implemented: 0 = Little-endian."]
    #[inline(always)]
    pub fn endianess(&self) -> ENDIANESS_R {
        ENDIANESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<AIRCR_SPEC> {
        VECTCLRACTIVE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<AIRCR_SPEC> {
        SYSRESETREQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - System reset request, Secure state only. 0 SYSRESETREQ functionality is available to both Security states. 1 SYSRESETREQ functionality is only available to Secure state."]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreqs(&mut self) -> SYSRESETREQS_W<AIRCR_SPEC> {
        SYSRESETREQS_W::new(self, 3)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority. See https://developer.arm.com/documentation/100235/0004/the-cortex-m33-peripherals/system-control-block/application-interrupt-and-reset-control-register?lang=en"]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<AIRCR_SPEC> {
        PRIGROUP_W::new(self, 8)
    }
    #[doc = "Bit 13 - BusFault, HardFault, and NMI Non-secure enable. 0 BusFault, HardFault, and NMI are Secure. 1 BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmins(&mut self) -> BFHFNMINS_W<AIRCR_SPEC> {
        BFHFNMINS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. 0 Priority ranges of Secure and Non-secure exceptions are identical. 1 Non-secure exceptions are de-prioritized."]
    #[inline(always)]
    #[must_use]
    pub fn pris(&mut self) -> PRIS_W<AIRCR_SPEC> {
        PRIS_W::new(self, 14)
    }
    #[doc = "Bits 16:31 - Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<AIRCR_SPEC> {
        VECTKEY_W::new(self, 16)
    }
}
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset.  

You can [`read`](crate::Reg::read) this register and get [`aircr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aircr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aircr::R`](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aircr::W`](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
