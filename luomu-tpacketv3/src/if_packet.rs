//! structures, types and constants from <linux/if_packet.h>

#![allow(dead_code)] // not all declared types are used yet
// naming mirrors Linux
#![allow(non_camel_case_types, clippy::struct_field_names)]

pub const TPACKET_V3: libc::c_int = 2;

// socket options
pub const PACKET_ADD_MEMBERSHIP: libc::c_int = libc::PACKET_ADD_MEMBERSHIP;
pub const PACKET_RX_RING: libc::c_int = 5;
pub const PACKET_VERSION: libc::c_int = 10;
pub const PACKET_STATISTICS: libc::c_int = 6;
pub const PACKET_FANOUT: libc::c_int = 18;

// FANOUT modes
pub const PACKET_FANOUT_HASH: libc::c_int = 0;
pub const PACKET_FANOUT_LB: libc::c_int = 1;
pub const PACKET_FANOUT_CPU: libc::c_int = 2;
pub const PACKET_FANOUT_ROLLOVER: libc::c_int = 3;
pub const PACKET_FANOUT_RND: libc::c_int = 4;
pub const PACKET_FANOUT_QM: libc::c_int = 5;
pub const PACKET_FANOUT_CBPF: libc::c_int = 6;
pub const PACKET_FANOUT_EBPF: libc::c_int = 7;
// FANOUT flags
pub const PACKET_FANOUT_FLAG_ROLLOVER: libc::c_int = 0x1000;
pub const PACKET_FANOUT_FLAG_UNIQUEID: libc::c_int = 0x2000;
pub const PACKET_FANOUT_FLAG_DEFRAG: libc::c_int = 0x8000;

pub const TP_STATUS_KERNEL: u32 = 0;
pub const TP_STATUS_USER: u32 = 1 << 0;
pub const TP_STATUS_COPY: u32 = 1 << 1;
pub const TP_STATUS_LOSING: u32 = 1 << 2;
pub const TP_STATUS_CSUMNOTREADY: u32 = 1 << 3;
pub const TP_STATUS_VLAN_VALID: u32 = 1 << 4;
pub const TP_STATUS_BLK_TMO: u32 = 1 << 5;
pub const TP_STATUS_VLAN_TPID_VALID: u32 = 1 << 6;
pub const TP_STATUS_CSUM_VALID: u32 = 1 << 7;
pub const TP_STATUS_GSO_TCP: u32 = 1 << 8;
pub const TP_STATUS_TS_SOFTWARE: u32 = 1 << 29;
pub const TP_STATUS_TS_SYS_HARDWARE: u32 = 1 << 30; /* deprecated, never set */
pub const TP_STATUS_TS_RAW_HARDWARE: u32 = 1u32 << 31;

#[repr(C)]
pub struct tpacket_req3 {
    pub tp_block_size: libc::c_uint,
    pub tp_block_nr: libc::c_uint,
    pub tp_frame_size: libc::c_uint,
    pub tp_frame_nr: libc::c_uint,
    pub tp_retire_blk_tov: libc::c_uint,
    pub tp_sizeof_priv: libc::c_uint,
    pub tp_feature_req_word: libc::c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct tpacket_hdr_variant1 {
    pub tp_rxhash: u32,
    pub tp_vlan_tci: u32,
    pub tp_vlan_tpid: u16,
    tp_padding: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct tpacket3_hdr {
    pub tp_next_offset: u32,
    pub tp_sec: u32,
    pub tp_nsec: u32,
    pub tp_snaplen: u32,
    pub tp_len: u32,
    pub tp_status: u32,
    pub tp_mac: u16,
    pub tp_net: u16,
    pub hv1: tpacket_hdr_variant1,
    tp_padding: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_bd_ts {
    pub ts_sec: libc::c_uint,
    pub ts_union: tpacket_bd_ts_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_ts_union {
    pub ts_usec: libc::c_uint,
    pub ts_nsec: libc::c_uint,
}

impl std::fmt::Debug for tpacket_bd_ts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("tpacket_bd_ts")
            .field("ts_sec", &self.ts_sec)
            .field("ts_nsec", unsafe { &self.ts_union.ts_nsec })
            .finish()
    }
}

impl std::fmt::Debug for tpacket_bd_ts_union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{}", self.ts_nsec) }
    }
}

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct tpacket_hdr_v1 {
    pub block_status: u32,
    pub num_pkts: u32,
    pub offset_to_first_pkt: u32,
    pub blk_len: u32,
    pub seq_num: u64,
    pub ts_first_pkt: tpacket_bd_ts,
    pub ts_last_pkt: tpacket_bd_ts,
}

#[repr(C)]
#[derive(Debug)]
pub struct tpacket_block_desc {
    pub version: u32,
    pub offset_to_priv: u32,
    pub hdr: tpacket_bd_header_u,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct tpacket_stats_v3 {
    pub tp_packets: libc::c_uint,
    pub tp_drops: libc::c_uint,
    pub tp_freeze_q_cnt: libc::c_uint,
}

#[repr(C)]
pub union tpacket_bd_header_u {
    pub bh1: tpacket_hdr_v1,
}

use std::fmt;

impl fmt::Debug for tpacket_bd_header_u {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { self.bh1.fmt(f) }
    }
}

pub const TPACKET_ALIGNMENT: usize = 16;
