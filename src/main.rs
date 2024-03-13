use farkle::*;
use macroquad::prelude::*;

fn create_game() -> FarkleGame {
    let dice_set = vec![Dice::default(); 6];

    let players = vec![
        Player::new("Player 1", dice_set.clone()).unwrap(),
        Player::new("Player 2", dice_set).unwrap(),
    ];

    FarkleGame::new(players).unwrap()
}

#[macroquad::main("Farkle")]
async fn main() {
    let mut game = create_game();
    let mut selected_dice: Vec<usize> = vec![];

    loop {
        clear_background(WHITE);

        draw_state(&mut game, &mut selected_dice);

        next_frame().await
    }
}

fn draw_state(game: &mut FarkleGame, selected_dice: &mut Vec<usize>) {
    draw_text(
        &format!("Current turn: {}", game.get_current_player().id()),
        10.0,
        50.0,
        30.0,
        BLACK,
    );

    if draw_wait_for_roll(game) {
        return;
    }
    draw_combination_selector(game, selected_dice);
}

fn draw_wait_for_roll(game: &mut FarkleGame) -> bool {
    if !game.is_waiting_for_roll() {
        return false;
    }

    draw_text("Press SPACE to roll", 10.0, 100.0, 30.0, BLACK);

    if is_key_released(KeyCode::Space) {
        game.roll().unwrap();
    }

    true
}

const KEY_CODES: [KeyCode; 6] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
];

fn draw_combination_selector(game: &mut FarkleGame, selected_dice: &mut Vec<usize>) -> bool {
    let Some(roll_result) = game.get_last_roll_result() else {
        return false;
    };

    draw_text("Press number to select dice", 10.0, 100.0, 30.0, BLACK);

    let can_use_selected = game.try_use_dice(selected_dice.clone()).is_ok();

    if can_use_selected {
        draw_text("Press SPACE to use selected dice", 10.0, 150.0, 30.0, BLACK);
    }

    for (i, result) in roll_result.iter().enumerate() {
        let x = 10.0 + (i as f32 * 60.0);
        let y = 200.0;

        draw_dice(*result, x, y, selected_dice.contains(&i));
    }

    if can_use_selected && is_key_released(KeyCode::Space) {
        game.use_dice(selected_dice.clone()).unwrap();
        selected_dice.clear();
        return true;
    }

    for (i, key_code) in KEY_CODES[0..roll_result.len()].iter().enumerate() {
        if is_key_released(*key_code) {
            if selected_dice.contains(&i) {
                selected_dice.retain(|&x| x != i);
            } else {
                selected_dice.push(i);
            }
        }
    }

    true
}

fn draw_dice(result: RollResult, x: f32, y: f32, selected: bool) {
    let size = 50.0;
    let color = if selected { RED } else { BLACK };

    draw_rectangle(x, y, size, size, color);
    draw_text(&format!("{}", *result), x + 20.0, y + 30.0, 30.0, WHITE);
}
