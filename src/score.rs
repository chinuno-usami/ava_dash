use crate::consts::*;

pub struct ScoreResource {
    pub perfect: usize,
    pub good: usize,
    pub bad: usize,
    pub miss: usize,
    pub combo: usize,
    pub max_combo: usize,
    pub score: usize,
    pub hp: f32,
    pub acc: f32,
}

impl ScoreResource {
    pub fn default() -> ScoreResource{
        ScoreResource {
            perfect: 0,
            good: 0,
            bad: 0,
            miss: 0,
            combo: 0,
            max_combo: 0,
            hp: 100.,
            acc: 1.,
            score: 0
        }
    }
    fn update_acc(&mut self) {
        //       (perfect * 300) + (good * 100) + (bad * 50)
        // acc = --------------------------------------------
        //           (perfect + good + bad + miss) * 300
        self.acc = (self.perfect * 300 + self.good * 100 + self.bad * 50) as f32 / ((self.perfect + self.good + self.good + self.miss) as f32 * 300.);
    }
    fn add_combo(&mut self) {
        self.combo += 1;
        if self.combo > self.max_combo {
            self.max_combo = self.combo;
        }
    }
    fn update_by_value(&mut self, value: f32) {
        // hp
        self.hp += value*0.01;
        if self.hp > 100.0 {
            self.hp = 100.0;
        }
        // score
        if value > 0.0 {
            self.score += value as usize;
        }
    }
    /// 增加合适的次数值以及得分
    pub fn increase_perfect(&mut self) {
        self.perfect += 1;
        // combo
        self.add_combo();
        // acc
        self.update_acc();
        // hp、score
        self.update_by_value(500.);
    }

    pub fn increase_good(&mut self) {
        self.good += 1;
        // combo
        self.add_combo();
        // acc
        self.update_acc();
        // hp、score
        self.update_by_value(300.);
    }
    pub fn increase_bad(&mut self) {
        self.bad += 1;
        // combo
        self.add_combo();
        // acc
        self.update_acc();
        // hp、score
        self.update_by_value(5.);
    }
    pub fn increase_miss(&mut self) {
        self.miss += 1;
        // combo
        self.combo = 0;
        // acc
        self.update_acc();
        // hp、score
        self.update_by_value(-1500.);
    }
}
