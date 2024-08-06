#[doc = "Register `DIRECT_CSR` reader"]
pub type R = crate::R<DIRECT_CSR_SPEC>;
#[doc = "Register `DIRECT_CSR` writer"]
pub type W = crate::W<DIRECT_CSR_SPEC>;
#[doc = "Field `EN` reader - Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted. The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select. (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `ASSERT_CS0N` reader - When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
pub type ASSERT_CS0N_R = crate::BitReader;
#[doc = "Field `ASSERT_CS0N` writer - When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
pub type ASSERT_CS0N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASSERT_CS1N` reader - When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
pub type ASSERT_CS1N_R = crate::BitReader;
#[doc = "Field `ASSERT_CS1N` writer - When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
pub type ASSERT_CS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CS0N` reader - When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
pub type AUTO_CS0N_R = crate::BitReader;
#[doc = "Field `AUTO_CS0N` writer - When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
pub type AUTO_CS0N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CS1N` reader - When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
pub type AUTO_CS1N_R = crate::BitReader;
#[doc = "Field `AUTO_CS1N` writer - When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
pub type AUTO_CS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULL` reader - When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored."]
pub type TXFULL_R = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes."]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `TXLEVEL` reader - Current level of DIRECT_TX FIFO"]
pub type TXLEVEL_R = crate::FieldReader;
#[doc = "Field `RXEMPTY` reader - When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined."]
pub type RXEMPTY_R = crate::BitReader;
#[doc = "Field `RXFULL` reader - When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full."]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `RXLEVEL` reader - Current level of DIRECT_RX FIFO"]
pub type RXLEVEL_R = crate::FieldReader;
#[doc = "Field `CLKDIV` reader - Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXDELAY` reader - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
pub type RXDELAY_R = crate::FieldReader;
#[doc = "Field `RXDELAY` writer - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
pub type RXDELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted. The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select. (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub fn assert_cs0n(&self) -> ASSERT_CS0N_R {
        ASSERT_CS0N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub fn assert_cs1n(&self) -> ASSERT_CS1N_R {
        ASSERT_CS1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub fn auto_cs0n(&self) -> AUTO_CS0N_R {
        AUTO_CS0N_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub fn auto_cs1n(&self) -> AUTO_CS1N_R {
        AUTO_CS1N_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored."]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Current level of DIRECT_TX FIFO"]
    #[inline(always)]
    pub fn txlevel(&self) -> TXLEVEL_R {
        TXLEVEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined."]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Current level of DIRECT_RX FIFO"]
    #[inline(always)]
    pub fn rxlevel(&self) -> RXLEVEL_R {
        RXLEVEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 22:29 - Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DIRECT_CSR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn assert_cs0n(&mut self) -> ASSERT_CS0N_W<DIRECT_CSR_SPEC> {
        ASSERT_CS0N_W::new(self, 2)
    }
    #[doc = "Bit 3 - When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn assert_cs1n(&mut self) -> ASSERT_CS1N_W<DIRECT_CSR_SPEC> {
        ASSERT_CS1N_W::new(self, 3)
    }
    #[doc = "Bit 6 - When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cs0n(&mut self) -> AUTO_CS0N_W<DIRECT_CSR_SPEC> {
        AUTO_CS0N_W::new(self, 6)
    }
    #[doc = "Bit 7 - When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cs1n(&mut self) -> AUTO_CS1N_W<DIRECT_CSR_SPEC> {
        AUTO_CS1N_W::new(self, 7)
    }
    #[doc = "Bits 22:29 - Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<DIRECT_CSR_SPEC> {
        CLKDIV_W::new(self, 22)
    }
    #[doc = "Bits 30:31 - Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
    #[inline(always)]
    #[must_use]
    pub fn rxdelay(&mut self) -> RXDELAY_W<DIRECT_CSR_SPEC> {
        RXDELAY_W::new(self, 30)
    }
}
#[doc = "Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported.  

You can [`read`](crate::Reg::read) this register and get [`direct_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRECT_CSR_SPEC;
impl crate::RegisterSpec for DIRECT_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direct_csr::R`](R) reader structure"]
impl crate::Readable for DIRECT_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`direct_csr::W`](W) writer structure"]
impl crate::Writable for DIRECT_CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRECT_CSR to value 0x0180_0000"]
impl crate::Resettable for DIRECT_CSR_SPEC {
    const RESET_VALUE: u32 = 0x0180_0000;
}
