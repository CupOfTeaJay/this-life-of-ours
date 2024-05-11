/*
    Such is Life
    Copyright (C) 2024 Clevermeld™ LLC

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

/// Classifiers for what positions during map generation will collapse to a
/// coastal, ocean, or some land tile.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Elevation {
    Coastal,
    Land,
    Ocean,
}

/// TODO:
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Terrain {
    Coastal,
    Desert,
    Grassland,
    Ice,
    Ocean,
    Snow,
    Steppe,
    Tundra,
}

impl Terrain {
    pub fn rep(&self) -> String {
        match self {
            Terrain::Coastal => "tiles/coastalTile.glb#Scene0".to_string(),
            Terrain::Desert => "tiles/desertTile.glb#Scene0".to_string(),
            Terrain::Grassland => "tiles/grasslandTile.glb#Scene0".to_string(),
            Terrain::Ice => "tiles/iceTile.glb#Scene0".to_string(),
            Terrain::Ocean => "tiles/oceanTile.glb#Scene0".to_string(),
            Terrain::Snow => "tiles/snowTile.glb#Scene0".to_string(),
            Terrain::Steppe => "tiles/steppeTile.glb#Scene0".to_string(),
            Terrain::Tundra => "tiles/tundraTile.glb#Scene0".to_string(),
        }
    }
}