/*
    Project Hex
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

#[derive(Component)]
enum AirUnitClass {
    Attack,  // Default aircraft class. Jack of all trades, master of none.
    Bomber,  // Aircraft class that specializes in air-to-ground combat.
    Fighter, // Aircraft class that specialize in air-to-air combat.
    Recon,   // Light aircraft. Best for reconniassance purposes.
    Rotary,  // Helicopters and the like.
}
