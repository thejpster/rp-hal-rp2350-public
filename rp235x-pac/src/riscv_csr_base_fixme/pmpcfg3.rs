#[doc = "Register `PMPCFG3` reader"]
pub type R = crate::R<PMPCFG3_SPEC>;
#[doc = "Field `R12_R` reader - Read permission for region 12"]
pub type R12_R_R = crate::BitReader;
#[doc = "Field `R12_W` reader - Write permission for region 12"]
pub type R12_W_R = crate::BitReader;
#[doc = "Field `R12_X` reader - Execute permission for region 12"]
pub type R12_X_R = crate::BitReader;
#[doc = "Address matching type for region 12. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R12_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R12_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R12_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R12_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R12_A_A {}
#[doc = "Field `R12_A` reader - Address matching type for region 12. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R12_A_R = crate::FieldReader<R12_A_A>;
impl R12_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R12_A_A> {
        match self.bits {
            0 => Some(R12_A_A::OFF),
            2 => Some(R12_A_A::NA4),
            3 => Some(R12_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R12_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R12_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R12_A_A::NAPOT
    }
}
#[doc = "Field `R12_L` reader - Lock region 12, and apply it to M-mode as well as U-mode."]
pub type R12_L_R = crate::BitReader;
#[doc = "Field `R13_R` reader - Read permission for region 13"]
pub type R13_R_R = crate::BitReader;
#[doc = "Field `R13_W` reader - Write permission for region 13"]
pub type R13_W_R = crate::BitReader;
#[doc = "Field `R13_X` reader - Execute permission for region 13"]
pub type R13_X_R = crate::BitReader;
#[doc = "Address matching type for region 13. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R13_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R13_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R13_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R13_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R13_A_A {}
#[doc = "Field `R13_A` reader - Address matching type for region 13. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R13_A_R = crate::FieldReader<R13_A_A>;
impl R13_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R13_A_A> {
        match self.bits {
            0 => Some(R13_A_A::OFF),
            2 => Some(R13_A_A::NA4),
            3 => Some(R13_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R13_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R13_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R13_A_A::NAPOT
    }
}
#[doc = "Field `R13_L` reader - Lock region 13, and apply it to M-mode as well as U-mode."]
pub type R13_L_R = crate::BitReader;
#[doc = "Field `R14_R` reader - Read permission for region 14"]
pub type R14_R_R = crate::BitReader;
#[doc = "Field `R14_W` reader - Write permission for region 14"]
pub type R14_W_R = crate::BitReader;
#[doc = "Field `R14_X` reader - Execute permission for region 14"]
pub type R14_X_R = crate::BitReader;
#[doc = "Address matching type for region 14. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R14_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R14_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R14_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R14_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R14_A_A {}
#[doc = "Field `R14_A` reader - Address matching type for region 14. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R14_A_R = crate::FieldReader<R14_A_A>;
impl R14_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R14_A_A> {
        match self.bits {
            0 => Some(R14_A_A::OFF),
            2 => Some(R14_A_A::NA4),
            3 => Some(R14_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R14_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R14_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R14_A_A::NAPOT
    }
}
#[doc = "Field `R14_L` reader - Lock region 14, and apply it to M-mode as well as U-mode."]
pub type R14_L_R = crate::BitReader;
#[doc = "Field `R15_R` reader - Read permission for region 15"]
pub type R15_R_R = crate::BitReader;
#[doc = "Field `R15_W` reader - Write permission for region 15"]
pub type R15_W_R = crate::BitReader;
#[doc = "Field `R15_X` reader - Execute permission for region 15"]
pub type R15_X_R = crate::BitReader;
#[doc = "Address matching type for region 15. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R15_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R15_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R15_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R15_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R15_A_A {}
#[doc = "Field `R15_A` reader - Address matching type for region 15. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R15_A_R = crate::FieldReader<R15_A_A>;
impl R15_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R15_A_A> {
        match self.bits {
            0 => Some(R15_A_A::OFF),
            2 => Some(R15_A_A::NA4),
            3 => Some(R15_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R15_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R15_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R15_A_A::NAPOT
    }
}
#[doc = "Field `R15_L` reader - Lock region 15, and apply it to M-mode as well as U-mode."]
pub type R15_L_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read permission for region 12"]
    #[inline(always)]
    pub fn r12_r(&self) -> R12_R_R {
        R12_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write permission for region 12"]
    #[inline(always)]
    pub fn r12_w(&self) -> R12_W_R {
        R12_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute permission for region 12"]
    #[inline(always)]
    pub fn r12_x(&self) -> R12_X_R {
        R12_X_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Address matching type for region 12. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r12_a(&self) -> R12_A_R {
        R12_A_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock region 12, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r12_l(&self) -> R12_L_R {
        R12_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read permission for region 13"]
    #[inline(always)]
    pub fn r13_r(&self) -> R13_R_R {
        R13_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write permission for region 13"]
    #[inline(always)]
    pub fn r13_w(&self) -> R13_W_R {
        R13_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Execute permission for region 13"]
    #[inline(always)]
    pub fn r13_x(&self) -> R13_X_R {
        R13_X_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Address matching type for region 13. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r13_a(&self) -> R13_A_R {
        R13_A_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - Lock region 13, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r13_l(&self) -> R13_L_R {
        R13_L_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read permission for region 14"]
    #[inline(always)]
    pub fn r14_r(&self) -> R14_R_R {
        R14_R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write permission for region 14"]
    #[inline(always)]
    pub fn r14_w(&self) -> R14_W_R {
        R14_W_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Execute permission for region 14"]
    #[inline(always)]
    pub fn r14_x(&self) -> R14_X_R {
        R14_X_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - Address matching type for region 14. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r14_a(&self) -> R14_A_R {
        R14_A_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 23 - Lock region 14, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r14_l(&self) -> R14_L_R {
        R14_L_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Read permission for region 15"]
    #[inline(always)]
    pub fn r15_r(&self) -> R15_R_R {
        R15_R_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write permission for region 15"]
    #[inline(always)]
    pub fn r15_w(&self) -> R15_W_R {
        R15_W_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Execute permission for region 15"]
    #[inline(always)]
    pub fn r15_x(&self) -> R15_X_R {
        R15_X_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Address matching type for region 15. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r15_a(&self) -> R15_A_R {
        R15_A_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - Lock region 15, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r15_l(&self) -> R15_L_R {
        R15_L_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Physical memory protection configuration for regions 12 through 15  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPCFG3_SPEC;
impl crate::RegisterSpec for PMPCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpcfg3::R`](R) reader structure"]
impl crate::Readable for PMPCFG3_SPEC {}
#[doc = "`reset()` method sets PMPCFG3 to value 0"]
impl crate::Resettable for PMPCFG3_SPEC {
    const RESET_VALUE: u32 = 0;
}
