#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tick: [TICK; 6],
}
impl RegisterBlock {
    #[doc = "0x00..0x48 - Cluster TICK%s, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn tick(&self, n: usize) -> &TICK {
        &self.tick[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x48 - Cluster TICK%s, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub fn tick_iter(&self) -> impl Iterator<Item = &TICK> {
        self.tick.iter()
    }
    #[doc = "0x00..0x0c - Cluster TICKPROC0, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn tickproc0(&self) -> &TICK {
        self.tick(0)
    }
    #[doc = "0x0c..0x18 - Cluster TICKPROC1, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn tickproc1(&self) -> &TICK {
        self.tick(1)
    }
    #[doc = "0x18..0x24 - Cluster TICKTIMER0, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn ticktimer0(&self) -> &TICK {
        self.tick(2)
    }
    #[doc = "0x24..0x30 - Cluster TICKTIMER1, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn ticktimer1(&self) -> &TICK {
        self.tick(3)
    }
    #[doc = "0x30..0x3c - Cluster TICKWATCHDOG, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn tickwatchdog(&self) -> &TICK {
        self.tick(4)
    }
    #[doc = "0x3c..0x48 - Cluster TICKRISCV, containing *_CTRL, *_CYCLES, *_COUNT"]
    #[inline(always)]
    pub const fn tickriscv(&self) -> &TICK {
        self.tick(5)
    }
}
#[doc = "Cluster TICK%s, containing *_CTRL, *_CYCLES, *_COUNT"]
pub use self::tick::TICK;
#[doc = r"Cluster"]
#[doc = "Cluster TICK%s, containing *_CTRL, *_CYCLES, *_COUNT"]
pub mod tick;
