use crate::codegen::template::basic::{
    AppendedFragment, AssertFragment, AtomicFragment, FunctionCallFragment, FunctionFragment,
};
use std::cell::RefCell;
use std::rc::Rc;

pub fn new_kb_test(test_frag: &mut AppendedFragment, name: &str) -> Rc<RefCell<FunctionFragment>> {
    let init_kb = Rc::new(RefCell::new(FunctionCallFragment::new(AtomicFragment {
        imports: vec!["crate::tao::initialize_kb".to_owned()],
        atom: "initialize_kb".to_owned(),
    })));
    let mut new_test = FunctionFragment::new(name.to_owned());
    new_test.mark_as_test();
    new_test.append(init_kb);
    let rc = Rc::new(RefCell::new(new_test));
    test_frag.append(rc.clone());
    rc
}

pub fn add_assert(test_function: &Rc<RefCell<FunctionFragment>>, lhs: String, rhs: String) {
    test_function
        .borrow_mut()
        .append(Rc::new(RefCell::new(AssertFragment::new_eq(
            Rc::new(RefCell::new(AtomicFragment::new(lhs))),
            Rc::new(RefCell::new(AtomicFragment::new(rhs))),
        ))));
}
