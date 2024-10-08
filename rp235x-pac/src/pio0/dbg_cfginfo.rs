#[doc = "Register `DBG_CFGINFO` reader"]
pub type R = crate::R<DBG_CFGINFO_SPEC>;
#[doc = "Register `DBG_CFGINFO` writer"]
pub type W = crate::W<DBG_CFGINFO_SPEC>;
#[doc = "Field `FIFO_DEPTH` reader - The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
pub type FIFO_DEPTH_R = crate::FieldReader;
#[doc = "Field `SM_COUNT` reader - The number of state machines this PIO instance is equipped with."]
pub type SM_COUNT_R = crate::FieldReader;
#[doc = "Field `IMEM_SIZE` reader - The size of the instruction memory, measured in units of one instruction"]
pub type IMEM_SIZE_R = crate::FieldReader;
#[doc = "Version of the core PIO hardware.  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VERSION_A {
    #[doc = "0: Version 0 (RP2040)"]
    V0 = 0,
    #[doc = "1: Version 1 (RP2350)"]
    V1 = 1,
}
impl From<VERSION_A> for u8 {
    #[inline(always)]
    fn from(variant: VERSION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VERSION_A {
    type Ux = u8;
}
impl crate::IsEnum for VERSION_A {}
#[doc = "Field `VERSION` reader - Version of the core PIO hardware."]
pub type VERSION_R = crate::FieldReader<VERSION_A>;
impl VERSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VERSION_A> {
        match self.bits {
            0 => Some(VERSION_A::V0),
            1 => Some(VERSION_A::V1),
            _ => None,
        }
    }
    #[doc = "Version 0 (RP2040)"]
    #[inline(always)]
    pub fn is_v0(&self) -> bool {
        *self == VERSION_A::V0
    }
    #[doc = "Version 1 (RP2350)"]
    #[inline(always)]
    pub fn is_v1(&self) -> bool {
        *self == VERSION_A::V1
    }
}
impl R {
    #[doc = "Bits 0:5 - The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - The number of state machines this PIO instance is equipped with."]
    #[inline(always)]
    pub fn sm_count(&self) -> SM_COUNT_R {
        SM_COUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - The size of the instruction memory, measured in units of one instruction"]
    #[inline(always)]
    pub fn imem_size(&self) -> IMEM_SIZE_R {
        IMEM_SIZE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - Version of the core PIO hardware."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here.  

You can [`read`](crate::Reg::read) this register and get [`dbg_cfginfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_cfginfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_CFGINFO_SPEC;
impl crate::RegisterSpec for DBG_CFGINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cfginfo::R`](R) reader structure"]
impl crate::Readable for DBG_CFGINFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_cfginfo::W`](W) writer structure"]
impl crate::Writable for DBG_CFGINFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CFGINFO to value 0x1000_0000"]
impl crate::Resettable for DBG_CFGINFO_SPEC {
    const RESET_VALUE: u32 = 0x1000_0000;
}
