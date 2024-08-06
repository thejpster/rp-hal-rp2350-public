#[doc = "Register `CH_AL2_CTRL` reader"]
pub type R = crate::R<CH_AL2_CTRL_SPEC>;
#[doc = "Register `CH_AL2_CTRL` writer"]
pub type W = crate::W<CH_AL2_CTRL_SPEC>;
#[doc = "Field `EN` reader - DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGH_PRIORITY` reader - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
pub type HIGH_PRIORITY_R = crate::BitReader;
#[doc = "Field `HIGH_PRIORITY` writer - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
pub type HIGH_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA_SIZE_A {
    #[doc = "0: `0`"]
    SIZE_BYTE = 0,
    #[doc = "1: `1`"]
    SIZE_HALFWORD = 1,
    #[doc = "2: `10`"]
    SIZE_WORD = 2,
}
impl From<DATA_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA_SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for DATA_SIZE_A {}
#[doc = "Field `DATA_SIZE` reader - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
pub type DATA_SIZE_R = crate::FieldReader<DATA_SIZE_A>;
impl DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATA_SIZE_A> {
        match self.bits {
            0 => Some(DATA_SIZE_A::SIZE_BYTE),
            1 => Some(DATA_SIZE_A::SIZE_HALFWORD),
            2 => Some(DATA_SIZE_A::SIZE_WORD),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_size_byte(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_BYTE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_size_halfword(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_HALFWORD
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_size_word(&self) -> bool {
        *self == DATA_SIZE_A::SIZE_WORD
    }
}
#[doc = "Field `DATA_SIZE` writer - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
pub type DATA_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATA_SIZE_A>;
impl<'a, REG> DATA_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn size_byte(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_SIZE_A::SIZE_BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn size_halfword(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_SIZE_A::SIZE_HALFWORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn size_word(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_SIZE_A::SIZE_WORD)
    }
}
#[doc = "Field `INCR_READ` reader - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
pub type INCR_READ_R = crate::BitReader;
#[doc = "Field `INCR_READ` writer - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
pub type INCR_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR_READ_REV` reader - If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
pub type INCR_READ_REV_R = crate::BitReader;
#[doc = "Field `INCR_READ_REV` writer - If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
pub type INCR_READ_REV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR_WRITE` reader - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
pub type INCR_WRITE_R = crate::BitReader;
#[doc = "Field `INCR_WRITE` writer - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
pub type INCR_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCR_WRITE_REV` reader - If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
pub type INCR_WRITE_REV_R = crate::BitReader;
#[doc = "Field `INCR_WRITE_REV` writer - If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
pub type INCR_WRITE_REV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 &lt;&lt; n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RING_SIZE_A {
    #[doc = "0: `0`"]
    RING_NONE = 0,
}
impl From<RING_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RING_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RING_SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for RING_SIZE_A {}
#[doc = "Field `RING_SIZE` reader - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 &lt;&lt; n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
pub type RING_SIZE_R = crate::FieldReader<RING_SIZE_A>;
impl RING_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RING_SIZE_A> {
        match self.bits {
            0 => Some(RING_SIZE_A::RING_NONE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ring_none(&self) -> bool {
        *self == RING_SIZE_A::RING_NONE
    }
}
#[doc = "Field `RING_SIZE` writer - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 &lt;&lt; n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
pub type RING_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, RING_SIZE_A>;
impl<'a, REG> RING_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ring_none(self) -> &'a mut crate::W<REG> {
        self.variant(RING_SIZE_A::RING_NONE)
    }
}
#[doc = "Field `RING_SEL` reader - Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 &lt;&lt; RING_SIZE) boundary. If 1, write addresses are wrapped."]
pub type RING_SEL_R = crate::BitReader;
#[doc = "Field `RING_SEL` writer - Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 &lt;&lt; RING_SIZE) boundary. If 1, write addresses are wrapped."]
pub type RING_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAIN_TO` reader - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.   
 Reset value is 0, which means for channels 1 and above the default will be to chain to channel 0 - set this field to avoid this behaviour."]
pub type CHAIN_TO_R = crate::FieldReader;
#[doc = "Field `CHAIN_TO` writer - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.   
 Reset value is 0, which means for channels 1 and above the default will be to chain to channel 0 - set this field to avoid this behaviour."]
pub type CHAIN_TO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TREQ_SEL_A {
    #[doc = "0: Select PIO0's TX FIFO 0 as TREQ"]
    PIO0_TX0 = 0,
    #[doc = "1: Select PIO0's TX FIFO 1 as TREQ"]
    PIO0_TX1 = 1,
    #[doc = "2: Select PIO0's TX FIFO 2 as TREQ"]
    PIO0_TX2 = 2,
    #[doc = "3: Select PIO0's TX FIFO 3 as TREQ"]
    PIO0_TX3 = 3,
    #[doc = "4: Select PIO0's RX FIFO 0 as TREQ"]
    PIO0_RX0 = 4,
    #[doc = "5: Select PIO0's RX FIFO 1 as TREQ"]
    PIO0_RX1 = 5,
    #[doc = "6: Select PIO0's RX FIFO 2 as TREQ"]
    PIO0_RX2 = 6,
    #[doc = "7: Select PIO0's RX FIFO 3 as TREQ"]
    PIO0_RX3 = 7,
    #[doc = "8: Select PIO1's TX FIFO 0 as TREQ"]
    PIO1_TX0 = 8,
    #[doc = "9: Select PIO1's TX FIFO 1 as TREQ"]
    PIO1_TX1 = 9,
    #[doc = "10: Select PIO1's TX FIFO 2 as TREQ"]
    PIO1_TX2 = 10,
    #[doc = "11: Select PIO1's TX FIFO 3 as TREQ"]
    PIO1_TX3 = 11,
    #[doc = "12: Select PIO1's RX FIFO 0 as TREQ"]
    PIO1_RX0 = 12,
    #[doc = "13: Select PIO1's RX FIFO 1 as TREQ"]
    PIO1_RX1 = 13,
    #[doc = "14: Select PIO1's RX FIFO 2 as TREQ"]
    PIO1_RX2 = 14,
    #[doc = "15: Select PIO1's RX FIFO 3 as TREQ"]
    PIO1_RX3 = 15,
    #[doc = "16: Select PIO2's TX FIFO 0 as TREQ"]
    PIO2_TX0 = 16,
    #[doc = "17: Select PIO2's TX FIFO 1 as TREQ"]
    PIO2_TX1 = 17,
    #[doc = "18: Select PIO2's TX FIFO 2 as TREQ"]
    PIO2_TX2 = 18,
    #[doc = "19: Select PIO2's TX FIFO 3 as TREQ"]
    PIO2_TX3 = 19,
    #[doc = "20: Select PIO2's RX FIFO 0 as TREQ"]
    PIO2_RX0 = 20,
    #[doc = "21: Select PIO2's RX FIFO 1 as TREQ"]
    PIO2_RX1 = 21,
    #[doc = "22: Select PIO2's RX FIFO 2 as TREQ"]
    PIO2_RX2 = 22,
    #[doc = "23: Select PIO2's RX FIFO 3 as TREQ"]
    PIO2_RX3 = 23,
    #[doc = "24: Select SPI0's TX FIFO as TREQ"]
    SPI0_TX = 24,
    #[doc = "25: Select SPI0's RX FIFO as TREQ"]
    SPI0_RX = 25,
    #[doc = "26: Select SPI1's TX FIFO as TREQ"]
    SPI1_TX = 26,
    #[doc = "27: Select SPI1's RX FIFO as TREQ"]
    SPI1_RX = 27,
    #[doc = "28: Select UART0's TX FIFO as TREQ"]
    UART0_TX = 28,
    #[doc = "29: Select UART0's RX FIFO as TREQ"]
    UART0_RX = 29,
    #[doc = "30: Select UART1's TX FIFO as TREQ"]
    UART1_TX = 30,
    #[doc = "31: Select UART1's RX FIFO as TREQ"]
    UART1_RX = 31,
    #[doc = "32: Select PWM Counter 0's Wrap Value as TREQ"]
    PWM_WRAP0 = 32,
    #[doc = "33: Select PWM Counter 1's Wrap Value as TREQ"]
    PWM_WRAP1 = 33,
    #[doc = "34: Select PWM Counter 2's Wrap Value as TREQ"]
    PWM_WRAP2 = 34,
    #[doc = "35: Select PWM Counter 3's Wrap Value as TREQ"]
    PWM_WRAP3 = 35,
    #[doc = "36: Select PWM Counter 4's Wrap Value as TREQ"]
    PWM_WRAP4 = 36,
    #[doc = "37: Select PWM Counter 5's Wrap Value as TREQ"]
    PWM_WRAP5 = 37,
    #[doc = "38: Select PWM Counter 6's Wrap Value as TREQ"]
    PWM_WRAP6 = 38,
    #[doc = "39: Select PWM Counter 7's Wrap Value as TREQ"]
    PWM_WRAP7 = 39,
    #[doc = "40: Select PWM Counter 8's Wrap Value as TREQ"]
    PWM_WRAP8 = 40,
    #[doc = "41: Select PWM Counter 9's Wrap Value as TREQ"]
    PWM_WRAP9 = 41,
    #[doc = "42: Select PWM Counter 10's Wrap Value as TREQ"]
    PWM_WRAP10 = 42,
    #[doc = "43: Select PWM Counter 11's Wrap Value as TREQ"]
    PWM_WRAP11 = 43,
    #[doc = "44: Select I2C0's TX FIFO as TREQ"]
    I2C0_TX = 44,
    #[doc = "45: Select I2C0's RX FIFO as TREQ"]
    I2C0_RX = 45,
    #[doc = "46: Select I2C1's TX FIFO as TREQ"]
    I2C1_TX = 46,
    #[doc = "47: Select I2C1's RX FIFO as TREQ"]
    I2C1_RX = 47,
    #[doc = "48: Select ADC as TREQ"]
    ADC = 48,
    #[doc = "49: Select XIP_STREAM as TREQ"]
    XIP_STREAM = 49,
    #[doc = "50: Select XIP_QMI's TX FIFO as TREQ"]
    XIP_QMITX = 50,
    #[doc = "51: Select XIP_QMI's RX FIFO as TREQ"]
    XIP_QMIRX = 51,
    #[doc = "52: Select HSTX as TREQ"]
    HSTX = 52,
    #[doc = "53: Select CORESIGHT as TREQ"]
    CORESIGHT = 53,
    #[doc = "54: Select SHA256 as TREQ"]
    SHA256 = 54,
    #[doc = "59: Select Timer 0 as TREQ"]
    TIMER0 = 59,
    #[doc = "60: Select Timer 1 as TREQ"]
    TIMER1 = 60,
    #[doc = "61: Select Timer 2 as TREQ (Optional)"]
    TIMER2 = 61,
    #[doc = "62: Select Timer 3 as TREQ (Optional)"]
    TIMER3 = 62,
    #[doc = "63: Permanent request, for unpaced transfers."]
    PERMANENT = 63,
}
impl From<TREQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TREQ_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TREQ_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TREQ_SEL_A {}
#[doc = "Field `TREQ_SEL` reader - Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
pub type TREQ_SEL_R = crate::FieldReader<TREQ_SEL_A>;
impl TREQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TREQ_SEL_A> {
        match self.bits {
            0 => Some(TREQ_SEL_A::PIO0_TX0),
            1 => Some(TREQ_SEL_A::PIO0_TX1),
            2 => Some(TREQ_SEL_A::PIO0_TX2),
            3 => Some(TREQ_SEL_A::PIO0_TX3),
            4 => Some(TREQ_SEL_A::PIO0_RX0),
            5 => Some(TREQ_SEL_A::PIO0_RX1),
            6 => Some(TREQ_SEL_A::PIO0_RX2),
            7 => Some(TREQ_SEL_A::PIO0_RX3),
            8 => Some(TREQ_SEL_A::PIO1_TX0),
            9 => Some(TREQ_SEL_A::PIO1_TX1),
            10 => Some(TREQ_SEL_A::PIO1_TX2),
            11 => Some(TREQ_SEL_A::PIO1_TX3),
            12 => Some(TREQ_SEL_A::PIO1_RX0),
            13 => Some(TREQ_SEL_A::PIO1_RX1),
            14 => Some(TREQ_SEL_A::PIO1_RX2),
            15 => Some(TREQ_SEL_A::PIO1_RX3),
            16 => Some(TREQ_SEL_A::PIO2_TX0),
            17 => Some(TREQ_SEL_A::PIO2_TX1),
            18 => Some(TREQ_SEL_A::PIO2_TX2),
            19 => Some(TREQ_SEL_A::PIO2_TX3),
            20 => Some(TREQ_SEL_A::PIO2_RX0),
            21 => Some(TREQ_SEL_A::PIO2_RX1),
            22 => Some(TREQ_SEL_A::PIO2_RX2),
            23 => Some(TREQ_SEL_A::PIO2_RX3),
            24 => Some(TREQ_SEL_A::SPI0_TX),
            25 => Some(TREQ_SEL_A::SPI0_RX),
            26 => Some(TREQ_SEL_A::SPI1_TX),
            27 => Some(TREQ_SEL_A::SPI1_RX),
            28 => Some(TREQ_SEL_A::UART0_TX),
            29 => Some(TREQ_SEL_A::UART0_RX),
            30 => Some(TREQ_SEL_A::UART1_TX),
            31 => Some(TREQ_SEL_A::UART1_RX),
            32 => Some(TREQ_SEL_A::PWM_WRAP0),
            33 => Some(TREQ_SEL_A::PWM_WRAP1),
            34 => Some(TREQ_SEL_A::PWM_WRAP2),
            35 => Some(TREQ_SEL_A::PWM_WRAP3),
            36 => Some(TREQ_SEL_A::PWM_WRAP4),
            37 => Some(TREQ_SEL_A::PWM_WRAP5),
            38 => Some(TREQ_SEL_A::PWM_WRAP6),
            39 => Some(TREQ_SEL_A::PWM_WRAP7),
            40 => Some(TREQ_SEL_A::PWM_WRAP8),
            41 => Some(TREQ_SEL_A::PWM_WRAP9),
            42 => Some(TREQ_SEL_A::PWM_WRAP10),
            43 => Some(TREQ_SEL_A::PWM_WRAP11),
            44 => Some(TREQ_SEL_A::I2C0_TX),
            45 => Some(TREQ_SEL_A::I2C0_RX),
            46 => Some(TREQ_SEL_A::I2C1_TX),
            47 => Some(TREQ_SEL_A::I2C1_RX),
            48 => Some(TREQ_SEL_A::ADC),
            49 => Some(TREQ_SEL_A::XIP_STREAM),
            50 => Some(TREQ_SEL_A::XIP_QMITX),
            51 => Some(TREQ_SEL_A::XIP_QMIRX),
            52 => Some(TREQ_SEL_A::HSTX),
            53 => Some(TREQ_SEL_A::CORESIGHT),
            54 => Some(TREQ_SEL_A::SHA256),
            59 => Some(TREQ_SEL_A::TIMER0),
            60 => Some(TREQ_SEL_A::TIMER1),
            61 => Some(TREQ_SEL_A::TIMER2),
            62 => Some(TREQ_SEL_A::TIMER3),
            63 => Some(TREQ_SEL_A::PERMANENT),
            _ => None,
        }
    }
    #[doc = "Select PIO0's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_tx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_TX0
    }
    #[doc = "Select PIO0's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_tx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_TX1
    }
    #[doc = "Select PIO0's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_tx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_TX2
    }
    #[doc = "Select PIO0's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_tx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_TX3
    }
    #[doc = "Select PIO0's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_rx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_RX0
    }
    #[doc = "Select PIO0's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_rx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_RX1
    }
    #[doc = "Select PIO0's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_rx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_RX2
    }
    #[doc = "Select PIO0's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio0_rx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO0_RX3
    }
    #[doc = "Select PIO1's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_tx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_TX0
    }
    #[doc = "Select PIO1's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_tx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_TX1
    }
    #[doc = "Select PIO1's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_tx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_TX2
    }
    #[doc = "Select PIO1's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_tx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_TX3
    }
    #[doc = "Select PIO1's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_rx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_RX0
    }
    #[doc = "Select PIO1's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_rx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_RX1
    }
    #[doc = "Select PIO1's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_rx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_RX2
    }
    #[doc = "Select PIO1's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio1_rx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO1_RX3
    }
    #[doc = "Select PIO2's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_tx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_TX0
    }
    #[doc = "Select PIO2's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_tx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_TX1
    }
    #[doc = "Select PIO2's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_tx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_TX2
    }
    #[doc = "Select PIO2's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_tx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_TX3
    }
    #[doc = "Select PIO2's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_rx0(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_RX0
    }
    #[doc = "Select PIO2's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_rx1(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_RX1
    }
    #[doc = "Select PIO2's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_rx2(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_RX2
    }
    #[doc = "Select PIO2's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn is_pio2_rx3(&self) -> bool {
        *self == TREQ_SEL_A::PIO2_RX3
    }
    #[doc = "Select SPI0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == TREQ_SEL_A::SPI0_TX
    }
    #[doc = "Select SPI0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == TREQ_SEL_A::SPI0_RX
    }
    #[doc = "Select SPI1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == TREQ_SEL_A::SPI1_TX
    }
    #[doc = "Select SPI1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == TREQ_SEL_A::SPI1_RX
    }
    #[doc = "Select UART0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == TREQ_SEL_A::UART0_TX
    }
    #[doc = "Select UART0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == TREQ_SEL_A::UART0_RX
    }
    #[doc = "Select UART1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == TREQ_SEL_A::UART1_TX
    }
    #[doc = "Select UART1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == TREQ_SEL_A::UART1_RX
    }
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap0(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP0
    }
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap1(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP1
    }
    #[doc = "Select PWM Counter 2's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap2(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP2
    }
    #[doc = "Select PWM Counter 3's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap3(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP3
    }
    #[doc = "Select PWM Counter 4's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap4(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP4
    }
    #[doc = "Select PWM Counter 5's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap5(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP5
    }
    #[doc = "Select PWM Counter 6's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap6(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP6
    }
    #[doc = "Select PWM Counter 7's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap7(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP7
    }
    #[doc = "Select PWM Counter 8's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap8(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP8
    }
    #[doc = "Select PWM Counter 9's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap9(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP9
    }
    #[doc = "Select PWM Counter 10's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap10(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP10
    }
    #[doc = "Select PWM Counter 11's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn is_pwm_wrap11(&self) -> bool {
        *self == TREQ_SEL_A::PWM_WRAP11
    }
    #[doc = "Select I2C0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_i2c0_tx(&self) -> bool {
        *self == TREQ_SEL_A::I2C0_TX
    }
    #[doc = "Select I2C0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_i2c0_rx(&self) -> bool {
        *self == TREQ_SEL_A::I2C0_RX
    }
    #[doc = "Select I2C1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_i2c1_tx(&self) -> bool {
        *self == TREQ_SEL_A::I2C1_TX
    }
    #[doc = "Select I2C1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_i2c1_rx(&self) -> bool {
        *self == TREQ_SEL_A::I2C1_RX
    }
    #[doc = "Select ADC as TREQ"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == TREQ_SEL_A::ADC
    }
    #[doc = "Select XIP_STREAM as TREQ"]
    #[inline(always)]
    pub fn is_xip_stream(&self) -> bool {
        *self == TREQ_SEL_A::XIP_STREAM
    }
    #[doc = "Select XIP_QMI's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_xip_qmitx(&self) -> bool {
        *self == TREQ_SEL_A::XIP_QMITX
    }
    #[doc = "Select XIP_QMI's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn is_xip_qmirx(&self) -> bool {
        *self == TREQ_SEL_A::XIP_QMIRX
    }
    #[doc = "Select HSTX as TREQ"]
    #[inline(always)]
    pub fn is_hstx(&self) -> bool {
        *self == TREQ_SEL_A::HSTX
    }
    #[doc = "Select CORESIGHT as TREQ"]
    #[inline(always)]
    pub fn is_coresight(&self) -> bool {
        *self == TREQ_SEL_A::CORESIGHT
    }
    #[doc = "Select SHA256 as TREQ"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == TREQ_SEL_A::SHA256
    }
    #[doc = "Select Timer 0 as TREQ"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == TREQ_SEL_A::TIMER0
    }
    #[doc = "Select Timer 1 as TREQ"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == TREQ_SEL_A::TIMER1
    }
    #[doc = "Select Timer 2 as TREQ (Optional)"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == TREQ_SEL_A::TIMER2
    }
    #[doc = "Select Timer 3 as TREQ (Optional)"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == TREQ_SEL_A::TIMER3
    }
    #[doc = "Permanent request, for unpaced transfers."]
    #[inline(always)]
    pub fn is_permanent(&self) -> bool {
        *self == TREQ_SEL_A::PERMANENT
    }
}
#[doc = "Field `TREQ_SEL` writer - Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
pub type TREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, TREQ_SEL_A>;
impl<'a, REG> TREQ_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PIO0's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio0_tx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_TX0)
    }
    #[doc = "Select PIO0's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio0_tx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_TX1)
    }
    #[doc = "Select PIO0's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio0_tx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_TX2)
    }
    #[doc = "Select PIO0's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio0_tx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_TX3)
    }
    #[doc = "Select PIO0's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio0_rx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_RX0)
    }
    #[doc = "Select PIO0's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio0_rx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_RX1)
    }
    #[doc = "Select PIO0's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio0_rx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_RX2)
    }
    #[doc = "Select PIO0's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio0_rx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO0_RX3)
    }
    #[doc = "Select PIO1's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio1_tx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_TX0)
    }
    #[doc = "Select PIO1's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio1_tx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_TX1)
    }
    #[doc = "Select PIO1's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio1_tx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_TX2)
    }
    #[doc = "Select PIO1's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio1_tx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_TX3)
    }
    #[doc = "Select PIO1's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio1_rx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_RX0)
    }
    #[doc = "Select PIO1's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio1_rx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_RX1)
    }
    #[doc = "Select PIO1's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio1_rx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_RX2)
    }
    #[doc = "Select PIO1's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio1_rx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO1_RX3)
    }
    #[doc = "Select PIO2's TX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio2_tx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_TX0)
    }
    #[doc = "Select PIO2's TX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio2_tx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_TX1)
    }
    #[doc = "Select PIO2's TX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio2_tx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_TX2)
    }
    #[doc = "Select PIO2's TX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio2_tx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_TX3)
    }
    #[doc = "Select PIO2's RX FIFO 0 as TREQ"]
    #[inline(always)]
    pub fn pio2_rx0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_RX0)
    }
    #[doc = "Select PIO2's RX FIFO 1 as TREQ"]
    #[inline(always)]
    pub fn pio2_rx1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_RX1)
    }
    #[doc = "Select PIO2's RX FIFO 2 as TREQ"]
    #[inline(always)]
    pub fn pio2_rx2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_RX2)
    }
    #[doc = "Select PIO2's RX FIFO 3 as TREQ"]
    #[inline(always)]
    pub fn pio2_rx3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PIO2_RX3)
    }
    #[doc = "Select SPI0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::SPI0_TX)
    }
    #[doc = "Select SPI0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::SPI0_RX)
    }
    #[doc = "Select SPI1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::SPI1_TX)
    }
    #[doc = "Select SPI1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::SPI1_RX)
    }
    #[doc = "Select UART0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::UART0_TX)
    }
    #[doc = "Select UART0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::UART0_RX)
    }
    #[doc = "Select UART1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::UART1_TX)
    }
    #[doc = "Select UART1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::UART1_RX)
    }
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP0)
    }
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP1)
    }
    #[doc = "Select PWM Counter 2's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP2)
    }
    #[doc = "Select PWM Counter 3's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP3)
    }
    #[doc = "Select PWM Counter 4's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap4(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP4)
    }
    #[doc = "Select PWM Counter 5's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap5(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP5)
    }
    #[doc = "Select PWM Counter 6's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap6(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP6)
    }
    #[doc = "Select PWM Counter 7's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap7(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP7)
    }
    #[doc = "Select PWM Counter 8's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap8(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP8)
    }
    #[doc = "Select PWM Counter 9's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap9(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP9)
    }
    #[doc = "Select PWM Counter 10's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap10(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP10)
    }
    #[doc = "Select PWM Counter 11's Wrap Value as TREQ"]
    #[inline(always)]
    pub fn pwm_wrap11(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PWM_WRAP11)
    }
    #[doc = "Select I2C0's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn i2c0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::I2C0_TX)
    }
    #[doc = "Select I2C0's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn i2c0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::I2C0_RX)
    }
    #[doc = "Select I2C1's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn i2c1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::I2C1_TX)
    }
    #[doc = "Select I2C1's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn i2c1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::I2C1_RX)
    }
    #[doc = "Select ADC as TREQ"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::ADC)
    }
    #[doc = "Select XIP_STREAM as TREQ"]
    #[inline(always)]
    pub fn xip_stream(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::XIP_STREAM)
    }
    #[doc = "Select XIP_QMI's TX FIFO as TREQ"]
    #[inline(always)]
    pub fn xip_qmitx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::XIP_QMITX)
    }
    #[doc = "Select XIP_QMI's RX FIFO as TREQ"]
    #[inline(always)]
    pub fn xip_qmirx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::XIP_QMIRX)
    }
    #[doc = "Select HSTX as TREQ"]
    #[inline(always)]
    pub fn hstx(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::HSTX)
    }
    #[doc = "Select CORESIGHT as TREQ"]
    #[inline(always)]
    pub fn coresight(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::CORESIGHT)
    }
    #[doc = "Select SHA256 as TREQ"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::SHA256)
    }
    #[doc = "Select Timer 0 as TREQ"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::TIMER0)
    }
    #[doc = "Select Timer 1 as TREQ"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::TIMER1)
    }
    #[doc = "Select Timer 2 as TREQ (Optional)"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::TIMER2)
    }
    #[doc = "Select Timer 3 as TREQ (Optional)"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::TIMER3)
    }
    #[doc = "Permanent request, for unpaced transfers."]
    #[inline(always)]
    pub fn permanent(self) -> &'a mut crate::W<REG> {
        self.variant(TREQ_SEL_A::PERMANENT)
    }
}
#[doc = "Field `IRQ_QUIET` reader - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
pub type IRQ_QUIET_R = crate::BitReader;
#[doc = "Field `IRQ_QUIET` writer - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
pub type IRQ_QUIET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSWAP` reader - Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
pub type BSWAP_R = crate::BitReader;
#[doc = "Field `BSWAP` writer - Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
pub type BSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNIFF_EN` reader - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
pub type SNIFF_EN_R = crate::BitReader;
#[doc = "Field `SNIFF_EN` writer - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
pub type SNIFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `WRITE_ERROR` reader - If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
pub type WRITE_ERROR_R = crate::BitReader;
#[doc = "Field `WRITE_ERROR` writer - If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
pub type WRITE_ERROR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `READ_ERROR` reader - If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
pub type READ_ERROR_R = crate::BitReader;
#[doc = "Field `READ_ERROR` writer - If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
pub type READ_ERROR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AHB_ERROR` reader - Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
pub type AHB_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    pub fn high_priority(&self) -> HIGH_PRIORITY_R {
        HIGH_PRIORITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    pub fn incr_read(&self) -> INCR_READ_R {
        INCR_READ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub fn incr_read_rev(&self) -> INCR_READ_REV_R {
        INCR_READ_REV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub fn incr_write(&self) -> INCR_WRITE_R {
        INCR_WRITE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub fn incr_write_rev(&self) -> INCR_WRITE_REV_R {
        INCR_WRITE_REV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 &lt;&lt; n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub fn ring_size(&self) -> RING_SIZE_R {
        RING_SIZE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 &lt;&lt; RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub fn ring_sel(&self) -> RING_SEL_R {
        RING_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.   
 Reset value is 0, which means for channels 1 and above the default will be to chain to channel 0 - set this field to avoid this behaviour."]
    #[inline(always)]
    pub fn chain_to(&self) -> CHAIN_TO_R {
        CHAIN_TO_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:22 - Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub fn treq_sel(&self) -> TREQ_SEL_R {
        TREQ_SEL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub fn irq_quiet(&self) -> IRQ_QUIET_R {
        IRQ_QUIET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub fn bswap(&self) -> BSWAP_R {
        BSWAP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub fn sniff_en(&self) -> SNIFF_EN_R {
        SNIFF_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    pub fn write_error(&self) -> WRITE_ERROR_R {
        WRITE_ERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    pub fn read_error(&self) -> READ_ERROR_R {
        READ_ERROR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    #[inline(always)]
    pub fn ahb_error(&self) -> AHB_ERROR_R {
        AHB_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CH_AL2_CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    #[must_use]
    pub fn high_priority(&mut self) -> HIGH_PRIORITY_W<CH_AL2_CTRL_SPEC> {
        HIGH_PRIORITY_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn data_size(&mut self) -> DATA_SIZE_W<CH_AL2_CTRL_SPEC> {
        DATA_SIZE_W::new(self, 2)
    }
    #[doc = "Bit 4 - If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    #[must_use]
    pub fn incr_read(&mut self) -> INCR_READ_W<CH_AL2_CTRL_SPEC> {
        INCR_READ_W::new(self, 4)
    }
    #[doc = "Bit 5 - If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    #[must_use]
    pub fn incr_read_rev(&mut self) -> INCR_READ_REV_W<CH_AL2_CTRL_SPEC> {
        INCR_READ_REV_W::new(self, 5)
    }
    #[doc = "Bit 6 - If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    #[must_use]
    pub fn incr_write(&mut self) -> INCR_WRITE_W<CH_AL2_CTRL_SPEC> {
        INCR_WRITE_W::new(self, 6)
    }
    #[doc = "Bit 7 - If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    #[must_use]
    pub fn incr_write_rev(&mut self) -> INCR_WRITE_REV_W<CH_AL2_CTRL_SPEC> {
        INCR_WRITE_REV_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 &lt;&lt; n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn ring_size(&mut self) -> RING_SIZE_W<CH_AL2_CTRL_SPEC> {
        RING_SIZE_W::new(self, 8)
    }
    #[doc = "Bit 12 - Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 &lt;&lt; RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    #[must_use]
    pub fn ring_sel(&mut self) -> RING_SEL_W<CH_AL2_CTRL_SPEC> {
        RING_SEL_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_.   
 Reset value is 0, which means for channels 1 and above the default will be to chain to channel 0 - set this field to avoid this behaviour."]
    #[inline(always)]
    #[must_use]
    pub fn chain_to(&mut self) -> CHAIN_TO_W<CH_AL2_CTRL_SPEC> {
        CHAIN_TO_W::new(self, 13)
    }
    #[doc = "Bits 17:22 - Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    #[must_use]
    pub fn treq_sel(&mut self) -> TREQ_SEL_W<CH_AL2_CTRL_SPEC> {
        TREQ_SEL_W::new(self, 17)
    }
    #[doc = "Bit 23 - In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    #[must_use]
    pub fn irq_quiet(&mut self) -> IRQ_QUIET_W<CH_AL2_CTRL_SPEC> {
        IRQ_QUIET_W::new(self, 23)
    }
    #[doc = "Bit 24 - Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    #[must_use]
    pub fn bswap(&mut self) -> BSWAP_W<CH_AL2_CTRL_SPEC> {
        BSWAP_W::new(self, 24)
    }
    #[doc = "Bit 25 - If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    #[must_use]
    pub fn sniff_en(&mut self) -> SNIFF_EN_W<CH_AL2_CTRL_SPEC> {
        SNIFF_EN_W::new(self, 25)
    }
    #[doc = "Bit 29 - If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    #[must_use]
    pub fn write_error(&mut self) -> WRITE_ERROR_W<CH_AL2_CTRL_SPEC> {
        WRITE_ERROR_W::new(self, 29)
    }
    #[doc = "Bit 30 - If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    #[must_use]
    pub fn read_error(&mut self) -> READ_ERROR_W<CH_AL2_CTRL_SPEC> {
        READ_ERROR_W::new(self, 30)
    }
}
#[doc = "DMA Channel 0 Control and Status  

You can [`read`](crate::Reg::read) this register and get [`ch_al2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_al2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_AL2_CTRL_SPEC;
impl crate::RegisterSpec for CH_AL2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_al2_ctrl::R`](R) reader structure"]
impl crate::Readable for CH_AL2_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_al2_ctrl::W`](W) writer structure"]
impl crate::Writable for CH_AL2_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x6000_0000;
}
#[doc = "`reset()` method sets CH_AL2_CTRL to value 0"]
impl crate::Resettable for CH_AL2_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
