#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    data0: DATA0,
    _reserved1: [u8; 0x2c],
    dmcontrol: DMCONTROL,
    dmstatus: DMSTATUS,
    hartinfo: HARTINFO,
    haltsum1: HALTSUM1,
    hawindowsel: HAWINDOWSEL,
    hawindow: HAWINDOW,
    abstracts: ABSTRACTS,
    command: COMMAND,
    abstractauto: ABSTRACTAUTO,
    _reserved10: [u8; 0x10],
    nextdm: NEXTDM,
    _reserved11: [u8; 0x08],
    progbuf0: PROGBUF0,
    progbuf1: PROGBUF1,
    _reserved13: [u8; 0x58],
    sbcs: SBCS,
    sbaddress0: SBADDRESS0,
    _reserved15: [u8; 0x08],
    sbdata0: SBDATA0,
    _reserved16: [u8; 0x0c],
    haltsum0: HALTSUM0,
}
impl RegisterBlock {
    #[doc = "0x10 - data0 through data11 are basic read/write registers that may be read or changed by abstract commands. abstractcs.datacount indicates how many of them are implemented, starting at data0, counting up.  

 Accessing these registers while an abstract command is executing causes abstractcs.cmderr to be set to 1 (busy) if it is 0.  

 Attempts to write them while abstractcs.busy is set does not change their value.  

 The values in these registers may not be preserved after an abstract command is executed. The only guarantees on their contents are the ones offered by the command in question. If the command fails, no assumptions can be made about the contents of these registers.  

 (Note: Hazard3 implements data0 only.)"]
    #[inline(always)]
    pub const fn data0(&self) -> &DATA0 {
        &self.data0
    }
    #[doc = "0x40 - This register controls the overall Debug Module as well as the currently selected harts, as defined in hasel."]
    #[inline(always)]
    pub const fn dmcontrol(&self) -> &DMCONTROL {
        &self.dmcontrol
    }
    #[doc = "0x44 - This register reports status for the overall Debug Module as well as the currently selected harts, as defined in hasel. Its address will not change in the future, because it contains version.  

 This entire register is read-only."]
    #[inline(always)]
    pub const fn dmstatus(&self) -> &DMSTATUS {
        &self.dmstatus
    }
    #[doc = "0x48 - This register gives information about the hart currently selected by hartsel.  

 This entire register is read-only."]
    #[inline(always)]
    pub const fn hartinfo(&self) -> &HARTINFO {
        &self.hartinfo
    }
    #[doc = "0x4c - Each bit in this read-only register indicates whether any of a group of harts is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 Each bit in haltsum1 is an OR reduction of 32 bits' worth of haltsum0. On RP2350, only the LSB is implemented."]
    #[inline(always)]
    pub const fn haltsum1(&self) -> &HALTSUM1 {
        &self.haltsum1
    }
    #[doc = "0x50 - This register selects which of the 32-bit portion of the hart array mask register is accessible in hawindow."]
    #[inline(always)]
    pub const fn hawindowsel(&self) -> &HAWINDOWSEL {
        &self.hawindowsel
    }
    #[doc = "0x54 - This register provides R/W access to a 32-bit portion of the hart array mask register. The position of the window is determined by hawindowsel. I.e. bit 0 refers to hart hawindowsel ∗ 32, while bit 31 refers to hart hawindowsel ∗ 32 + 31.  

 On RP2350 only the two least-significant bits of this register are implemented, since there are only two cores. This is still useful to run/halt/reset both cores exactly simultaneously."]
    #[inline(always)]
    pub const fn hawindow(&self) -> &HAWINDOW {
        &self.hawindow
    }
    #[doc = "0x58 - Abstract Control and Status. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0."]
    #[inline(always)]
    pub const fn abstracts(&self) -> &ABSTRACTS {
        &self.abstracts
    }
    #[doc = "0x5c - Writes to this register cause the corresponding abstract command to be executed.  

 Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

 If cmderr is non-zero, writes to this register are ignored."]
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    #[doc = "0x60 - Abstract Command Autoexec. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0."]
    #[inline(always)]
    pub const fn abstractauto(&self) -> &ABSTRACTAUTO {
        &self.abstractauto
    }
    #[doc = "0x74 - If there is more than one DM accessible on this DMI, this register contains the base address of thenext one in the chain, or 0 if this is the last one in the chain."]
    #[inline(always)]
    pub const fn nextdm(&self) -> &NEXTDM {
        &self.nextdm
    }
    #[doc = "0x80 - progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)"]
    #[inline(always)]
    pub const fn progbuf0(&self) -> &PROGBUF0 {
        &self.progbuf0
    }
    #[doc = "0x84 - progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)"]
    #[inline(always)]
    pub const fn progbuf1(&self) -> &PROGBUF1 {
        &self.progbuf1
    }
    #[doc = "0xe0 - System Bus Access Control and Status"]
    #[inline(always)]
    pub const fn sbcs(&self) -> &SBCS {
        &self.sbcs
    }
    #[doc = "0xe4 - System Bus Address 31:0  

 When the system bus master is busy, writes to this register will set sbbusyerror and don’t do anything else.  

 If sberror is 0, sbbusyerror is 0, and sbreadonaddr is set then writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus read from the new value of sbaddress.  

 3. If the read succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy."]
    #[inline(always)]
    pub const fn sbaddress0(&self) -> &SBADDRESS0 {
        &self.sbaddress0
    }
    #[doc = "0xf0 - System Bus Data 31:0  

 Any successful system bus read updates sbdata. If the width of the read access is less than the width of sbdata, the contents of the remaining high bits may take on any value.  

 If sberror or sbbusyerror both aren’t 0 then accesses do nothing.  

 If the bus master is busy then accesses set sbbusyerror, and don’t do anything else. Writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus write of the new value of sbdata to sbaddress.  

 3. If the write succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

 Reads from this register start the following:  

 1. “Return” the data.  

 2. Set sbbusy.  

 3. If sbreadondata is set, perform a system bus read from the address contained in sbaddress, placing the result in sbdata.  

 4. If the read was successful, and sbautoincrement is set, increment sbaddress.  

 5. Clear sbbusy."]
    #[inline(always)]
    pub const fn sbdata0(&self) -> &SBDATA0 {
        &self.sbdata0
    }
    #[doc = "0x100 - Each bit in this read-only register indicates whether one specific hart is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 On RP2350, only the two LSBs of this register are implemented, one for each core/hart.  

 This entire register is read-only."]
    #[inline(always)]
    pub const fn haltsum0(&self) -> &HALTSUM0 {
        &self.haltsum0
    }
}
#[doc = "DATA0 (rw) register accessor: data0 through data11 are basic read/write registers that may be read or changed by abstract commands. abstractcs.datacount indicates how many of them are implemented, starting at data0, counting up.  

 Accessing these registers while an abstract command is executing causes abstractcs.cmderr to be set to 1 (busy) if it is 0.  

 Attempts to write them while abstractcs.busy is set does not change their value.  

 The values in these registers may not be preserved after an abstract command is executed. The only guarantees on their contents are the ones offered by the command in question. If the command fails, no assumptions can be made about the contents of these registers.  

 (Note: Hazard3 implements data0 only.)  

You can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@data0`]
module"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "data0 through data11 are basic read/write registers that may be read or changed by abstract commands. abstractcs.datacount indicates how many of them are implemented, starting at data0, counting up.  

 Accessing these registers while an abstract command is executing causes abstractcs.cmderr to be set to 1 (busy) if it is 0.  

 Attempts to write them while abstractcs.busy is set does not change their value.  

 The values in these registers may not be preserved after an abstract command is executed. The only guarantees on their contents are the ones offered by the command in question. If the command fails, no assumptions can be made about the contents of these registers.  

 (Note: Hazard3 implements data0 only.)"]
pub mod data0;
#[doc = "DMCONTROL (rw) register accessor: This register controls the overall Debug Module as well as the currently selected harts, as defined in hasel.  

You can [`read`](crate::Reg::read) this register and get [`dmcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmcontrol`]
module"]
pub type DMCONTROL = crate::Reg<dmcontrol::DMCONTROL_SPEC>;
#[doc = "This register controls the overall Debug Module as well as the currently selected harts, as defined in hasel."]
pub mod dmcontrol;
#[doc = "DMSTATUS (r) register accessor: This register reports status for the overall Debug Module as well as the currently selected harts, as defined in hasel. Its address will not change in the future, because it contains version.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`dmstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@dmstatus`]
module"]
pub type DMSTATUS = crate::Reg<dmstatus::DMSTATUS_SPEC>;
#[doc = "This register reports status for the overall Debug Module as well as the currently selected harts, as defined in hasel. Its address will not change in the future, because it contains version.  

 This entire register is read-only."]
pub mod dmstatus;
#[doc = "HARTINFO (rw) register accessor: This register gives information about the hart currently selected by hartsel.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`hartinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hartinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@hartinfo`]
module"]
pub type HARTINFO = crate::Reg<hartinfo::HARTINFO_SPEC>;
#[doc = "This register gives information about the hart currently selected by hartsel.  

 This entire register is read-only."]
pub mod hartinfo;
#[doc = "HALTSUM1 (r) register accessor: Each bit in this read-only register indicates whether any of a group of harts is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 Each bit in haltsum1 is an OR reduction of 32 bits' worth of haltsum0. On RP2350, only the LSB is implemented.  

You can [`read`](crate::Reg::read) this register and get [`haltsum1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@haltsum1`]
module"]
pub type HALTSUM1 = crate::Reg<haltsum1::HALTSUM1_SPEC>;
#[doc = "Each bit in this read-only register indicates whether any of a group of harts is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 Each bit in haltsum1 is an OR reduction of 32 bits' worth of haltsum0. On RP2350, only the LSB is implemented."]
pub mod haltsum1;
#[doc = "HAWINDOWSEL (rw) register accessor: This register selects which of the 32-bit portion of the hart array mask register is accessible in hawindow.  

You can [`read`](crate::Reg::read) this register and get [`hawindowsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindowsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@hawindowsel`]
module"]
pub type HAWINDOWSEL = crate::Reg<hawindowsel::HAWINDOWSEL_SPEC>;
#[doc = "This register selects which of the 32-bit portion of the hart array mask register is accessible in hawindow."]
pub mod hawindowsel;
#[doc = "HAWINDOW (rw) register accessor: This register provides R/W access to a 32-bit portion of the hart array mask register. The position of the window is determined by hawindowsel. I.e. bit 0 refers to hart hawindowsel ∗ 32, while bit 31 refers to hart hawindowsel ∗ 32 + 31.  

 On RP2350 only the two least-significant bits of this register are implemented, since there are only two cores. This is still useful to run/halt/reset both cores exactly simultaneously.  

You can [`read`](crate::Reg::read) this register and get [`hawindow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hawindow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@hawindow`]
module"]
pub type HAWINDOW = crate::Reg<hawindow::HAWINDOW_SPEC>;
#[doc = "This register provides R/W access to a 32-bit portion of the hart array mask register. The position of the window is determined by hawindowsel. I.e. bit 0 refers to hart hawindowsel ∗ 32, while bit 31 refers to hart hawindowsel ∗ 32 + 31.  

 On RP2350 only the two least-significant bits of this register are implemented, since there are only two cores. This is still useful to run/halt/reset both cores exactly simultaneously."]
pub mod hawindow;
#[doc = "ABSTRACTS (rw) register accessor: Abstract Control and Status. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

You can [`read`](crate::Reg::read) this register and get [`abstracts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstracts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@abstracts`]
module"]
pub type ABSTRACTS = crate::Reg<abstracts::ABSTRACTS_SPEC>;
#[doc = "Abstract Control and Status. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0."]
pub mod abstracts;
#[doc = "COMMAND (w) register accessor: Writes to this register cause the corresponding abstract command to be executed.  

 Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

 If cmderr is non-zero, writes to this register are ignored.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@command`]
module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Writes to this register cause the corresponding abstract command to be executed.  

 Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

 If cmderr is non-zero, writes to this register are ignored."]
pub mod command;
#[doc = "ABSTRACTAUTO (rw) register accessor: Abstract Command Autoexec. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0.  

You can [`read`](crate::Reg::read) this register and get [`abstractauto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractauto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@abstractauto`]
module"]
pub type ABSTRACTAUTO = crate::Reg<abstractauto::ABSTRACTAUTO_SPEC>;
#[doc = "Abstract Command Autoexec. Writing this register while an abstract command is executing causes cmderr to be set to 1 (busy) if it is 0."]
pub mod abstractauto;
#[doc = "NEXTDM (r) register accessor: If there is more than one DM accessible on this DMI, this register contains the base address of thenext one in the chain, or 0 if this is the last one in the chain.  

You can [`read`](crate::Reg::read) this register and get [`nextdm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@nextdm`]
module"]
pub type NEXTDM = crate::Reg<nextdm::NEXTDM_SPEC>;
#[doc = "If there is more than one DM accessible on this DMI, this register contains the base address of thenext one in the chain, or 0 if this is the last one in the chain."]
pub mod nextdm;
#[doc = "PROGBUF0 (rw) register accessor: progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)  

You can [`read`](crate::Reg::read) this register and get [`progbuf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@progbuf0`]
module"]
pub type PROGBUF0 = crate::Reg<progbuf0::PROGBUF0_SPEC>;
#[doc = "progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)"]
pub mod progbuf0;
#[doc = "PROGBUF1 (rw) register accessor: progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)  

You can [`read`](crate::Reg::read) this register and get [`progbuf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`progbuf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@progbuf1`]
module"]
pub type PROGBUF1 = crate::Reg<progbuf1::PROGBUF1_SPEC>;
#[doc = "progbuf0 through progbuf15 provide read/write access to the program buffer. abstractcs.progbufsize indicates how many of them are implemented starting at progbuf0, counting up.  

 (Hazard3 implements a 2-word program buffer.)"]
pub mod progbuf1;
#[doc = "SBCS (rw) register accessor: System Bus Access Control and Status  

You can [`read`](crate::Reg::read) this register and get [`sbcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbcs`]
module"]
pub type SBCS = crate::Reg<sbcs::SBCS_SPEC>;
#[doc = "System Bus Access Control and Status"]
pub mod sbcs;
#[doc = "SBADDRESS0 (rw) register accessor: System Bus Address 31:0  

 When the system bus master is busy, writes to this register will set sbbusyerror and don’t do anything else.  

 If sberror is 0, sbbusyerror is 0, and sbreadonaddr is set then writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus read from the new value of sbaddress.  

 3. If the read succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

You can [`read`](crate::Reg::read) this register and get [`sbaddress0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbaddress0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbaddress0`]
module"]
pub type SBADDRESS0 = crate::Reg<sbaddress0::SBADDRESS0_SPEC>;
#[doc = "System Bus Address 31:0  

 When the system bus master is busy, writes to this register will set sbbusyerror and don’t do anything else.  

 If sberror is 0, sbbusyerror is 0, and sbreadonaddr is set then writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus read from the new value of sbaddress.  

 3. If the read succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy."]
pub mod sbaddress0;
#[doc = "SBDATA0 (rw) register accessor: System Bus Data 31:0  

 Any successful system bus read updates sbdata. If the width of the read access is less than the width of sbdata, the contents of the remaining high bits may take on any value.  

 If sberror or sbbusyerror both aren’t 0 then accesses do nothing.  

 If the bus master is busy then accesses set sbbusyerror, and don’t do anything else. Writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus write of the new value of sbdata to sbaddress.  

 3. If the write succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

 Reads from this register start the following:  

 1. “Return” the data.  

 2. Set sbbusy.  

 3. If sbreadondata is set, perform a system bus read from the address contained in sbaddress, placing the result in sbdata.  

 4. If the read was successful, and sbautoincrement is set, increment sbaddress.  

 5. Clear sbbusy.  

You can [`read`](crate::Reg::read) this register and get [`sbdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@sbdata0`]
module"]
pub type SBDATA0 = crate::Reg<sbdata0::SBDATA0_SPEC>;
#[doc = "System Bus Data 31:0  

 Any successful system bus read updates sbdata. If the width of the read access is less than the width of sbdata, the contents of the remaining high bits may take on any value.  

 If sberror or sbbusyerror both aren’t 0 then accesses do nothing.  

 If the bus master is busy then accesses set sbbusyerror, and don’t do anything else. Writes to this register start the following:  

 1. Set sbbusy.  

 2. Perform a bus write of the new value of sbdata to sbaddress.  

 3. If the write succeeded and sbautoincrement is set, increment sbaddress.  

 4. Clear sbbusy.  

 Reads from this register start the following:  

 1. “Return” the data.  

 2. Set sbbusy.  

 3. If sbreadondata is set, perform a system bus read from the address contained in sbaddress, placing the result in sbdata.  

 4. If the read was successful, and sbautoincrement is set, increment sbaddress.  

 5. Clear sbbusy."]
pub mod sbdata0;
#[doc = "HALTSUM0 (r) register accessor: Each bit in this read-only register indicates whether one specific hart is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 On RP2350, only the two LSBs of this register are implemented, one for each core/hart.  

 This entire register is read-only.  

You can [`read`](crate::Reg::read) this register and get [`haltsum0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@haltsum0`]
module"]
pub type HALTSUM0 = crate::Reg<haltsum0::HALTSUM0_SPEC>;
#[doc = "Each bit in this read-only register indicates whether one specific hart is halted or not. Unavailable/nonexistent harts are not considered to be halted.  

 On RP2350, only the two LSBs of this register are implemented, one for each core/hart.  

 This entire register is read-only."]
pub mod haltsum0;
