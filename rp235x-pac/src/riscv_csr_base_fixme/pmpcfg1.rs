#[doc = "Register `PMPCFG1` reader"]
pub type R = crate::R<PMPCFG1_SPEC>;
#[doc = "Register `PMPCFG1` writer"]
pub type W = crate::W<PMPCFG1_SPEC>;
#[doc = "Field `R4_R` reader - Read permission for region 4"]
pub type R4_R_R = crate::BitReader;
#[doc = "Field `R4_R` writer - Read permission for region 4"]
pub type R4_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R4_W` reader - Write permission for region 4"]
pub type R4_W_R = crate::BitReader;
#[doc = "Field `R4_W` writer - Write permission for region 4"]
pub type R4_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R4_X` reader - Execute permission for region 4"]
pub type R4_X_R = crate::BitReader;
#[doc = "Field `R4_X` writer - Execute permission for region 4"]
pub type R4_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 4. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R4_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R4_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R4_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R4_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R4_A_A {}
#[doc = "Field `R4_A` reader - Address matching type for region 4. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R4_A_R = crate::FieldReader<R4_A_A>;
impl R4_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R4_A_A> {
        match self.bits {
            0 => Some(R4_A_A::OFF),
            2 => Some(R4_A_A::NA4),
            3 => Some(R4_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R4_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R4_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R4_A_A::NAPOT
    }
}
#[doc = "Field `R4_A` writer - Address matching type for region 4. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R4_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R4_A_A>;
impl<'a, REG> R4_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R4_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R4_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R4_A_A::NAPOT)
    }
}
#[doc = "Field `R4_L` reader - Lock region 4, and apply it to M-mode as well as U-mode."]
pub type R4_L_R = crate::BitReader;
#[doc = "Field `R4_L` writer - Lock region 4, and apply it to M-mode as well as U-mode."]
pub type R4_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5_R` reader - Read permission for region 5"]
pub type R5_R_R = crate::BitReader;
#[doc = "Field `R5_R` writer - Read permission for region 5"]
pub type R5_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5_W` reader - Write permission for region 5"]
pub type R5_W_R = crate::BitReader;
#[doc = "Field `R5_W` writer - Write permission for region 5"]
pub type R5_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5_X` reader - Execute permission for region 5"]
pub type R5_X_R = crate::BitReader;
#[doc = "Field `R5_X` writer - Execute permission for region 5"]
pub type R5_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 5. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R5_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R5_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R5_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R5_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R5_A_A {}
#[doc = "Field `R5_A` reader - Address matching type for region 5. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R5_A_R = crate::FieldReader<R5_A_A>;
impl R5_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R5_A_A> {
        match self.bits {
            0 => Some(R5_A_A::OFF),
            2 => Some(R5_A_A::NA4),
            3 => Some(R5_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R5_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R5_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R5_A_A::NAPOT
    }
}
#[doc = "Field `R5_A` writer - Address matching type for region 5. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R5_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R5_A_A>;
impl<'a, REG> R5_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R5_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R5_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R5_A_A::NAPOT)
    }
}
#[doc = "Field `R5_L` reader - Lock region 5, and apply it to M-mode as well as U-mode."]
pub type R5_L_R = crate::BitReader;
#[doc = "Field `R5_L` writer - Lock region 5, and apply it to M-mode as well as U-mode."]
pub type R5_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R6_R` reader - Read permission for region 6"]
pub type R6_R_R = crate::BitReader;
#[doc = "Field `R6_R` writer - Read permission for region 6"]
pub type R6_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R6_W` reader - Write permission for region 6"]
pub type R6_W_R = crate::BitReader;
#[doc = "Field `R6_W` writer - Write permission for region 6"]
pub type R6_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R6_X` reader - Execute permission for region 6"]
pub type R6_X_R = crate::BitReader;
#[doc = "Field `R6_X` writer - Execute permission for region 6"]
pub type R6_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 6. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R6_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R6_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R6_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R6_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R6_A_A {}
#[doc = "Field `R6_A` reader - Address matching type for region 6. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R6_A_R = crate::FieldReader<R6_A_A>;
impl R6_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R6_A_A> {
        match self.bits {
            0 => Some(R6_A_A::OFF),
            2 => Some(R6_A_A::NA4),
            3 => Some(R6_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R6_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R6_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R6_A_A::NAPOT
    }
}
#[doc = "Field `R6_A` writer - Address matching type for region 6. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R6_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R6_A_A>;
impl<'a, REG> R6_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R6_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R6_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R6_A_A::NAPOT)
    }
}
#[doc = "Field `R6_L` reader - Lock region 6, and apply it to M-mode as well as U-mode."]
pub type R6_L_R = crate::BitReader;
#[doc = "Field `R6_L` writer - Lock region 6, and apply it to M-mode as well as U-mode."]
pub type R6_L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R7_R` reader - Read permission for region 7"]
pub type R7_R_R = crate::BitReader;
#[doc = "Field `R7_R` writer - Read permission for region 7"]
pub type R7_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R7_W` reader - Write permission for region 7"]
pub type R7_W_R = crate::BitReader;
#[doc = "Field `R7_W` writer - Write permission for region 7"]
pub type R7_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R7_X` reader - Execute permission for region 7"]
pub type R7_X_R = crate::BitReader;
#[doc = "Field `R7_X` writer - Execute permission for region 7"]
pub type R7_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address matching type for region 7. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R7_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R7_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R7_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R7_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R7_A_A {}
#[doc = "Field `R7_A` reader - Address matching type for region 7. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R7_A_R = crate::FieldReader<R7_A_A>;
impl R7_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R7_A_A> {
        match self.bits {
            0 => Some(R7_A_A::OFF),
            2 => Some(R7_A_A::NA4),
            3 => Some(R7_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R7_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R7_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R7_A_A::NAPOT
    }
}
#[doc = "Field `R7_A` writer - Address matching type for region 7. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R7_A_W<'a, REG> = crate::FieldWriter<'a, REG, 2, R7_A_A>;
impl<'a, REG> R7_A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(R7_A_A::OFF)
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn na4(self) -> &'a mut crate::W<REG> {
        self.variant(R7_A_A::NA4)
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn napot(self) -> &'a mut crate::W<REG> {
        self.variant(R7_A_A::NAPOT)
    }
}
#[doc = "Field `R7_L` reader - Lock region 7, and apply it to M-mode as well as U-mode."]
pub type R7_L_R = crate::BitReader;
#[doc = "Field `R7_L` writer - Lock region 7, and apply it to M-mode as well as U-mode."]
pub type R7_L_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read permission for region 4"]
    #[inline(always)]
    pub fn r4_r(&self) -> R4_R_R {
        R4_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write permission for region 4"]
    #[inline(always)]
    pub fn r4_w(&self) -> R4_W_R {
        R4_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute permission for region 4"]
    #[inline(always)]
    pub fn r4_x(&self) -> R4_X_R {
        R4_X_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Address matching type for region 4. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r4_a(&self) -> R4_A_R {
        R4_A_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock region 4, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r4_l(&self) -> R4_L_R {
        R4_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read permission for region 5"]
    #[inline(always)]
    pub fn r5_r(&self) -> R5_R_R {
        R5_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write permission for region 5"]
    #[inline(always)]
    pub fn r5_w(&self) -> R5_W_R {
        R5_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Execute permission for region 5"]
    #[inline(always)]
    pub fn r5_x(&self) -> R5_X_R {
        R5_X_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Address matching type for region 5. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r5_a(&self) -> R5_A_R {
        R5_A_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - Lock region 5, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r5_l(&self) -> R5_L_R {
        R5_L_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read permission for region 6"]
    #[inline(always)]
    pub fn r6_r(&self) -> R6_R_R {
        R6_R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write permission for region 6"]
    #[inline(always)]
    pub fn r6_w(&self) -> R6_W_R {
        R6_W_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Execute permission for region 6"]
    #[inline(always)]
    pub fn r6_x(&self) -> R6_X_R {
        R6_X_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - Address matching type for region 6. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r6_a(&self) -> R6_A_R {
        R6_A_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 23 - Lock region 6, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r6_l(&self) -> R6_L_R {
        R6_L_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Read permission for region 7"]
    #[inline(always)]
    pub fn r7_r(&self) -> R7_R_R {
        R7_R_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write permission for region 7"]
    #[inline(always)]
    pub fn r7_w(&self) -> R7_W_R {
        R7_W_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Execute permission for region 7"]
    #[inline(always)]
    pub fn r7_x(&self) -> R7_X_R {
        R7_X_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Address matching type for region 7. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r7_a(&self) -> R7_A_R {
        R7_A_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - Lock region 7, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r7_l(&self) -> R7_L_R {
        R7_L_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read permission for region 4"]
    #[inline(always)]
    #[must_use]
    pub fn r4_r(&mut self) -> R4_R_W<PMPCFG1_SPEC> {
        R4_R_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write permission for region 4"]
    #[inline(always)]
    #[must_use]
    pub fn r4_w(&mut self) -> R4_W_W<PMPCFG1_SPEC> {
        R4_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Execute permission for region 4"]
    #[inline(always)]
    #[must_use]
    pub fn r4_x(&mut self) -> R4_X_W<PMPCFG1_SPEC> {
        R4_X_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Address matching type for region 4. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r4_a(&mut self) -> R4_A_W<PMPCFG1_SPEC> {
        R4_A_W::new(self, 3)
    }
    #[doc = "Bit 7 - Lock region 4, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r4_l(&mut self) -> R4_L_W<PMPCFG1_SPEC> {
        R4_L_W::new(self, 7)
    }
    #[doc = "Bit 8 - Read permission for region 5"]
    #[inline(always)]
    #[must_use]
    pub fn r5_r(&mut self) -> R5_R_W<PMPCFG1_SPEC> {
        R5_R_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write permission for region 5"]
    #[inline(always)]
    #[must_use]
    pub fn r5_w(&mut self) -> R5_W_W<PMPCFG1_SPEC> {
        R5_W_W::new(self, 9)
    }
    #[doc = "Bit 10 - Execute permission for region 5"]
    #[inline(always)]
    #[must_use]
    pub fn r5_x(&mut self) -> R5_X_W<PMPCFG1_SPEC> {
        R5_X_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Address matching type for region 5. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r5_a(&mut self) -> R5_A_W<PMPCFG1_SPEC> {
        R5_A_W::new(self, 11)
    }
    #[doc = "Bit 15 - Lock region 5, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r5_l(&mut self) -> R5_L_W<PMPCFG1_SPEC> {
        R5_L_W::new(self, 15)
    }
    #[doc = "Bit 16 - Read permission for region 6"]
    #[inline(always)]
    #[must_use]
    pub fn r6_r(&mut self) -> R6_R_W<PMPCFG1_SPEC> {
        R6_R_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write permission for region 6"]
    #[inline(always)]
    #[must_use]
    pub fn r6_w(&mut self) -> R6_W_W<PMPCFG1_SPEC> {
        R6_W_W::new(self, 17)
    }
    #[doc = "Bit 18 - Execute permission for region 6"]
    #[inline(always)]
    #[must_use]
    pub fn r6_x(&mut self) -> R6_X_W<PMPCFG1_SPEC> {
        R6_X_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - Address matching type for region 6. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r6_a(&mut self) -> R6_A_W<PMPCFG1_SPEC> {
        R6_A_W::new(self, 19)
    }
    #[doc = "Bit 23 - Lock region 6, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r6_l(&mut self) -> R6_L_W<PMPCFG1_SPEC> {
        R6_L_W::new(self, 23)
    }
    #[doc = "Bit 24 - Read permission for region 7"]
    #[inline(always)]
    #[must_use]
    pub fn r7_r(&mut self) -> R7_R_W<PMPCFG1_SPEC> {
        R7_R_W::new(self, 24)
    }
    #[doc = "Bit 25 - Write permission for region 7"]
    #[inline(always)]
    #[must_use]
    pub fn r7_w(&mut self) -> R7_W_W<PMPCFG1_SPEC> {
        R7_W_W::new(self, 25)
    }
    #[doc = "Bit 26 - Execute permission for region 7"]
    #[inline(always)]
    #[must_use]
    pub fn r7_x(&mut self) -> R7_X_W<PMPCFG1_SPEC> {
        R7_X_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Address matching type for region 7. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    #[must_use]
    pub fn r7_a(&mut self) -> R7_A_W<PMPCFG1_SPEC> {
        R7_A_W::new(self, 27)
    }
    #[doc = "Bit 31 - Lock region 7, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    #[must_use]
    pub fn r7_l(&mut self) -> R7_L_W<PMPCFG1_SPEC> {
        R7_L_W::new(self, 31)
    }
}
#[doc = "Physical memory protection configuration for regions 4 through 7  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPCFG1_SPEC;
impl crate::RegisterSpec for PMPCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpcfg1::R`](R) reader structure"]
impl crate::Readable for PMPCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmpcfg1::W`](W) writer structure"]
impl crate::Writable for PMPCFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPCFG1 to value 0"]
impl crate::Resettable for PMPCFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
