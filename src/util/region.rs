/// Represents a rectangular part of the viewport.
#[derive(Clone,Copy,std::fmt::Debug)]
pub struct Region {
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16
}

impl Region {

    /// Creates a new (rectangular) region with the left-bottom corner at (min_x, min_y) and the right-top corner at (max_x, max_y).
    /// 
    /// Coordinates in the range [-10_000, 10_000] will have a place on the viewport. 
    /// The coordinates of the bottom-left corner of the viewport are (-10_000, -10_000) and the coordinates of the top-right
    /// corner of the viewport are (10_000, 10_000). The center of the viewport will be (0, 0).
    /// 
    /// This struct uses integer coordinates rather than floating point numbers because they are exact, which is sometimes needed
    /// in container calculations to determine which components overlap.
    pub fn new(min_x: i16, min_y: i16, max_x: i16, max_y: i16) -> Region {
        Region {
            min_x,
            min_y,
            max_x,
            max_y
        }
    }

    /// Creates a new (rectangular) region with the left-bottom corner at (min_x, min_y) and the right-top corner at (max_x, max_y).
    /// 
    /// Since this function takes floating point numbers as parameters, the resulting region might not be exactly correct. So please use
    /// Region::new instead if you can give exact coordinates. Using this function is fine if the result doesn't need to be exact and if
    /// you can't compute the exact coordinates anyway.
    /// 
    /// This struct uses integer coordinates rather than floating point numbers because they are exact, which is sometimes needed
    /// in container calculations to determine which components overlap.
    pub fn from_floats(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Region {
        Region {
            min_x: from_float(min_x),
            min_y: from_float(min_y),
            max_x: from_float(max_x),
            max_y: from_float(max_y)
        }
    }

    pub fn entire_viewport() -> Region {
        Region {
            min_x: -10_000,
            min_y: -10_000,
            max_x: 10_000,
            max_y: 10_000
        }
    }

    /// Returns true if and only if this region intersects with the given other region.
    pub fn intersects_with(&self, other: &Region) -> bool {
        self.min_x <= other.max_x && self.min_y <= other.max_y && other.min_x <= self.max_x && other.min_y <= self.max_y
    }

    pub fn is_inside(&self, point: (i16,i16)) -> bool {
        point.0 >= self.min_x && point.0 <= self.max_x && point.1 >= self.min_y && point.1 <= self.max_y
    }

    pub fn is_float_inside(&self, point: (f32,f32)) -> bool {
        point.0 >= to_float(self.min_x) && point.0 <= to_float(self.max_x) && point.1 >= to_float(self.min_y) && point.1 <= to_float(self.max_y)
    }

    pub fn is_fully_covered(&self, regions: Vec<Region>) -> bool {
        let mut uncovered_regions = vec![self];
        for region in regions {
            for uncovered_region in &uncovered_regions {
                if region.intersects_with(uncovered_region) {
                    // TODO Finish this stuff
                }
            }
        }

        uncovered_regions.is_empty()
    }

    /// Returns the x-coordinate of the bottom-left corner of this region, in OpenGL coordinates (in the range [-1.0, 1.0]).
    pub fn get_float_min_x(&self) -> f32 {
        to_float(self.min_x)
    }

    /// Returns the y-coordinate of the bottom-left corner of this region, in OpenGL coordinates (in the range [-1.0, 1.0]).
    pub fn get_float_min_y(&self) -> f32 {
        to_float(self.min_y)
    }

    /// Returns the x-coordinate of the top-right corner of this region, in OpenGL coordinates (in the range [-1.0, 1.0]).
    pub fn get_float_max_x(&self) -> f32 {
        to_float(self.max_x)
    }

    /// Returns the y-coordinate of the top-right corner of this region, in OpenGL coordinates (in the range [-1.0, 1.0]).
    pub fn get_float_max_y(&self) -> f32 {
        to_float(self.max_y)
    }

    pub fn get_float_width(&self) -> f32 {
        to_float(self.get_width())
    }

    pub fn get_float_height(&self) -> f32 {
        to_float(self.get_height())
    }

    pub fn get_min_x(&self) -> i16 {
        self.min_x
    }

    pub fn get_min_y(&self) -> i16 {
        self.min_y
    }

    pub fn get_max_x(&self) -> i16 {
        self.max_x
    }

    pub fn get_max_y(&self) -> i16 {
        self.max_y
    }

    pub fn get_width(&self) -> i16 {
        self.max_x - self.min_x + 1
    }

    pub fn get_height(&self) -> i16 {
        self.max_y - self.min_y + 1
    }
}

fn to_float(integer: i16) -> f32 {
    integer as f32 / 10_000.0
}

fn from_float(floating: f32) -> i16 {
    (floating * 10_000.0) as i16
}