#[doc = "Register `PMPCFG0` reader"]
pub type R = crate::R<PMPCFG0_SPEC>;
#[doc = "Register `PMPCFG0` writer"]
pub type W = crate::W<PMPCFG0_SPEC>;
#[doc = "Field `R0_R` reader - Read permission for region 0"]
pub type R0_R_R = crate::BitReader;
#[doc = "Field `R0_R` writer - Read permission for region 0"]
pub type R0_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0_W` reader - Write permission for region 0"]
pub type R0_W_R = crate::BitReader;
#[doc = "Field `R0_W` writer - Write permission for region 0"]
pub type R0_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0_X` reader - Execute permission for region 0"]
pub type R0_X_R = crate::BitReader;
#[doc = "Field `R0_X` writer - Execute permission for region 0"]
pub type R0_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 0. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R0_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R0_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R0_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R0_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R0_A_A {}
#[doc = "Field `R0_A` reader - Address matching type for region 0. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R0_A_R = crate::FieldReader<R0_A_A>;
impl R0_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R0_A_A> {
        match self.bits {
            0 => Some(R0_A_A::OFF),
            2 => Some(R0_A_A::NA4),
            3 => Some(R0_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R0_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R0_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R0_A_A::NAPOT
    }
}
#[doc = "Field `R0_A` writer - Address matching type for region 0. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R0_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R0_A_A>;
impl<'a, REG> R0_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R0_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R0_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R0_A_A::NAPOT)
    }
}
#[doc = "Field `R0_L` reader - Lock region 0, and apply it to M-mode as well as U-mode."]
pub type R0_L_R = crate::BitReader;
#[doc = "Field `R0_L` writer - Lock region 0, and apply it to M-mode as well as U-mode."]
pub type R0_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1_R` reader - Read permission for region 1"]
pub type R1_R_R = crate::BitReader;
#[doc = "Field `R1_R` writer - Read permission for region 1"]
pub type R1_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1_W` reader - Write permission for region 1"]
pub type R1_W_R = crate::BitReader;
#[doc = "Field `R1_W` writer - Write permission for region 1"]
pub type R1_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1_X` reader - Execute permission for region 1"]
pub type R1_X_R = crate::BitReader;
#[doc = "Field `R1_X` writer - Execute permission for region 1"]
pub type R1_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 1. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R1_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R1_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R1_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R1_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R1_A_A {}
#[doc = "Field `R1_A` reader - Address matching type for region 1. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R1_A_R = crate::FieldReader<R1_A_A>;
impl R1_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R1_A_A> {
        match self.bits {
            0 => Some(R1_A_A::OFF),
            2 => Some(R1_A_A::NA4),
            3 => Some(R1_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R1_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R1_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R1_A_A::NAPOT
    }
}
#[doc = "Field `R1_A` writer - Address matching type for region 1. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R1_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R1_A_A>;
impl<'a, REG> R1_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R1_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R1_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R1_A_A::NAPOT)
    }
}
#[doc = "Field `R1_L` reader - Lock region 1, and apply it to M-mode as well as U-mode."]
pub type R1_L_R = crate::BitReader;
#[doc = "Field `R1_L` writer - Lock region 1, and apply it to M-mode as well as U-mode."]
pub type R1_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2_R` reader - Read permission for region 2"]
pub type R2_R_R = crate::BitReader;
#[doc = "Field `R2_R` writer - Read permission for region 2"]
pub type R2_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2_W` reader - Write permission for region 2"]
pub type R2_W_R = crate::BitReader;
#[doc = "Field `R2_W` writer - Write permission for region 2"]
pub type R2_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2_X` reader - Execute permission for region 2"]
pub type R2_X_R = crate::BitReader;
#[doc = "Field `R2_X` writer - Execute permission for region 2"]
pub type R2_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 2. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R2_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R2_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R2_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R2_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R2_A_A {}
#[doc = "Field `R2_A` reader - Address matching type for region 2. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R2_A_R = crate::FieldReader<R2_A_A>;
impl R2_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R2_A_A> {
        match self.bits {
            0 => Some(R2_A_A::OFF),
            2 => Some(R2_A_A::NA4),
            3 => Some(R2_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R2_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R2_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R2_A_A::NAPOT
    }
}
#[doc = "Field `R2_A` writer - Address matching type for region 2. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R2_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R2_A_A>;
impl<'a, REG> R2_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R2_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R2_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R2_A_A::NAPOT)
    }
}
#[doc = "Field `R2_L` reader - Lock region 2, and apply it to M-mode as well as U-mode."]
pub type R2_L_R = crate::BitReader;
#[doc = "Field `R2_L` writer - Lock region 2, and apply it to M-mode as well as U-mode."]
pub type R2_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R3_R` reader - Read permission for region 3"]
pub type R3_R_R = crate::BitReader;
#[doc = "Field `R3_R` writer - Read permission for region 3"]
pub type R3_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R3_W` reader - Write permission for region 3"]
pub type R3_W_R = crate::BitReader;
#[doc = "Field `R3_W` writer - Write permission for region 3"]
pub type R3_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R3_X` reader - Execute permission for region 3"]
pub type R3_X_R = crate::BitReader;
#[doc = "Field `R3_X` writer - Execute permission for region 3"]
pub type R3_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 3. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R3_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R3_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R3_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R3_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R3_A_A {}
#[doc = "Field `R3_A` reader - Address matching type for region 3. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R3_A_R = crate::FieldReader<R3_A_A>;
impl R3_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R3_A_A> {
        match self.bits {
            0 => Some(R3_A_A::OFF),
            2 => Some(R3_A_A::NA4),
            3 => Some(R3_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R3_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R3_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R3_A_A::NAPOT
    }
}
#[doc = "Field `R3_A` writer - Address matching type for region 3. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R3_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R3_A_A>;
impl<'a, REG> R3_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R3_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R3_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R3_A_A::NAPOT)
    }
}
#[doc = "Field `R3_L` reader - Lock region 3, and apply it to M-mode as well as U-mode."]
pub type R3_L_R = crate::BitReader;
#[doc = "Field `R3_L` writer - Lock region 3, and apply it to M-mode as well as U-mode."]
pub type R3_L_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read permission for region 0"]
    #[inline(always)]
    pub fn r0_r(&self) -> R0_R_R {
        R0_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write permission for region 0"]
    #[inline(always)]
    pub fn r0_w(&self) -> R0_W_R {
        R0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute permission for region 0"]
    #[inline(always)]
    pub fn r0_x(&self) -> R0_X_R {
        R0_X_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Address matching type for region 0. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r0_a(&self) -> R0_A_R {
        R0_A_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock region 0, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r0_l(&self) -> R0_L_R {
        R0_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read permission for region 1"]
    #[inline(always)]
    pub fn r1_r(&self) -> R1_R_R {
        R1_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write permission for region 1"]
    #[inline(always)]
    pub fn r1_w(&self) -> R1_W_R {
        R1_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Execute permission for region 1"]
    #[inline(always)]
    pub fn r1_x(&self) -> R1_X_R {
        R1_X_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Address matching type for region 1. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r1_a(&self) -> R1_A_R {
        R1_A_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - Lock region 1, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r1_l(&self) -> R1_L_R {
        R1_L_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read permission for region 2"]
    #[inline(always)]
    pub fn r2_r(&self) -> R2_R_R {
        R2_R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write permission for region 2"]
    #[inline(always)]
    pub fn r2_w(&self) -> R2_W_R {
        R2_W_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Execute permission for region 2"]
    #[inline(always)]
    pub fn r2_x(&self) -> R2_X_R {
        R2_X_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - Address matching type for region 2. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r2_a(&self) -> R2_A_R {
        R2_A_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 23 - Lock region 2, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r2_l(&self) -> R2_L_R {
        R2_L_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Read permission for region 3"]
    #[inline(always)]
    pub fn r3_r(&self) -> R3_R_R {
        R3_R_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write permission for region 3"]
    #[inline(always)]
    pub fn r3_w(&self) -> R3_W_R {
        R3_W_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Execute permission for region 3"]
    #[inline(always)]
    pub fn r3_x(&self) -> R3_X_R {
        R3_X_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Address matching type for region 3. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r3_a(&self) -> R3_A_R {
        R3_A_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - Lock region 3, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r3_l(&self) -> R3_L_R {
        R3_L_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read permission for region 0"]
    #[inline(always)]
    #[must_use]
    pub fn r0_r(&mut self) -> R0_R_W<PMPCFG0_SPEC> {
        R0_R_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write permission for region 0"]
    #[inline(always)]
    #[must_use]
    pub fn r0_w(&mut self) -> R0_W_W<PMPCFG0_SPEC> {
        R0_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Execute permission for region 0"]
    #[inline(always)]
    #[must_use]
    pub fn r0_x(&mut self) -> R0_X_W<PMPCFG0_SPEC> {
        R0_X_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Address matching type for region 0. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r0_a(&mut self) -> R0_A_W<PMPCFG0_SPEC> {
        R0_A_W::new(self, 3)
    }
    #[doc = "Bit 7 - Lock region 0, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r0_l(&mut self) -> R0_L_W<PMPCFG0_SPEC> {
        R0_L_W::new(self, 7)
    }
    #[doc = "Bit 8 - Read permission for region 1"]
    #[inline(always)]
    #[must_use]
    pub fn r1_r(&mut self) -> R1_R_W<PMPCFG0_SPEC> {
        R1_R_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write permission for region 1"]
    #[inline(always)]
    #[must_use]
    pub fn r1_w(&mut self) -> R1_W_W<PMPCFG0_SPEC> {
        R1_W_W::new(self, 9)
    }
    #[doc = "Bit 10 - Execute permission for region 1"]
    #[inline(always)]
    #[must_use]
    pub fn r1_x(&mut self) -> R1_X_W<PMPCFG0_SPEC> {
        R1_X_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Address matching type for region 1. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r1_a(&mut self) -> R1_A_W<PMPCFG0_SPEC> {
        R1_A_W::new(self, 11)
    }
    #[doc = "Bit 15 - Lock region 1, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r1_l(&mut self) -> R1_L_W<PMPCFG0_SPEC> {
        R1_L_W::new(self, 15)
    }
    #[doc = "Bit 16 - Read permission for region 2"]
    #[inline(always)]
    #[must_use]
    pub fn r2_r(&mut self) -> R2_R_W<PMPCFG0_SPEC> {
        R2_R_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write permission for region 2"]
    #[inline(always)]
    #[must_use]
    pub fn r2_w(&mut self) -> R2_W_W<PMPCFG0_SPEC> {
        R2_W_W::new(self, 17)
    }
    #[doc = "Bit 18 - Execute permission for region 2"]
    #[inline(always)]
    #[must_use]
    pub fn r2_x(&mut self) -> R2_X_W<PMPCFG0_SPEC> {
        R2_X_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - Address matching type for region 2. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r2_a(&mut self) -> R2_A_W<PMPCFG0_SPEC> {
        R2_A_W::new(self, 19)
    }
    #[doc = "Bit 23 - Lock region 2, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r2_l(&mut self) -> R2_L_W<PMPCFG0_SPEC> {
        R2_L_W::new(self, 23)
    }
    #[doc = "Bit 24 - Read permission for region 3"]
    #[inline(always)]
    #[must_use]
    pub fn r3_r(&mut self) -> R3_R_W<PMPCFG0_SPEC> {
        R3_R_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write permission for region 3"]
    #[inline(always)]
    #[must_use]
    pub fn r3_w(&mut self) -> R3_W_W<PMPCFG0_SPEC> {
        R3_W_W::new(self, 25)
    }
    #[doc = "Bit 26 - Execute permission for region 3"]
    #[inline(always)]
    #[must_use]
    pub fn r3_x(&mut self) -> R3_X_W<PMPCFG0_SPEC> {
        R3_X_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Address matching type for region 3. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r3_a(&mut self) -> R3_A_W<PMPCFG0_SPEC> {
        R3_A_W::new(self, 27)
    }
    #[doc = "Bit 31 - Lock region 3, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r3_l(&mut self) -> R3_L_W<PMPCFG0_SPEC> {
        R3_L_W::new(self, 31)
    }
}
#[doc = "Physical memory protection configuration for regions 0 through 3  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPCFG0_SPEC;
impl crate::RegisterSpec for PMPCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpcfg0::R`](R) reader structure"]
impl crate::Readable for PMPCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpcfg0::W`](W) writer structure"]
impl crate::Writable for PMPCFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPCFG0 to value 0"]
impl crate::Resettable for PMPCFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
