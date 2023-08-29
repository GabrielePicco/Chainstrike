use anchor_lang::prelude::*;
use crate::{Facing, Game, MakeMove};
use crate::errors::KamikazeJoeError;

pub fn handler(
    ctx: Context<MakeMove>,
    direction: Facing,
    energy: u8,
) -> Result<()> {
    let player_key = *ctx.accounts.player.unsigned_key();

    // Check if game is active
    if !ctx.accounts.game.is_game_active() {
        return Err(KamikazeJoeError::GameEnded.into());
    }

    // Find player in game_account Players Vec
    let player_index = match ctx.accounts.game.get_player_index(player_key) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    // Check if energy is valid
    if energy > 5 {
        return Err(KamikazeJoeError::NotValidEnergy.into());
    }

    return move_player(&mut ctx.accounts.game, player_index, direction, energy);
}

fn move_player(game: &mut Account<Game>, player_index: usize, direction: Facing, energy: u8) -> Result<()>  {

    // Check if energy is valid
    if game.players[player_index].energy <= 0 {
        return Err(KamikazeJoeError::NotValidEnergy.into());
    }

    let mut final_x = game.players[player_index].x;
    let mut final_y = game.players[player_index].y;
    let mut is_valid = false;

    // Movement loop
    for _ in 0..energy {

        let x: u8;
        let y: u8;

        match direction {
            Facing::Down => {
                if final_y == 0 {
                    break;
                }
                x = final_x;
                y = final_y - 1;
            },
            Facing::Up => {
                x = final_x;
                y = final_y + 1;
            },
            Facing::Right => {
                x = final_x + 1;
                y = final_y;
            },
            Facing::Left => {
                if final_x == 0 {
                    break
                }
                x = final_x - 1;
                y = final_y;
            },
        };

        msg!(&format!("Try moving to {x}, {y}"));

        // Check if movement is valid
        if game.is_cell_valid(x as usize, y as usize) {
            final_x = x;
            final_y = y;
            is_valid = true;
            if game.is_recharge(x as usize, y as usize){
                game.players[player_index].energy = 100;
            }
        }else {
            break;
        }
    }

    if !is_valid {
        return Err(KamikazeJoeError::InvalidMovement.into());
    }

    // Move player
    game.players[player_index].x = final_x;
    game.players[player_index].y = final_y;
    game.players[player_index].facing = direction;

    // Reduce energy
    game.reduce_energy(player_index, energy);

    // Check if game ended
    game.check_if_won(player_index);

    Ok(())
}