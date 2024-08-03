#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    write_once0: WRITE_ONCE0,
    write_once1: WRITE_ONCE1,
    bootlock_stat: BOOTLOCK_STAT,
    bootlock0: BOOTLOCK0,
    bootlock1: BOOTLOCK1,
    bootlock2: BOOTLOCK2,
    bootlock3: BOOTLOCK3,
    bootlock4: BOOTLOCK4,
    bootlock5: BOOTLOCK5,
    bootlock6: BOOTLOCK6,
    bootlock7: BOOTLOCK7,
}
impl RegisterBlock {
    #[doc = "0x800 - This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
    #[inline(always)]
    pub const fn write_once0(&self) -> &WRITE_ONCE0 {
        &self.write_once0
    }
    #[doc = "0x804 - This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
    #[inline(always)]
    pub const fn write_once1(&self) -> &WRITE_ONCE1 {
        &self.write_once1
    }
    #[doc = "0x808 - Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use."]
    #[inline(always)]
    pub const fn bootlock_stat(&self) -> &BOOTLOCK_STAT {
        &self.bootlock_stat
    }
    #[doc = "0x80c - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock0(&self) -> &BOOTLOCK0 {
        &self.bootlock0
    }
    #[doc = "0x810 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock1(&self) -> &BOOTLOCK1 {
        &self.bootlock1
    }
    #[doc = "0x814 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock2(&self) -> &BOOTLOCK2 {
        &self.bootlock2
    }
    #[doc = "0x818 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock3(&self) -> &BOOTLOCK3 {
        &self.bootlock3
    }
    #[doc = "0x81c - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock4(&self) -> &BOOTLOCK4 {
        &self.bootlock4
    }
    #[doc = "0x820 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock5(&self) -> &BOOTLOCK5 {
        &self.bootlock5
    }
    #[doc = "0x824 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock6(&self) -> &BOOTLOCK6 {
        &self.bootlock6
    }
    #[doc = "0x828 - Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock7(&self) -> &BOOTLOCK7 {
        &self.bootlock7
    }
}
#[doc = "WRITE_ONCE0 (rw) register accessor: This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset.  

You can [`read`](crate::Reg::read) this register and get [`write_once0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_once0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@write_once0`]
module"]
pub type WRITE_ONCE0 = crate::Reg<write_once0::WRITE_ONCE0_SPEC>;
#[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
pub mod write_once0;
#[doc = "WRITE_ONCE1 (rw) register accessor: This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset.  

You can [`read`](crate::Reg::read) this register and get [`write_once1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_once1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@write_once1`]
module"]
pub type WRITE_ONCE1 = crate::Reg<write_once1::WRITE_ONCE1_SPEC>;
#[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
pub mod write_once1;
#[doc = "BOOTLOCK_STAT (rw) register accessor: Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use.  

You can [`read`](crate::Reg::read) this register and get [`bootlock_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock_stat`]
module"]
pub type BOOTLOCK_STAT = crate::Reg<bootlock_stat::BOOTLOCK_STAT_SPEC>;
#[doc = "Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use."]
pub mod bootlock_stat;
#[doc = "BOOTLOCK0 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock0`]
module"]
pub type BOOTLOCK0 = crate::Reg<bootlock0::BOOTLOCK0_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock0;
#[doc = "BOOTLOCK1 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock1`]
module"]
pub type BOOTLOCK1 = crate::Reg<bootlock1::BOOTLOCK1_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock1;
#[doc = "BOOTLOCK2 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock2`]
module"]
pub type BOOTLOCK2 = crate::Reg<bootlock2::BOOTLOCK2_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock2;
#[doc = "BOOTLOCK3 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock3`]
module"]
pub type BOOTLOCK3 = crate::Reg<bootlock3::BOOTLOCK3_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock3;
#[doc = "BOOTLOCK4 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock4`]
module"]
pub type BOOTLOCK4 = crate::Reg<bootlock4::BOOTLOCK4_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock4;
#[doc = "BOOTLOCK5 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock5`]
module"]
pub type BOOTLOCK5 = crate::Reg<bootlock5::BOOTLOCK5_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock5;
#[doc = "BOOTLOCK6 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock6`]
module"]
pub type BOOTLOCK6 = crate::Reg<bootlock6::BOOTLOCK6_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock6;
#[doc = "BOOTLOCK7 (rw) register accessor: Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero.  

You can [`read`](crate::Reg::read) this register and get [`bootlock7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootlock7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@bootlock7`]
module"]
pub type BOOTLOCK7 = crate::Reg<bootlock7::BOOTLOCK7_SPEC>;
#[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 &lt;&lt; n, and on failed claim is zero."]
pub mod bootlock7;
