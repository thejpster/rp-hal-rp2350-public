#[doc = "Register `DMCONTROL` reader"]
pub type R = crate::R<DMCONTROL_SPEC>;
#[doc = "Register `DMCONTROL` writer"]
pub type W = crate::W<DMCONTROL_SPEC>;
#[doc = "Field `DMACTIVE` reader - This bit serves as a reset signal for the Debug Module itself.  

 0: The module’s state, including authentication mechanism, takes its reset values (the dmactive bit is the only bit which can be written to something other than its reset value).  

 1: The module functions normally.  

 No other mechanism should exist that may result in resetting the Debug Module after power up, with the possible (but not recommended) exception of a global reset signal that resets the entire platform.  

 (On RP2350, the Debug Module is reset by a power-on reset, a brownout reset, the RUN pin, and a rescue reset.)  

 A debugger may pulse this bit low to get the Debug Module into a known state."]
pub type DMACTIVE_R = crate::BitReader;
#[doc = "Field `DMACTIVE` writer - This bit serves as a reset signal for the Debug Module itself.  

 0: The module’s state, including authentication mechanism, takes its reset values (the dmactive bit is the only bit which can be written to something other than its reset value).  

 1: The module functions normally.  

 No other mechanism should exist that may result in resetting the Debug Module after power up, with the possible (but not recommended) exception of a global reset signal that resets the entire platform.  

 (On RP2350, the Debug Module is reset by a power-on reset, a brownout reset, the RUN pin, and a rescue reset.)  

 A debugger may pulse this bit low to get the Debug Module into a known state."]
pub type DMACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMRESET` reader - This bit controls the reset signal from the DM to the rest of the system. The signal should reset every part of the system, including every hart, except for the DM and any logic required to access the DM. To perform a system reset the debugger writes 1, and then writes 0 to deassert the reset.  

 On RP2350 this performs a cold reset, the equivalent of a watchdog reset with all WDSEL bits set. This includes both cores and all peripherals."]
pub type NDMRESET_R = crate::BitReader;
#[doc = "Field `NDMRESET` writer - This bit controls the reset signal from the DM to the rest of the system. The signal should reset every part of the system, including every hart, except for the DM and any logic required to access the DM. To perform a system reset the debugger writes 1, and then writes 0 to deassert the reset.  

 On RP2350 this performs a cold reset, the equivalent of a watchdog reset with all WDSEL bits set. This includes both cores and all peripherals."]
pub type NDMRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRRESETHALTREQ` reader - This optional field clears the halt-on-reset request bit for all currently selected harts.  

 Writes apply to the new value of hartsel and hasel."]
pub type CLRRESETHALTREQ_R = crate::BitReader;
#[doc = "Field `CLRRESETHALTREQ` writer - This optional field clears the halt-on-reset request bit for all currently selected harts.  

 Writes apply to the new value of hartsel and hasel."]
pub type CLRRESETHALTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRESETHALTREQ` reader - This optional field writes the halt-on-reset request bit for all currently selected harts, unless clrresethaltreq is simultaneously set to 1.  

 When set to 1, each selected hart will halt upon the next deassertion of its reset. The halt-on-reset request bit is not automatically cleared. The debugger must write to clrresethaltreq to clear it.  

 Writes apply to the new value of hartsel and hasel."]
pub type SETRESETHALTREQ_R = crate::BitReader;
#[doc = "Field `SETRESETHALTREQ` writer - This optional field writes the halt-on-reset request bit for all currently selected harts, unless clrresethaltreq is simultaneously set to 1.  

 When set to 1, each selected hart will halt upon the next deassertion of its reset. The halt-on-reset request bit is not automatically cleared. The debugger must write to clrresethaltreq to clear it.  

 Writes apply to the new value of hartsel and hasel."]
pub type SETRESETHALTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARTSELHI` reader - The high 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On Hazard3 this field is always tied to all-zeroes."]
pub type HARTSELHI_R = crate::FieldReader<u16>;
#[doc = "Field `HARTSELHI` writer - The high 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On Hazard3 this field is always tied to all-zeroes."]
pub type HARTSELHI_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HARTSELLO` reader - The low 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On RP2350, since there are only two cores (with one hart each), only the least-significant bit is writable. The others are tied to 0."]
pub type HARTSELLO_R = crate::FieldReader<u16>;
#[doc = "Field `HARTSELLO` writer - The low 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On RP2350, since there are only two cores (with one hart each), only the least-significant bit is writable. The others are tied to 0."]
pub type HARTSELLO_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HASEL` reader - Selects the definition of currently selected harts.  

 0: There is a single currently selected hart, that is selected by hartsel.  

 1: There may be multiple currently selected harts – the hart selected by hartsel, plus those selected by the hart array mask register.  

 Hazard3 does support the hart array mask."]
pub type HASEL_R = crate::BitReader;
#[doc = "Field `HASEL` writer - Selects the definition of currently selected harts.  

 0: There is a single currently selected hart, that is selected by hartsel.  

 1: There may be multiple currently selected harts – the hart selected by hartsel, plus those selected by the hart array mask register.  

 Hazard3 does support the hart array mask."]
pub type HASEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKHAVERESET` reader - 0: No effect.  

 1: Clears havereset for any selected harts.  

 Writes apply to the new value of hartsel and hasel."]
pub type ACKHAVERESET_R = crate::BitReader;
#[doc = "Field `ACKHAVERESET` writer - 0: No effect.  

 1: Clears havereset for any selected harts.  

 Writes apply to the new value of hartsel and hasel."]
pub type ACKHAVERESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARTRESET` reader - This optional field writes the reset bit for all the currently selected harts. To perform a reset the debugger writes 1, and then writes 0 to deassert the reset signal.  

 While this bit is 1, the debugger must not change which harts are selected.  

 Writes apply to the new value of hartsel and hasel.  

 (The exact behaviour of this field is implementation-defined: on RP2350 it (triggers a local reset of the selected core(s) only.)"]
pub type HARTRESET_R = crate::BitReader;
#[doc = "Field `HARTRESET` writer - This optional field writes the reset bit for all the currently selected harts. To perform a reset the debugger writes 1, and then writes 0 to deassert the reset signal.  

 While this bit is 1, the debugger must not change which harts are selected.  

 Writes apply to the new value of hartsel and hasel.  

 (The exact behaviour of this field is implementation-defined: on RP2350 it (triggers a local reset of the selected core(s) only.)"]
pub type HARTRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMEREQ` reader - Writing 1 causes the currently selected harts to resume once, if they are halted when the write occurs. It also clears the resume ack bit for those harts.  

 resumereq is ignored if haltreq is set.  

 Writes apply to the new value of hartsel and hasel."]
pub type RESUMEREQ_R = crate::BitReader;
#[doc = "Field `RESUMEREQ` writer - Writing 1 causes the currently selected harts to resume once, if they are halted when the write occurs. It also clears the resume ack bit for those harts.  

 resumereq is ignored if haltreq is set.  

 Writes apply to the new value of hartsel and hasel."]
pub type RESUMEREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALTREQ` writer - Writing 0 clears the halt request bit for all currently selected harts. This may cancel outstanding halt requests for those harts.  

 Writing 1 sets the halt request bit for all currently selected harts. Running harts will halt whenever their halt request bit is set.  

 Writes apply to the new value of hartsel and hasel."]
pub type HALTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit serves as a reset signal for the Debug Module itself.  

 0: The module’s state, including authentication mechanism, takes its reset values (the dmactive bit is the only bit which can be written to something other than its reset value).  

 1: The module functions normally.  

 No other mechanism should exist that may result in resetting the Debug Module after power up, with the possible (but not recommended) exception of a global reset signal that resets the entire platform.  

 (On RP2350, the Debug Module is reset by a power-on reset, a brownout reset, the RUN pin, and a rescue reset.)  

 A debugger may pulse this bit low to get the Debug Module into a known state."]
    #[inline(always)]
    pub fn dmactive(&self) -> DMACTIVE_R {
        DMACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls the reset signal from the DM to the rest of the system. The signal should reset every part of the system, including every hart, except for the DM and any logic required to access the DM. To perform a system reset the debugger writes 1, and then writes 0 to deassert the reset.  

 On RP2350 this performs a cold reset, the equivalent of a watchdog reset with all WDSEL bits set. This includes both cores and all peripherals."]
    #[inline(always)]
    pub fn ndmreset(&self) -> NDMRESET_R {
        NDMRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This optional field clears the halt-on-reset request bit for all currently selected harts.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    pub fn clrresethaltreq(&self) -> CLRRESETHALTREQ_R {
        CLRRESETHALTREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This optional field writes the halt-on-reset request bit for all currently selected harts, unless clrresethaltreq is simultaneously set to 1.  

 When set to 1, each selected hart will halt upon the next deassertion of its reset. The halt-on-reset request bit is not automatically cleared. The debugger must write to clrresethaltreq to clear it.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    pub fn setresethaltreq(&self) -> SETRESETHALTREQ_R {
        SETRESETHALTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:15 - The high 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On Hazard3 this field is always tied to all-zeroes."]
    #[inline(always)]
    pub fn hartselhi(&self) -> HARTSELHI_R {
        HARTSELHI_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - The low 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On RP2350, since there are only two cores (with one hart each), only the least-significant bit is writable. The others are tied to 0."]
    #[inline(always)]
    pub fn hartsello(&self) -> HARTSELLO_R {
        HARTSELLO_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Selects the definition of currently selected harts.  

 0: There is a single currently selected hart, that is selected by hartsel.  

 1: There may be multiple currently selected harts – the hart selected by hartsel, plus those selected by the hart array mask register.  

 Hazard3 does support the hart array mask."]
    #[inline(always)]
    pub fn hasel(&self) -> HASEL_R {
        HASEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 0: No effect.  

 1: Clears havereset for any selected harts.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    pub fn ackhavereset(&self) -> ACKHAVERESET_R {
        ACKHAVERESET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This optional field writes the reset bit for all the currently selected harts. To perform a reset the debugger writes 1, and then writes 0 to deassert the reset signal.  

 While this bit is 1, the debugger must not change which harts are selected.  

 Writes apply to the new value of hartsel and hasel.  

 (The exact behaviour of this field is implementation-defined: on RP2350 it (triggers a local reset of the selected core(s) only.)"]
    #[inline(always)]
    pub fn hartreset(&self) -> HARTRESET_R {
        HARTRESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Writing 1 causes the currently selected harts to resume once, if they are halted when the write occurs. It also clears the resume ack bit for those harts.  

 resumereq is ignored if haltreq is set.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    pub fn resumereq(&self) -> RESUMEREQ_R {
        RESUMEREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit serves as a reset signal for the Debug Module itself.  

 0: The module’s state, including authentication mechanism, takes its reset values (the dmactive bit is the only bit which can be written to something other than its reset value).  

 1: The module functions normally.  

 No other mechanism should exist that may result in resetting the Debug Module after power up, with the possible (but not recommended) exception of a global reset signal that resets the entire platform.  

 (On RP2350, the Debug Module is reset by a power-on reset, a brownout reset, the RUN pin, and a rescue reset.)  

 A debugger may pulse this bit low to get the Debug Module into a known state."]
    #[inline(always)]
    #[must_use]
    pub fn dmactive(&mut self) -> DMACTIVE_W<DMCONTROL_SPEC> {
        DMACTIVE_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit controls the reset signal from the DM to the rest of the system. The signal should reset every part of the system, including every hart, except for the DM and any logic required to access the DM. To perform a system reset the debugger writes 1, and then writes 0 to deassert the reset.  

 On RP2350 this performs a cold reset, the equivalent of a watchdog reset with all WDSEL bits set. This includes both cores and all peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn ndmreset(&mut self) -> NDMRESET_W<DMCONTROL_SPEC> {
        NDMRESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - This optional field clears the halt-on-reset request bit for all currently selected harts.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    #[must_use]
    pub fn clrresethaltreq(&mut self) -> CLRRESETHALTREQ_W<DMCONTROL_SPEC> {
        CLRRESETHALTREQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - This optional field writes the halt-on-reset request bit for all currently selected harts, unless clrresethaltreq is simultaneously set to 1.  

 When set to 1, each selected hart will halt upon the next deassertion of its reset. The halt-on-reset request bit is not automatically cleared. The debugger must write to clrresethaltreq to clear it.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    #[must_use]
    pub fn setresethaltreq(&mut self) -> SETRESETHALTREQ_W<DMCONTROL_SPEC> {
        SETRESETHALTREQ_W::new(self, 3)
    }
    #[doc = "Bits 6:15 - The high 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On Hazard3 this field is always tied to all-zeroes."]
    #[inline(always)]
    #[must_use]
    pub fn hartselhi(&mut self) -> HARTSELHI_W<DMCONTROL_SPEC> {
        HARTSELHI_W::new(self, 6)
    }
    #[doc = "Bits 16:25 - The low 10 bits of hartsel: the DM-specific index of the hart to select. This hart is always part of the currently selected harts.  

 On RP2350, since there are only two cores (with one hart each), only the least-significant bit is writable. The others are tied to 0."]
    #[inline(always)]
    #[must_use]
    pub fn hartsello(&mut self) -> HARTSELLO_W<DMCONTROL_SPEC> {
        HARTSELLO_W::new(self, 16)
    }
    #[doc = "Bit 26 - Selects the definition of currently selected harts.  

 0: There is a single currently selected hart, that is selected by hartsel.  

 1: There may be multiple currently selected harts – the hart selected by hartsel, plus those selected by the hart array mask register.  

 Hazard3 does support the hart array mask."]
    #[inline(always)]
    #[must_use]
    pub fn hasel(&mut self) -> HASEL_W<DMCONTROL_SPEC> {
        HASEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - 0: No effect.  

 1: Clears havereset for any selected harts.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    #[must_use]
    pub fn ackhavereset(&mut self) -> ACKHAVERESET_W<DMCONTROL_SPEC> {
        ACKHAVERESET_W::new(self, 28)
    }
    #[doc = "Bit 29 - This optional field writes the reset bit for all the currently selected harts. To perform a reset the debugger writes 1, and then writes 0 to deassert the reset signal.  

 While this bit is 1, the debugger must not change which harts are selected.  

 Writes apply to the new value of hartsel and hasel.  

 (The exact behaviour of this field is implementation-defined: on RP2350 it (triggers a local reset of the selected core(s) only.)"]
    #[inline(always)]
    #[must_use]
    pub fn hartreset(&mut self) -> HARTRESET_W<DMCONTROL_SPEC> {
        HARTRESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - Writing 1 causes the currently selected harts to resume once, if they are halted when the write occurs. It also clears the resume ack bit for those harts.  

 resumereq is ignored if haltreq is set.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    #[must_use]
    pub fn resumereq(&mut self) -> RESUMEREQ_W<DMCONTROL_SPEC> {
        RESUMEREQ_W::new(self, 30)
    }
    #[doc = "Bit 31 - Writing 0 clears the halt request bit for all currently selected harts. This may cancel outstanding halt requests for those harts.  

 Writing 1 sets the halt request bit for all currently selected harts. Running harts will halt whenever their halt request bit is set.  

 Writes apply to the new value of hartsel and hasel."]
    #[inline(always)]
    #[must_use]
    pub fn haltreq(&mut self) -> HALTREQ_W<DMCONTROL_SPEC> {
        HALTREQ_W::new(self, 31)
    }
}
#[doc = "This register controls the overall Debug Module as well as the currently selected harts, as defined in hasel.  

You can [`read`](crate::Reg::read) this register and get [`dmcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMCONTROL_SPEC;
impl crate::RegisterSpec for DMCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcontrol::R`](R) reader structure"]
impl crate::Readable for DMCONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmcontrol::W`](W) writer structure"]
impl crate::Writable for DMCONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMCONTROL to value 0"]
impl crate::Resettable for DMCONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
