//
//  ViewController.swift
//  ios_app
//
//  Created by Ivan Schuetz on 06.08.19.
//  Copyright © 2019 com.schuetz. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    @IBOutlet var greetingLabel: UILabel?
    @IBOutlet var additionLabel: UILabel?

    override func viewDidLoad() {
        super.viewDidLoad()
        
        let session = session_new()
        
        let string = session_greet(session!, "Ivan")!.takeRetainedValue()
        let s = string as NSString
        self.greetingLabel?.text = s as String
        
        let result = session_add(session!, 100)
        self.additionLabel?.text = "\(result)"
    }
}
