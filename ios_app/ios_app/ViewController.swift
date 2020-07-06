import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()

        let greetResult = greet("MyName")!.takeRetainedValue() as NSString as String
        print("greetResult: \(greetResult)")

        let addResult = add(1, 2)
        print("addResult: \(addResult)")

        var myStruct = ParamStruct(string: NSString(string: "foo").utf8String, int_: 1)
        let structPointer = withUnsafeMutablePointer(to: &myStruct) {
            UnsafeMutablePointer<ParamStruct>($0)
        }
        let passClassResult: Void = pass_struct(structPointer)
        print("passClassResult: \(passClassResult)")

        let returnClassResult = return_struct()
        print("returnClassResult: \(returnClassResult)")

        register_callback { string in
            print("callback called: \(String(describing: string))")
        }
    }
}
