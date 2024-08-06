#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    arm: ARM,
    disarm: DISARM,
    sensitivity: SENSITIVITY,
    lock: LOCK,
    trig_status: TRIG_STATUS,
    trig_force: TRIG_FORCE,
}
impl RegisterBlock {
    #[doc = "0x00 - Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn arm(&self) -> &ARM {
        &self.arm
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn disarm(&self) -> &DISARM {
        &self.disarm
    }
    #[doc = "0x08 - Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn sensitivity(&self) -> &SENSITIVITY {
        &self.sensitivity
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x10 - Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only."]
    #[inline(always)]
    pub const fn trig_status(&self) -> &TRIG_STATUS {
        &self.trig_status
    }
    #[doc = "0x14 - Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn trig_force(&self) -> &TRIG_FORCE {
        &self.trig_force
    }
}
#[doc = "ARM (rw) register accessor: Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`arm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@arm`]
module"]
pub type ARM = crate::Reg<arm::ARM_SPEC>;
#[doc = "Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only."]
pub mod arm;
#[doc = "DISARM (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`disarm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disarm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@disarm`]
module"]
pub type DISARM = crate::Reg<disarm::DISARM_SPEC>;
#[doc = ""]
pub mod disarm;
#[doc = "SENSITIVITY (rw) register accessor: Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`sensitivity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensitivity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sensitivity`]
module"]
pub type SENSITIVITY = crate::Reg<sensitivity::SENSITIVITY_SPEC>;
#[doc = "Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only."]
pub mod sensitivity;
#[doc = "LOCK (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = ""]
pub mod lock;
#[doc = "TRIG_STATUS (rw) register accessor: Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`trig_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trig_status`]
module"]
pub type TRIG_STATUS = crate::Reg<trig_status::TRIG_STATUS_SPEC>;
#[doc = "Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only."]
pub mod trig_status;
#[doc = "TRIG_FORCE (rw) register accessor: Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only.  

You can [`read`](crate::Reg::read) this register and get [`trig_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@trig_force`]
module"]
pub type TRIG_FORCE = crate::Reg<trig_force::TRIG_FORCE_SPEC>;
#[doc = "Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only."]
pub mod trig_force;
