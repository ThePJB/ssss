use crate::kmath::*;
use crate::scene::*;
use crate::kinput::*;

pub struct FloatSlider {
    pub curr: f32,
    pub min: f32,
    pub max: f32,
    pub held: bool,
    pub label: String,
}

impl FloatSlider {
    pub fn new(default: f32, min: f32, max: f32, label: String) -> FloatSlider {
        FloatSlider {
            curr: default, 
            min, 
            max,
            held: false,
            label
        }
    }

    pub fn frame(&mut self, inputs: &FrameInputState, outputs: &mut FrameOutputs, r: Rect) -> bool {
        if inputs.lmb == KeyStatus::JustPressed && r.contains(inputs.mouse_pos) {
            self.held = true;
        }
        if self.held {
            // let p = inputs.mouse_pos.transform(inputs.screen_rect, r);
            // let p = r.relative_point(inputs.mouse_pos);
            let slider_t = remap(inputs.mouse_pos.y, r.bot(), r.top(), 0.0, 1.0).max(0.0).min(1.0);
            // let slider_t = 1.0 - p.y.max(0.0).min(1.0);
            self.curr = lerp(self.min, self.max, slider_t);
        }

        // todo scroll mouse for fine adjustment

        // rendering
        let bg = Vec4::new(1.0, 0.0, 0.2, 1.0).hsv_to_rgb();
        let fg = Vec4::new(1.0, 0.0, 0.6, 1.0).hsv_to_rgb();
        let slider_t = unlerp(self.curr, self.min, self.max);
        outputs.canvas.put_rect(r, 1.0, bg);
        let (lr,r) = r.split_ud(0.1);
        let (lr, maxr) = lr.split_ud(0.5);
        outputs.glyphs.push_str(&self.label, lr.left(), lr.top(), lr.h * 0.7/0.8*0.5, lr.h,  1.5, Vec4::new(1.0, 1.0, 1.0, 1.0));
        outputs.glyphs.push_str(&format!("{}", self.max), maxr.left(), maxr.top(), lr.h * 0.7/0.8*0.5, maxr.h,  1.5, Vec4::new(0.5, 0.5, 0.5, 1.0));
        let (r, minr) = r.split_ud(0.95);
        outputs.glyphs.push_str(&format!("{}", self.min), minr.left(), minr.top(), lr.h * 0.7/0.8*0.5, minr.h,  1.5, Vec4::new(0.5, 0.5, 0.5, 1.0));
        outputs.canvas.put_rect(r.fit_aspect_ratio(0.02), 1.1, fg);
        let sx = r.centroid().x;
        let sy = r.top() + (1.0 - slider_t) * r.h;
        let sw = r.w;
        let sh = r.h * 0.1;
        outputs.canvas.put_rect(Rect::new_centered(sx, sy, sw, sh), 1.6, fg);
        outputs.glyphs.push_str(&format!("{:.4}", self.curr), sx - sw/2.0, sy - sh/4.0, sh * 0.7/0.8*0.5 / 2.0, sh / 2.0,  1.7, Vec4::new(0.9, 0.9, 0.9, 1.0));

        if self.held && inputs.lmb == KeyStatus::JustReleased {
            self.held = false;
            return true;
        }
        if self.held && inputs.lmb == KeyStatus::Pressed && inputs.mouse_delta != Vec2::new(0.0, 0.0) {
            return true;
        }
        return false;
    }
}
