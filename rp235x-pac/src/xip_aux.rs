#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stream: STREAM,
    qmi_direct_tx: QMI_DIRECT_TX,
    qmi_direct_rx: QMI_DIRECT_RX,
}
impl RegisterBlock {
    #[doc = "0x00 - Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)"]
    #[inline(always)]
    pub const fn stream(&self) -> &STREAM {
        &self.stream
    }
    #[doc = "0x04 - Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)"]
    #[inline(always)]
    pub const fn qmi_direct_tx(&self) -> &QMI_DIRECT_TX {
        &self.qmi_direct_tx
    }
    #[doc = "0x08 - Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)"]
    #[inline(always)]
    pub const fn qmi_direct_rx(&self) -> &QMI_DIRECT_RX {
        &self.qmi_direct_rx
    }
}
#[doc = "STREAM (rw) register accessor: Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)  

You can [`read`](crate::Reg::read) this register and get [`stream::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stream::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@stream`]
module"]
pub type STREAM = crate::Reg<stream::STREAM_SPEC>;
#[doc = "Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)"]
pub mod stream;
#[doc = "QMI_DIRECT_TX (rw) register accessor: Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)  

You can [`read`](crate::Reg::read) this register and get [`qmi_direct_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmi_direct_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@qmi_direct_tx`]
module"]
pub type QMI_DIRECT_TX = crate::Reg<qmi_direct_tx::QMI_DIRECT_TX_SPEC>;
#[doc = "Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)"]
pub mod qmi_direct_tx;
#[doc = "QMI_DIRECT_RX (rw) register accessor: Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)  

You can [`read`](crate::Reg::read) this register and get [`qmi_direct_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmi_direct_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@qmi_direct_rx`]
module"]
pub type QMI_DIRECT_RX = crate::Reg<qmi_direct_rx::QMI_DIRECT_RX_SPEC>;
#[doc = "Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)"]
pub mod qmi_direct_rx;
