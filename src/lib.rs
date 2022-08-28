use std::fmt::{self, Error, Formatter};

use ldrawy::*;

#[derive(Debug)]
pub enum ScreenVal {
    Pixel(u32),
    Percent(f32),
}
impl Default for ScreenVal {
    fn default() -> Self {
        ScreenVal::Percent(100.0)
    }
}
#[macro_export]
macro_rules! sval {
    ($val:expr) => {
        ScreenVal::Pixel($val)
    };

    (% $val:expr) => {
        ScreenVal::Percent($val)
    };
}

#[derive(Debug)]
pub struct Position {
    x: ScreenVal,
    y: ScreenVal,
}
impl Default for Position {
    fn default() -> Self {
        Self::ZERO
    }
}
impl Position {
    pub fn new(x: ScreenVal, y: ScreenVal) -> Self {
        Self { x, y }
    }
    fn to_px(&self, parent: &Rect) -> Vec2 {
        let mut v = Vec2::new(0., 0.);
        v.x = match self.x {
            ScreenVal::Pixel(val) => val as f32 + parent.pos.x,
            ScreenVal::Percent(val) => parent.pos.x + parent.size.x * val / 100.,
        };
        v.y = match self.y {
            ScreenVal::Pixel(val) => val as f32 + parent.pos.y,
            ScreenVal::Percent(val) => parent.pos.y + parent.size.y * val / 100.,
        };
        v
    }

    pub const ZERO: Position = Position {
        x: sval!(0),
        y: sval!(0),
    };
}

#[derive(Default, Debug)]
pub struct Size {
    pub width: ScreenVal,
    pub height: ScreenVal,
}
impl Size {
    pub fn new(width: ScreenVal, height: ScreenVal) -> Self {
        Self { width, height }
    }
    fn to_px(&self, parent: &Rect) -> Vec2 {
        let mut v = Vec2::new(0., 0.);
        v.x = match self.width {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.x / 100.,
        };
        v.y = match self.height {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.y / 100.,
        };
        v
    }
}

#[derive(Default, Debug)]
pub struct Margin {
    top: ScreenVal,
    right: ScreenVal,
    bottom: ScreenVal,
    left: ScreenVal,
}
impl Margin {
    fn to_rect(&self, parent: &Rect) -> Rect {
        let left = match self.left {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.x / 100.,
        };
        let bottom = match self.bottom {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.y / 100.,
        };
        let right = match self.right {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.x / 100.,
        };
        let top = match self.top {
            ScreenVal::Pixel(val) => val as f32,
            ScreenVal::Percent(val) => val as f32 * parent.size.y / 100.,
        };
        Rect::new(
            Vec2::new(parent.pos.x + left, parent.pos.y + bottom),
            Vec2::new(parent.size.x - left - right, parent.size.y - top - bottom),
        )
    }
}

#[derive(Debug)]
pub enum Transform {
    Relative { position: Position, size: Size },
    Margin(Margin),
}
impl Default for Transform {
    fn default() -> Self {
        Transform::Relative {
            position: Position::default(),
            size: Size::default(),
        }
    }
}
impl Transform {
    fn get_rect(&self, parent: &Rect) -> Rect {
        match self {
            Transform::Relative { position, size } => {
                Rect::new(position.to_px(parent), size.to_px(parent))
            }
            Transform::Margin(margin) => margin.to_rect(parent),
        }
    }
}

pub trait Component {
    fn process(&mut self) {}
}

#[derive(Default)]
pub struct View {
    pub id: String,
    children: Vec<View>,
    components: Vec<Box<dyn Component + 'static>>,
    pub color: Color,
    pub transform: Transform,
}
impl fmt::Debug for View {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "View {{ id: {}, children: {:?}, components: {:?}, color: {:?}, transform: {:?} }}",
            self.id,
            self.children,
            self.components.len(),
            self.color,
            self.transform
        )
    }
}

impl<'a> View {
    pub fn new() -> Self {
        View {
            id: String::new(),
            children: Vec::new(),
            components: Vec::new(),
            color: Color::default(),
            transform: Transform::default(),
        }
    }
    pub fn child<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut View),
    {
        let mut view = View::new();
        f(&mut view);
        self.children.push(view);
        self
    }
    pub fn component<C, F>(&mut self, f: F) -> &mut Self
    where
        C: Component + Default + 'static,
        F: FnOnce(&mut C),
    {
        let mut comp = C::default();
        f(&mut comp);
        self.components.push(Box::new(comp));
        self
    }
    pub fn process_batch(&mut self, batch: &mut ShapeBatch, parent: &Rect) {
        let rect = self.transform.get_rect(parent);
        batch.add_rect(&rect, self.color);
        for child in &mut self.children {
            child.process_batch(batch, &rect);
        }
    }
}

#[derive(Default)]
pub struct Button {
    pub on_press: Option<Box<dyn FnMut()>>,
}
impl Component for Button {
    fn process(&mut self) {}
}
#[derive(Default)]
pub struct Text {
    pub text: String,
}
impl Component for Text {
    fn process(&mut self) {}
}

#[derive(Default)]
pub struct Window {
    root: View,
}

#[derive(Default)]
struct InternalEngine {
    main: Window,
}

impl InternalEngine {
    pub fn run(main: Window) -> ! {
        let engine = InternalEngine { main: main };
        ldrawy::Window::create_and_run(WindowSettings::new(60), engine)
    }
}

impl UserWindowHandler for InternalEngine {
    fn process_render(&mut self, wnd: &ldrawy::Window) -> Result<(), LErr> {
        if wnd.frame_count() % 300 == 0 {
            println!("{:#?}", self.main.root);
        }

        let mut canvas = wnd.start_frame(Color::WHITE);
        let mut batch = ShapeBatch::default();
        let mut brush =
            Brush::from_source(wnd, UNLIT_VERT.to_string(), UNLIT_FRAG.to_string(), None);
        brush
            .update_uniform(Uniform::from_str(
                "wnd",
                UniformValue::Vec2([canvas.get_size().x as f32, canvas.get_size().y as f32]),
            ))
            .expect("Error updated uniform.");
        self.main.root.process_batch(&mut batch, &canvas.get_rect());
        canvas.draw_batch(wnd, &brush, batch.bake_buffers(wnd), &DrawParams::default());
        canvas.finish_canvas()?;
        Ok(())
    }
    fn process_logic(&mut self, wnd: &ldrawy::Window) -> Result<(), LErr> {
        //TODO: Create context based on window inputs and send over to process it.
        Ok(())
    }
}

pub trait VStructure {
    fn root(&self) -> View;
}
pub trait VDesign {
    fn design(&self) -> DesignQuery;
}
pub struct DesignQuery;

pub fn render_window(view: impl VStructure) -> ! {
    let root = view.root();
    let wnd = crate::Window { root: root };
    InternalEngine::run(wnd);
}

const UNLIT_FRAG: &str = r#"
        #version 330 core
        in vec4 frag_color;
        in vec2 frag_uv;

        uniform sampler2D main_tex;
        
        out vec4 out_color;
        
        void main(){
            out_color=vec4(frag_color);
        }"#;

const UNLIT_VERT: &str = r#"
        #version 330 core
        in vec3 pos;
        in vec4 color;
        in vec2 uv;

        uniform vec2 wnd;

        out vec4 frag_color;
        out vec2 frag_uv;

        void main(){
            frag_uv=uv;
            frag_color=color;
            gl_Position=vec4(((pos.x/wnd.x) - 0.5) * 2.0, ((pos.y/wnd.y) - 0.5) * 2.0, pos.z,1.);
        }"#;
