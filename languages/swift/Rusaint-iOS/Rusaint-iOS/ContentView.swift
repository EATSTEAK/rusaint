//
//  ContentView.swift
//  Rusaint-iOS
//
//  Created by 이조은 on 8/25/24.
//

import SwiftUI
import Rusaint

struct ContentView: View {
    @State private var session: USaintSession? = nil
    @State private var semesterGrades: [SemesterGrade?]? = nil
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
        }
        .padding()
        .onAppear{
            setupSession()
            print("print \(String(describing: session))")
            print("print \(String(describing: semesterGrades))")
        }
    }

    func setupSession() {
        Task {
            do {
                self.session = try await USaintSessionBuilder().withPassword(id: "202000000", password: "password")
                getSemesterGrades()
                print("Session initialized successfully: \(String(describing: session))")
            } catch {
                print("Failed to initialize session: \(error)")
            }
        }
    }

    func getSemesterGrades() {
        Task {
            do {
                self.semesterGrades = try await CourseGradesApplicationBuilder().build(session: session!).semesters(courseType: CourseType.bachelor)
                print("Session initialized successfully: \(String(describing: semesterGrades))")
            } catch {
                print("Failed to initialize session: \(error)")
            }
        }
    }
}


#Preview {
    ContentView()
}
