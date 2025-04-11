#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i32, i32, &[u8])| {
    use py_spy::line_numbers::LineTableBuf;
    use py_spy::python_interpreters::CodeObject;

    let co_firstlineno = data.0;
    let lasti = data.1;
    let rem = data.2;
    let mut ct = LineTableBuf::new(rem);
    // println!("running {:?} {:?} {:?}", co_firstlineno, lasti, rem);
    let code = py_spy::python_bindings::v3_13_0::PyCodeObject {
        co_firstlineno: co_firstlineno,
        ..Default::default()
    };
    let res: Result<i32, py_spy::line_numbers::Error> = code.get_line_number(lasti, &mut ct);
    let _ = res;
});
