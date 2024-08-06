#[repr(C)]
#[doc = "Cluster TICK%s, containing *_CTRL, *_CYCLES, *_COUNT"]
pub struct TICK {
    ctrl: CTRL,
    cycles: CYCLES,
    count: COUNT,
}
impl TICK {
    #[doc = "0x00 - Controls the tick generator"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cycles(&self) -> &CYCLES {
        &self.cycles
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn count(&self) -> &COUNT {
        &self.count
    }
}
#[doc = "CTRL (rw) register accessor: Controls the tick generator  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Controls the tick generator"]
pub mod ctrl;
#[doc = "CYCLES (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`cycles::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycles::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@cycles`]
module"]
pub type CYCLES = crate::Reg<cycles::CYCLES_SPEC>;
#[doc = ""]
pub mod cycles;
#[doc = "COUNT (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@count`]
module"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = ""]
pub mod count;
