pub struct VelocityComponent {
    vx: i32,
    vy: i32,
}

impl VelocityComponent {
    pub fn new(vx: i32, vy: i32) -> VelocityComponent
    {
        VelocityComponent{
            vx,
            vy
        }
    }
    /*
     *
     * Move by a specific velocity
     */
    pub fn set_velocity(&mut self, _vx: i32, _vy: i32) -> () {
        self.vx = _vx;
        self.vy = _vy;
    }

    pub fn reset_velocity(&mut self) {
        self.vx = 0;
        self.vy = 0;
    }

    pub fn vx(&self) -> i32 {
        self.vx
    }

    pub fn vy(&self) -> i32 {
        self.vy
    }
}