#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl_status: CTRL_STATUS,
    trace_capture_fifo: TRACE_CAPTURE_FIFO,
}
impl RegisterBlock {
    #[doc = "0x00 - miscellaneous control/status bits"]
    #[inline(always)]
    pub const fn ctrl_status(&self) -> &CTRL_STATUS {
        &self.ctrl_status
    }
    #[doc = "0x04 - trace capture fifo"]
    #[inline(always)]
    pub const fn trace_capture_fifo(&self) -> &TRACE_CAPTURE_FIFO {
        &self.trace_capture_fifo
    }
}
#[doc = "CTRL_STATUS (rw) register accessor: miscellaneous control/status bits  

You can [`read`](crate::Reg::read) this register and get [`ctrl_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl_status`]
module"]
pub type CTRL_STATUS = crate::Reg<ctrl_status::CTRL_STATUS_SPEC>;
#[doc = "miscellaneous control/status bits"]
pub mod ctrl_status;
#[doc = "TRACE_CAPTURE_FIFO (r) register accessor: trace capture fifo  

You can [`read`](crate::Reg::read) this register and get [`trace_capture_fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trace_capture_fifo`]
module"]
pub type TRACE_CAPTURE_FIFO = crate::Reg<trace_capture_fifo::TRACE_CAPTURE_FIFO_SPEC>;
#[doc = "trace capture fifo"]
pub mod trace_capture_fifo;
