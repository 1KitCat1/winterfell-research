use winterfell::math::{fields::f128::BaseElement, FieldElement};
use winterfell::{TraceTable, Serializable};

const INV_ALPHA: u128 = 226854911280625642308916371969163307691;
const CONST_42: BaseElement = BaseElement::new(42);


fn main() {

}

fn vdf(seed: BaseElement, n: usize) -> BaseElement {
    let mut currect_state = seed;

    for _ in 0..n {
        currect_state = (currect_state - CONST_42).exp(INV_ALPHA);
    }
    currect_state
}

#[derive(Clone)]
struct VdfInputs {
    seed: BaseElement,
    result: BaseElement,
}

impl Serializable for VdfInputs {
    fn write_into<W: winterfell::ByteWriter>(&self, target: &mut W) {
        target.write(self.seed);
        target.write(self.result);
    }
}
