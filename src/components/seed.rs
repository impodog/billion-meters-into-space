use super::*;

#[derive(Default, Resource)]
pub struct Seed {
    seed: <ChaCha8Rng as SeedableRng>::Seed,
}

impl Seed {
    pub fn gen(&self, seed: u64) -> u64 {
        let mut rng = ChaCha8Rng::from_seed(self.seed);
        let mut rng_user = ChaCha8Rng::seed_from_u64(seed);
        rng.gen_range(0..=u64::MAX) ^ rng_user.gen_range(0..=u64::MAX)
    }

    pub fn chance(&self, seed: u64, chance: u64, max: u64) -> bool {
        let value = self.gen(seed);
        value % max < chance
    }
}
