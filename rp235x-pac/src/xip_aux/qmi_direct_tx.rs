#[doc = "Register `QMI_DIRECT_TX` reader"]
pub type R = crate::R<QMI_DIRECT_TX_SPEC>;
#[doc = "Register `QMI_DIRECT_TX` writer"]
pub type W = crate::W<QMI_DIRECT_TX_SPEC>;
#[doc = "Field `DATA` writer - Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO. For 16-bit data, the least-significant byte is transmitted first."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IWIDTH_A {
    #[doc = "0: Single width"]
    S = 0,
    #[doc = "1: Dual width"]
    D = 1,
    #[doc = "2: Quad width"]
    Q = 2,
}
impl From<IWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: IWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IWIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for IWIDTH_A {}
#[doc = "Field `IWIDTH` writer - Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely."]
pub type IWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IWIDTH_A>;
impl<'a, REG> IWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single width"]
    #[inline(always)]
    pub fn s(self) -> &'a mut crate::W<REG> {
        self.variant(IWIDTH_A::S)
    }
    #[doc = "Dual width"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(IWIDTH_A::D)
    }
    #[doc = "Quad width"]
    #[inline(always)]
    pub fn q(self) -> &'a mut crate::W<REG> {
        self.variant(IWIDTH_A::Q)
    }
}
#[doc = "Field `DWIDTH` writer - Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely."]
pub type DWIDTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` writer - Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input. For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer."]
pub type OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOPUSH` writer - Inhibit the RX FIFO push that would correspond to this TX FIFO entry. Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer."]
pub type NOPUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO. For 16-bit data, the least-significant byte is transmitted first."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<QMI_DIRECT_TX_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely."]
    #[inline(always)]
    #[must_use]
    pub fn iwidth(&mut self) -> IWIDTH_W<QMI_DIRECT_TX_SPEC> {
        IWIDTH_W::new(self, 16)
    }
    #[doc = "Bit 18 - Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely."]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<QMI_DIRECT_TX_SPEC> {
        DWIDTH_W::new(self, 18)
    }
    #[doc = "Bit 19 - Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input. For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<QMI_DIRECT_TX_SPEC> {
        OE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Inhibit the RX FIFO push that would correspond to this TX FIFO entry. Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer."]
    #[inline(always)]
    #[must_use]
    pub fn nopush(&mut self) -> NOPUSH_W<QMI_DIRECT_TX_SPEC> {
        NOPUSH_W::new(self, 20)
    }
}
#[doc = "Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)  

You can [`read`](crate::Reg::read) this register and get [`qmi_direct_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmi_direct_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QMI_DIRECT_TX_SPEC;
impl crate::RegisterSpec for QMI_DIRECT_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qmi_direct_tx::R`](R) reader structure"]
impl crate::Readable for QMI_DIRECT_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qmi_direct_tx::W`](W) writer structure"]
impl crate::Writable for QMI_DIRECT_TX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QMI_DIRECT_TX to value 0"]
impl crate::Resettable for QMI_DIRECT_TX_SPEC {
    const RESET_VALUE: u32 = 0;
}
