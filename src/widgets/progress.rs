use std::rc::Rc;
use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    Event
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    ActionSender
};
use ::widgets::Base;

pub struct Progress<M> {
    base: Rc<Base<Progress<M>, M>>,
    value: i64,
    min: i64,
    max: i64,
    width: usize
}

impl <M> Progress<M> {
    pub fn new() -> Progress<M> {
        Progress {
            base: Base::new(),
            value: 50,
            min: 0,
            max: 100,
            width: 10,
        }
    }

    pub fn set_update_handler<F: Fn(&mut Progress<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn set_min(&mut self, min: i64) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: i64) {
        self.max = max;
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
}

impl <M> Drawable<M> for Progress<M> {
    fn draw_at(&self, rb: &RustBox, _model: &M, x_pos: usize, y_pos: usize, available_width: usize, available_height: usize) {
        //rb.print(0, 0, RB_NORMAL, )
        fn get_sym(n: i64) -> char{
            match n {
                0 => ' ',
                1 => '▏',
                2 => '▎',
                3 => '▍',
                4 => '▌',
                5 => '▋',
                6 => '▊',
                7 => '▉',
                _ => '█',
            }
        };

        let n_subchars = 8.0;

        // normalize
        let max: f64 = (self.max as f64) - (self.min as f64);
        let value: f64 = (self.value as f64) - (self.min as f64);
        let ratio: f64 = value / max;

        let width: f64 = self.width as f64;
        let to_fill = ratio * n_subchars * width;

        let full = (to_fill / n_subchars).floor() as usize;
        let remaining = to_fill % n_subchars;
        let subchar = get_sym(remaining.round() as i64);

        for x_delta in 0 .. full {
            rb.print_char(x_pos + x_delta, y_pos, RB_NORMAL, Color::Default, Color::Default, '█');
        }

        let subchar_x_pos = full;
        if subchar_x_pos <= width as usize {
             rb.print_char(x_pos + subchar_x_pos, y_pos, RB_NORMAL, Color::Default, Color::Default, subchar);
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        // TODO: Have struct contain height
        1
    }
}

impl <M> EventReceiver<M> for Progress<M> {
    fn handle_event(&mut self, _model: &mut M, _event: &Event) -> bool {
        false
    }
}

impl <M> Widget<M> for Progress<M> {
    fn update(&mut self, model: &M) {
        self.base.clone().update(self, model);
    }
}

impl <M> ActionSender<M> for Progress<M> {
    type Action = ();
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }
    fn do_action(&mut self, model: &mut M, action: Self::Action) {
        self.base.do_action(model, action)
    }
}
