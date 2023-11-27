mod aerials;
mod ground;
mod other;
mod specials;
mod throws;
mod tilts;

pub fn install() {
	aerials::install();
	ground::install();
	other::install();
	specials::install();
	throws::install();
	tilts::install();
}