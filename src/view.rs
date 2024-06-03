//! Functionalities to represent the current game state.

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const COLOR_DARKGREEN: Color = Color::RGB(10, 80, 25);

/// An object of this type may be drawn onto any SDL surface.
pub trait Draw {
    /// Draw this object onto the window canvas.
    fn draw(&self, canvas: &mut Canvas<Window>);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Ball
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// The game's main protagonist.
pub struct Ball;

impl Draw for Ball {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // TODO: Stub!
        let draw_rect = Rect::new(
            canvas.viewport().w / 2 + 5,
            canvas.viewport().h / 2 + 5,
            10,
            10,
        );

        let color = canvas.draw_color();
        canvas.set_draw_color(COLOR_DARKGREEN);
        canvas.draw_rect(draw_rect).unwrap();
        canvas.set_draw_color(color);
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Board
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// Here all the action takes place.
pub struct Board;

impl Draw for Board {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // TODO: Stub!
        let draw_rect = Rect::new(
            canvas.viewport().x + 2,
            canvas.viewport().y + 2,
            (canvas.viewport().w - 4).try_into().unwrap(),
            (canvas.viewport().h - 4).try_into().unwrap(),
        );

        let color = canvas.draw_color();
        canvas.set_draw_color(COLOR_DARKGREEN);
        canvas.draw_rect(draw_rect).unwrap();
        canvas.set_draw_color(color);
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Paddle
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

/// Represents the antagonists which tries to keep the protagonist imprisoned.
pub struct Paddle;

impl Draw for Paddle {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // TODO: Stub!
        let draw_rect = Rect::new(
            canvas.viewport().x + 20,
            canvas.viewport().h / 2 + 50,
            10,
            50,
        );

        let color = canvas.draw_color();
        canvas.set_draw_color(COLOR_DARKGREEN);
        canvas.draw_rect(draw_rect).unwrap();
        canvas.set_draw_color(color);
    }
}
