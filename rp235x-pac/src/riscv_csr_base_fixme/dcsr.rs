#[doc = "Register `DCSR` reader"]
pub type R = crate::R<DCSR_SPEC>;
#[doc = "Register `DCSR` writer"]
pub type W = crate::W<DCSR_SPEC>;
#[doc = "Field `PRV` reader - Read the privilege mode the core was in when entering Debug Mode, and set the privilege mode the core will execute in when returning from Debug Mode."]
pub type PRV_R = crate::FieldReader;
#[doc = "Field `PRV` writer - Read the privilege mode the core was in when entering Debug Mode, and set the privilege mode the core will execute in when returning from Debug Mode."]
pub type PRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STEP` reader - When 1, re-enter Debug Mode after each instruction executed in M-mode or U-mode."]
pub type STEP_R = crate::BitReader;
#[doc = "Field `STEP` writer - When 1, re-enter Debug Mode after each instruction executed in M-mode or U-mode."]
pub type STEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Set by hardware when entering debug mode.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAUSE_A {
    #[doc = "1: An ebreak instruction was executed when the relevant `dcsr.ebreakx` bit was set."]
    EBREAK = 1,
    #[doc = "2: The trigger module caused a breakpoint exception."]
    TRIGGER = 2,
    #[doc = "3: Processor entered Debug Mode due to a halt request, or a reset-halt request present when the core reset was released."]
    HALTREQ = 3,
    #[doc = "4: Processor entered Debug Mode after executing one instruction with single-stepping enabled."]
    STEP = 4,
}
impl From<CAUSE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAUSE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAUSE_A {
    type Ux = u8;
}
impl crate::IsEnum for CAUSE_A {}
#[doc = "Field `CAUSE` reader - Set by hardware when entering debug mode."]
pub type CAUSE_R = crate::FieldReader<CAUSE_A>;
impl CAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAUSE_A> {
        match self.bits {
            1 => Some(CAUSE_A::EBREAK),
            2 => Some(CAUSE_A::TRIGGER),
            3 => Some(CAUSE_A::HALTREQ),
            4 => Some(CAUSE_A::STEP),
            _ => None,
        }
    }
    #[doc = "An ebreak instruction was executed when the relevant `dcsr.ebreakx` bit was set."]
    #[inline(always)]
    pub fn is_ebreak(&self) -> bool {
        *self == CAUSE_A::EBREAK
    }
    #[doc = "The trigger module caused a breakpoint exception."]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == CAUSE_A::TRIGGER
    }
    #[doc = "Processor entered Debug Mode due to a halt request, or a reset-halt request present when the core reset was released."]
    #[inline(always)]
    pub fn is_haltreq(&self) -> bool {
        *self == CAUSE_A::HALTREQ
    }
    #[doc = "Processor entered Debug Mode after executing one instruction with single-stepping enabled."]
    #[inline(always)]
    pub fn is_step(&self) -> bool {
        *self == CAUSE_A::STEP
    }
}
#[doc = "Field `STOPTIME` reader - Hardwired to 1: core-local timers don't increment in debug mode. External timers (e.g. hart-shared) may be configured to ignore this."]
pub type STOPTIME_R = crate::BitReader;
#[doc = "Field `STOPCOUNT` reader - Hardwired to 1: `mcycle`/`mcycleh` and `minstret`/`minstreth` do not increment in Debug Mode."]
pub type STOPCOUNT_R = crate::BitReader;
#[doc = "Field `STEPIE` reader - Hardwired to 0: no interrupts are taken during hardware single-stepping."]
pub type STEPIE_R = crate::BitReader;
#[doc = "Field `EBREAKU` reader - When 1, `ebreak` instructions executed in U-mode will break to Debug Mode instead of trapping."]
pub type EBREAKU_R = crate::BitReader;
#[doc = "Field `EBREAKU` writer - When 1, `ebreak` instructions executed in U-mode will break to Debug Mode instead of trapping."]
pub type EBREAKU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBREAKM` reader - When 1, `ebreak` instructions executed in M-mode will break to Debug Mode instead of trapping"]
pub type EBREAKM_R = crate::BitReader;
#[doc = "Field `EBREAKM` writer - When 1, `ebreak` instructions executed in M-mode will break to Debug Mode instead of trapping"]
pub type EBREAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XDEBUGVER` reader - Hardwired to 4: external debug support as per RISC-V 0.13.2 debug specification."]
pub type XDEBUGVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Read the privilege mode the core was in when entering Debug Mode, and set the privilege mode the core will execute in when returning from Debug Mode."]
    #[inline(always)]
    pub fn prv(&self) -> PRV_R {
        PRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - When 1, re-enter Debug Mode after each instruction executed in M-mode or U-mode."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Set by hardware when entering debug mode."]
    #[inline(always)]
    pub fn cause(&self) -> CAUSE_R {
        CAUSE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Hardwired to 1: core-local timers don't increment in debug mode. External timers (e.g. hart-shared) may be configured to ignore this."]
    #[inline(always)]
    pub fn stoptime(&self) -> STOPTIME_R {
        STOPTIME_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hardwired to 1: `mcycle`/`mcycleh` and `minstret`/`minstreth` do not increment in Debug Mode."]
    #[inline(always)]
    pub fn stopcount(&self) -> STOPCOUNT_R {
        STOPCOUNT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hardwired to 0: no interrupts are taken during hardware single-stepping."]
    #[inline(always)]
    pub fn stepie(&self) -> STEPIE_R {
        STEPIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, `ebreak` instructions executed in U-mode will break to Debug Mode instead of trapping."]
    #[inline(always)]
    pub fn ebreaku(&self) -> EBREAKU_R {
        EBREAKU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, `ebreak` instructions executed in M-mode will break to Debug Mode instead of trapping"]
    #[inline(always)]
    pub fn ebreakm(&self) -> EBREAKM_R {
        EBREAKM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Hardwired to 4: external debug support as per RISC-V 0.13.2 debug specification."]
    #[inline(always)]
    pub fn xdebugver(&self) -> XDEBUGVER_R {
        XDEBUGVER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read the privilege mode the core was in when entering Debug Mode, and set the privilege mode the core will execute in when returning from Debug Mode."]
    #[inline(always)]
    #[must_use]
    pub fn prv(&mut self) -> PRV_W<DCSR_SPEC> {
        PRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - When 1, re-enter Debug Mode after each instruction executed in M-mode or U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<DCSR_SPEC> {
        STEP_W::new(self, 2)
    }
    #[doc = "Bit 12 - When 1, `ebreak` instructions executed in U-mode will break to Debug Mode instead of trapping."]
    #[inline(always)]
    #[must_use]
    pub fn ebreaku(&mut self) -> EBREAKU_W<DCSR_SPEC> {
        EBREAKU_W::new(self, 12)
    }
    #[doc = "Bit 15 - When 1, `ebreak` instructions executed in M-mode will break to Debug Mode instead of trapping"]
    #[inline(always)]
    #[must_use]
    pub fn ebreakm(&mut self) -> EBREAKM_W<DCSR_SPEC> {
        EBREAKM_W::new(self, 15)
    }
}
#[doc = "Debug control and status register. Access outside of Debug Mode will cause an illegal instruction exception.  

You can [`read`](crate::Reg::read) this register and get [`dcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCSR_SPEC;
impl crate::RegisterSpec for DCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcsr::R`](R) reader structure"]
impl crate::Readable for DCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcsr::W`](W) writer structure"]
impl crate::Writable for DCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCSR to value 0x4000_0603"]
impl crate::Resettable for DCSR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0603;
}
