use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = vec2(20.0, 100.0);

enum PlayerSide {
    Left,
    Right,
}

struct Player {
    side: PlayerSide,
    position: Vec2,
    speed: f32,
    points: i32,
}

impl Player {
    pub fn new(player_side: PlayerSide) -> Self {
        match player_side {
            PlayerSide::Left => Self {
                side: player_side,
                position: vec2(0.0, screen_height() / 2.0),
                speed: 300.0,
                points: 0,
            },
            PlayerSide::Right => Self {
                side: player_side,
                position: vec2(screen_width() - PLAYER_SIZE.x, screen_height() / 2.0),
                speed: 300.0,
                points: 0,
            },
        }
    }

    pub fn reset(&mut self) {
        self.position.y = screen_height() / 2.0;
    }

    pub fn update(&mut self) {
        match self.side {
            PlayerSide::Right => {
                if is_key_down(KeyCode::Up) && self.position.y > 0.0 + PLAYER_SIZE.y / 2.0 {
                    self.position.y -= self.speed * get_frame_time();
                }

                if is_key_down(KeyCode::Down)
                    && self.position.y < screen_height() - PLAYER_SIZE.y / 2.0
                {
                    self.position.y += self.speed * get_frame_time();
                }
            }
            PlayerSide::Left => {
                if is_key_down(KeyCode::W) && self.position.y > 0.0 + PLAYER_SIZE.y / 2.0 {
                    self.position.y -= self.speed * get_frame_time();
                }

                if is_key_down(KeyCode::S)
                    && self.position.y < screen_height() - PLAYER_SIZE.y / 2.0
                {
                    self.position.y += self.speed * get_frame_time();
                }
            }
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(
            self.position.x,
            self.position.y - 100.0 / 2.0,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y,
            WHITE,
        );
    }
}

struct Ball {
    position: Vec2,
    speed: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: vec2(300.0, 300.0),
            radius: 15.0,
        }
    }

    pub fn reset(&mut self) {
        self.position.x = screen_width() / 2.0;
        self.position.y = screen_height() / 2.0;
    }

    pub fn update(&mut self, player1: &Player, player2: &Player) {
        self.position += self.speed * get_frame_time();

        if self.position.y <= 0.0 || self.position.y >= screen_height() - self.radius {
            self.speed.y = -self.speed.y;
        }

        let player1_rect = Rect::new(
            player1.position.x,
            player1.position.y - PLAYER_SIZE.y / 2.0,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y,
        );

        let player2_rect = Rect::new(
            player2.position.x,
            player2.position.y - PLAYER_SIZE.y / 2.0,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y,
        );

        let ball_rect = Rect::new(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        );

        if ball_rect.overlaps(&player1_rect) || ball_rect.overlaps(&player2_rect) {
            self.speed.x = -self.speed.x;
        }
    }

    pub fn draw(&mut self) {
        draw_circle(self.position.x, self.position.y, 15.0, BLUE)
    }
}

#[macroquad::main("PongGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text(
            "Press SPACE to start the game",
            screen_width() / 2. - 100.,
            screen_height() / 2.,
            20.,
            WHITE,
        );

        if is_key_down(KeyCode::Space) {
            game_loop().await
        }

        next_frame().await
    }
}

async fn game_loop() {
    let mut first_player = Player::new(PlayerSide::Left);
    let mut second_player = Player::new(PlayerSide::Right);
    let mut ball = Ball::new();
    let mut is_game_over = false;
    let mut winner: String = String::new();
    let mut score = vec2(0., 0.);

    loop {
        if !is_game_over {
            draw_text(
                &format!("{}:{}", score.x, score.y).to_string(),
                screen_width() / 2.0,
                20.0,
                20.,
                WHITE,
            );

            first_player.update();
            second_player.update();
            ball.update(&first_player, &second_player);

            first_player.draw();
            second_player.draw();
            ball.draw();

            if ball.position.x < first_player.position.x {
                is_game_over = true;
                winner = "Second Player is the Winner".to_string();
                score.y += 1.0;
            }

            if ball.position.x > second_player.position.x {
                is_game_over = true;
                winner = "First Player is the Winner".to_string();
                score.x += 1.0;
            }

            next_frame().await
        }

        if is_game_over {
            draw_text(
                &winner,
                screen_width() / 2.0 - 100.,
                screen_height() / 2.0,
                20.,
                WHITE,
            );
            draw_text(
                "Press Space to Play Again",
                screen_width() / 2.0 - 100.,
                screen_height() / 2.0 + 20.0,
                20.,
                WHITE,
            );

            if is_key_pressed(KeyCode::Space) {
                is_game_over = false;
                ball.reset();
                first_player.reset();
                second_player.reset();
            }

            next_frame().await
        };
    }
}
