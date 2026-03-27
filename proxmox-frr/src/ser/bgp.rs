use std::net::{IpAddr, Ipv4Addr};

use proxmox_network_types::ip_address::{Ipv4Cidr, Ipv6Cidr};
use serde::{Deserialize, Serialize};

use crate::ser::route_map::RouteMapName;
use crate::ser::{FrrWord, InterfaceName, IpRoute};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BgpRouterName {
    asn: u32,
    vrf: Option<FrrWord>,
}

impl BgpRouterName {
    pub fn new(asn: u32, vrf: Option<FrrWord>) -> Self {
        Self { asn, vrf }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NeighborRemoteAs {
    Internal,
    External,
    #[serde(untagged)]
    Asn(#[serde(deserialize_with = "proxmox_serde::perl::deserialize_u32")] u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NeighborGroup {
    pub name: FrrWord,
    #[serde(deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub bfd: bool,
    pub remote_as: NeighborRemoteAs,
    #[serde(default)]
    pub ips: Vec<IpAddr>,
    #[serde(default)]
    pub interfaces: Vec<InterfaceName>,
    pub ebgp_multihop: Option<u8>,
    pub update_source: Option<InterfaceName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ipv4UnicastAF {
    #[serde(flatten)]
    pub common_options: CommonAddressFamilyOptions,
    #[serde(default)]
    pub networks: Vec<Ipv4Cidr>,
    #[serde(default)]
    pub redistribute: Vec<Redistribution>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ipv6UnicastAF {
    #[serde(flatten)]
    pub common_options: CommonAddressFamilyOptions,
    #[serde(default)]
    pub networks: Vec<Ipv6Cidr>,
    #[serde(default)]
    pub redistribute: Vec<Redistribution>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct L2vpnEvpnAF {
    #[serde(flatten)]
    pub common_options: CommonAddressFamilyOptions,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub advertise_all_vni: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub advertise_default_gw: Option<bool>,
    #[serde(default)]
    pub default_originate: Vec<DefaultOriginate>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub advertise_ipv4_unicast: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub advertise_ipv6_unicast: Option<bool>,
    pub autort_as: Option<u32>,
    pub route_targets: Option<RouteTargets>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultOriginate {
    Ipv4,
    Ipv6,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RedistributeProtocol {
    Connected,
    Static,
    Ospf,
    Kernel,
    Isis,
    Ospf6,
    Openfabric,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Redistribution {
    pub protocol: RedistributeProtocol,
    pub metric: Option<u32>,
    pub route_map: Option<RouteMapName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RouteTargets {
    #[serde(default)]
    pub import: Vec<FrrWord>,
    #[serde(default)]
    pub export: Vec<FrrWord>,
    #[serde(default)]
    pub both: Vec<FrrWord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AddressFamilyNeighbor {
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "proxmox_serde::perl::deserialize_bool"
    )]
    pub soft_reconfiguration_inbound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_map_in: Option<RouteMapName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_map_out: Option<RouteMapName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CommonAddressFamilyOptions {
    #[serde(default)]
    pub import_vrf: Vec<FrrWord>,
    #[serde(default)]
    pub neighbors: Vec<AddressFamilyNeighbor>,
    #[serde(default)]
    pub custom_frr_config: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct AddressFamilies {
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_unicast: Option<Ipv4UnicastAF>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_unicast: Option<Ipv6UnicastAF>,
    #[serde(skip_serializing_if = "Option::is_none")]
    l2vpn_evpn: Option<L2vpnEvpnAF>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Vrf {
    pub vni: Option<u32>,
    #[serde(default)]
    pub ip_routes: Vec<IpRoute>,
    #[serde(default)]
    pub custom_frr_config: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BgpRouter {
    pub asn: u32,
    pub router_id: Ipv4Addr,
    #[serde(default)]
    pub coalesce_time: Option<u32>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub default_ipv4_unicast: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub hard_administrative_reset: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub graceful_restart_notification: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub disable_ebgp_connected_route_check: Option<bool>,
    #[serde(default, deserialize_with = "proxmox_serde::perl::deserialize_bool")]
    pub bestpath_as_path_multipath_relax: Option<bool>,
    #[serde(default)]
    pub neighbor_groups: Vec<NeighborGroup>,
    #[serde(default)]
    pub address_families: AddressFamilies,
    #[serde(default)]
    pub custom_frr_config: Vec<String>,
}
