macro_rules! into_str {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = String::from("");
            $(
                let s = $x;
                temp_vec.push_str(&s.to_string());
            )*
            temp_vec
        }
    };
}

pub mod svg_methods {
    use crate::svg::{Document, Node};
    use crate::svg::node::element::{Element, Style};
    use crate::paint::color::Color;

    pub fn create_svg_tag(width: f32, height: f32) -> Document {
        let mut svg = Document::new();
        svg.assign("width", into_str![width]);
        svg.assign("height", into_str![height]);
        svg.assign("viewBox", into_str!["0 0 ",width," ", height]);
        svg.assign("xmlns:xlink", "http://www.w3.org/1999/xlink");
        svg
    }

    pub fn create_style_tag(style: &str) -> Style {
        Style::new(style)
    }

    pub fn create_image_tag(href: &str) -> Element {
        let mut image = Element::new("image");
        image.assign("xlink:href", href);
        image
    }

    pub fn create_group() -> Element {
        Element::new("g")
    }

    pub fn group(elements: Vec<Element>) -> Element {
        let mut g = Element::new("g");
        for item in elements {
            g.append(item)
        }
        g
    }

    pub fn create_path_tag(d: &str) -> Element {
        let mut path_tag = Element::new("path");
        path_tag.assign("d", d);
        path_tag.assign("stroke-linecap", "round");
        path_tag.assign("stroke-linejoin", "round");
        path_tag
    }

    pub fn create_rect_tag(width: f32, height: f32, x: f32, y: f32) -> Element {
        let mut rect = Element::new("rect");
        rect.assign("width", into_str![width]);
        rect.assign("height", into_str![height]);
        rect.assign("x", into_str![x]);
        rect.assign("y", into_str![y]);
        rect
    }

    pub fn create_defs_tag() -> Element {
        Element::new("defs")
    }

    pub fn create_filter_tag() -> Element {
        Element::new("filter")
    }

    pub fn create_linear_gradient(vector: (f32, f32), stop: Vec<(String, Color)>, key: String) -> Element {
        let mut linear_gradient = Element::new("linearGradient");
        linear_gradient.assign("id", key);
        linear_gradient.assign("x1", into_str![0]);
        linear_gradient.assign("y1", into_str![0]);
        linear_gradient.assign("x2", into_str![vector.0 * 100.0, "%"]);
        linear_gradient.assign("y2", into_str![vector.1 * 100.0, "%"]);
        for (key, value) in stop {
            let mut stop = Element::new("stop");
            let Color { r, g, b, a } = value;
            stop.assign("offset", key.clone());
            stop.assign("stop-color", into_str![r.to_u8(), g.to_u8(), b.to_u8()]);
            stop.assign("stop-opacity", into_str![a.to_u8()]);
            linear_gradient.append(stop);
        }
        linear_gradient
    }

    pub fn create_use_tag(id: String) -> Element {
        let mut use_tag = Element::new("use");
        use_tag.assign("xlink:href", into_str!["#", id]);
        use_tag
    }

    pub fn create_mask_tag(elements: Vec<Element>, id: String) -> Element {
        let mut mask_tag = Element::new("mask");
        mask_tag.assign("id", id);
        for element in elements {
            mask_tag.append(element);
        }
        mask_tag
    }

    pub fn create_clip_tag(elements: Vec<Element>, id: String) -> Element {
        let mut clip_tag = Element::new("clipPath");
        clip_tag.assign("id", id);
        for element in elements {
            clip_tag.append(element);
        }
        clip_tag
    }

    pub fn apply_shadow(defs: &mut Element, element: &mut Element, color: (u8, u8, u8, f32), offset: (f32, f32), blur: f32) {
        let mut filter = create_filter_tag();
        let (r, g, b, a) = color;
        let (dx, dy) = offset;
        let id = into_str!["shadow-", r.clone(), g.clone(), b.clone(), a.clone(),dx.clone(), dy.clone(), blur.clone()];

        let mut fe_color_matrix = Element::new("feColorMatrix");
        fe_color_matrix.assign("type", "matrix");
        fe_color_matrix.assign("in", "SourceAlpha");
        fe_color_matrix.assign("result", "matrix");
        fe_color_matrix.assign("color-interpolation-filters", "sRGB");
        fe_color_matrix.assign("values", into_str![
            " 0 0 0 0 ",r as f32 / 255.0,
            " 0 0 0 0 ",g as f32 / 255.0,
            " 0 0 0 0 ",b as f32 / 255.0,
            " 0 0 0 ",a," 0"
        ]);

        let mut fe_offset = Element::new("feOffset");
        fe_offset.assign("dx", into_str![dx]);
        fe_offset.assign("dy", into_str![dy]);
        fe_offset.assign("in", "matrix");
        fe_offset.assign("result", "offset");

        let mut fe_gaussian_blur = Element::new("feGaussianBlur");

        fe_gaussian_blur.assign("stdDeviation", into_str![blur]);
        fe_gaussian_blur.assign("in", "offset");
        fe_gaussian_blur.assign("result", "blur");

        let mut fe_merge = Element::new("feMerge");

        let mut fe_merge_node = Element::new("feMergeNode");
        fe_merge_node.assign("in", "blur");
        let mut fe_merge_node1 = Element::new("feMergeNode");
        fe_merge_node1.assign("in", "SourceGraphic");

        fe_merge.append(fe_merge_node);
        fe_merge.append(fe_merge_node1);

        filter.append(fe_color_matrix);
        filter.append(fe_offset);
        filter.append(fe_gaussian_blur);
        filter.append(fe_merge);

        filter.assign("x", "-150%");
        filter.assign("y", "-150%");
        filter.assign("width", "400%");
        filter.assign("height", "400%");
        filter.assign("id", into_str![&id]);

        defs.append(filter);
        element.assign("filter", into_str!["url(#",&id,")"]);
    }
}

use svg_methods::*;
use crate::svg::{Document, Node};
use svg::node::element::Element;
use crate::paint::FillRule;
use crate::backend::PainterBackend;
use crate::f32x2;
use crate::operate::Segment;
use crate::paint::shader::Shader;
use crate::paint::Paint;
use crate::paint::stroke::{lint_cap_to_string, lint_join_to_string, Stroke};
use crate::path::PathData;

pub struct SvgRender {
    svg: Document,
    defs: Element,
    content: Vec<Element>,
    pub width: f32,
    pub height: f32,
    pub view_box: (f32, f32, f32, f32),
    use_count: usize,
}

impl Default for SvgRender {
    fn default() -> Self {
        SvgRender::new(20.0, 20.0)
    }
}

impl SvgRender {
    pub fn new(width: f32, height: f32) -> Self {
        SvgRender {
            svg: create_svg_tag(width, height),
            defs: create_defs_tag(),
            content: vec![],
            width,
            height,
            view_box: (0.0, 0.0, width, height),
            use_count: 0,
        }
    }

    pub fn resize_svg(&mut self, size: f32x2, view_box: Option<(f32, f32, f32, f32)>) {
        self.width = size.x();
        self.height = size.y();
        self.view_box = view_box.unwrap_or((0.0, 0.0, self.width, self.height));
        let (x, y, w, h) = &self.view_box;
        self.svg.assign("width", into_str![&self.width]);
        self.svg.assign("height", into_str![&self.width]);
        self.svg.assign("viewBox", into_str![x, " ", y, " ", w, " ", h]);
    }

    pub fn append_path(&mut self, path_data: &PathData, paint: Option<&Paint>, stroke: Option<&Stroke>, fill_rule: Option<FillRule>, clip: Option<crate::paint::ClipMask>) {
        let mut path_tag = create_path_tag(&String::from(path_data));
        let fill_rule = fill_rule.unwrap_or(FillRule::Nonzero);
        path_tag.assign("fill-rule", String::from(fill_rule));
        let (fill_url, fill_defs) = paint.and_then(|p| Some(SvgRender::build_paint(p))).unwrap_or((String::from("transparent"), None));
        path_tag.assign("fill", fill_url);
        if let Some(node) = fill_defs {
            self.defs.append(node);
        }
        if let Some(stroke) = stroke {
            let (stroke_url, stroke_defs) = SvgRender::build_paint(&stroke.paint);
            path_tag.assign("stroke", stroke_url);
            path_tag.assign("stroke-width", stroke.width);
            path_tag.assign("stroke-linejoin", lint_join_to_string(stroke.line_join));
            path_tag.assign("stroke-linecap", lint_cap_to_string(stroke.line_cap));
            path_tag.assign("stroke-miterlimit", stroke.miter_limit);
            // todo: line-dash-offset
            if let Some(node) = stroke_defs {
                self.defs.append(node);
            }
        }
        if let Some(clip) = clip {
            let mut clip_path = create_path_tag(&String::from(&clip.path));
            clip_path.assign("fill-rule", String::from(clip.fill_rule.clone()));
            let id = self.create_use_id();
            let clip_tag = create_clip_tag(vec![clip_path], id.clone());
            path_tag.assign("clip-path", into_str!["url(#", id, ")"]);
            self.defs.append(clip_tag);
        }
        self.content.push(path_tag);
    }

    pub fn append_image(&mut self, image_url: &str) {
        create_image_tag(image_url);
    }

    fn create_use_id(&mut self) -> String {
        self.use_count += 1;
        format!("use_id_{}", self.use_count)
    }

    fn build_paint(paint: &Paint) -> (String, Option<Element>) {
        match paint.shader {
            Shader::SolidColor(color) => {
                let color = color.to_color_u8();
                let r = color.red();
                let g = color.green();
                let b = color.blue();
                let a = color.alpha();
                (format!("rgba({}, {}, {}, {})", r, g, b, a as f32 / 256.0), None)
            }
            Shader::LinearGradient(_) => { todo!() }
            Shader::RadialGradient(_) => { todo!() }
            Shader::Pattern => { todo!() }
        }
    }
}

impl PainterBackend for SvgRender {
    fn resize(&mut self, size: f32x2) {
        todo!()
    }

    fn draw(&mut self, segment: &Segment) {
        match segment {
            Segment::Pixel(ref seg) => {
                todo!()
                // self.append_image()
            }
            Segment::Vector(ref seg) => {
                seg.fill.as_ref().and_then(|paint| Some(self.append_path(&seg.path, Some(paint), None, Some(seg.fill_rule), seg.clip.clone())));
                seg.stroke.as_ref().and_then(|stroke| Some(self.append_path(&seg.path, None, Some(stroke), Some(seg.fill_rule), seg.clip.clone())))
            }
        };
    }

    fn finish(&mut self) -> Vec<u8> {
        let mut svg = self.svg.clone();
        svg.add(self.defs.clone())
            .add(group(self.content.clone()))
            .to_string().as_bytes().to_vec()
    }
}

#[test]
fn test() {
    let mut svg = SvgRender::default();
    svg.resize_svg(f32x2([40.0, 40.0]), Some((20.0, 20.0, 60.0, 60.0)));
    let result = svg.svg.to_string();
    println!("{}", result)
}