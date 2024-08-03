#[doc = "Register `DMSTATUS` reader"]
pub type R = crate::R<DMSTATUS_SPEC>;
#[doc = "Field `VERSION` reader - 0: There is no Debug Module present. 1: There is a Debug Module and it conforms to version 0.11 of the RISC-V debug specification.  

 2: There is a Debug Module and it conforms to version 0.13 of the RISC-V debug specification.  

 15: There is a Debug Module but it does not con- form to any available version of the RISC-V debug spec."]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `CONFSTPTRVALID` reader - 0: confstrptr0–confstrptr3 hold information which is not relevant to the configuration string.  

 1: confstrptr0–confstrptr3 hold the address of the configuration string."]
pub type CONFSTPTRVALID_R = crate::BitReader;
#[doc = "Field `HASRESETHALTREQ` reader - 1 if this Debug Module supports halt-on-reset functionality controllable by the setresethaltreq and clrresethaltreq bits. 0 otherwise."]
pub type HASRESETHALTREQ_R = crate::BitReader;
#[doc = "Field `AUTHBUSY` reader - 0: The authentication module is ready to process the next read/write to authdata.  

 1: The authentication module is busy. Accessing authdata results in unspecified behavior. authbusy only becomes set in immediate response to an access to authdata."]
pub type AUTHBUSY_R = crate::BitReader;
#[doc = "Field `AUTHENTICATED` reader - 0: Authentication is required before using the DM.  

 1: The authentication check has passed.  

 On components that don’t implement authentication, this bit must be preset as 1. (Note: the version of Hazard3 on RP2350 does not implement authentication.)"]
pub type AUTHENTICATED_R = crate::BitReader;
#[doc = "Field `ANYHALTED` reader - This field is 1 when any currently selected hart is halted."]
pub type ANYHALTED_R = crate::BitReader;
#[doc = "Field `ALLHALTED` reader - This field is 1 when all currently selected harts are halted."]
pub type ALLHALTED_R = crate::BitReader;
#[doc = "Field `ANYRUNNING` reader - This field is 1 when any currently selected hart is running."]
pub type ANYRUNNING_R = crate::BitReader;
#[doc = "Field `ALLRUNNING` reader - This field is 1 when all currently selected harts are running."]
pub type ALLRUNNING_R = crate::BitReader;
#[doc = "Field `ANYUNAVAIL` reader - This field is 1 when any currently selected hart is unavailable."]
pub type ANYUNAVAIL_R = crate::BitReader;
#[doc = "Field `ALLUNAVAIL` reader - This field is 1 when all currently selected harts are unavailable."]
pub type ALLUNAVAIL_R = crate::BitReader;
#[doc = "Field `ANYNONEXISTENT` reader - This field is 1 when any currently selected hart does not exist in this platform."]
pub type ANYNONEXISTENT_R = crate::BitReader;
#[doc = "Field `ALLNONEXISTENT` reader - This field is 1 when all currently selected harts do not exist on this platform."]
pub type ALLNONEXISTENT_R = crate::BitReader;
#[doc = "Field `ANYRESUMEACK` reader - This field is 1 when any currently selected hart has acknowledged its last resume request."]
pub type ANYRESUMEACK_R = crate::BitReader;
#[doc = "Field `ALLRESUMEACK` reader - This field is 1 when all currently selected harts have acknowledged their last resume request."]
pub type ALLRESUMEACK_R = crate::BitReader;
#[doc = "Field `ANYHAVERESET` reader - This field is 1 when at least one currently selected hart has been reset and reset has not been acknowledged for that hart."]
pub type ANYHAVERESET_R = crate::BitReader;
#[doc = "Field `ALLHAVERESET` reader - This field is 1 when all currently selected harts have been reset and reset has not been acknowledged for any of them."]
pub type ALLHAVERESET_R = crate::BitReader;
#[doc = "Field `IMPEBREAK` reader - If 1, then there is an implicit ebreak instruction at the non-existent word immediately after the Program Buffer. This saves the debugger from having to write the ebreak itself, and allows the Program Buffer to be one word smaller."]
pub type IMPEBREAK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - 0: There is no Debug Module present. 1: There is a Debug Module and it conforms to version 0.11 of the RISC-V debug specification.  

 2: There is a Debug Module and it conforms to version 0.13 of the RISC-V debug specification.  

 15: There is a Debug Module but it does not con- form to any available version of the RISC-V debug spec."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: confstrptr0–confstrptr3 hold information which is not relevant to the configuration string.  

 1: confstrptr0–confstrptr3 hold the address of the configuration string."]
    #[inline(always)]
    pub fn confstptrvalid(&self) -> CONFSTPTRVALID_R {
        CONFSTPTRVALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 if this Debug Module supports halt-on-reset functionality controllable by the setresethaltreq and clrresethaltreq bits. 0 otherwise."]
    #[inline(always)]
    pub fn hasresethaltreq(&self) -> HASRESETHALTREQ_R {
        HASRESETHALTREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: The authentication module is ready to process the next read/write to authdata.  

 1: The authentication module is busy. Accessing authdata results in unspecified behavior. authbusy only becomes set in immediate response to an access to authdata."]
    #[inline(always)]
    pub fn authbusy(&self) -> AUTHBUSY_R {
        AUTHBUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Authentication is required before using the DM.  

 1: The authentication check has passed.  

 On components that don’t implement authentication, this bit must be preset as 1. (Note: the version of Hazard3 on RP2350 does not implement authentication.)"]
    #[inline(always)]
    pub fn authenticated(&self) -> AUTHENTICATED_R {
        AUTHENTICATED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This field is 1 when any currently selected hart is halted."]
    #[inline(always)]
    pub fn anyhalted(&self) -> ANYHALTED_R {
        ANYHALTED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This field is 1 when all currently selected harts are halted."]
    #[inline(always)]
    pub fn allhalted(&self) -> ALLHALTED_R {
        ALLHALTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This field is 1 when any currently selected hart is running."]
    #[inline(always)]
    pub fn anyrunning(&self) -> ANYRUNNING_R {
        ANYRUNNING_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This field is 1 when all currently selected harts are running."]
    #[inline(always)]
    pub fn allrunning(&self) -> ALLRUNNING_R {
        ALLRUNNING_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This field is 1 when any currently selected hart is unavailable."]
    #[inline(always)]
    pub fn anyunavail(&self) -> ANYUNAVAIL_R {
        ANYUNAVAIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This field is 1 when all currently selected harts are unavailable."]
    #[inline(always)]
    pub fn allunavail(&self) -> ALLUNAVAIL_R {
        ALLUNAVAIL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This field is 1 when any currently selected hart does not exist in this platform."]
    #[inline(always)]
    pub fn anynonexistent(&self) -> ANYNONEXISTENT_R {
        ANYNONEXISTENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This field is 1 when all currently selected harts do not exist on this platform."]
    #[inline(always)]
    pub fn allnonexistent(&self) -> ALLNONEXISTENT_R {
        ALLNONEXISTENT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This field is 1 when any currently selected hart has acknowledged its last resume request."]
    #[inline(always)]
    pub fn anyresumeack(&self) -> ANYRESUMEACK_R {
        ANYRESUMEACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This field is 1 when all currently selected harts have acknowledged their last resume request."]
    #[inline(always)]
    pub fn allresumeack(&self) -> ALLRESUMEACK_R {
        ALLRESUMEACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This field is 1 when at least one currently selected hart has been reset and reset has not been acknowledged for that hart."]
    #[inline(always)]
    pub fn anyhavereset(&self) -> ANYHAVERESET_R {
        ANYHAVERESET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This field is 1 when all currently selected harts have been reset and reset has not been acknowledged for any of them."]
    #[inline(always)]
    pub fn allhavereset(&self) -> ALLHAVERESET_R {
        ALLHAVERESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - If 1, then there is an implicit ebreak instruction at the non-existent word immediately after the Program Buffer. This saves the debugger from having to write the ebreak itself, and allows the Program Buffer to be one word smaller."]
    #[inline(always)]
    pub fn impebreak(&self) -> IMPEBREAK_R {
        IMPEBREAK_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "This register reports status for the overall Debug Module as well as the currently selected harts, as defined in hasel. Its address will not change in the future, because it contains version.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`dmstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMSTATUS_SPEC;
impl crate::RegisterSpec for DMSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmstatus::R`](R) reader structure"]
impl crate::Readable for DMSTATUS_SPEC {}
#[doc = "`reset()` method sets DMSTATUS to value 0x0040_00a2"]
impl crate::Resettable for DMSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x0040_00a2;
}
