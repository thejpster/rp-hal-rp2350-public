#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl_status: CTRL_STATUS,
    trace_capture_fifo: TRACE_CAPTURE_FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    #[inline(always)]
    pub const fn ctrl_status(&self) -> &CTRL_STATUS {
        &self.ctrl_status
    }
    #[doc = "0x04 - FIFO for trace data captured from the TPIU"]
    #[inline(always)]
    pub const fn trace_capture_fifo(&self) -> &TRACE_CAPTURE_FIFO {
        &self.trace_capture_fifo
    }
}
#[doc = "CTRL_STATUS (rw) register accessor: Control and status register  

You can [`read`](crate::Reg::read) this register and get [`ctrl_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl_status`]
module"]
pub type CTRL_STATUS = crate::Reg<ctrl_status::CTRL_STATUS_SPEC>;
#[doc = "Control and status register"]
pub mod ctrl_status;
#[doc = "TRACE_CAPTURE_FIFO (rw) register accessor: FIFO for trace data captured from the TPIU  

You can [`read`](crate::Reg::read) this register and get [`trace_capture_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_capture_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trace_capture_fifo`]
module"]
pub type TRACE_CAPTURE_FIFO = crate::Reg<trace_capture_fifo::TRACE_CAPTURE_FIFO_SPEC>;
#[doc = "FIFO for trace data captured from the TPIU"]
pub mod trace_capture_fifo;
