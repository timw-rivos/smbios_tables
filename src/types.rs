// Copyright 2024 Rivos, Inc.
// SPDX-License-Identifier: Apache-2.0

use bitfield::bitfield;
use bitflags::bitflags;
use zerocopy::AsBytes;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct BiosCharacteristics(u64);

bitflags! {
    impl BiosCharacteristics: u64 {
        const Reserved = 1 << 1;
        const Unknown = 1 << 2;
        const Unsupported = 1 << 3;
        const IsaSupported = 1 << 4;
        const McaSupported = 1 << 5;
        const EisaSupported = 1 << 6;
        const PciSupported = 1 << 7;
        const PcCardSupported = 1 << 8;
        const PnpSupported = 1 << 9;
        const ApmSupported = 1 << 10;
        const Upgradeable = 1 << 11;
        const ShadowingAllowed = 1 << 12;
        const VlVesaSupported = 1 << 13;
        const EscdAvailable = 1 << 14;
        const BootFromCdSupported = 1 << 15;
        const SelectableBootSupported = 1 << 16;
        const SocketedRom = 1 << 17;
        const BootFromPcCardSupported = 1 << 18;
        const EddSpecSupported = 1 << 19;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct BiosCharacteristicsEx1(u8);
bitflags! {
    impl BiosCharacteristicsEx1: u8 {
        const AcpiSupported = 1 << 0;
        const UsbLegacySupported = 1 << 1;
        const AgpSupported = 1 << 2;
        const SmartBatterySupported = 1 << 7;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct BiosCharacteristicsEx2(u8);
bitflags! {
    impl BiosCharacteristicsEx2: u8 {
        const BiosBootSpecSupported = 1 << 0;
        const FnKeyNetworkBootSupported = 1 << 1;
        const TargetedContentDistribution = 1 << 2;
        const UefiSupported = 1 << 3;
        const VirtualMachine = 1 << 4;
        const ManufacturingModeSupported = 1 << 5;
        const ManufacturingModeEnabled = 1 << 6;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum WakeupType {
    Reserved = 0,
    Other = 1,
    #[default]
    Unknown = 2,
    ApmTimer = 3,
    ModemRing = 4,
    LanRemote = 5,
    PowerSwitch = 6,
    PciPme = 7,
    AcPowerRestored = 8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ProcessorType {
    Other = 1,
    #[default]
    Unknown = 2,
    CentralProcessor = 3,
    MathProcessor = 4,
    DspProcessor = 5,
    VideoProcessor = 6,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ProcessorFamily {
    Other = 1,
    #[default]
    Unknown = 2,
    ObtainFrom2 = 0xfe,
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ProcessorFamily2 {
    RiscvRv32 = 0x200,
    RiscvRv64 = 0x201,
    RiscvRv128 = 0x202,
    #[default]
    Reserved = 0xfffe,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ProcessorUpgrade {
    Other = 1,
    #[default]
    Unknown = 2,
    DaughterBoard = 3,
    None = 6,
}

bitflags! {
    pub struct RiscvProcessorCharacteristics1: u16 {
        const Reserved = 1 << 0;
        const Unknown = 1 << 1;
        const Bits64Capable = 1 << 2;
        const MultiCore = 1 << 3;
        const HardwareThread = 1 << 4;
        const ExecuteProtection = 1 << 5;
        const EnhancedVirtualization = 1 << 6;
        const PowerPerformanceControl = 1 << 7;
        const Bits128Capable = 1 << 8;
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, AsBytes)]
    pub struct CacheConfiguration(u16);
    pub op_mode, set_op_mode: 9, 8;
    pub enabled, set_enabled: 7;
    pub location, set_location: 6, 5;
    pub socketed, set_socketed: 3;
    pub level, set_level: 2, 0;
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum SramType {
    Other = 1 << 0,
    #[default]
    Unknown = 1 << 1,
    NonBurst = 1 << 2,
    Burst = 1 << 3,
    PipelineBurst = 1 << 4,
    Synchronous = 1 << 5,
    Asynchronous = 1 << 6,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum EccType {
    Other = 1,
    #[default]
    Unknown = 2,
    None = 3,
    Parity = 4,
    SingleBitEcc = 5,
    MultiBitEcc = 6,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum SystemCacheType {
    Other = 1,
    #[default]
    Unknown = 2,
    Instruction = 3,
    Data = 4,
    Unified = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum Associativity {
    Other = 1,
    #[default]
    Unknown = 2,
    DirectMapped = 3,
    SetAssociative2Way = 4,
    SetAssociative4Way = 5,
    FullyAssociative = 6,
    SetAssociative8Way = 7,
    SetAssociative16Way = 8,
    SetAssociative12Way = 9,
    SetAssociative24Way = 10,
    SetAssociative32Way = 11,
    SetAssociative48Way = 12,
    SetAssociative64Way = 13,
    SetAssociative20Way = 14,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum SlotType {
    Other = 1,
    #[default]
    Unknown = 2,
    PcieGen5Sff8639 = 0x25,
    PcieGen5 = 0xbf,
    PcieGen5x1 = 0xc0,
    PcieGen5x2 = 0xc1,
    PcieGen5x4 = 0xc2,
    PcieGen5x8 = 0xc3,
    PcieGen5x16 = 0xc4,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum SlotWidth {
    Other = 1,
    #[default]
    Unknown = 2,
    Width8Bits = 3,
    Width16Bits = 4,
    Width32Bits = 5,
    Width64Bits = 6,
    Width128Bits = 7,
    Widthx1 = 8,
    Widthx2 = 9,
    Widthx4 = 10,
    Widthx8 = 11,
    Widthx12 = 12,
    Widthx16 = 13,
    Widthx32 = 14,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum CurrentUsage {
    Other = 1,
    #[default]
    Unknown = 2,
    Available = 3,
    InUse = 4,
    Unavailable = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum SlotLength {
    Other = 1,
    #[default]
    Unknown = 2,
    ShortLength = 3,
    LongLength = 4,
    FormFactor2_5Drive = 5,
    FormFactor3_5Drive = 6,
}

bitflags! {
    pub struct SlotCharacteristics1: u8 {
        const Unknown = 1 << 0;
        const Provides5Volts = 1 << 1;
        const Provides3_3Volts = 1 << 2;
    }
}

bitflags! {
    pub struct SlotCharacteristics2: u8 {
        const PmeSupported = 1 << 0;
        const HotplugSupported = 1 << 1;
        const SmbusSupported = 1 << 2;
        const BifurcationSupported = 1 << 3;
        const SurpriseRemovalSupported = 1 << 4;
        const Cxl1Supported = 1 << 5;
        const Cxl2Supported = 1 << 6;
        const Cxl3Supported = 1 << 7;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ArrayLocation {
    Other = 1,
    #[default]
    Unknown = 2,
    SystemBoard = 3,
    CxlAddonCard = 0xa4,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ArrayUse {
    Other = 1,
    #[default]
    Unknown = 2,
    SystemMemory = 3,
    VideoMemory = 4,
    FlashMemory = 5,
    NonVolatileRam = 6,
    CacheMemory = 7,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ErrorCorrectionType {
    Other = 1,
    #[default]
    Unknown = 2,
    None = 3,
    Parity = 4,
    SingleBitEcc = 5,
    MultiBitEcc = 6,
    Crc = 7,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum FormFactor {
    Other = 1,
    #[default]
    Unknown = 2,
    Simm = 3,
    Sip = 4,
    Chip = 5,
    Dip = 6,
    Zip = 7,
    Proprietary = 8,
    Dimm = 9,
    Tsop = 10,
    RowOfChips = 11,
    Rimm = 12,
    Sodimm = 13,
    Srimm = 14,
    FbDimm = 15,
    Die = 16,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum MemoryType {
    Other = 1,
    #[default]
    Unknown = 2,
    Dram = 3,
    Edram = 4,
    Vram = 5,
    Sram = 6,
    Ram = 7,
    Rom = 8,
    Flash = 9,
    Eeprom = 0xa,
    Feprom = 0xb,
    Eprom = 0xc,
    Cdram = 0xd,
    Dram3d = 0xe,
    Sdram = 0xf,
    Sgram = 0x10,
    Rdram = 0x11,
    Ddr = 0x12,
    Ddr2 = 0x13,
    Ddr2FbDimm = 0x14,
    Ddr3 = 0x18,
    Fbd2 = 0x19,
    Ddr4 = 0x1a,
    Lpddr = 0x1b,
    Lpddr2 = 0x1c,
    Lpddr3 = 0x1d,
    Lpddr4 = 0x1e,
    LogicalNonVolatile = 0x1f,
    Hbm = 0x20,
    Hbm2 = 0x21,
    Ddr5 = 0x22,
    Lpddr5 = 0x23,
    Hbm3 = 0x24,
}

bitflags! {
    pub struct TypeDetail: u16 {
        const Other = 1 << 1;
        const Unknown = 1 << 2;
        const FastPaged = 1 << 3;
        const StaticColumn = 1 << 4;
        const PseudoStatic = 1 << 5;
        const Rambus = 1 << 6;
        const Synchronous = 1 << 7;
        const Cmos = 1 << 8;
        const Edo = 1 << 9;
        const WindowDram = 1 << 10;
        const CacheDram = 1 << 11;
        const NonVolatile = 1 << 12;
        const Registered = 1 << 13;
        const Unbuffered = 1 << 14;
        const Lrdimm = 1 << 15;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum MemoryTechnology {
    Other = 1,
    #[default]
    Unknown = 2,
    Dram = 3,
    NvdimmN = 4,
    NvdimmF = 5,
    NvdimmP = 6,
    IntelOptane = 7,
}

bitflags! {
    pub struct OperatingMode: u16 {
        const Other = 1 << 1;
        const Unknown = 1 << 2;
        const Volatile = 1 << 3;
        const ByteAccessiblePersistent = 1 << 4;
        const BlockAccessiblePersistent = 1 << 5;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum PartitionRowPosition {
    Reserved = 0,
    #[default]
    Unknown = 0xff,
}

#[derive(Debug, Clone, Default)]
pub enum BootStatus<'a> {
    #[default]
    NoErrorsDetected,
    NoBootableMedia,
    NormalOperatingSystemFailedToLoad,
    FirmwareDetectedHardwareFailure,
    OperatingSystemDetectedHardwareFailure,
    UserRequestedBoot,
    SystemSecurityViolation,
    PreviouslyRequestedImage(&'a [u8]),
    SystemWatchdogTimer,
    // Note: first argument is code, must be 128 <= code <= 191
    VendorSpecific(u8, &'a [u8]),
    // Note: first argument is code, must be >= 192
    ProductSpecific(u8, &'a [u8]),
}

bitflags! {
    pub struct TpmDeviceCharacteristics: u64 {
        const Unsupported = 1 << 2;
        const FamilyConfigurableViaFirmwareUpdate = 1 << 3;
        const FamilyConfigurableViaPlatformSoftware = 1 << 4;
        const FamilyConfigurableViaOemProprietary = 1 << 5;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum ProcessorArchitecture {
    Riscv32 = 6,
    #[default]
    Riscv64 = 7,
    Riscv128 = 8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, AsBytes)]
pub enum Xlen {
    Unsupported = 0,
    Xlen32 = 1,
    #[default]
    Xlen64 = 2,
    Xlen128 = 3,
}
