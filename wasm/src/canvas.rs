use crate::web_sys::{HtmlCanvasElement, HtmlElement, HtmlImageElement, ImageData};
use crate::wasm_bindgen::prelude::{wasm_bindgen};
use crate::wasm_bindgen::{JsValue, JsCast, Clamped};
use crate::base64;

use crate::painter_core::backend::skia_cpu::{SkiaCPURender, ImageDataRender};
use crate::painter_core::backend::svg::SvgRender;
use crate::painter_core::context2d::Context;
use crate::painter_font::FontDB;

use crate::context::Context2d;

enum Backend {
    Pix,
    Png,
    Svg,
}

pub(crate) enum RenderTarget {
    Div(HtmlElement),
    Canvas(HtmlCanvasElement),
    Img(HtmlImageElement),
}

impl RenderTarget {
    pub(crate) fn render_target(&self, data: Vec<u8>, width: u32, height: u32) -> Option<()> {
        match self {
            RenderTarget::Div(div) => {
                let svg_str = String::from_utf8(data).ok()?;
                div.set_inner_html(&svg_str);
            }
            RenderTarget::Canvas(canvas) => {
                let ctx = canvas.get_context("2d").ok()??.dyn_into::<web_sys::CanvasRenderingContext2d>().ok()?;
                let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(data.as_slice()), width, height).ok()?;
                ctx.put_image_data(&image_data, 0.0, 0.0).ok()?;
            }
            RenderTarget::Img(img) => {
                let data_url = base64::encode(data);
                img.set_attribute("src", format!("data:image/png;base64,{}", data_url.as_str()).as_str()).ok()?;
                img.set_attribute("width", format!("{}", width).as_str()).ok()?;
                img.set_attribute("height", format!("{}", height).as_str()).ok()?;
            }
        }
        Some(())
    }
}

#[wasm_bindgen]
pub struct Canvas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) ctx: Context<'static>,
    pub(crate) backend: Backend,
    pub(crate) render_target: Option<RenderTarget>,
    pub(crate) font_db: FontDB,
    hight_quality: bool,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, backend: JsValue) -> Self {
        let mut ctx = Context::new_wh(width as f32, height as f32);
        let backend = backend.as_string().unwrap_or(String::from("pix"));
        let backend = match backend.as_str() {
            "pix" => Backend::Pix,
            "svg" => Backend::Svg,
            _ => Backend::Png
        };
        let mut font_db = FontDB::new();

        Canvas {
            width,
            height,
            ctx,
            backend,
            render_target: None,
            font_db,
            hight_quality: false,
        }
    }

    #[wasm_bindgen(js_name = forceHightQuality)]
    pub fn force_hq(&mut self) {
        self.hight_quality = true
    }

    pub fn bind(&mut self, node_selector: &str) -> Option<usize> {
        let document = web_sys::window()?.document()?;
        let element = document.query_selector(node_selector).ok()??;
        self.render_target = match element.tag_name().as_str() {
            "CANVAS" => {
                let canvas = element.dyn_into::<HtmlCanvasElement>()
                    .map_err(|_| ()).ok()?;
                self.backend = Backend::Pix;
                Some(RenderTarget::Canvas(canvas))
            }
            "IMG" => {
                let img = element.dyn_into::<HtmlImageElement>()
                    .map_err(|_| ()).ok()?;
                self.backend = Backend::Png;
                Some(RenderTarget::Img(img))
            }
            "DIV" => {
                let div = element.dyn_into::<HtmlElement>()
                    .map_err(|_| ()).ok()?;
                self.backend = Backend::Svg;
                Some(RenderTarget::Div(div))
            }
            _ => {
                None
            }
        };
        Some(0)
    }

    #[wasm_bindgen(js_name = loadFont)]
    pub fn load_font(&mut self, buf: Vec<u8>) -> Option<usize> {
        self.font_db.load_font(buf.as_slice())?;
        Some(0)
    }

    pub fn render(&self) -> Option<usize> {
        self.render_target.as_ref()?.render_target(self.to_vec(), self.width, self.height)?;
        Some(0)
    }

    #[wasm_bindgen(js_name = getContext2d)]
    pub fn get_context_2d(self) -> Context2d {
        Context2d::new(self)
    }

    #[wasm_bindgen(js_name = toArrayBuffer)]
    pub fn to_vec(&self) -> Vec<u8> {
        match self.backend {
            Backend::Pix => {
                let mut r = SkiaCPURender::new(self.width, self.height);
                if self.hight_quality { r.force_hq(); }

                let renderer = ImageDataRender(r);
                self.ctx.render(Box::new(renderer))
            }
            Backend::Svg => {
                let renderer = SvgRender::new(self.width as f32, self.height as f32);
                self.ctx.render(Box::new(renderer))
            }
            Backend::Png => {
                let mut renderer = SkiaCPURender::new(self.width, self.height);
                if self.hight_quality { renderer.force_hq(); }
                self.ctx.render(Box::new(renderer))
            }
        }
    }
}