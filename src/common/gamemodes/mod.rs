mod turbo;
mod airdash;
mod attacks;
mod vampire;
mod superboss;

pub fn install() {
	turbo::install();
	airdash::install();
	attacks::install();
    vampire::install();
    superboss::install();
}