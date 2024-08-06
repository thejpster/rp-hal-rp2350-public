#[doc = "Register `PAGE12_LOCK1` reader"]
pub type R = crate::R<PAGE12_LOCK1_SPEC>;
#[doc = "Register `PAGE12_LOCK1` writer"]
pub type W = crate::W<PAGE12_LOCK1_SPEC>;
#[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_S_A {
    #[doc = "0: Page is fully accessible by Secure software."]
    READ_WRITE = 0,
    #[doc = "1: Page can be read by Secure software, but can not be written."]
    READ_ONLY = 1,
    #[doc = "3: Page can not be accessed by Secure software."]
    INACCESSIBLE = 3,
}
impl From<LOCK_S_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_S_A {
    type Ux = u8;
}
impl crate::IsEnum for LOCK_S_A {}
#[doc = "Field `LOCK_S` reader - Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
pub type LOCK_S_R = crate::FieldReader<LOCK_S_A>;
impl LOCK_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_S_A {
        match self.bits {
            0 => LOCK_S_A::READ_WRITE,
            1 => LOCK_S_A::READ_ONLY,
            3 => LOCK_S_A::INACCESSIBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Page is fully accessible by Secure software."]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_S_A::READ_WRITE
    }
    #[doc = "Page can be read by Secure software, but can not be written."]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_S_A::READ_ONLY
    }
    #[doc = "Page can not be accessed by Secure software."]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == LOCK_S_A::INACCESSIBLE
    }
}
#[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_NS_A {
    #[doc = "0: Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    READ_WRITE = 0,
    #[doc = "1: Page can be read by Non-secure software"]
    READ_ONLY = 1,
    #[doc = "3: Page can not be accessed by Non-secure software."]
    INACCESSIBLE = 3,
}
impl From<LOCK_NS_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_NS_A {
    type Ux = u8;
}
impl crate::IsEnum for LOCK_NS_A {}
#[doc = "Field `LOCK_NS` reader - Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
pub type LOCK_NS_R = crate::FieldReader<LOCK_NS_A>;
impl LOCK_NS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_NS_A {
        match self.bits {
            0 => LOCK_NS_A::READ_WRITE,
            1 => LOCK_NS_A::READ_ONLY,
            3 => LOCK_NS_A::INACCESSIBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Page can be read by Non-secure software, and Secure software may permit Non-secure writes."]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_NS_A::READ_WRITE
    }
    #[doc = "Page can be read by Non-secure software"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_NS_A::READ_ONLY
    }
    #[doc = "Page can not be accessed by Non-secure software."]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == LOCK_NS_A::INACCESSIBLE
    }
}
#[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_BL_A {
    #[doc = "0: Bootloader permits user reads and writes to this page"]
    READ_WRITE = 0,
    #[doc = "1: Bootloader permits user reads of this page"]
    READ_ONLY = 1,
    #[doc = "3: Bootloader does not permit user access to this page"]
    INACCESSIBLE = 3,
}
impl From<LOCK_BL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_BL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_BL_A {
    type Ux = u8;
}
impl crate::IsEnum for LOCK_BL_A {}
#[doc = "Field `LOCK_BL` reader - Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
pub type LOCK_BL_R = crate::FieldReader<LOCK_BL_A>;
impl LOCK_BL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_BL_A {
        match self.bits {
            0 => LOCK_BL_A::READ_WRITE,
            1 => LOCK_BL_A::READ_ONLY,
            3 => LOCK_BL_A::INACCESSIBLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Bootloader permits user reads and writes to this page"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_BL_A::READ_WRITE
    }
    #[doc = "Bootloader permits user reads of this page"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_BL_A::READ_ONLY
    }
    #[doc = "Bootloader does not permit user access to this page"]
    #[inline(always)]
    pub fn is_inaccessible(&self) -> bool {
        *self == LOCK_BL_A::INACCESSIBLE
    }
}
#[doc = "Field `R1` reader - Redundant copy of bits 7:0"]
pub type R1_R = crate::FieldReader;
#[doc = "Field `R2` reader - Redundant copy of bits 7:0"]
pub type R2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn lock_s(&self) -> LOCK_S_R {
        LOCK_S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn lock_ns(&self) -> LOCK_NS_R {
        LOCK_NS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn lock_bl(&self) -> LOCK_BL_R {
        LOCK_BL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn r2(&self) -> R2_R {
        R2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions.  

You can [`read`](crate::Reg::read) this register and get [`page12_lock1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`page12_lock1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAGE12_LOCK1_SPEC;
impl crate::RegisterSpec for PAGE12_LOCK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`page12_lock1::R`](R) reader structure"]
impl crate::Readable for PAGE12_LOCK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`page12_lock1::W`](W) writer structure"]
impl crate::Writable for PAGE12_LOCK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAGE12_LOCK1 to value 0"]
impl crate::Resettable for PAGE12_LOCK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
