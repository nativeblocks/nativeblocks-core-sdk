//
//  sampleApp.swift
//  sample
//
//  Created by alireza on 2026-07-27.
//

import SwiftUI

@main
struct sampleApp: App {

    private let instanceManager = InstanceManager()

    var body: some Scene {
        WindowGroup {
            SampleNavigationStack(instanceManager: instanceManager)
        }
    }
}
