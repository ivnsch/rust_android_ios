//
//  ViewController.swift
//  ios_app
//
//  Created by Ivan Schuetz on 06.08.19.
//  Copyright © 2019 com.schuetz. All rights reserved.
//

import UIKit

var callbackLabelGlobal: UILabel?
var eventLabelGlobal: UILabel?

class ViewController: UIViewController {

    @IBOutlet var greetingLabel: UILabel?
    @IBOutlet var additionLabel: UILabel?
    @IBOutlet var jsonLabel: UILabel?
    @IBOutlet var callbackLabel: UILabel?
    @IBOutlet var eventLabel: UILabel?

    override func viewDidLoad() {
        super.viewDidLoad()
        
        let session = session_new()
        
        let string = session_greet(session!, "Ivan")!.takeRetainedValue()
        let s = string as NSString
        self.greetingLabel?.text = s as String
        
        let result = session_add(session!, 100)
        self.additionLabel?.text = "\(result)"
        
        let jsonRes = session_json(session!, """
            {"string_field": "foo", "int_field": 1}
            """)!.takeRetainedValue()
        self.jsonLabel?.text = "\(jsonRes)"
        
        callbackLabelGlobal = callbackLabel // C closure can't access context, so needs a global
        session_call(session!) { (a_number, a_boolean) in
            callbackLabelGlobal?.text = "Callback result: a_number: \(a_number), a_boolean: \(a_boolean)"
        }
        
        eventLabelGlobal = eventLabel // C closure can't access context, so needs a global
        session_observe(session!) { (a_number, a_boolean) in
            DispatchQueue.main.async {
                eventLabelGlobal?.text = "Event: a_number: \(a_number), a_boolean: \(a_boolean)"
            }
        }

        session_send_to_observers(session!, 1)
        DispatchQueue.main.asyncAfter(deadline: .now() + 2, execute: {
            session_send_to_observers(session!, 2)
        })
    }
}
