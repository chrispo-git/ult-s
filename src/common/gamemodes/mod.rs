mod turbo;
mod airdash;
mod attacks;
mod vampire;
mod superboss;
mod itemduel;

pub fn install() {
	turbo::install();
	airdash::install();
	attacks::install();
    vampire::install();
    superboss::install();
    itemduel::install();
}