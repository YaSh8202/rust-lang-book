use yash_art::utils::mix;
use yash_art::kinds::RGBColor;

fn main() {
    let c1 = RGBColor::Red;
    let c2 = RGBColor::Green;
    mix(c1, c2);
}
