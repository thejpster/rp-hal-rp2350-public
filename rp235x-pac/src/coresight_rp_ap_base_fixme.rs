#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    dbgkey: DBGKEY,
    dbg_pow_state_swcore: DBG_POW_STATE_SWCORE,
    dbg_pow_state_xip: DBG_POW_STATE_XIP,
    dbg_pow_state_sram0: DBG_POW_STATE_SRAM0,
    dbg_pow_state_sram1: DBG_POW_STATE_SRAM1,
    dbg_pow_ovrd: DBG_POW_OVRD,
    dbg_pow_output_to_gpio: DBG_POW_OUTPUT_TO_GPIO,
    _reserved8: [u8; 0x0ddc],
    idr: IDR,
}
impl RegisterBlock {
    #[doc = "0x00 - This register is primarily used for DFT but can also be used to overcome some power up problems. However, it should not be used to force power up of domains. Use DBG_POW_OVRD for that."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Serial key load interface (write-only)"]
    #[inline(always)]
    pub const fn dbgkey(&self) -> &DBGKEY {
        &self.dbgkey
    }
    #[doc = "0x08 - This register indicates the state of the power sequencer for the switched-core domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  
 Bits 9-11 describe the states of the power manager clocks which change as clock generators in the switched-core become available following switched-core power up.  
 This bus can be sent to GPIO for debug. See DBG_POW_OUTPUT_TO_GPIO in the DBG_POW_OVRD register."]
    #[inline(always)]
    pub const fn dbg_pow_state_swcore(&self) -> &DBG_POW_STATE_SWCORE {
        &self.dbg_pow_state_swcore
    }
    #[doc = "0x0c - This register indicates the state of the power sequencer for the XIP domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
    #[inline(always)]
    pub const fn dbg_pow_state_xip(&self) -> &DBG_POW_STATE_XIP {
        &self.dbg_pow_state_xip
    }
    #[doc = "0x10 - This register indicates the state of the power sequencer for the SRAM0 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
    #[inline(always)]
    pub const fn dbg_pow_state_sram0(&self) -> &DBG_POW_STATE_SRAM0 {
        &self.dbg_pow_state_sram0
    }
    #[doc = "0x14 - This register indicates the state of the power sequencer for the SRAM1 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
    #[inline(always)]
    pub const fn dbg_pow_state_sram1(&self) -> &DBG_POW_STATE_SRAM1 {
        &self.dbg_pow_state_sram1
    }
    #[doc = "0x18 - This register allows external control of the power sequencer outputs for all the switched power domains. If any of the power sequencers stall at any stage then force power up operation of all domains by running this sequence:  
 - set DBG_POW_OVRD = 0x3b to force small power switches on, large power switches off, resets on and isolation on  
 - allow time for the domain power supplies to reach full rail  
 - set DBG_POW_OVRD = 0x3b to force large power switches on  
 - set DBG_POW_OVRD = 0x37 to remove isolation  
 - set DBG_POW_OVRD = 0x17 to remove resets"]
    #[inline(always)]
    pub const fn dbg_pow_ovrd(&self) -> &DBG_POW_OVRD {
        &self.dbg_pow_ovrd
    }
    #[doc = "0x1c - Send some, or all, bits of DBG_POW_STATE_SWCORE to gpios.  
 Bit 0 sends bit 0 of DBG_POW_STATE_SWCORE to GPIO 34  
 Bit 1 sends bit 1 of DBG_POW_STATE_SWCORE to GPIO 35  
 Bit 2 sends bit 2 of DBG_POW_STATE_SWCORE to GPIO 36  
 .  
 .  
 Bit 11 sends bit 11 of DBG_POW_STATE_SWCORE to GPIO 45"]
    #[inline(always)]
    pub const fn dbg_pow_output_to_gpio(&self) -> &DBG_POW_OUTPUT_TO_GPIO {
        &self.dbg_pow_output_to_gpio
    }
    #[doc = "0xdfc - Standard Coresight ID Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
}
#[doc = "CTRL (rw) register accessor: This register is primarily used for DFT but can also be used to overcome some power up problems. However, it should not be used to force power up of domains. Use DBG_POW_OVRD for that.  

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "This register is primarily used for DFT but can also be used to overcome some power up problems. However, it should not be used to force power up of domains. Use DBG_POW_OVRD for that."]
pub mod ctrl;
#[doc = "DBGKEY (rw) register accessor: Serial key load interface (write-only)  

You can [`read`](crate::Reg::read) this register and get [`dbgkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbgkey`]
module"]
pub type DBGKEY = crate::Reg<dbgkey::DBGKEY_SPEC>;
#[doc = "Serial key load interface (write-only)"]
pub mod dbgkey;
#[doc = "DBG_POW_STATE_SWCORE (r) register accessor: This register indicates the state of the power sequencer for the switched-core domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  
 Bits 9-11 describe the states of the power manager clocks which change as clock generators in the switched-core become available following switched-core power up.  
 This bus can be sent to GPIO for debug. See DBG_POW_OUTPUT_TO_GPIO in the DBG_POW_OVRD register.  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_state_swcore::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_state_swcore`]
module"]
pub type DBG_POW_STATE_SWCORE = crate::Reg<dbg_pow_state_swcore::DBG_POW_STATE_SWCORE_SPEC>;
#[doc = "This register indicates the state of the power sequencer for the switched-core domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  
 Bits 9-11 describe the states of the power manager clocks which change as clock generators in the switched-core become available following switched-core power up.  
 This bus can be sent to GPIO for debug. See DBG_POW_OUTPUT_TO_GPIO in the DBG_POW_OVRD register."]
pub mod dbg_pow_state_swcore;
#[doc = "DBG_POW_STATE_XIP (r) register accessor: This register indicates the state of the power sequencer for the XIP domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_state_xip::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_state_xip`]
module"]
pub type DBG_POW_STATE_XIP = crate::Reg<dbg_pow_state_xip::DBG_POW_STATE_XIP_SPEC>;
#[doc = "This register indicates the state of the power sequencer for the XIP domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
pub mod dbg_pow_state_xip;
#[doc = "DBG_POW_STATE_SRAM0 (r) register accessor: This register indicates the state of the power sequencer for the SRAM0 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_state_sram0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_state_sram0`]
module"]
pub type DBG_POW_STATE_SRAM0 = crate::Reg<dbg_pow_state_sram0::DBG_POW_STATE_SRAM0_SPEC>;
#[doc = "This register indicates the state of the power sequencer for the SRAM0 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
pub mod dbg_pow_state_sram0;
#[doc = "DBG_POW_STATE_SRAM1 (r) register accessor: This register indicates the state of the power sequencer for the SRAM1 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete.  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_state_sram1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_state_sram1`]
module"]
pub type DBG_POW_STATE_SRAM1 = crate::Reg<dbg_pow_state_sram1::DBG_POW_STATE_SRAM1_SPEC>;
#[doc = "This register indicates the state of the power sequencer for the SRAM1 domain.  
 The sequencer timing is managed by the POWMAN_SEQ_* registers. See the header file for those registers for more information on the timing.  
 Power up of the domain commences by clearing bit 0 (IS_PD) then bits 1-8 are set in sequence. Bit 8 (IS_PU) indicates the sequence is complete.  
 Power down of the domain commences by clearing bit 8 (IS_PU) then bits 7-1 are cleared in sequence. Bit 0 (IS_PU) is then set to indicate the sequence is complete."]
pub mod dbg_pow_state_sram1;
#[doc = "DBG_POW_OVRD (rw) register accessor: This register allows external control of the power sequencer outputs for all the switched power domains. If any of the power sequencers stall at any stage then force power up operation of all domains by running this sequence:  
 - set DBG_POW_OVRD = 0x3b to force small power switches on, large power switches off, resets on and isolation on  
 - allow time for the domain power supplies to reach full rail  
 - set DBG_POW_OVRD = 0x3b to force large power switches on  
 - set DBG_POW_OVRD = 0x37 to remove isolation  
 - set DBG_POW_OVRD = 0x17 to remove resets  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_ovrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pow_ovrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_ovrd`]
module"]
pub type DBG_POW_OVRD = crate::Reg<dbg_pow_ovrd::DBG_POW_OVRD_SPEC>;
#[doc = "This register allows external control of the power sequencer outputs for all the switched power domains. If any of the power sequencers stall at any stage then force power up operation of all domains by running this sequence:  
 - set DBG_POW_OVRD = 0x3b to force small power switches on, large power switches off, resets on and isolation on  
 - allow time for the domain power supplies to reach full rail  
 - set DBG_POW_OVRD = 0x3b to force large power switches on  
 - set DBG_POW_OVRD = 0x37 to remove isolation  
 - set DBG_POW_OVRD = 0x17 to remove resets"]
pub mod dbg_pow_ovrd;
#[doc = "DBG_POW_OUTPUT_TO_GPIO (rw) register accessor: Send some, or all, bits of DBG_POW_STATE_SWCORE to gpios.  
 Bit 0 sends bit 0 of DBG_POW_STATE_SWCORE to GPIO 34  
 Bit 1 sends bit 1 of DBG_POW_STATE_SWCORE to GPIO 35  
 Bit 2 sends bit 2 of DBG_POW_STATE_SWCORE to GPIO 36  
 .  
 .  
 Bit 11 sends bit 11 of DBG_POW_STATE_SWCORE to GPIO 45  

You can [`read`](crate::Reg::read) this register and get [`dbg_pow_output_to_gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_pow_output_to_gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dbg_pow_output_to_gpio`]
module"]
pub type DBG_POW_OUTPUT_TO_GPIO = crate::Reg<dbg_pow_output_to_gpio::DBG_POW_OUTPUT_TO_GPIO_SPEC>;
#[doc = "Send some, or all, bits of DBG_POW_STATE_SWCORE to gpios.  
 Bit 0 sends bit 0 of DBG_POW_STATE_SWCORE to GPIO 34  
 Bit 1 sends bit 1 of DBG_POW_STATE_SWCORE to GPIO 35  
 Bit 2 sends bit 2 of DBG_POW_STATE_SWCORE to GPIO 36  
 .  
 .  
 Bit 11 sends bit 11 of DBG_POW_STATE_SWCORE to GPIO 45"]
pub mod dbg_pow_output_to_gpio;
#[doc = "IDR (r) register accessor: Standard Coresight ID Register  

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Standard Coresight ID Register"]
pub mod idr;
