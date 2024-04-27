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

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

/// Makes all of the tiles pickable per bevy_mod_picking.
pub fn make_tiles_pickable(
    mut commands: Commands,
    query: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(PickableBundle::default());
    }
}