mod turbo;
mod airdash;
mod attacks;
mod vampire;
mod superboss;
mod itemduel;
mod hitfall;

pub fn install() {
	turbo::install();
    hitfall::install();
	airdash::install();
	attacks::install();
    vampire::install();
    superboss::install();
    itemduel::install();
}