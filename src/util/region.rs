/// Represents a rectangular part of the viewport.
#[derive(Clone,Copy,std::fmt::Debug,PartialEq,Eq)]
pub struct Region {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32
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
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Region {
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

    /// Constructs a Region spanning the entire (initial) viewport. 
    /// As soon as the user starts scrolling, it will no longer cover the entire viewport.
    pub fn entire_viewport() -> Region {
        Region {
            min_x: -10_000,
            min_y: -10_000,
            max_x: 10_000,
            max_y: 10_000
        }
    }

    /// Returns true if and only if this region intersects with the given other region.
    pub fn intersects_with(&self, other: Region) -> bool {
        self.min_x <= other.max_x && self.min_y <= other.max_y && other.min_x <= self.max_x && other.min_y <= self.max_y
    }

    /// Returns true if (and only if) this region is entirely covered by the given 'cover' region.
    /// With 'entirely covered', I mean that any point that any point inside this region is also inside 'cover'.
    pub fn is_covered_by(&self, cover: Region) -> bool {
        self.min_x >= cover.min_x && self.min_y >= cover.min_y && self.max_x <= cover.max_x && self.max_y <= cover.max_y
    }

    /// Checks if the given point (x, y) is inside or on the border of this region.
    /// That is, when min_x <= point.0 <= max_x and min_y <= point.1 <= max_y
    pub fn is_inside(&self, point: (i32,i32)) -> bool {
        point.0 >= self.min_x && point.0 <= self.max_x && point.1 >= self.min_y && point.1 <= self.max_y
    }

    /// Checks if the given point (x, y) is inside or on the border of this region.
    /// Note that this method may give incorrect results near the border of this region due to the inexact nature of floating point numbers.
    /// That's why the is_inside method is preferred if exact coordinates are available.
    pub fn is_float_inside(&self, point: (f32,f32)) -> bool {
        point.0 >= to_float(self.min_x) && point.0 <= to_float(self.max_x) && point.1 >= to_float(self.min_y) && point.1 <= to_float(self.max_y)
    }

    /// Gets a Vec of Region's that are covered by this Region, but not by any of the Region's in regions (the parameter) 
    /// or by any of the other Region's in the result.
    pub fn get_uncovered_regions(&self, regions: &Vec<Region>) -> Vec<Region> {
        let mut uncovered_regions = vec![*self];
        let mut uncovered_to_add = Vec::new();
        for region in regions {
            
            uncovered_regions.drain_filter(|uncovered| {
                if uncovered.is_covered_by(*region) {
                    // The current region is entirely covered by the region, so it must be removed completely
                    true
                } else if uncovered.intersects_with(*region) {
                    // This is the most complex scenario because the regions partially overlap
                    // We will split it into at most 4 regions: above, below, left and right

                    // The left region
                    if uncovered.min_x < region.min_x {
                        uncovered_to_add.push(Region {
                            min_x: uncovered.min_x,
                            max_x: region.min_x - 1,
                            min_y: uncovered.min_y,
                            max_y: uncovered.max_y
                        });
                    }

                    // The right region
                    if uncovered.max_x > region.max_x {
                        uncovered_to_add.push(Region {
                            min_x: region.max_x + 1,
                            max_x: uncovered.max_x,
                            min_y: uncovered.min_y,
                            max_y: uncovered.max_y
                        });
                    }

                    // The below region
                    if uncovered.min_y < region.min_y {
                        uncovered_to_add.push(Region {
                            min_x: i32::max(region.min_x, uncovered.min_x),
                            max_x: i32::min(region.max_x, uncovered.max_x),
                            min_y: uncovered.min_y,
                            max_y: region.min_y - 1
                        });
                    }

                    // The above region
                    if uncovered.max_y > region.max_y {
                        uncovered_to_add.push(Region {
                            min_x: i32::max(region.min_x, uncovered.min_x),
                            max_x: i32::min(region.max_x, uncovered.max_x),
                            min_y: region.max_y + 1,
                            max_y: uncovered.max_y
                        });
                    }

                    // Finally remove the original region
                    true
                } else {
                    // The regions don't share any space, so just continue normally
                    false
                }
            });

            uncovered_regions.append(&mut uncovered_to_add);
        }

        uncovered_regions
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

    pub fn get_min_x(&self) -> i32 {
        self.min_x
    }

    pub fn get_min_y(&self) -> i32 {
        self.min_y
    }

    pub fn get_max_x(&self) -> i32 {
        self.max_x
    }

    pub fn get_max_y(&self) -> i32 {
        self.max_y
    }

    pub fn get_width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    pub fn get_height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }
}

fn to_float(integer: i32) -> f32 {
    integer as f32 / 10_000.0
}

fn from_float(floating: f32) -> i32 {
    (floating * 10_000.0) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_float(){
        assert_eq!(-1.0, to_float(-10_000));
        assert_eq!(-0.5, to_float(-5_000));
        assert_eq!(0.0, to_float(0));
        assert_eq!(0.5, to_float(5_000));
        assert_eq!(1.0, to_float(10_000));
    }

    #[test]
    fn test_float_conversions(){
        test_float_conversion(0);
        test_float_conversion(-1);
        test_float_conversion(1);
        test_float_conversion(1234);
    }

    // Note that this only holds for small values
    fn test_float_conversion(value: i32){
        assert_eq!(value, from_float(to_float(value)));
        assert_eq!(to_float(value), to_float(from_float(to_float(value))));
    }

    #[test]
    fn test_intersects_with(){
        assert!(Region::new(0, 0, 0, 0).intersects_with(Region::new(0, 0, 0, 0)));
        assert!(Region::new(0, 0, 10, 10).intersects_with(Region::new(10, 10, 20, 20)));
        assert!(!Region::new(0, 0, 10, 10).intersects_with(Region::new(11, 10, 20, 20)));
        assert!(Region::new(-5, -20, 10, -10).intersects_with(Region::new(3, -15, 20, -11)));
        assert!(!Region::new(20, 10, 60, 100).intersects_with(Region::new(80, -20, 100, 50)));
    }

    #[test]
    fn test_is_covered_by(){
        assert!(Region::new(0, 0, 0, 0).is_covered_by(Region::new(0, 0, 0, 0)));
        assert!(Region::new(20, 50, 30, 70).is_covered_by(Region::new(10, 40, 40, 80)));
        assert!(!Region::new(20, 50, 30, 70).is_covered_by(Region::new(21, 40, 40, 80)));
        assert!(!Region::new(20, 80, 30, 90).is_covered_by(Region::new(10, 0, 15, 30)));
    }

    #[test]
    fn test_is_inside(){
        assert!(Region::new(-10, -10, 10, 10).is_inside((0, 0)));
        assert!(Region::new(-10, -10, 10, 10).is_inside((10, -10)));
        assert!(!Region::new(-10, -10, 10, 10).is_inside((11, -10)));
        assert!(Region::new(20, 50, 30, 80).is_inside((22, 70)));
        assert!(!Region::new(20, 50, 30, 80).is_inside((19, 70)));
    }

    #[test]
    fn test_get_uncovered_regions(){
        {
            let region = Region::new(10, 0, 40, 50);

            // Edge cases
            assert_eq!(vec![region], region.get_uncovered_regions(&Vec::new()));
            assert_eq!(Vec::<Region>::new(), region.get_uncovered_regions(&vec![region]));

            // 1 partially overlapping region
            assert_eq!(vec![Region::new(20, 0, 40, 50)], region.get_uncovered_regions(&vec![Region::new(10, 0, 19, 50)]));
            // Now the overlapping region is bigger
            assert_eq!(vec![Region::new(20, 0, 40, 50)], region.get_uncovered_regions(&vec![Region::new(-10, -10, 19, 65)]));

            // Now there is a partially overlapping region on each side
            assert_eq!(vec![Region::new(20, 10, 30, 40)], region.get_uncovered_regions(&vec![
                Region::new(-10, -20, 19, 80), Region::new(31, -10, 60, 70), Region::new(-10, 41, 100, 60), Region::new(0, -70, 50, 9)]));

            // Now a bar region test
            assert!(set_comparison(vec![Region::new(10, 0, 14, 50), Region::new(21, 0, 24, 50), Region::new(31, 0, 34, 50)], 
                region.get_uncovered_regions(&vec![Region::new(15, 0, 20, 50), Region::new(25, 0, 30, 50), Region::new(35, 0, 40, 50)])));

            // Now just a single region in the middle
            assert!(set_comparison(vec![Region::new(10, 0, 19, 50), Region::new(31, 0, 40, 50), Region::new(20, 0, 30, 9), Region::new(20, 41, 30, 50)], 
                region.get_uncovered_regions(&vec![Region::new(20, 10, 30, 40)])));
        }
    }

    fn odd_set_comparison(a: &Vec<Region>, b: &Vec<Region>) -> bool {

        'outer_loop:
        for region_a in a {
            for region_b in b {
                if region_a == region_b {
                    continue 'outer_loop;
                }
            }
            return false;
        }

        true
    }

    fn set_comparison(a: Vec<Region>, b: Vec<Region>) -> bool {
        odd_set_comparison(&a, &b) && odd_set_comparison(&b, &a)
    }
}