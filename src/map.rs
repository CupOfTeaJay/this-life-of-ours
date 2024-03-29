/*
 * map.rs
 * 
 * This file is responsible for randomly generating a map.
 */

use bevy::prelude::*;

use crate::hex_grid::{HexBundle, HexPosition};
use crate::tile::TileBundle;
use crate::wave_function::WaveFunction;

const HEX_FACTOR: f32 = 0.75;
const MAP_WIDTH: u8 = 25;  // TODO: Map size should obviously be configurable.
const MAP_HEIGHT: u8 = 13; // TODO: Map size should obviously be configurable.
const TILE_Y_POS: f32 = 0.0;

pub struct MapPlugin;

impl Plugin for MapPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hex_grid, generate_map).chain());
    }
}

/// Helper function that determines where to place the vertical-most HexBundle of a file in the 
/// hex-grid as a function of the file number (Map is generated sequentially, file by file).
fn determine_z_pos_start(
    total_ranks: u8,
    iteration: u8
) -> f32 {
    let height_unit: f32 = HEX_FACTOR.sqrt();
    if iteration % 2 == 0 {
        -height_unit*((total_ranks - 1) as f32)
    }
    else {
        -height_unit*((total_ranks - 2) as f32)
    }
}

/// Spawns a blank hex-grid.
fn spawn_hex_grid(
    mut commands: Commands,
) {
    let height_unit: f32 = HEX_FACTOR.sqrt();
    let mut x_pos: f32 = -HEX_FACTOR*(MAP_WIDTH as f32) + HEX_FACTOR;
    for i in 0..MAP_WIDTH {
        let mut z_pos: f32 = determine_z_pos_start(MAP_WIDTH, i);
        for j in 0..MAP_HEIGHT {
            commands.spawn(HexBundle::new(i, j, Vec3::new(x_pos, TILE_Y_POS, z_pos)));
            z_pos += height_unit*2.0;
        }
        x_pos += HEX_FACTOR*2.0;
    }
}

/// Generates a map upon the hex-grid by collapsing each hex's wave-function.
fn generate_map (
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(&Transform, &HexPosition, &WaveFunction)>
) {
    for (transform, hex_pos, wave_function) in &query {
        commands.spawn(
            TileBundle {
                grid_pos: hex_pos.clone(),
                model: SceneBundle {
                    scene: asset_server.load(wave_function.collapse()),
                    transform: *transform,
                    ..Default::default()
                }
            }
        );
    }
}

