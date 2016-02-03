use graphics::types::Rectangle;

pub struct Point(pub f64, pub f64);

pub struct RenderInfo {
    pub offset: Point,
    pub size: Point,
    pub rotation: f64,
}

impl RenderInfo {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> RenderInfo {
        RenderInfo {
            offset: Point(x, y),
            size: Point(w, h),
            rotation: 0.0,
        }
    }

    pub fn set_rotate(&mut self, rot: f64) {
        self.rotation = rot;
    }

    pub fn get_rectangle_info(&self) -> Rectangle {
        [self.offset.0, self.offset.1, self.size.0, self.size.1]
    }
}

pub struct Player {
    renInfo: RenderInfo,
    life: u32,
    point: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            renInfo: RenderInfo::new(25.0, 50.0, 50.0, 100.0),
            life: 3,
            point: 0,
        }
    }

    pub fn update_by_resize(&mut self, width: f64, height: f64) {

    }

    pub fn is_game_over(&self) -> bool {
        self.life <= 0
    }

    pub fn add_point(&mut self, add: u64) {
        self.point += add;
    }
}

pub enum LandType {
    default_,
    small_,
    tall_,
}

impl LandType {
    pub fn get_height(&self) -> f64 {
        match *self {
            LandType::default_  => 50.0,
            LandType::small_    => 100.0,
            LandType::tall_     => 200.0,
        }
    }
}

pub struct Land {
    renInfo: RenderInfo,
    landType: LandType,
}

impl Land {
    pub fn new(x: f64, h: f64, lType: LandType) -> Land {
        let height = lType.get_height();
        Land {
            renInfo: RenderInfo::new(x, h - height, 50.0, height),
            landType: lType,
        }
    }
}

pub struct PointObj {
    renInfo: RenderInfo,
}

impl PointObj {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> PointObj {
        PointObj {
            renInfo: RenderInfo::new(x, y, w, h),
        }
    }
}

pub trait RenderUtils<'util> {
    fn get_render_info(&'util self) -> &'util RenderInfo;
    // return offset + size x
    fn update_render_info(&'util mut self, args: f64) -> f64;
}

impl <'a> RenderUtils<'a> for Player {
    fn get_render_info(&'a self) -> &'a RenderInfo {
        &self.renInfo
    }

    fn update_render_info(&'a mut self, args: f64) -> f64 {
        println!("Player Update!");
        0.0
    }
}

impl <'a> RenderUtils<'a> for Land {
    fn get_render_info(&'a self) -> &'a RenderInfo {
        &self.renInfo
    }

    fn update_render_info(&'a mut self, args: f64) -> f64 {
        self.renInfo.offset.0 -= args; 
        self.renInfo.offset.0 + self.renInfo.size.0 
    }
}

// Render Utils Functions
macro_rules! expr { ($x:expr) => ($x) }

macro_rules! crash_checker {
    ($obj1: ident,  $obj2: ident, $axis: tt) => (
        (expr!($obj1.offset.$axis) <= expr!($obj2.offset.$axis) 
         &&
         expr!($obj2.offset.$axis) < expr!($obj1.offset.$axis) + expr!($obj1.size.$axis))
        ||
        (expr!($obj2.offset.$axis) <= expr!($obj1.offset.$axis) 
         &&
         expr!($obj1.offset.$axis) < expr!($obj2.offset.$axis) + expr!($obj2.size.$axis))
    );
}

pub fn is_crash(obj1: &RenderInfo, obj2: &RenderInfo) -> bool {
    crash_checker!(obj1, obj2, 0) && crash_checker!(obj1, obj2, 1)
}
