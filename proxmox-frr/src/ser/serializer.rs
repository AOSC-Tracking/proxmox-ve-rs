use anyhow::Context;
use minijinja::Environment;

use crate::ser::FrrConfig;
use proxmox_sortable_macro::sortable;

#[sortable]
pub static TEMPLATES: [(&str, &str); 12] = sorted!([
    (
        "fabricd.jinja",
        include_str!("../../../proxmox-frr-templates/templates/fabricd.jinja"),
    ),
    (
        "bgpd.jinja",
        include_str!("../../../proxmox-frr-templates/templates/bgpd.jinja")
    ),
    (
        "isisd.jinja",
        include_str!("../../../proxmox-frr-templates/templates/isisd.jinja"),
    ),
    (
        "ospfd.jinja",
        include_str!("../../../proxmox-frr-templates/templates/ospfd.jinja"),
    ),
    (
        "bgp_router.jinja",
        include_str!("../../../proxmox-frr-templates/templates/bgp_router.jinja"),
    ),
    (
        "interface.jinja",
        include_str!("../../../proxmox-frr-templates/templates/interface.jinja"),
    ),
    (
        "access_lists.jinja",
        include_str!("../../../proxmox-frr-templates/templates/access_lists.jinja"),
    ),
    (
        "prefix_lists.jinja",
        include_str!("../../../proxmox-frr-templates/templates/prefix_lists.jinja"),
    ),
    (
        "route_maps.jinja",
        include_str!("../../../proxmox-frr-templates/templates/route_maps.jinja"),
    ),
    (
        "ip_routes.jinja",
        include_str!("../../../proxmox-frr-templates/templates/ip_routes.jinja"),
    ),
    (
        "protocol_routemaps.jinja",
        include_str!("../../../proxmox-frr-templates/templates/protocol_routemaps.jinja"),
    ),
    (
        "frr.conf.jinja",
        include_str!("../../../proxmox-frr-templates/templates/frr.conf.jinja"),
    ),
]);

fn create_env<'a>() -> Environment<'a> {
    let mut env = Environment::new();

    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);

    // avoid unnecessary additional newlines
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);

    env.set_loader(move |name| {
        Ok(TEMPLATES
            .binary_search_by(|v| v.0.cmp(name))
            .map(|i| TEMPLATES[i].1)
            .map(|template| (*template).to_owned())
            .ok())
    });

    env
}

/// Render the passed [`FrrConfig`] into a single string containing the whole config.
pub fn dump(config: &FrrConfig) -> Result<String, anyhow::Error> {
    create_env()
        .get_template("frr.conf.jinja")
        .with_context(|| "could not obtain frr template from environment")?
        .render(config)
        .with_context(|| "could not render frr template")
}

/// Render the passed [`FrrConfig`] into the literal Frr config.
///
/// The Frr config is returned as lines stored in a Vec.
pub fn to_raw_config(config: &FrrConfig) -> Result<Vec<String>, anyhow::Error> {
    Ok(dump(config)?.lines().map(|line| line.to_owned()).collect())
}
