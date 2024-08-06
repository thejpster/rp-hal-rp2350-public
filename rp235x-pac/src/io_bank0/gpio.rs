#[repr(C)]
#[doc = "Cluster GPIO%s, containing GPIO*_STATUS, GPIO*_CTRL"]
pub struct GPIO {
    gpio_status: GPIO_STATUS,
    gpio_ctrl: GPIO_CTRL,
}
impl GPIO {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn gpio_status(&self) -> &GPIO_STATUS {
        &self.gpio_status
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn gpio_ctrl(&self) -> &GPIO_CTRL {
        &self.gpio_ctrl
    }
}
#[doc = "GPIO_STATUS (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`gpio_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_status`]
module"]
pub type GPIO_STATUS = crate::Reg<gpio_status::GPIO_STATUS_SPEC>;
#[doc = ""]
pub mod gpio_status;
#[doc = "GPIO_CTRL (rw) register accessor:   

You can [`read`](crate::Reg::read) this register and get [`gpio_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@gpio_ctrl`]
module"]
pub type GPIO_CTRL = crate::Reg<gpio_ctrl::GPIO_CTRL_SPEC>;
#[doc = ""]
pub mod gpio_ctrl;
