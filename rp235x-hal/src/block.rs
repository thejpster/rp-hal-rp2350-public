//! Support for the RP235x Boot ROM's "Block" structures
//!
//! Blocks contain pointers, to form Block Loops.
//!
//! The `IMAGE_DEF` Block (here the `ImageDef` type) tells the ROM how to boot a
//! firmware image. The `PARTITION_TABLE` Block (here the `PartitionTable` type)
//! tells the ROM how to divide the flash space up into partitions.

/// The smallest block we can have - it has one item in it.
pub type ImageDef = Block<1>;

/// A Block as understood by the Boot ROM.
///
/// This could be an Image Definition, or a Partition Table, or maybe some other
/// kind of block.
///
/// It contains within the special start and end markers the Boot ROM is looking
/// for.
#[derive(Debug)]
#[repr(C)]
pub struct Block<const N: usize> {
    marker_start: u32,
    items: [u32; N],
    length: u32,
    offset: u32,
    marker_end: u32,
}

#[allow(missing_docs)]
impl<const N: usize> Block<N> {
    // These all have a 1 byte size

    pub const ITEM_1BS_VECTOR_TABLE: u8 = 0x03;
    pub const ITEM_1BS_ROLLING_WINDOW_DELTA: u8 = 0x05;
    pub const ITEM_1BS_SIGNATURE: u8 = 0x09;
    pub const ITEM_1BS_SALT: u8 = 0x0c;
    pub const ITEM_1BS_NEXT_BLOCK_OFFSET: u8 = 0x41;
    pub const ITEM_1BS_IMAGE_TYPE: u8 = 0x42;
    pub const ITEM_1BS_ENTRY_POINT: u8 = 0x44;
    pub const ITEM_2BS_HASH_DEF: u8 = 0x47;
    pub const ITEM_1BS_VERSION: u8 = 0x48;
    pub const ITEM_1BS_HASH_VALUE: u8 = 0x4b;

    // These all have a 2-byte size

    pub const ITEM_2BS_LOAD_MAP: u8 = 0x06;
    pub const ITEM_2BS_PARTITION_TABLE: u8 = 0x0a;
    pub const ITEM_2BS_IGNORED: u8 = 0xfe;
    pub const ITEM_2BS_LAST: u8 = 0xff;

    // Options for ITEM_1BS_IMAGE_TYPE

    pub const IMAGE_TYPE_INVALID: u16 = 0x0000;
    pub const IMAGE_TYPE_EXE: u16 = 0x0001;
    pub const IMAGE_TYPE_DATA: u16 = 0x0002;
    pub const IMAGE_TYPE_EXE_TYPE_SECURITY_UNSPECIFIED: u16 = 0x0000;
    pub const IMAGE_TYPE_EXE_TYPE_SECURITY_NS: u16 = 0x0010;
    pub const IMAGE_TYPE_EXE_TYPE_SECURITY_S: u16 = 0x0020;
    pub const IMAGE_TYPE_EXE_CPU_ARM: u16 = 0x0000;
    pub const IMAGE_TYPE_EXE_CPU_RISCV: u16 = 0x0100;
    pub const IMAGE_TYPE_EXE_CHIP_RP2040: u16 = 0x0000;
    pub const IMAGE_TYPE_EXE_CHIP_RP2350: u16 = 0x1000;
    pub const IMAGE_TYPE_TBYB: u16 = 0x8000;

    /// This is the magic ROM value the Pico-SDK calls
    /// `PICOBIN_BLOCK_MARKER_START`
    const BLOCK_MARKER_START: u32 = 0xffffded3;

    /// This is the magic ROM value the Pico-SDK calls
    /// `PICOBIN_BLOCK_MARKER_END`
    const BLOCK_MARKER_END: u32 = 0xab123579;

    /// Construct a new Binary Block, with the given items.
    ///
    /// The length, and the Start and End markers are added automatically. The
    /// Block Loop pointer initially points to itself.
    pub const fn new(items: [u32; N]) -> Block<N> {
        Block {
            marker_start: Self::BLOCK_MARKER_START,
            items,
            length: Self::make_length(N as u16),
            // offset from this block to next block in loop. By default
            // we form a Block Loop with a single Block in it.
            offset: 0x0000_0000,
            marker_end: Self::BLOCK_MARKER_END,
        }
    }

    /// Change the Block Loop offset pointer.
    pub const fn with_offset(self, offset: u32) -> Block<N> {
        Block { offset, ..self }
    }

    /// Make a word containing a 1 byte length.
    ///
    /// The `command` arg should contain `1BS`
    pub const fn make_1bs_item(value: u16, length: u8, command: u8) -> u32 {
        ((value as u32) << 16) | ((length as u32) << 8) | (command as u32)
    }

    /// Make a word containing a 2 byte length.
    ///
    /// The `command` arg should contain `2BS`
    pub const fn make_2bs_item(value: u8, length: u16, command: u8) -> u32 {
        ((value as u32) << 24) | ((length as u32) << 8) | (command as u32)
    }

    /// Create Image Type item, of type INVALID.
    pub const fn make_image_type_invalid() -> u32 {
        let value = Self::IMAGE_TYPE_INVALID;
        Self::make_1bs_item(value, 1, Self::ITEM_1BS_IMAGE_TYPE)
    }

    /// Create Image Type item, of type DATA.
    pub const fn make_image_type_data() -> u32 {
        let value = Self::IMAGE_TYPE_DATA;
        Self::make_1bs_item(value, 1, Self::ITEM_1BS_IMAGE_TYPE)
    }

    /// Create Image Type item, of type EXE.
    pub const fn make_image_type_exe(security: Security, arch: Architecture) -> u32 {
        let mut value = Self::IMAGE_TYPE_EXE | Self::IMAGE_TYPE_EXE_CHIP_RP2350;

        match arch {
            Architecture::Arm => {
                value |= Self::IMAGE_TYPE_EXE_CPU_ARM;
            }
            Architecture::Riscv => {
                value |= Self::IMAGE_TYPE_EXE_CPU_RISCV;
            }
        }

        match security {
            Security::Unspecified => value |= Self::IMAGE_TYPE_EXE_TYPE_SECURITY_UNSPECIFIED,
            Security::NonSecure => value |= Self::IMAGE_TYPE_EXE_TYPE_SECURITY_NS,
            Security::Secure => value |= Self::IMAGE_TYPE_EXE_TYPE_SECURITY_S,
        }

        Self::make_1bs_item(value, 1, Self::ITEM_1BS_IMAGE_TYPE)
    }

    /// Create a Block Length item.
    pub const fn make_length(length: u16) -> u32 {
        Self::make_2bs_item(0, length, Self::ITEM_2BS_LAST)
    }

    /// Create a Vector Table item.
    ///
    /// This is only allowed on Arm systems.
    pub const fn make_vector_table(table_ptr: u32) -> [u32; 2] {
        [
            Self::make_1bs_item(0, 2, Self::ITEM_1BS_VECTOR_TABLE),
            table_ptr,
        ]
    }

    /// Create an Entry Point item.
    pub const fn make_entry_point(entry_point: u32, initial_sp: u32) -> [u32; 3] {
        [
            Self::make_1bs_item(0, 3, Self::ITEM_1BS_ENTRY_POINT),
            entry_point,
            initial_sp,
        ]
    }

    /// Create an Rolling Window item.
    ///
    /// The delta is the number of bytes into the image that 0x10000000 should
    /// be mapped.
    pub const fn make_rolling_window(delta: u32) -> [u32; 2] {
        [
            Self::make_1bs_item(0, 3, Self::ITEM_1BS_ROLLING_WINDOW_DELTA),
            delta,
        ]
    }
}

impl Block<0> {
    /// Construct an empty block.
    pub const fn empty() -> Block<0> {
        Block {
            // This is the magic ROM value the Pico-SDK calls PICOBIN_BLOCK_MARKER_START
            marker_start: 0xffffded3,
            items: [],
            length: Self::make_length(0),
            // offset from this block to next block in loop. By default
            // we form a Block Loop with a single Block in it.
            offset: 0x0000_0000,
            // This is the magic ROM value the Pico-SDK calls PICOBIN_BLOCK_MARKER_END
            marker_end: 0xab123579,
        }
    }

    /// Make the block one word larger
    pub const fn extend(self, word: u32) -> Block<1> {
        Block::new([word])
    }
}

impl Block<1> {
    /// Make the block one word larger
    pub const fn extend(self, word: u32) -> Block<2> {
        Block::new([self.items[0], word])
    }
}

impl Block<2> {
    /// Make the block one word larger
    pub const fn extend(self, word: u32) -> Block<3> {
        Block::new([self.items[0], self.items[1], word])
    }
}

impl ImageDef {
    /// Construct a new IMAGE_DEF Block, for an EXE with the given security and
    /// architecture.
    pub const fn arch_exe(security: Security, architecture: Architecture) -> Self {
        Self::new([Self::make_image_type_exe(security, architecture)])
    }

    /// Construct a new IMAGE_DEF Block, for an EXE with the given security.
    ///
    /// The target architecture is taken from the current build target (i.e. Arm
    /// or RISC-V).
    pub const fn exe(security: Security) -> Self {
        if cfg!(all(target_arch = "riscv32", target_os = "none")) {
            Self::arch_exe(security, Architecture::Riscv)
        } else {
            Self::arch_exe(security, Architecture::Arm)
        }
    }

    /// Construct a new IMAGE_DEF Block, for a Non-Secure EXE.
    ///
    /// The target architecture is taken from the current build target (i.e. Arm
    /// or RISC-V).
    pub const fn non_secure_exe() -> Self {
        Self::exe(Security::NonSecure)
    }

    /// Construct a new IMAGE_DEF Block, for a Secure EXE.
    ///
    /// The target architecture is taken from the current build target (i.e. Arm
    /// or RISC-V).
    pub const fn secure_exe() -> Self {
        Self::exe(Security::Secure)
    }
}

/// We make our partition table this fixed size.
pub const PARTITION_TABLE_MAX_ITEMS: usize = 128;

/// Describes a unpartitioned space
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UnpartitionedSpace {
    permissions: u32,
    flags: u32,
}

impl UnpartitionedSpace {
    /// Create a new unpartitioned space.
    ///
    /// It defaults to no permissions.
    pub const fn new() -> Self {
        Self {
            permissions: 0,
            flags: 0,
        }
    }

    /// Add a permission
    pub const fn with_permission(self, permission: Permission) -> Self {
        Self {
            permissions: self.permissions | permission as u32,
            ..self
        }
    }

    /// Set a flag
    pub const fn with_flag(self, flag: UnpartitionedFlag) -> Self {
        Self {
            flags: self.flags | flag as u32,
            ..self
        }
    }
}
/// Describes a Partition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partition {
    first_sector: u32,
    last_sector: u32,
    permissions: u32,
    flags: u32,
    id: Option<u64>,
    extra_families: [u32; 4],
    extra_families_len: usize,
    name: [u8; 128],
}

impl Partition {
    const FLAGS_HAS_ID: u32 = 0b1;
    const FLAGS_LINK_TYPE_A_PARTITION: u32 = 0b01 << 1;
    const FLAGS_LINK_TYPE_OWNER: u32 = 0b10 << 1;
    const FLAGS_LINK_MASK: u32 = 0b111111 << 1;
    const FLAGS_HAS_NAME: u32 = 0b1 << 12;
    const FLAGS_HAS_EXTRA_FAMILIES_SHIFT: u8 = 7;
    const FLAGS_HAS_EXTRA_FAMILIES_MASK: u32 = 0b11 << Self::FLAGS_HAS_EXTRA_FAMILIES_SHIFT;

    /// Create a new partition, with the given start and end sectors.
    ///
    /// It defaults to no permissions.
    pub const fn new(first_sector: u32, last_sector: u32) -> Self {
        // 0x2000 sectors of 4 KiB is 32 MiB, which is the total XIP area
        assert!(first_sector < 0x2000);
        assert!(last_sector < 0x2000);
        assert!(first_sector <= last_sector);
        Self {
            first_sector,
            last_sector,
            permissions: 0,
            flags: 0,
            id: None,
            extra_families: [0; 4],
            extra_families_len: 0,
            name: [0; 128],
        }
    }

    /// Add a permission
    pub const fn with_permission(self, permission: Permission) -> Self {
        Self {
            permissions: self.permissions | permission as u32,
            ..self
        }
    }

    /// Set the name of the partition
    pub const fn with_name(self, name: &str) -> Self {
        let mut new_name = [0u8; 128];
        let name = name.as_bytes();
        let mut idx = 0;
        new_name[0] = name.len() as u8;
        while idx < name.len() {
            new_name[idx + 1] = name[idx];
            idx += 1;
        }
        Self {
            name: new_name,
            flags: self.flags | Self::FLAGS_HAS_NAME,
            ..self
        }
    }

    /// Set the extra families for the partition.
    ///
    /// You can supply up to four.
    pub const fn with_extra_families(self, extra_families: &[u32]) -> Self {
        assert!(extra_families.len() <= 4);
        let mut new_extra_families = [0u32; 4];
        let mut idx = 0;
        while idx < extra_families.len() {
            new_extra_families[idx] = extra_families[idx];
            idx += 1;
        }
        Self {
            extra_families: new_extra_families,
            extra_families_len: extra_families.len(),
            flags: (self.flags & !Self::FLAGS_HAS_EXTRA_FAMILIES_MASK)
                | (extra_families.len() as u32) << Self::FLAGS_HAS_EXTRA_FAMILIES_SHIFT,
            ..self
        }
    }

    /// Set the ID
    pub const fn with_id(self, id: u64) -> Self {
        Self {
            id: Some(id),
            flags: self.flags | Self::FLAGS_HAS_ID,
            ..self
        }
    }

    /// Add a link
    pub const fn with_link(self, link: Link) -> Self {
        let mut new_flags = self.flags & !Self::FLAGS_LINK_MASK;
        match link {
            Link::Nothing => {}
            Link::ToA { partition_idx } => {
                assert!(partition_idx < 16);
                new_flags |= Self::FLAGS_LINK_TYPE_A_PARTITION;
                new_flags |= (partition_idx as u32) << 3;
            }
            Link::ToOwner { partition_idx } => {
                assert!(partition_idx < 16);
                new_flags |= Self::FLAGS_LINK_TYPE_OWNER;
                new_flags |= (partition_idx as u32) << 3;
            }
        }
        Self {
            flags: new_flags,
            ..self
        }
    }

    /// Set a flag
    pub const fn with_flag(self, flag: PartitionFlag) -> Self {
        Self {
            flags: self.flags | flag as u32,
            ..self
        }
    }
}

/// Describes a partition table.
///
/// Don't store this as a static - make sure you convert it to a block.
#[derive(Clone)]
pub struct PartitionTable {
    /// This must look like a block, including the 1 word header and the 3 word footer.
    contents: [u32; PARTITION_TABLE_MAX_ITEMS],
    /// This value doesn't include the 1 word header or the 3 word footer
    num_items: usize,
}

impl PartitionTable {
    /// Create an empty Block, big enough for a partition table.
    ///
    /// At a minimum you need to call [`Self::add_partition_item`].
    pub const fn new() -> PartitionTable {
        let mut contents = [0; PARTITION_TABLE_MAX_ITEMS];
        contents[0] = Block::<0>::BLOCK_MARKER_START;
        contents[1] = Block::<0>::make_length(0);
        contents[2] = 0;
        contents[3] = Block::<0>::BLOCK_MARKER_END;
        PartitionTable {
            contents,
            num_items: 0,
        }
    }

    /// Add a partition to the partition table
    pub const fn add_partition_item(
        self,
        unpartitioned: UnpartitionedSpace,
        partitions: &[Partition],
    ) -> Self {
        let mut new_table = PartitionTable::new();
        let mut idx = 0;
        // copy over old table, with the header but not the footer
        while idx < self.num_items + 1 {
            new_table.contents[idx] = self.contents[idx];
            idx += 1;
        }

        // 1. add item header space (we fill this in later)
        let header_idx = idx;
        new_table.contents[idx] = 0;
        idx += 1;

        // 2. unpartitioned space
        new_table.contents[idx] = unpartitioned.permissions | unpartitioned.flags;
        idx += 1;

        // 3. partition info

        let mut partition_no = 0;
        while partition_no < partitions.len() {
            // a. permissions_and_location (4K units)
            new_table.contents[idx] = partitions[partition_no].permissions
                | (partitions[partition_no].first_sector) & 0x0000_1FFF
                | ((partitions[partition_no].last_sector) & 0x0000_1FFF) << 13;
            idx += 1;

            // b. permissions_and_flags
            new_table.contents[idx] =
                partitions[partition_no].permissions | partitions[partition_no].flags;
            idx += 1;

            // c. ID
            if let Some(id) = partitions[partition_no].id {
                new_table.contents[idx] = id as u32;
                new_table.contents[idx + 1] = (id >> 32) as u32;
                idx += 2;
            }

            // d. Extra Families
            let mut extra_families_idx = 0;
            while extra_families_idx < partitions[partition_no].extra_families_len {
                new_table.contents[idx] =
                    partitions[partition_no].extra_families[extra_families_idx];
                idx += 1;
                extra_families_idx += 1;
            }

            // e. Name
            let mut name_idx = 0;
            while name_idx < partitions[partition_no].name[0] as usize {
                let name_chunk = [
                    partitions[partition_no].name[name_idx],
                    partitions[partition_no].name[name_idx + 1],
                    partitions[partition_no].name[name_idx + 2],
                    partitions[partition_no].name[name_idx + 3],
                ];
                new_table.contents[idx] = u32::from_le_bytes(name_chunk);
                name_idx += 4;
                idx += 1;
            }

            partition_no += 1;
        }

        let len = idx - header_idx;
        new_table.contents[header_idx] = Block::<0>::make_2bs_item(
            partitions.len() as u8,
            len as u16,
            Block::<0>::ITEM_2BS_PARTITION_TABLE,
        );

        // 7. New Footer
        new_table.contents[idx] = Block::<0>::make_length(idx as u16 - 1);
        new_table.contents[idx + 1] = 0;
        new_table.contents[idx + 2] = Block::<0>::BLOCK_MARKER_END;

        // ignore the header
        new_table.num_items = idx - 1;
        new_table
    }

    /// Add a version number to the partition table
    pub const fn with_version(self, major: u16, minor: u16) -> Self {
        let mut new_table = PartitionTable::new();
        let mut idx = 0;
        // copy over old table, with the header but not the footer
        while idx < self.num_items + 1 {
            new_table.contents[idx] = self.contents[idx];
            idx += 1;
        }

        // 1. add item
        new_table.contents[idx] = Block::<0>::make_2bs_item(0, 2, Block::<0>::ITEM_1BS_VERSION);
        idx += 1;
        new_table.contents[idx] = (major as u32) << 16 | minor as u32;
        idx += 1;

        // 2. New Footer
        new_table.contents[idx] = Block::<0>::make_length(idx as u16 - 1);
        new_table.contents[idx + 1] = 0;
        new_table.contents[idx + 2] = Block::<0>::BLOCK_MARKER_END;

        // ignore the header
        new_table.num_items = idx - 1;
        new_table
    }

    /// Add a a SHA256 hash of the Block
    ///
    /// Adds a `HASH_DEF` covering all the previous items in the Block, and a
    /// `HASH_VALUE` with a SHA-256 hash of them.
    pub const fn with_sha256(self) -> Self {
        let mut new_table = PartitionTable::new();
        let mut idx = 0;
        // copy over old table, with the header but not the footer
        while idx < self.num_items + 1 {
            new_table.contents[idx] = self.contents[idx];
            idx += 1;
        }

        // 1. HASH_DEF says what is hashed
        new_table.contents[idx] = Block::<0>::make_2bs_item(1, 2, Block::<0>::ITEM_2BS_HASH_DEF);
        idx += 1;
        // we're hashing all the previous contents - including this line.
        new_table.contents[idx] = (idx + 1) as u32;
        idx += 1;

        // calculate hash over prior contents
        let input = unsafe {
            core::slice::from_raw_parts(new_table.contents.as_ptr() as *const u8, idx * 4)
        };
        let hash: [u8; 32] = sha2_const_stable::Sha256::new().update(input).finalize();

        // 2. HASH_VALUE contains the hash
        new_table.contents[idx] = Block::<0>::make_2bs_item(0, 9, Block::<0>::ITEM_1BS_HASH_VALUE);
        idx += 1;

        let mut hash_idx = 0;
        while hash_idx < hash.len() {
            new_table.contents[idx] = u32::from_le_bytes([
                hash[hash_idx],
                hash[hash_idx + 1],
                hash[hash_idx + 2],
                hash[hash_idx + 3],
            ]);
            idx += 1;
            hash_idx += 4;
        }

        // 3. New Footer
        new_table.contents[idx] = Block::<0>::make_length(idx as u16 - 1);
        new_table.contents[idx + 1] = 0;
        new_table.contents[idx + 2] = Block::<0>::BLOCK_MARKER_END;

        // ignore the header
        new_table.num_items = idx - 1;
        new_table
    }
}

impl Default for PartitionTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Flags that a Partition can have
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum PartitionFlag {
    NotBootableArm = 1 << 9,
    NotBootableRiscv = 1 << 10,
    Uf2DownloadAbNonBootableOwnerAffinity = 1 << 11,
    Uf2DownloadNoReboot = 1 << 13,
    AcceptsDefaultFamilyRp2040 = 1 << 14,
    AcceptsDefaultFamilyData = 1 << 16,
    AcceptsDefaultFamilyRp2350ArmS = 1 << 17,
    AcceptsDefaultFamilyRp2350Riscv = 1 << 18,
    AcceptsDefaultFamilyRp2350ArmNs = 1 << 19,
}

/// Flags that a Partition can have
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum UnpartitionedFlag {
    Uf2DownloadNoReboot = 1 << 13,
    AcceptsDefaultFamilyRp2040 = 1 << 14,
    AcceptsDefaultFamilyAbsolute = 1 << 15,
    AcceptsDefaultFamilyData = 1 << 16,
    AcceptsDefaultFamilyRp2350ArmS = 1 << 17,
    AcceptsDefaultFamilyRp2350Riscv = 1 << 18,
    AcceptsDefaultFamilyRp2350ArmNs = 1 << 19,
}

/// Kinds of linked partition
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Link {
    /// Not linked to anything
    Nothing,
    /// This is a B partition - link to our A partition.
    ToA {
        /// The index of our matching A partition.
        partition_idx: u8,
    },
    /// Link to the partition that owns this one.
    ToOwner {
        /// The idx of our owner
        partition_idx: u8,
    },
}

/// Permissions that a Partition can have
#[repr(u32)]
pub enum Permission {
    /// Can be read in Secure Mode
    ///
    /// Corresponds to `PERMISSION_S_R_BITS` in the Pico SDK
    SecureRead = 1 << 26,
    /// Can be written in Secure Mode
    ///
    /// Corresponds to `PERMISSION_S_W_BITS` in the Pico SDK
    SecureWrite = 1 << 27,
    /// Can be read in Non-Secure Mode
    ///
    /// Corresponds to `PERMISSION_NS_R_BITS` in the Pico SDK
    NonSecureRead = 1 << 28,
    /// Can be written in Non-Secure Mode
    ///
    /// Corresponds to `PERMISSION_NS_W_BITS` in the Pico SDK
    NonSecureWrite = 1 << 29,
    /// Can be read in Non-Secure Bootloader mode
    ///
    /// Corresponds to `PERMISSION_NSBOOT_R_BITS` in the Pico SDK
    BootRead = 1 << 30,
    /// Can be written in Non-Secure Bootloader mode
    ///
    /// Corresponds to `PERMISSION_NSBOOT_W_BITS` in the Pico SDK
    BootWrite = 1 << 31,
}

/// The supported RP2350 CPU architectures
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Architecture {
    /// Core is in Arm Cortex-M33 mode
    Arm,
    /// Core is in RISC-V / Hazard3 mode
    Riscv,
}

/// The kinds of Secure Boot we support
#[derive(Debug, Copy, Clone)]
pub enum Security {
    /// Security mode not given
    Unspecified,
    /// Start in Non-Secure mode
    NonSecure,
    /// Start in Secure mode
    Secure,
}

#[cfg(test)]
mod test {
    use super::*;

    /// I used this JSON, with `picotool partition create`:
    ///
    /// ```json
    /// {
    ///   "version": [1, 0],
    ///   "unpartitioned": {
    ///     "families": ["absolute"],
    ///     "permissions": {
    ///       "secure": "rw",
    ///       "nonsecure": "rw",
    ///       "bootloader": "rw"
    ///     }
    ///   },
    ///   "partitions": [
    ///     {
    ///       "name": "A",
    ///       "id": 0,
    ///       "size": "2044K",
    ///       "families": ["rp2350-arm-s", "rp2350-riscv"],
    ///       "permissions": {
    ///         "secure": "rw",
    ///         "nonsecure": "rw",
    ///         "bootloader": "rw"
    ///       }
    ///     },
    ///     {
    ///       "name": "B",
    ///       "id": 1,
    ///       "size": "2044K",
    ///       "families": ["rp2350-arm-s", "rp2350-riscv"],
    ///       "permissions": {
    ///         "secure": "rw",
    ///         "nonsecure": "rw",
    ///         "bootloader": "rw"
    ///       },
    ///       "link": ["a", 0]
    ///     }
    ///   ]
    /// }
    /// ```
    #[test]
    fn make_hashed_partition_table() {
        let table = PartitionTable::new()
            .add_partition_item(
                UnpartitionedSpace::new()
                    .with_permission(Permission::SecureRead)
                    .with_permission(Permission::SecureWrite)
                    .with_permission(Permission::NonSecureRead)
                    .with_permission(Permission::NonSecureWrite)
                    .with_permission(Permission::BootRead)
                    .with_permission(Permission::BootWrite)
                    .with_flag(UnpartitionedFlag::AcceptsDefaultFamilyAbsolute),
                &[
                    Partition::new(2, 512)
                        .with_id(0)
                        .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350ArmS)
                        .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350Riscv)
                        .with_permission(Permission::SecureRead)
                        .with_permission(Permission::SecureWrite)
                        .with_permission(Permission::NonSecureRead)
                        .with_permission(Permission::NonSecureWrite)
                        .with_permission(Permission::BootRead)
                        .with_permission(Permission::BootWrite)
                        .with_name("A"),
                    Partition::new(513, 1023)
                        .with_id(1)
                        .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350ArmS)
                        .with_flag(PartitionFlag::AcceptsDefaultFamilyRp2350Riscv)
                        .with_link(Link::ToA { partition_idx: 0 })
                        .with_permission(Permission::SecureRead)
                        .with_permission(Permission::SecureWrite)
                        .with_permission(Permission::NonSecureRead)
                        .with_permission(Permission::NonSecureWrite)
                        .with_permission(Permission::BootRead)
                        .with_permission(Permission::BootWrite)
                        .with_name("B"),
                ],
            )
            .with_version(1, 0)
            .with_sha256();
        let expected = &[
            0xffffded3, // start
            0x02000c0a, // Item = PARTITION_TABLE
            0xfc008000, // Unpartitioned Space - permissions_and_flags
            0xfc400002, // Partition 0 - permissions_and_location (512 * 4096, 2 * 4096)
            0xfc061001, //     permissions_and_flags HAS_ID | HAS_NAME | ARM-S | RISC-V
            0x00000000, //     ID
            0x00000000, //     ID
            0x00004101, //     Name ("A")
            0xfc7fe201, // Partition 1 - permissions_and_location (1023 * 4096, 513 * 4096)
            0xfc061003, //     permissions_and_flags LINKA(0) | HAS_ID | HAS_NAME | ARM-S | RISC-V
            0x00000001, //     ID
            0x00000000, //     ID
            0x00004201, //     Name ("B")
            0x00000248, // Item = Version
            0x00010000, // 0, 1
            0x01000247, // HASH_DEF with 2 words, and SHA256 hash
            0x00000011, // 17 words hashed
            0x0000094b, // HASH_VALUE with 9 words
            0x1945cdad, // Hash word 0
            0x6b5f9773, // Hash word 1
            0xe2bf39bd, // Hash word 2
            0xb243e599, // Hash word 3
            0xab2f0e9a, // Hash word 4
            0x4d5d6d0b, // Hash word 5
            0xf973050f, // Hash word 6
            0x5ab6dadb, // Hash word 7
            0x000019ff, // Last Item
            0x00000000, // Block Loop Next Offset
            0xab123579, // End
        ];
        assert_eq!(
            &table.contents[..29],
            expected,
            "{:#010x?}\n != \n{:#010x?}",
            &table.contents[0..29],
            expected,
        );
    }
}
