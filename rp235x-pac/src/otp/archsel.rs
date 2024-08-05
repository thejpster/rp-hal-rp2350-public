#[doc = "Register `ARCHSEL` reader"]
pub type R = crate::R<ARCHSEL_SPEC>;
#[doc = "Register `ARCHSEL` writer"]
pub type W = crate::W<ARCHSEL_SPEC>;
#[doc = "Select architecture for core 0.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE0_A {
    #[doc = "0: Switch core 0 to Arm (Cortex-M33)"]
    ARM = 0,
    #[doc = "1: Switch core 0 to RISC-V (Hazard3)"]
    RISCV = 1,
}
impl From<CORE0_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE0` reader - Select architecture for core 0."]
pub type CORE0_R = crate::BitReader<CORE0_A>;
impl CORE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CORE0_A {
        match self.bits {
            false => CORE0_A::ARM,
            true => CORE0_A::RISCV,
        }
    }
    #[doc = "Switch core 0 to Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == CORE0_A::ARM
    }
    #[doc = "Switch core 0 to RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn is_riscv(&self) -> bool {
        *self == CORE0_A::RISCV
    }
}
#[doc = "Field `CORE0` writer - Select architecture for core 0."]
pub type CORE0_W<'a, REG> = crate::BitWriter<'a, REG, CORE0_A>;
impl<'a, REG> CORE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch core 0 to Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn arm(self) -> &'a mut crate::W<REG> {
        self.variant(CORE0_A::ARM)
    }
    #[doc = "Switch core 0 to RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn riscv(self) -> &'a mut crate::W<REG> {
        self.variant(CORE0_A::RISCV)
    }
}
#[doc = "Select architecture for core 1.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE1_A {
    #[doc = "0: Switch core 1 to Arm (Cortex-M33)"]
    ARM = 0,
    #[doc = "1: Switch core 1 to RISC-V (Hazard3)"]
    RISCV = 1,
}
impl From<CORE1_A> for bool {
    #[inline(always)]
    fn from(variant: CORE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE1` reader - Select architecture for core 1."]
pub type CORE1_R = crate::BitReader<CORE1_A>;
impl CORE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CORE1_A {
        match self.bits {
            false => CORE1_A::ARM,
            true => CORE1_A::RISCV,
        }
    }
    #[doc = "Switch core 1 to Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == CORE1_A::ARM
    }
    #[doc = "Switch core 1 to RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn is_riscv(&self) -> bool {
        *self == CORE1_A::RISCV
    }
}
#[doc = "Field `CORE1` writer - Select architecture for core 1."]
pub type CORE1_W<'a, REG> = crate::BitWriter<'a, REG, CORE1_A>;
impl<'a, REG> CORE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch core 1 to Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn arm(self) -> &'a mut crate::W<REG> {
        self.variant(CORE1_A::ARM)
    }
    #[doc = "Switch core 1 to RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn riscv(self) -> &'a mut crate::W<REG> {
        self.variant(CORE1_A::RISCV)
    }
}
impl R {
    #[doc = "Bit 0 - Select architecture for core 0."]
    #[inline(always)]
    pub fn core0(&self) -> CORE0_R {
        CORE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select architecture for core 1."]
    #[inline(always)]
    pub fn core1(&self) -> CORE1_R {
        CORE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select architecture for core 0."]
    #[inline(always)]
    #[must_use]
    pub fn core0(&mut self) -> CORE0_W<ARCHSEL_SPEC> {
        CORE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select architecture for core 1."]
    #[inline(always)]
    #[must_use]
    pub fn core1(&mut self) -> CORE1_W<ARCHSEL_SPEC> {
        CORE1_W::new(self, 1)
    }
}
#[doc = "Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module.  

You can [`read`](crate::Reg::read) this register and get [`archsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`archsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARCHSEL_SPEC;
impl crate::RegisterSpec for ARCHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`archsel::R`](R) reader structure"]
impl crate::Readable for ARCHSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`archsel::W`](W) writer structure"]
impl crate::Writable for ARCHSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARCHSEL to value 0"]
impl crate::Resettable for ARCHSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
