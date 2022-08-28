use ldrawy::*;
use LView::*;

fn main() -> Result<(), LErr> {
    LView::render_window(StartPlay)
}

struct StartPlay;
// impl VDesign for StartPlay {
//     fn design(&self) -> DesignQuery {
//         let design = DesignQuery::new();
//         desing.all().foreach(|v| v.transform = relative(90., 90.));
//         design.with_id("txt-button").foreach(|v| {
//             v.color = Color::new(0, 0, 0, 255);
//             v.bold = true;
//         });
//     }
// }
impl VStructure for StartPlay {
    fn root(&self) -> View {
        let mut root = View::new();
        root.transform = Transform::Relative {
            position: Position::ZERO,
            size: Size::new(sval!(% 90.), sval!(% 90.)),
        };
        root.color = Color::BLUE;
        root.child(|vbutton| {
            vbutton.color = Color::GREEN;
            vbutton.transform = Transform::Relative {
                position: Position::ZERO,
                size: Size::new(sval!(% 50.), sval!(% 10.)),
            };
            vbutton.component(|button: &mut Button| {
                button.on_press = Some(Box::new(StartPlay::print_play));
            });
            vbutton.child(|vtext| {
                vtext.id = "txt-button".to_string();
                vtext.transform = Transform::Relative {
                    position: Position::ZERO,
                    size: Size::new(sval!(% 80.), sval!(% 80.)),
                };
                vtext.component(|text: &mut Text| {
                    text.text = "Play!".to_string();
                });
            });
        });
        root
    }
}
impl StartPlay {
    fn print_play() {
        println!("Play!");
    }
}
