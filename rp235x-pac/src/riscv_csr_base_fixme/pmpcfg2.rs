#[doc = "Register `PMPCFG2` reader"]
pub type R = crate::R<PMPCFG2_SPEC>;
#[doc = "Field `R8_R` reader - Read permission for region 8"]
pub type R8_R_R = crate::BitReader;
#[doc = "Field `R8_W` reader - Write permission for region 8"]
pub type R8_W_R = crate::BitReader;
#[doc = "Field `R8_X` reader - Execute permission for region 8"]
pub type R8_X_R = crate::BitReader;
#[doc = "Address matching type for region 8. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R8_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R8_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R8_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R8_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R8_A_A {}
#[doc = "Field `R8_A` reader - Address matching type for region 8. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R8_A_R = crate::FieldReader<R8_A_A>;
impl R8_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R8_A_A> {
        match self.bits {
            0 => Some(R8_A_A::OFF),
            2 => Some(R8_A_A::NA4),
            3 => Some(R8_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R8_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R8_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R8_A_A::NAPOT
    }
}
#[doc = "Field `R8_L` reader - Lock region 8, and apply it to M-mode as well as U-mode."]
pub type R8_L_R = crate::BitReader;
#[doc = "Field `R9_R` reader - Read permission for region 9"]
pub type R9_R_R = crate::BitReader;
#[doc = "Field `R9_W` reader - Write permission for region 9"]
pub type R9_W_R = crate::BitReader;
#[doc = "Field `R9_X` reader - Execute permission for region 9"]
pub type R9_X_R = crate::BitReader;
#[doc = "Address matching type for region 9. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R9_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R9_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R9_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R9_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R9_A_A {}
#[doc = "Field `R9_A` reader - Address matching type for region 9. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R9_A_R = crate::FieldReader<R9_A_A>;
impl R9_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R9_A_A> {
        match self.bits {
            0 => Some(R9_A_A::OFF),
            2 => Some(R9_A_A::NA4),
            3 => Some(R9_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R9_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R9_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R9_A_A::NAPOT
    }
}
#[doc = "Field `R9_L` reader - Lock region 9, and apply it to M-mode as well as U-mode."]
pub type R9_L_R = crate::BitReader;
#[doc = "Field `R10_R` reader - Read permission for region 10"]
pub type R10_R_R = crate::BitReader;
#[doc = "Field `R10_W` reader - Write permission for region 10"]
pub type R10_W_R = crate::BitReader;
#[doc = "Field `R10_X` reader - Execute permission for region 10"]
pub type R10_X_R = crate::BitReader;
#[doc = "Address matching type for region 10. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R10_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R10_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R10_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R10_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R10_A_A {}
#[doc = "Field `R10_A` reader - Address matching type for region 10. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R10_A_R = crate::FieldReader<R10_A_A>;
impl R10_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R10_A_A> {
        match self.bits {
            0 => Some(R10_A_A::OFF),
            2 => Some(R10_A_A::NA4),
            3 => Some(R10_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R10_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R10_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R10_A_A::NAPOT
    }
}
#[doc = "Field `R10_L` reader - Lock region 10, and apply it to M-mode as well as U-mode."]
pub type R10_L_R = crate::BitReader;
#[doc = "Field `R11_R` reader - Read permission for region 11"]
pub type R11_R_R = crate::BitReader;
#[doc = "Field `R11_W` reader - Write permission for region 11"]
pub type R11_W_R = crate::BitReader;
#[doc = "Field `R11_X` reader - Execute permission for region 11"]
pub type R11_X_R = crate::BitReader;
#[doc = "Address matching type for region 11. Writing an unsupported value (TOR) will set the region to OFF.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum R11_A_A {
    #[doc = "0: Disable region"]
    OFF = 0,
    #[doc = "2: Naturally aligned 4-byte"]
    NA4 = 2,
    #[doc = "3: Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    NAPOT = 3,
}
impl From<R11_A_A> for u8 {
    #[inline(always)]
    fn from(variant: R11_A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R11_A_A {
    type Ux = u8;
}
impl crate::IsEnum for R11_A_A {}
#[doc = "Field `R11_A` reader - Address matching type for region 11. Writing an unsupported value (TOR) will set the region to OFF."]
pub type R11_A_R = crate::FieldReader<R11_A_A>;
impl R11_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R11_A_A> {
        match self.bits {
            0 => Some(R11_A_A::OFF),
            2 => Some(R11_A_A::NA4),
            3 => Some(R11_A_A::NAPOT),
            _ => None,
        }
    }
    #[doc = "Disable region"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == R11_A_A::OFF
    }
    #[doc = "Naturally aligned 4-byte"]
    #[inline(always)]
    pub fn is_na4(&self) -> bool {
        *self == R11_A_A::NA4
    }
    #[doc = "Naturally aligned power-of-two (8 bytes to 4 GiB)"]
    #[inline(always)]
    pub fn is_napot(&self) -> bool {
        *self == R11_A_A::NAPOT
    }
}
#[doc = "Field `R11_L` reader - Lock region 11, and apply it to M-mode as well as U-mode."]
pub type R11_L_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read permission for region 8"]
    #[inline(always)]
    pub fn r8_r(&self) -> R8_R_R {
        R8_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write permission for region 8"]
    #[inline(always)]
    pub fn r8_w(&self) -> R8_W_R {
        R8_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Execute permission for region 8"]
    #[inline(always)]
    pub fn r8_x(&self) -> R8_X_R {
        R8_X_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Address matching type for region 8. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r8_a(&self) -> R8_A_R {
        R8_A_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock region 8, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r8_l(&self) -> R8_L_R {
        R8_L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read permission for region 9"]
    #[inline(always)]
    pub fn r9_r(&self) -> R9_R_R {
        R9_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write permission for region 9"]
    #[inline(always)]
    pub fn r9_w(&self) -> R9_W_R {
        R9_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Execute permission for region 9"]
    #[inline(always)]
    pub fn r9_x(&self) -> R9_X_R {
        R9_X_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Address matching type for region 9. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r9_a(&self) -> R9_A_R {
        R9_A_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - Lock region 9, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r9_l(&self) -> R9_L_R {
        R9_L_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Read permission for region 10"]
    #[inline(always)]
    pub fn r10_r(&self) -> R10_R_R {
        R10_R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write permission for region 10"]
    #[inline(always)]
    pub fn r10_w(&self) -> R10_W_R {
        R10_W_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Execute permission for region 10"]
    #[inline(always)]
    pub fn r10_x(&self) -> R10_X_R {
        R10_X_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - Address matching type for region 10. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r10_a(&self) -> R10_A_R {
        R10_A_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 23 - Lock region 10, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r10_l(&self) -> R10_L_R {
        R10_L_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Read permission for region 11"]
    #[inline(always)]
    pub fn r11_r(&self) -> R11_R_R {
        R11_R_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write permission for region 11"]
    #[inline(always)]
    pub fn r11_w(&self) -> R11_W_R {
        R11_W_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Execute permission for region 11"]
    #[inline(always)]
    pub fn r11_x(&self) -> R11_X_R {
        R11_X_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Address matching type for region 11. Writing an unsupported value (TOR) will set the region to OFF."]
    #[inline(always)]
    pub fn r11_a(&self) -> R11_A_R {
        R11_A_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - Lock region 11, and apply it to M-mode as well as U-mode."]
    #[inline(always)]
    pub fn r11_l(&self) -> R11_L_R {
        R11_L_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Physical memory protection configuration for regions 8 through 11  

You can [`read`](crate::Reg::read) this register and get [`pmpcfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMPCFG2_SPEC;
impl crate::RegisterSpec for PMPCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpcfg2::R`](R) reader structure"]
impl crate::Readable for PMPCFG2_SPEC {}
#[doc = "`reset()` method sets PMPCFG2 to value 0x001f_1f1f"]
impl crate::Resettable for PMPCFG2_SPEC {
    const RESET_VALUE: u32 = 0x001f_1f1f;
}
