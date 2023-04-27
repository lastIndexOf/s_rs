mod circular_reference;
mod cons_list;
mod struct_generic;
mod weak_reference;

use circular_reference::test_circular_reference;
use cons_list::test_cons_list;
use struct_generic::struct_generic_test;
use weak_reference::test_weak_reference;

fn main() {
    struct_generic_test();
    test_cons_list();
    test_circular_reference();
    test_weak_reference();
}
