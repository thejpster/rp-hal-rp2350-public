#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    setup_packet_low: SETUP_PACKET_LOW,
    setup_packet_high: SETUP_PACKET_HIGH,
    ep1_in_control: EP1_IN_CONTROL,
    ep1_out_control: EP1_OUT_CONTROL,
    ep2_in_control: EP2_IN_CONTROL,
    ep2_out_control: EP2_OUT_CONTROL,
    ep3_in_control: EP3_IN_CONTROL,
    ep3_out_control: EP3_OUT_CONTROL,
    ep4_in_control: EP4_IN_CONTROL,
    ep4_out_control: EP4_OUT_CONTROL,
    ep5_in_control: EP5_IN_CONTROL,
    ep5_out_control: EP5_OUT_CONTROL,
    ep6_in_control: EP6_IN_CONTROL,
    ep6_out_control: EP6_OUT_CONTROL,
    ep7_in_control: EP7_IN_CONTROL,
    ep7_out_control: EP7_OUT_CONTROL,
    ep8_in_control: EP8_IN_CONTROL,
    ep8_out_control: EP8_OUT_CONTROL,
    ep9_in_control: EP9_IN_CONTROL,
    ep9_out_control: EP9_OUT_CONTROL,
    ep10_in_control: EP10_IN_CONTROL,
    ep10_out_control: EP10_OUT_CONTROL,
    ep11_in_control: EP11_IN_CONTROL,
    ep11_out_control: EP11_OUT_CONTROL,
    ep12_in_control: EP12_IN_CONTROL,
    ep12_out_control: EP12_OUT_CONTROL,
    ep13_in_control: EP13_IN_CONTROL,
    ep13_out_control: EP13_OUT_CONTROL,
    ep14_in_control: EP14_IN_CONTROL,
    ep14_out_control: EP14_OUT_CONTROL,
    ep15_in_control: EP15_IN_CONTROL,
    ep15_out_control: EP15_OUT_CONTROL,
    ep0_in_buffer_control: EP0_IN_BUFFER_CONTROL,
    ep0_out_buffer_control: EP0_OUT_BUFFER_CONTROL,
    ep1_in_buffer_control: EP1_IN_BUFFER_CONTROL,
    ep1_out_buffer_control: EP1_OUT_BUFFER_CONTROL,
    ep2_in_buffer_control: EP2_IN_BUFFER_CONTROL,
    ep2_out_buffer_control: EP2_OUT_BUFFER_CONTROL,
    ep3_in_buffer_control: EP3_IN_BUFFER_CONTROL,
    ep3_out_buffer_control: EP3_OUT_BUFFER_CONTROL,
    ep4_in_buffer_control: EP4_IN_BUFFER_CONTROL,
    ep4_out_buffer_control: EP4_OUT_BUFFER_CONTROL,
    ep5_in_buffer_control: EP5_IN_BUFFER_CONTROL,
    ep5_out_buffer_control: EP5_OUT_BUFFER_CONTROL,
    ep6_in_buffer_control: EP6_IN_BUFFER_CONTROL,
    ep6_out_buffer_control: EP6_OUT_BUFFER_CONTROL,
    ep7_in_buffer_control: EP7_IN_BUFFER_CONTROL,
    ep7_out_buffer_control: EP7_OUT_BUFFER_CONTROL,
    ep8_in_buffer_control: EP8_IN_BUFFER_CONTROL,
    ep8_out_buffer_control: EP8_OUT_BUFFER_CONTROL,
    ep9_in_buffer_control: EP9_IN_BUFFER_CONTROL,
    ep9_out_buffer_control: EP9_OUT_BUFFER_CONTROL,
    ep10_in_buffer_control: EP10_IN_BUFFER_CONTROL,
    ep10_out_buffer_control: EP10_OUT_BUFFER_CONTROL,
    ep11_in_buffer_control: EP11_IN_BUFFER_CONTROL,
    ep11_out_buffer_control: EP11_OUT_BUFFER_CONTROL,
    ep12_in_buffer_control: EP12_IN_BUFFER_CONTROL,
    ep12_out_buffer_control: EP12_OUT_BUFFER_CONTROL,
    ep13_in_buffer_control: EP13_IN_BUFFER_CONTROL,
    ep13_out_buffer_control: EP13_OUT_BUFFER_CONTROL,
    ep14_in_buffer_control: EP14_IN_BUFFER_CONTROL,
    ep14_out_buffer_control: EP14_OUT_BUFFER_CONTROL,
    ep15_in_buffer_control: EP15_IN_BUFFER_CONTROL,
    ep15_out_buffer_control: EP15_OUT_BUFFER_CONTROL,
}
impl RegisterBlock {
    #[doc = "0x00 - Bytes 0-3 of the SETUP packet from the host."]
    #[inline(always)]
    pub const fn setup_packet_low(&self) -> &SETUP_PACKET_LOW {
        &self.setup_packet_low
    }
    #[doc = "0x04 - Bytes 4-7 of the setup packet from the host."]
    #[inline(always)]
    pub const fn setup_packet_high(&self) -> &SETUP_PACKET_HIGH {
        &self.setup_packet_high
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ep1_in_control(&self) -> &EP1_IN_CONTROL {
        &self.ep1_in_control
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ep1_out_control(&self) -> &EP1_OUT_CONTROL {
        &self.ep1_out_control
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ep2_in_control(&self) -> &EP2_IN_CONTROL {
        &self.ep2_in_control
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn ep2_out_control(&self) -> &EP2_OUT_CONTROL {
        &self.ep2_out_control
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn ep3_in_control(&self) -> &EP3_IN_CONTROL {
        &self.ep3_in_control
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ep3_out_control(&self) -> &EP3_OUT_CONTROL {
        &self.ep3_out_control
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn ep4_in_control(&self) -> &EP4_IN_CONTROL {
        &self.ep4_in_control
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn ep4_out_control(&self) -> &EP4_OUT_CONTROL {
        &self.ep4_out_control
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn ep5_in_control(&self) -> &EP5_IN_CONTROL {
        &self.ep5_in_control
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn ep5_out_control(&self) -> &EP5_OUT_CONTROL {
        &self.ep5_out_control
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn ep6_in_control(&self) -> &EP6_IN_CONTROL {
        &self.ep6_in_control
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn ep6_out_control(&self) -> &EP6_OUT_CONTROL {
        &self.ep6_out_control
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn ep7_in_control(&self) -> &EP7_IN_CONTROL {
        &self.ep7_in_control
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn ep7_out_control(&self) -> &EP7_OUT_CONTROL {
        &self.ep7_out_control
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn ep8_in_control(&self) -> &EP8_IN_CONTROL {
        &self.ep8_in_control
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn ep8_out_control(&self) -> &EP8_OUT_CONTROL {
        &self.ep8_out_control
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn ep9_in_control(&self) -> &EP9_IN_CONTROL {
        &self.ep9_in_control
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn ep9_out_control(&self) -> &EP9_OUT_CONTROL {
        &self.ep9_out_control
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn ep10_in_control(&self) -> &EP10_IN_CONTROL {
        &self.ep10_in_control
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn ep10_out_control(&self) -> &EP10_OUT_CONTROL {
        &self.ep10_out_control
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn ep11_in_control(&self) -> &EP11_IN_CONTROL {
        &self.ep11_in_control
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn ep11_out_control(&self) -> &EP11_OUT_CONTROL {
        &self.ep11_out_control
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn ep12_in_control(&self) -> &EP12_IN_CONTROL {
        &self.ep12_in_control
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn ep12_out_control(&self) -> &EP12_OUT_CONTROL {
        &self.ep12_out_control
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn ep13_in_control(&self) -> &EP13_IN_CONTROL {
        &self.ep13_in_control
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn ep13_out_control(&self) -> &EP13_OUT_CONTROL {
        &self.ep13_out_control
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn ep14_in_control(&self) -> &EP14_IN_CONTROL {
        &self.ep14_in_control
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn ep14_out_control(&self) -> &EP14_OUT_CONTROL {
        &self.ep14_out_control
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn ep15_in_control(&self) -> &EP15_IN_CONTROL {
        &self.ep15_in_control
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn ep15_out_control(&self) -> &EP15_OUT_CONTROL {
        &self.ep15_out_control
    }
    #[doc = "0x80 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep0_in_buffer_control(&self) -> &EP0_IN_BUFFER_CONTROL {
        &self.ep0_in_buffer_control
    }
    #[doc = "0x84 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep0_out_buffer_control(&self) -> &EP0_OUT_BUFFER_CONTROL {
        &self.ep0_out_buffer_control
    }
    #[doc = "0x88 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep1_in_buffer_control(&self) -> &EP1_IN_BUFFER_CONTROL {
        &self.ep1_in_buffer_control
    }
    #[doc = "0x8c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep1_out_buffer_control(&self) -> &EP1_OUT_BUFFER_CONTROL {
        &self.ep1_out_buffer_control
    }
    #[doc = "0x90 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep2_in_buffer_control(&self) -> &EP2_IN_BUFFER_CONTROL {
        &self.ep2_in_buffer_control
    }
    #[doc = "0x94 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep2_out_buffer_control(&self) -> &EP2_OUT_BUFFER_CONTROL {
        &self.ep2_out_buffer_control
    }
    #[doc = "0x98 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep3_in_buffer_control(&self) -> &EP3_IN_BUFFER_CONTROL {
        &self.ep3_in_buffer_control
    }
    #[doc = "0x9c - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep3_out_buffer_control(&self) -> &EP3_OUT_BUFFER_CONTROL {
        &self.ep3_out_buffer_control
    }
    #[doc = "0xa0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep4_in_buffer_control(&self) -> &EP4_IN_BUFFER_CONTROL {
        &self.ep4_in_buffer_control
    }
    #[doc = "0xa4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep4_out_buffer_control(&self) -> &EP4_OUT_BUFFER_CONTROL {
        &self.ep4_out_buffer_control
    }
    #[doc = "0xa8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep5_in_buffer_control(&self) -> &EP5_IN_BUFFER_CONTROL {
        &self.ep5_in_buffer_control
    }
    #[doc = "0xac - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep5_out_buffer_control(&self) -> &EP5_OUT_BUFFER_CONTROL {
        &self.ep5_out_buffer_control
    }
    #[doc = "0xb0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep6_in_buffer_control(&self) -> &EP6_IN_BUFFER_CONTROL {
        &self.ep6_in_buffer_control
    }
    #[doc = "0xb4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep6_out_buffer_control(&self) -> &EP6_OUT_BUFFER_CONTROL {
        &self.ep6_out_buffer_control
    }
    #[doc = "0xb8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep7_in_buffer_control(&self) -> &EP7_IN_BUFFER_CONTROL {
        &self.ep7_in_buffer_control
    }
    #[doc = "0xbc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep7_out_buffer_control(&self) -> &EP7_OUT_BUFFER_CONTROL {
        &self.ep7_out_buffer_control
    }
    #[doc = "0xc0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep8_in_buffer_control(&self) -> &EP8_IN_BUFFER_CONTROL {
        &self.ep8_in_buffer_control
    }
    #[doc = "0xc4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep8_out_buffer_control(&self) -> &EP8_OUT_BUFFER_CONTROL {
        &self.ep8_out_buffer_control
    }
    #[doc = "0xc8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep9_in_buffer_control(&self) -> &EP9_IN_BUFFER_CONTROL {
        &self.ep9_in_buffer_control
    }
    #[doc = "0xcc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep9_out_buffer_control(&self) -> &EP9_OUT_BUFFER_CONTROL {
        &self.ep9_out_buffer_control
    }
    #[doc = "0xd0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep10_in_buffer_control(&self) -> &EP10_IN_BUFFER_CONTROL {
        &self.ep10_in_buffer_control
    }
    #[doc = "0xd4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep10_out_buffer_control(&self) -> &EP10_OUT_BUFFER_CONTROL {
        &self.ep10_out_buffer_control
    }
    #[doc = "0xd8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep11_in_buffer_control(&self) -> &EP11_IN_BUFFER_CONTROL {
        &self.ep11_in_buffer_control
    }
    #[doc = "0xdc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep11_out_buffer_control(&self) -> &EP11_OUT_BUFFER_CONTROL {
        &self.ep11_out_buffer_control
    }
    #[doc = "0xe0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep12_in_buffer_control(&self) -> &EP12_IN_BUFFER_CONTROL {
        &self.ep12_in_buffer_control
    }
    #[doc = "0xe4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep12_out_buffer_control(&self) -> &EP12_OUT_BUFFER_CONTROL {
        &self.ep12_out_buffer_control
    }
    #[doc = "0xe8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep13_in_buffer_control(&self) -> &EP13_IN_BUFFER_CONTROL {
        &self.ep13_in_buffer_control
    }
    #[doc = "0xec - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep13_out_buffer_control(&self) -> &EP13_OUT_BUFFER_CONTROL {
        &self.ep13_out_buffer_control
    }
    #[doc = "0xf0 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep14_in_buffer_control(&self) -> &EP14_IN_BUFFER_CONTROL {
        &self.ep14_in_buffer_control
    }
    #[doc = "0xf4 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep14_out_buffer_control(&self) -> &EP14_OUT_BUFFER_CONTROL {
        &self.ep14_out_buffer_control
    }
    #[doc = "0xf8 - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep15_in_buffer_control(&self) -> &EP15_IN_BUFFER_CONTROL {
        &self.ep15_in_buffer_control
    }
    #[doc = "0xfc - Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
    #[inline(always)]
    pub const fn ep15_out_buffer_control(&self) -> &EP15_OUT_BUFFER_CONTROL {
        &self.ep15_out_buffer_control
    }
}
#[doc = "SETUP_PACKET_LOW (rw) register accessor: Bytes 0-3 of the SETUP packet from the host.  

You can [`read`](crate::Reg::read) this register and get [`setup_packet_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup_packet_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_packet_low`]
module"]
pub type SETUP_PACKET_LOW = crate::Reg<setup_packet_low::SETUP_PACKET_LOW_SPEC>;
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
pub mod setup_packet_low;
#[doc = "SETUP_PACKET_HIGH (rw) register accessor: Bytes 4-7 of the setup packet from the host.  

You can [`read`](crate::Reg::read) this register and get [`setup_packet_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup_packet_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@setup_packet_high`]
module"]
pub type SETUP_PACKET_HIGH = crate::Reg<setup_packet_high::SETUP_PACKET_HIGH_SPEC>;
#[doc = "Bytes 4-7 of the setup packet from the host."]
pub mod setup_packet_high;
#[doc = "EP1_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep1_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep1_in_control`]
module"]
pub type EP1_IN_CONTROL = crate::Reg<ep1_in_control::EP1_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep1_in_control;
#[doc = "EP1_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep1_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep1_out_control`]
module"]
pub type EP1_OUT_CONTROL = crate::Reg<ep1_out_control::EP1_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep1_out_control;
#[doc = "EP2_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep2_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep2_in_control`]
module"]
pub type EP2_IN_CONTROL = crate::Reg<ep2_in_control::EP2_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep2_in_control;
#[doc = "EP2_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep2_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep2_out_control`]
module"]
pub type EP2_OUT_CONTROL = crate::Reg<ep2_out_control::EP2_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep2_out_control;
#[doc = "EP3_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep3_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep3_in_control`]
module"]
pub type EP3_IN_CONTROL = crate::Reg<ep3_in_control::EP3_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep3_in_control;
#[doc = "EP3_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep3_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep3_out_control`]
module"]
pub type EP3_OUT_CONTROL = crate::Reg<ep3_out_control::EP3_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep3_out_control;
#[doc = "EP4_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep4_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep4_in_control`]
module"]
pub type EP4_IN_CONTROL = crate::Reg<ep4_in_control::EP4_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep4_in_control;
#[doc = "EP4_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep4_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep4_out_control`]
module"]
pub type EP4_OUT_CONTROL = crate::Reg<ep4_out_control::EP4_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep4_out_control;
#[doc = "EP5_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep5_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep5_in_control`]
module"]
pub type EP5_IN_CONTROL = crate::Reg<ep5_in_control::EP5_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep5_in_control;
#[doc = "EP5_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep5_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep5_out_control`]
module"]
pub type EP5_OUT_CONTROL = crate::Reg<ep5_out_control::EP5_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep5_out_control;
#[doc = "EP6_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep6_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep6_in_control`]
module"]
pub type EP6_IN_CONTROL = crate::Reg<ep6_in_control::EP6_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep6_in_control;
#[doc = "EP6_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep6_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep6_out_control`]
module"]
pub type EP6_OUT_CONTROL = crate::Reg<ep6_out_control::EP6_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep6_out_control;
#[doc = "EP7_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep7_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep7_in_control`]
module"]
pub type EP7_IN_CONTROL = crate::Reg<ep7_in_control::EP7_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep7_in_control;
#[doc = "EP7_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep7_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep7_out_control`]
module"]
pub type EP7_OUT_CONTROL = crate::Reg<ep7_out_control::EP7_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep7_out_control;
#[doc = "EP8_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep8_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep8_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep8_in_control`]
module"]
pub type EP8_IN_CONTROL = crate::Reg<ep8_in_control::EP8_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep8_in_control;
#[doc = "EP8_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep8_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep8_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep8_out_control`]
module"]
pub type EP8_OUT_CONTROL = crate::Reg<ep8_out_control::EP8_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep8_out_control;
#[doc = "EP9_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep9_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep9_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep9_in_control`]
module"]
pub type EP9_IN_CONTROL = crate::Reg<ep9_in_control::EP9_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep9_in_control;
#[doc = "EP9_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep9_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep9_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep9_out_control`]
module"]
pub type EP9_OUT_CONTROL = crate::Reg<ep9_out_control::EP9_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep9_out_control;
#[doc = "EP10_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep10_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep10_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep10_in_control`]
module"]
pub type EP10_IN_CONTROL = crate::Reg<ep10_in_control::EP10_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep10_in_control;
#[doc = "EP10_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep10_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep10_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep10_out_control`]
module"]
pub type EP10_OUT_CONTROL = crate::Reg<ep10_out_control::EP10_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep10_out_control;
#[doc = "EP11_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep11_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep11_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep11_in_control`]
module"]
pub type EP11_IN_CONTROL = crate::Reg<ep11_in_control::EP11_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep11_in_control;
#[doc = "EP11_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep11_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep11_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep11_out_control`]
module"]
pub type EP11_OUT_CONTROL = crate::Reg<ep11_out_control::EP11_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep11_out_control;
#[doc = "EP12_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep12_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep12_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep12_in_control`]
module"]
pub type EP12_IN_CONTROL = crate::Reg<ep12_in_control::EP12_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep12_in_control;
#[doc = "EP12_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep12_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep12_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep12_out_control`]
module"]
pub type EP12_OUT_CONTROL = crate::Reg<ep12_out_control::EP12_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep12_out_control;
#[doc = "EP13_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep13_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep13_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep13_in_control`]
module"]
pub type EP13_IN_CONTROL = crate::Reg<ep13_in_control::EP13_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep13_in_control;
#[doc = "EP13_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep13_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep13_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep13_out_control`]
module"]
pub type EP13_OUT_CONTROL = crate::Reg<ep13_out_control::EP13_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep13_out_control;
#[doc = "EP14_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep14_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep14_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep14_in_control`]
module"]
pub type EP14_IN_CONTROL = crate::Reg<ep14_in_control::EP14_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep14_in_control;
#[doc = "EP14_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep14_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep14_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep14_out_control`]
module"]
pub type EP14_OUT_CONTROL = crate::Reg<ep14_out_control::EP14_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep14_out_control;
#[doc = "EP15_IN_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep15_in_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep15_in_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep15_in_control`]
module"]
pub type EP15_IN_CONTROL = crate::Reg<ep15_in_control::EP15_IN_CONTROL_SPEC>;
#[doc = ""]
pub mod ep15_in_control;
#[doc = "EP15_OUT_CONTROL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`ep15_out_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep15_out_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep15_out_control`]
module"]
pub type EP15_OUT_CONTROL = crate::Reg<ep15_out_control::EP15_OUT_CONTROL_SPEC>;
#[doc = ""]
pub mod ep15_out_control;
#[doc = "EP0_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep0_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep0_in_buffer_control`]
module"]
pub type EP0_IN_BUFFER_CONTROL = crate::Reg<ep0_in_buffer_control::EP0_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_in_buffer_control;
#[doc = "EP0_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep0_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep0_out_buffer_control`]
module"]
pub type EP0_OUT_BUFFER_CONTROL = crate::Reg<ep0_out_buffer_control::EP0_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep0_out_buffer_control;
#[doc = "EP1_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep1_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep1_in_buffer_control`]
module"]
pub type EP1_IN_BUFFER_CONTROL = crate::Reg<ep1_in_buffer_control::EP1_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_in_buffer_control;
#[doc = "EP1_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep1_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep1_out_buffer_control`]
module"]
pub type EP1_OUT_BUFFER_CONTROL = crate::Reg<ep1_out_buffer_control::EP1_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep1_out_buffer_control;
#[doc = "EP2_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep2_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep2_in_buffer_control`]
module"]
pub type EP2_IN_BUFFER_CONTROL = crate::Reg<ep2_in_buffer_control::EP2_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_in_buffer_control;
#[doc = "EP2_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep2_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep2_out_buffer_control`]
module"]
pub type EP2_OUT_BUFFER_CONTROL = crate::Reg<ep2_out_buffer_control::EP2_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep2_out_buffer_control;
#[doc = "EP3_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep3_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep3_in_buffer_control`]
module"]
pub type EP3_IN_BUFFER_CONTROL = crate::Reg<ep3_in_buffer_control::EP3_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_in_buffer_control;
#[doc = "EP3_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep3_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep3_out_buffer_control`]
module"]
pub type EP3_OUT_BUFFER_CONTROL = crate::Reg<ep3_out_buffer_control::EP3_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep3_out_buffer_control;
#[doc = "EP4_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep4_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep4_in_buffer_control`]
module"]
pub type EP4_IN_BUFFER_CONTROL = crate::Reg<ep4_in_buffer_control::EP4_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_in_buffer_control;
#[doc = "EP4_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep4_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep4_out_buffer_control`]
module"]
pub type EP4_OUT_BUFFER_CONTROL = crate::Reg<ep4_out_buffer_control::EP4_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep4_out_buffer_control;
#[doc = "EP5_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep5_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep5_in_buffer_control`]
module"]
pub type EP5_IN_BUFFER_CONTROL = crate::Reg<ep5_in_buffer_control::EP5_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_in_buffer_control;
#[doc = "EP5_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep5_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep5_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep5_out_buffer_control`]
module"]
pub type EP5_OUT_BUFFER_CONTROL = crate::Reg<ep5_out_buffer_control::EP5_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep5_out_buffer_control;
#[doc = "EP6_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep6_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep6_in_buffer_control`]
module"]
pub type EP6_IN_BUFFER_CONTROL = crate::Reg<ep6_in_buffer_control::EP6_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_in_buffer_control;
#[doc = "EP6_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep6_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep6_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep6_out_buffer_control`]
module"]
pub type EP6_OUT_BUFFER_CONTROL = crate::Reg<ep6_out_buffer_control::EP6_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep6_out_buffer_control;
#[doc = "EP7_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep7_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep7_in_buffer_control`]
module"]
pub type EP7_IN_BUFFER_CONTROL = crate::Reg<ep7_in_buffer_control::EP7_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_in_buffer_control;
#[doc = "EP7_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep7_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep7_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep7_out_buffer_control`]
module"]
pub type EP7_OUT_BUFFER_CONTROL = crate::Reg<ep7_out_buffer_control::EP7_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep7_out_buffer_control;
#[doc = "EP8_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep8_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep8_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep8_in_buffer_control`]
module"]
pub type EP8_IN_BUFFER_CONTROL = crate::Reg<ep8_in_buffer_control::EP8_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_in_buffer_control;
#[doc = "EP8_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep8_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep8_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep8_out_buffer_control`]
module"]
pub type EP8_OUT_BUFFER_CONTROL = crate::Reg<ep8_out_buffer_control::EP8_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep8_out_buffer_control;
#[doc = "EP9_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep9_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep9_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep9_in_buffer_control`]
module"]
pub type EP9_IN_BUFFER_CONTROL = crate::Reg<ep9_in_buffer_control::EP9_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_in_buffer_control;
#[doc = "EP9_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep9_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep9_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep9_out_buffer_control`]
module"]
pub type EP9_OUT_BUFFER_CONTROL = crate::Reg<ep9_out_buffer_control::EP9_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep9_out_buffer_control;
#[doc = "EP10_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep10_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep10_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep10_in_buffer_control`]
module"]
pub type EP10_IN_BUFFER_CONTROL = crate::Reg<ep10_in_buffer_control::EP10_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_in_buffer_control;
#[doc = "EP10_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep10_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep10_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep10_out_buffer_control`]
module"]
pub type EP10_OUT_BUFFER_CONTROL =
    crate::Reg<ep10_out_buffer_control::EP10_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep10_out_buffer_control;
#[doc = "EP11_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep11_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep11_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep11_in_buffer_control`]
module"]
pub type EP11_IN_BUFFER_CONTROL = crate::Reg<ep11_in_buffer_control::EP11_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_in_buffer_control;
#[doc = "EP11_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep11_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep11_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep11_out_buffer_control`]
module"]
pub type EP11_OUT_BUFFER_CONTROL =
    crate::Reg<ep11_out_buffer_control::EP11_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep11_out_buffer_control;
#[doc = "EP12_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep12_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep12_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep12_in_buffer_control`]
module"]
pub type EP12_IN_BUFFER_CONTROL = crate::Reg<ep12_in_buffer_control::EP12_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_in_buffer_control;
#[doc = "EP12_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep12_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep12_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep12_out_buffer_control`]
module"]
pub type EP12_OUT_BUFFER_CONTROL =
    crate::Reg<ep12_out_buffer_control::EP12_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep12_out_buffer_control;
#[doc = "EP13_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep13_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep13_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep13_in_buffer_control`]
module"]
pub type EP13_IN_BUFFER_CONTROL = crate::Reg<ep13_in_buffer_control::EP13_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_in_buffer_control;
#[doc = "EP13_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep13_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep13_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep13_out_buffer_control`]
module"]
pub type EP13_OUT_BUFFER_CONTROL =
    crate::Reg<ep13_out_buffer_control::EP13_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep13_out_buffer_control;
#[doc = "EP14_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep14_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep14_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep14_in_buffer_control`]
module"]
pub type EP14_IN_BUFFER_CONTROL = crate::Reg<ep14_in_buffer_control::EP14_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_in_buffer_control;
#[doc = "EP14_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep14_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep14_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep14_out_buffer_control`]
module"]
pub type EP14_OUT_BUFFER_CONTROL =
    crate::Reg<ep14_out_buffer_control::EP14_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep14_out_buffer_control;
#[doc = "EP15_IN_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep15_in_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep15_in_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep15_in_buffer_control`]
module"]
pub type EP15_IN_BUFFER_CONTROL = crate::Reg<ep15_in_buffer_control::EP15_IN_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_in_buffer_control;
#[doc = "EP15_OUT_BUFFER_CONTROL (rw) register accessor: Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode.  

You can [`read`](crate::Reg::read) this register and get [`ep15_out_buffer_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep15_out_buffer_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ep15_out_buffer_control`]
module"]
pub type EP15_OUT_BUFFER_CONTROL =
    crate::Reg<ep15_out_buffer_control::EP15_OUT_BUFFER_CONTROL_SPEC>;
#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
pub mod ep15_out_buffer_control;
