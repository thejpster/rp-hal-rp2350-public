#[doc = "Register `ARCHSEL_STATUS` reader"]
pub type R = crate::R<ARCHSEL_STATUS_SPEC>;
#[doc = "Register `ARCHSEL_STATUS` writer"]
pub type W = crate::W<ARCHSEL_STATUS_SPEC>;
#[doc = "Current architecture for core 0. Updated on processor warm reset.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE0_A {
    #[doc = "0: Core 0 is currently Arm (Cortex-M33)"]
    ARM = 0,
    #[doc = "1: Core 0 is currently RISC-V (Hazard3)"]
    RISCV = 1,
}
impl From<CORE0_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE0` reader - Current architecture for core 0. Updated on processor warm reset."]
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
    #[doc = "Core 0 is currently Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == CORE0_A::ARM
    }
    #[doc = "Core 0 is currently RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn is_riscv(&self) -> bool {
        *self == CORE0_A::RISCV
    }
}
#[doc = "Current architecture for core 0. Updated on processor warm reset.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CORE1_A {
    #[doc = "0: Core 1 is currently Arm (Cortex-M33)"]
    ARM = 0,
    #[doc = "1: Core 1 is currently RISC-V (Hazard3)"]
    RISCV = 1,
}
impl From<CORE1_A> for bool {
    #[inline(always)]
    fn from(variant: CORE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORE1` reader - Current architecture for core 0. Updated on processor warm reset."]
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
    #[doc = "Core 1 is currently Arm (Cortex-M33)"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == CORE1_A::ARM
    }
    #[doc = "Core 1 is currently RISC-V (Hazard3)"]
    #[inline(always)]
    pub fn is_riscv(&self) -> bool {
        *self == CORE1_A::RISCV
    }
}
impl R {
    #[doc = "Bit 0 - Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub fn core0(&self) -> CORE0_R {
        CORE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub fn core1(&self) -> CORE1_R {
        CORE1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update.  

You can [`read`](crate::Reg::read) this register and get [`archsel_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`archsel_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARCHSEL_STATUS_SPEC;
impl crate::RegisterSpec for ARCHSEL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`archsel_status::R`](R) reader structure"]
impl crate::Readable for ARCHSEL_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`archsel_status::W`](W) writer structure"]
impl crate::Writable for ARCHSEL_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARCHSEL_STATUS to value 0"]
impl crate::Resettable for ARCHSEL_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
