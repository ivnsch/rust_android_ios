import XCTest
@testable import ios_app

class ios_appTests: XCTestCase {

    func testGreet() {
        let res = greet("Ivan")!.takeRetainedValue() as String
        XCTAssertEqual("Hello 👋 Ivan!", res)
    }

    func testAdd() {
        let res = add_values(1, 2)
        XCTAssertEqual(3, res)
    }

    func testPassStruct() {
        var myStruct = ParamStruct(string: NSString(string: "foo").utf8String, int_: 1)
        let structPointer = withUnsafeMutablePointer(to: &myStruct) {
            UnsafeMutablePointer<ParamStruct>($0)
        }
        pass_struct(structPointer)
        // There's no result. Only testing that it doesn't crash.
    }

    func testReturnStruct() {
        let res = return_struct()

        let unmanagedString: Unmanaged<CFString> = res.string
        let cfStr: CFString = unmanagedString.takeRetainedValue()
        let str = cfStr as String

        XCTAssertEqual(str, "my string parameter")
        XCTAssertEqual(res.int_, 123)
    }

    func testRegistersCallback() {
        register_callback { (string: CFString?) in
            let cfStr: CFString = string!
            let str = cfStr as String
            XCTAssertEqual(str, "Hello callback!")
        }
    }
}
