#[doc = "Register `M1_RFMT` reader"]
pub type R = crate::R<M1_RFMT_SPEC>;
#[doc = "Register `M1_RFMT` writer"]
pub type W = crate::W<M1_RFMT_SPEC>;
#[doc = "The transfer width used for the command prefix, if any  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREFIX_WIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<PREFIX_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: PREFIX_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREFIX_WIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for PREFIX_WIDTH_A {}
#[doc = "Field `PREFIX_WIDTH` reader - The transfer width used for the command prefix, if any"]
pub type PREFIX_WIDTH_R = crate::FieldReader<PREFIX_WIDTH_A>;
impl PREFIX_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PREFIX_WIDTH_A> {
        match self.bits {
            0 => Some(PREFIX_WIDTH_A::S),
            1 => Some(PREFIX_WIDTH_A::D),
            2 => Some(PREFIX_WIDTH_A::Q),
            _ => None,
        }
    }
    #[doc = "Single width"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == PREFIX_WIDTH_A::S
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == PREFIX_WIDTH_A::D
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn is_q(&self) -> bool {
        *self == PREFIX_WIDTH_A::Q
    }
}
#[doc = "Field `PREFIX_WIDTH` writer - The transfer width used for the command prefix, if any"]
pub type PREFIX_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PREFIX_WIDTH_A>;
impl<'a, REG> PREFIX_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(PREFIX_WIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(PREFIX_WIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(PREFIX_WIDTH_A::Q)
    }
}
#[doc = "The transfer width used for the address. The address phase always transfers 24 bits in total.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADDR_WIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<ADDR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDR_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADDR_WIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for ADDR_WIDTH_A {}
#[doc = "Field `ADDR_WIDTH` reader - The transfer width used for the address. The address phase always transfers 24 bits in total."]
pub type ADDR_WIDTH_R = crate::FieldReader<ADDR_WIDTH_A>;
impl ADDR_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADDR_WIDTH_A> {
        match self.bits {
            0 => Some(ADDR_WIDTH_A::S),
            1 => Some(ADDR_WIDTH_A::D),
            2 => Some(ADDR_WIDTH_A::Q),
            _ => None,
        }
    }
    #[doc = "Single width"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == ADDR_WIDTH_A::S
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == ADDR_WIDTH_A::D
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn is_q(&self) -> bool {
        *self == ADDR_WIDTH_A::Q
    }
}
#[doc = "Field `ADDR_WIDTH` writer - The transfer width used for the address. The address phase always transfers 24 bits in total."]
pub type ADDR_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADDR_WIDTH_A>;
impl<'a, REG> ADDR_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_WIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_WIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR_WIDTH_A::Q)
    }
}
#[doc = "The width used for the post-address command suffix, if any  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUFFIX_WIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<SUFFIX_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SUFFIX_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUFFIX_WIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for SUFFIX_WIDTH_A {}
#[doc = "Field `SUFFIX_WIDTH` reader - The width used for the post-address command suffix, if any"]
pub type SUFFIX_WIDTH_R = crate::FieldReader<SUFFIX_WIDTH_A>;
impl SUFFIX_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUFFIX_WIDTH_A> {
        match self.bits {
            0 => Some(SUFFIX_WIDTH_A::S),
            1 => Some(SUFFIX_WIDTH_A::D),
            2 => Some(SUFFIX_WIDTH_A::Q),
            _ => None,
        }
    }
    #[doc = "Single width"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == SUFFIX_WIDTH_A::S
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == SUFFIX_WIDTH_A::D
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn is_q(&self) -> bool {
        *self == SUFFIX_WIDTH_A::Q
    }
}
#[doc = "Field `SUFFIX_WIDTH` writer - The width used for the post-address command suffix, if any"]
pub type SUFFIX_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SUFFIX_WIDTH_A>;
impl<'a, REG> SUFFIX_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(SUFFIX_WIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(SUFFIX_WIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(SUFFIX_WIDTH_A::Q)
    }
}
#[doc = "The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUMMY_WIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<DUMMY_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DUMMY_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUMMY_WIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for DUMMY_WIDTH_A {}
#[doc = "Field `DUMMY_WIDTH` reader - The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
pub type DUMMY_WIDTH_R = crate::FieldReader<DUMMY_WIDTH_A>;
impl DUMMY_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DUMMY_WIDTH_A> {
        match self.bits {
            0 => Some(DUMMY_WIDTH_A::S),
            1 => Some(DUMMY_WIDTH_A::D),
            2 => Some(DUMMY_WIDTH_A::Q),
            _ => None,
        }
    }
    #[doc = "Single width"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == DUMMY_WIDTH_A::S
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == DUMMY_WIDTH_A::D
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn is_q(&self) -> bool {
        *self == DUMMY_WIDTH_A::Q
    }
}
#[doc = "Field `DUMMY_WIDTH` writer - The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
pub type DUMMY_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DUMMY_WIDTH_A>;
impl<'a, REG> DUMMY_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_WIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_WIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_WIDTH_A::Q)
    }
}
#[doc = "The width used for the data transfer  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA_WIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for DATA_WIDTH_A {}
#[doc = "Field `DATA_WIDTH` reader - The width used for the data transfer"]
pub type DATA_WIDTH_R = crate::FieldReader<DATA_WIDTH_A>;
impl DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::S),
            1 => Some(DATA_WIDTH_A::D),
            2 => Some(DATA_WIDTH_A::Q),
            _ => None,
        }
    }
    #[doc = "Single width"]
    #[inline(always)]
    pub fn is_s(&self) -> bool {
        *self == DATA_WIDTH_A::S
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == DATA_WIDTH_A::D
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn is_q(&self) -> bool {
        *self == DATA_WIDTH_A::Q
    }
}
#[doc = "Field `DATA_WIDTH` writer - The width used for the data transfer"]
pub type DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATA_WIDTH_A>;
impl<'a, REG> DATA_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::Q)
    }
}
#[doc = "Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREFIX_LEN_A {
    #[doc = "0: No prefix"]
    NONE = 0,
    #[doc = "1: 8-bit prefix"]
    _8 = 1,
}
impl From<PREFIX_LEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFIX_LEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFIX_LEN` reader - Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
pub type PREFIX_LEN_R = crate::BitReader<PREFIX_LEN_A>;
impl PREFIX_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PREFIX_LEN_A {
        match self.bits {
            false => PREFIX_LEN_A::NONE,
            true => PREFIX_LEN_A::_8,
        }
    }
    #[doc = "No prefix"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PREFIX_LEN_A::NONE
    }
    #[doc = "8-bit prefix"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PREFIX_LEN_A::_8
    }
}
#[doc = "Field `PREFIX_LEN` writer - Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
pub type PREFIX_LEN_W<'a, REG> = crate::BitWriter<'a, REG, PREFIX_LEN_A>;
impl<'a, REG> PREFIX_LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No prefix"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PREFIX_LEN_A::NONE)
    }
    #[doc = "8-bit prefix"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(PREFIX_LEN_A::_8)
    }
}
#[doc = "Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUFFIX_LEN_A {
    #[doc = "0: No suffix"]
    NONE = 0,
    #[doc = "2: 8-bit suffix"]
    _8 = 2,
}
impl From<SUFFIX_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: SUFFIX_LEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUFFIX_LEN_A {
    type Ux = u8;
}
impl crate::IsEnum for SUFFIX_LEN_A {}
#[doc = "Field `SUFFIX_LEN` reader - Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
pub type SUFFIX_LEN_R = crate::FieldReader<SUFFIX_LEN_A>;
impl SUFFIX_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SUFFIX_LEN_A> {
        match self.bits {
            0 => Some(SUFFIX_LEN_A::NONE),
            2 => Some(SUFFIX_LEN_A::_8),
            _ => None,
        }
    }
    #[doc = "No suffix"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SUFFIX_LEN_A::NONE
    }
    #[doc = "8-bit suffix"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SUFFIX_LEN_A::_8
    }
}
#[doc = "Field `SUFFIX_LEN` writer - Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
pub type SUFFIX_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SUFFIX_LEN_A>;
impl<'a, REG> SUFFIX_LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No suffix"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SUFFIX_LEN_A::NONE)
    }
    #[doc = "8-bit suffix"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(SUFFIX_LEN_A::_8)
    }
}
#[doc = "Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUMMY_LEN_A {
    #[doc = "0: No dummy phase"]
    NONE = 0,
    #[doc = "1: 4 dummy bits"]
    _4 = 1,
    #[doc = "2: 8 dummy bits"]
    _8 = 2,
    #[doc = "3: 12 dummy bits"]
    _12 = 3,
    #[doc = "4: 16 dummy bits"]
    _16 = 4,
    #[doc = "5: 20 dummy bits"]
    _20 = 5,
    #[doc = "6: 24 dummy bits"]
    _24 = 6,
    #[doc = "7: 28 dummy bits"]
    _28 = 7,
}
impl From<DUMMY_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DUMMY_LEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUMMY_LEN_A {
    type Ux = u8;
}
impl crate::IsEnum for DUMMY_LEN_A {}
#[doc = "Field `DUMMY_LEN` reader - Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
pub type DUMMY_LEN_R = crate::FieldReader<DUMMY_LEN_A>;
impl DUMMY_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUMMY_LEN_A {
        match self.bits {
            0 => DUMMY_LEN_A::NONE,
            1 => DUMMY_LEN_A::_4,
            2 => DUMMY_LEN_A::_8,
            3 => DUMMY_LEN_A::_12,
            4 => DUMMY_LEN_A::_16,
            5 => DUMMY_LEN_A::_20,
            6 => DUMMY_LEN_A::_24,
            7 => DUMMY_LEN_A::_28,
            _ => unreachable!(),
        }
    }
    #[doc = "No dummy phase"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DUMMY_LEN_A::NONE
    }
    #[doc = "4 dummy bits"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == DUMMY_LEN_A::_4
    }
    #[doc = "8 dummy bits"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DUMMY_LEN_A::_8
    }
    #[doc = "12 dummy bits"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == DUMMY_LEN_A::_12
    }
    #[doc = "16 dummy bits"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DUMMY_LEN_A::_16
    }
    #[doc = "20 dummy bits"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == DUMMY_LEN_A::_20
    }
    #[doc = "24 dummy bits"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == DUMMY_LEN_A::_24
    }
    #[doc = "28 dummy bits"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == DUMMY_LEN_A::_28
    }
}
#[doc = "Field `DUMMY_LEN` writer - Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
pub type DUMMY_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DUMMY_LEN_A, crate::Safe>;
impl<'a, REG> DUMMY_LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No dummy phase"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::NONE)
    }
    #[doc = "4 dummy bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_4)
    }
    #[doc = "8 dummy bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_8)
    }
    #[doc = "12 dummy bits"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_12)
    }
    #[doc = "16 dummy bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_16)
    }
    #[doc = "20 dummy bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_20)
    }
    #[doc = "24 dummy bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_24)
    }
    #[doc = "28 dummy bits"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut crate::W<REG> {
        self.variant(DUMMY_LEN_A::_28)
    }
}
#[doc = "Field `DTR` reader - Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
pub type DTR_R = crate::BitReader;
#[doc = "Field `DTR` writer - Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
pub type DTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - The transfer width used for the command prefix, if any"]
    #[inline(always)]
    pub fn prefix_width(&self) -> PREFIX_WIDTH_R {
        PREFIX_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The width used for the post-address command suffix, if any"]
    #[inline(always)]
    pub fn suffix_width(&self) -> SUFFIX_WIDTH_R {
        SUFFIX_WIDTH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    pub fn dummy_width(&self) -> DUMMY_WIDTH_R {
        DUMMY_WIDTH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The width used for the data transfer"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    pub fn prefix_len(&self) -> PREFIX_LEN_R {
        PREFIX_LEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    pub fn suffix_len(&self) -> SUFFIX_LEN_R {
        SUFFIX_LEN_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    pub fn dummy_len(&self) -> DUMMY_LEN_R {
        DUMMY_LEN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - The transfer width used for the command prefix, if any"]
    #[inline(always)]
    #[must_use]
    pub fn prefix_width(&mut self) -> PREFIX_WIDTH_W<M1_RFMT_SPEC> {
        PREFIX_WIDTH_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    #[must_use]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W<M1_RFMT_SPEC> {
        ADDR_WIDTH_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - The width used for the post-address command suffix, if any"]
    #[inline(always)]
    #[must_use]
    pub fn suffix_width(&mut self) -> SUFFIX_WIDTH_W<M1_RFMT_SPEC> {
        SUFFIX_WIDTH_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    #[must_use]
    pub fn dummy_width(&mut self) -> DUMMY_WIDTH_W<M1_RFMT_SPEC> {
        DUMMY_WIDTH_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - The width used for the data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<M1_RFMT_SPEC> {
        DATA_WIDTH_W::new(self, 8)
    }
    #[doc = "Bit 12 - Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    #[must_use]
    pub fn prefix_len(&mut self) -> PREFIX_LEN_W<M1_RFMT_SPEC> {
        PREFIX_LEN_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    #[must_use]
    pub fn suffix_len(&mut self) -> SUFFIX_LEN_W<M1_RFMT_SPEC> {
        SUFFIX_LEN_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    #[must_use]
    pub fn dummy_len(&mut self) -> DUMMY_LEN_W<M1_RFMT_SPEC> {
        DUMMY_LEN_W::new(self, 16)
    }
    #[doc = "Bit 28 - Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<M1_RFMT_SPEC> {
        DTR_W::new(self, 28)
    }
}
#[doc = "Read transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.  

You can [`read`](crate::Reg::read) this register and get [`m1_rfmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1_rfmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1_RFMT_SPEC;
impl crate::RegisterSpec for M1_RFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1_rfmt::R`](R) reader structure"]
impl crate::Readable for M1_RFMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m1_rfmt::W`](W) writer structure"]
impl crate::Writable for M1_RFMT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1_RFMT to value 0x1000"]
impl crate::Resettable for M1_RFMT_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
