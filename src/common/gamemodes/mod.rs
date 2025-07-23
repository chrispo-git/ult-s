mod turbo;
mod airdash;
mod attacks;

pub fn install() {
	turbo::install();
	airdash::install();
	attacks::install();
}