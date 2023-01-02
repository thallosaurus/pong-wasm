use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

impl Rect {
    pub fn contains(&self, other: Self) -> bool {
        self.x < other.x && self.x + self.w > other.x + other.w
            && self.y < other.y && self.y + self.h > other.y + other.h
    }
}


mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::ui::Rect;

    #[cfg(test)]
    fn configure() {
        use wasm_bindgen_test::wasm_bindgen_test_configure;

        wasm_bindgen_test_configure!(run_in_browser);
    }

    #[wasm_bindgen_test]
    fn rect1_contains_rect2() {
        let rect1 = Rect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_doesnt_contain_rect2() {
        let rect1 = Rect { x: 7.0, y: 7.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(!rect1.contains(rect2));
    }

    #[wasm_bindgen_test]
    fn rect1_cuts_rect2() {
        let rect1 = Rect { x: 5.0, y: 5.0, w: 10.0, h: 10.0 };
        let rect2 = Rect { x: 2.0, y: 2.0, w: 5.0, h: 5.0 };
        
        assert!(!rect1.contains(rect2));
    }
}