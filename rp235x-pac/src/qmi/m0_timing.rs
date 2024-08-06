#[doc = "Register `M0_TIMING` reader"]
pub type R = crate::R<M0_TIMING_SPEC>;
#[doc = "Register `M0_TIMING` writer"]
pub type W = crate::W<M0_TIMING_SPEC>;
#[doc = "Field `CLKDIV` reader - Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXDELAY` reader - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
pub type RXDELAY_R = crate::FieldReader;
#[doc = "Field `RXDELAY` writer - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
pub type RXDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MIN_DESELECT` reader - After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
pub type MIN_DESELECT_R = crate::FieldReader;
#[doc = "Field `MIN_DESELECT` writer - After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
pub type MIN_DESELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAX_SELECT` reader - Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
pub type MAX_SELECT_R = crate::FieldReader;
#[doc = "Field `MAX_SELECT` writer - Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
pub type MAX_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SELECT_HOLD` reader - Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
pub type SELECT_HOLD_R = crate::FieldReader;
#[doc = "Field `SELECT_HOLD` writer - Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
pub type SELECT_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SELECT_SETUP` reader - Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
pub type SELECT_SETUP_R = crate::BitReader;
#[doc = "Field `SELECT_SETUP` writer - Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
pub type SELECT_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAGEBREAK_A {
    #[doc = "0: No page boundary is enforced"]
    NONE = 0,
    #[doc = "1: Break bursts crossing a 256-byte page boundary"]
    _256 = 1,
    #[doc = "2: Break bursts crossing a 1024-byte quad-page boundary"]
    _1024 = 2,
    #[doc = "3: Break bursts crossing a 4096-byte sector boundary"]
    _4096 = 3,
}
impl From<PAGEBREAK_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGEBREAK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAGEBREAK_A {
    type Ux = u8;
}
impl crate::IsEnum for PAGEBREAK_A {}
#[doc = "Field `PAGEBREAK` reader - When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
pub type PAGEBREAK_R = crate::FieldReader<PAGEBREAK_A>;
impl PAGEBREAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PAGEBREAK_A {
        match self.bits {
            0 => PAGEBREAK_A::NONE,
            1 => PAGEBREAK_A::_256,
            2 => PAGEBREAK_A::_1024,
            3 => PAGEBREAK_A::_4096,
            _ => unreachable!(),
        }
    }
    #[doc = "No page boundary is enforced"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PAGEBREAK_A::NONE
    }
    #[doc = "Break bursts crossing a 256-byte page boundary"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PAGEBREAK_A::_256
    }
    #[doc = "Break bursts crossing a 1024-byte quad-page boundary"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PAGEBREAK_A::_1024
    }
    #[doc = "Break bursts crossing a 4096-byte sector boundary"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == PAGEBREAK_A::_4096
    }
}
#[doc = "Field `PAGEBREAK` writer - When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
pub type PAGEBREAK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PAGEBREAK_A, crate::Safe>;
impl<'a, REG> PAGEBREAK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No page boundary is enforced"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PAGEBREAK_A::NONE)
    }
    #[doc = "Break bursts crossing a 256-byte page boundary"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(PAGEBREAK_A::_256)
    }
    #[doc = "Break bursts crossing a 1024-byte quad-page boundary"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(PAGEBREAK_A::_1024)
    }
    #[doc = "Break bursts crossing a 4096-byte sector boundary"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(PAGEBREAK_A::_4096)
    }
}
#[doc = "Field `COOLDOWN` reader - Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
pub type COOLDOWN_R = crate::FieldReader;
#[doc = "Field `COOLDOWN` writer - Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
pub type COOLDOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:16 - After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
    #[inline(always)]
    pub fn min_deselect(&self) -> MIN_DESELECT_R {
        MIN_DESELECT_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:22 - Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
    #[inline(always)]
    pub fn max_select(&self) -> MAX_SELECT_R {
        MAX_SELECT_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:24 - Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
    #[inline(always)]
    pub fn select_hold(&self) -> SELECT_HOLD_R {
        SELECT_HOLD_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
    #[inline(always)]
    pub fn select_setup(&self) -> SELECT_SETUP_R {
        SELECT_SETUP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
    #[inline(always)]
    pub fn pagebreak(&self) -> PAGEBREAK_R {
        PAGEBREAK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
    #[inline(always)]
    pub fn cooldown(&self) -> COOLDOWN_R {
        COOLDOWN_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<M0_TIMING_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
    #[inline(always)]
    #[must_use]
    pub fn rxdelay(&mut self) -> RXDELAY_W<M0_TIMING_SPEC> {
        RXDELAY_W::new(self, 8)
    }
    #[doc = "Bits 12:16 - After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
    #[inline(always)]
    #[must_use]
    pub fn min_deselect(&mut self) -> MIN_DESELECT_W<M0_TIMING_SPEC> {
        MIN_DESELECT_W::new(self, 12)
    }
    #[doc = "Bits 17:22 - Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
    #[inline(always)]
    #[must_use]
    pub fn max_select(&mut self) -> MAX_SELECT_W<M0_TIMING_SPEC> {
        MAX_SELECT_W::new(self, 17)
    }
    #[doc = "Bits 23:24 - Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
    #[inline(always)]
    #[must_use]
    pub fn select_hold(&mut self) -> SELECT_HOLD_W<M0_TIMING_SPEC> {
        SELECT_HOLD_W::new(self, 23)
    }
    #[doc = "Bit 25 - Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
    #[inline(always)]
    #[must_use]
    pub fn select_setup(&mut self) -> SELECT_SETUP_W<M0_TIMING_SPEC> {
        SELECT_SETUP_W::new(self, 25)
    }
    #[doc = "Bits 28:29 - When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pagebreak(&mut self) -> PAGEBREAK_W<M0_TIMING_SPEC> {
        PAGEBREAK_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
    #[inline(always)]
    #[must_use]
    pub fn cooldown(&mut self) -> COOLDOWN_W<M0_TIMING_SPEC> {
        COOLDOWN_W::new(self, 30)
    }
}
#[doc = "Timing configuration register for memory address window 0.  

You can [`read`](crate::Reg::read) this register and get [`m0_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M0_TIMING_SPEC;
impl crate::RegisterSpec for M0_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m0_timing::R`](R) reader structure"]
impl crate::Readable for M0_TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m0_timing::W`](W) writer structure"]
impl crate::Writable for M0_TIMING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M0_TIMING to value 0x4000_0004"]
impl crate::Resettable for M0_TIMING_SPEC {
    const RESET_VALUE: u32 = 0x4000_0004;
}
