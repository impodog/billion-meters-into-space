use super::*;

#[derive(Resource)]
pub struct Seed {
    rng: ChaCha8Rng,
}

impl Default for Seed {
    fn default() -> Self {
        let seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        let rng = ChaCha8Rng::from_seed(seed);
        Self { rng }
    }
}

impl Seed {
    pub fn rand_range<T, R>(&mut self, range: R) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn rand_non_center(&mut self) -> Vec2 {
        if self.rng.gen_bool(0.5) {
            let x = self.rand_range(WIDTH / 6.0..WIDTH / 2.0);
            let x = if self.rng.gen_bool(0.5) { -x } else { x };
            let y = self.rand_range(-HEIGHT..HEIGHT);
            Vec2::new(x, y)
        } else {
            let x = self.rand_range(-WIDTH..WIDTH);
            let y = self.rand_range(HEIGHT / 6.0..HEIGHT / 2.0);
            let y = if self.rng.gen_bool(0.5) { -y } else { y };
            Vec2::new(x, y)
        }
    }

    pub fn rand_non_center_vec3(&mut self) -> Vec3 {
        let result = self.rand_non_center();
        Vec3::new(result.x, result.y, 0.0)
    }

    pub fn chance(&mut self, chance: f64, max: f64) -> bool {
        let rand = self.rand_range(0.0..=max);
        rand < chance
    }
}

pub(super) fn setup_seed(mut commands: Commands) {
    commands.insert_resource(Seed::default());
}
