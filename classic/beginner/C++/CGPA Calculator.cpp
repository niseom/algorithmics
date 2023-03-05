/**
 * Author: YOUR_NAME
 * Date: 2023
 * Description: This Rust code implements a CGPA calculator program that allows a user to input
 * information about the courses they have taken and calculates their cumulative grade point average.
 * The program uses a Course struct to store information about each course, and a CGPA_Calculator
 * struct to store information about the student's courses and calculate their CGPA. The program
 * reads input from the user, adds Course objects to the CGPA_Calculator's course list, and calculates
 * the CGPA based on the user's input. This implementation is designed to be simple and easy to use,
 * but it may not handle all edge cases or be suitable for complex academic programs.
 */


#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Course {
    private:
        string course_name;
        double credit_hours;
        double grade;
    
    public:
        Course(string name, double credits) {
            course_name = name;
            credit_hours = credits;
            grade = 0.0;
        }
        
        void setGrade(double grade) {
            this->grade = grade;
        }
        
        string getCourseName() {
            return this->course_name;
        }

        double getCreditHours() {
            return this->credit_hours;
        }

        double getGrade(){
            return this->grade;
        }

        double getGradePoint() {
            if (grade >= 90.0) {
                return 4.0;
            } else if (grade >= 80.0) {
                return 3.0;
            } else if (grade >= 70.0) {
                return 1.0;
            } else {
                return 0.0;
            }
        }
};

class CGPA_Calculator {
    private:
        string student_name;
        int num_courses;
        vector<Course> course_list;

    public:
        CGPA_Calculator(){
            student_name = "";
            num_courses = 0;
        }

        void addCourse(Course course) {
            course_list.push_back(course);
            num_courses++;
        }

        vector<Course> getCourseList() {
            return course_list;
        }

        int getNumCourses() {
            return num_courses;
        }

        double calculateCGPA(){
            double total_grade_points = 0.0;
            double total_credit_hours = 0.0;
            double cgpa = 0.0;

            for (int i = 0; i < num_courses; i++) {
                Course course = course_list[i];
                double grade_point = course.getGradePoint();
                double credit_hours = course.getCreditHours();

                total_grade_points += grade_point * credit_hours;
                total_credit_hours += credit_hours;
            }

            if (total_credit_hours > 0.0) {
                cgpa = total_grade_points / total_credit_hours;
            }

            return cgpa;
        }
};


int main() {
    CGPA_Calculator cgpa_calculator;

    // Add some example course
    Course course1("Course 1", 3.0);
    course1.setGrade(90.0);
    cgpa_calculator.addCourse(course1);

    // Calculate and print the CGPA
    double cgpa = cgpa_calculator.calculateCGPA();
    cout << "CGPA: " << cgpa << endl;

    //Pring the individual course grades
    vector<Course> course_list = cgpa_calculator.getCourseList();
    for (int i = 0; i < cgpa_calculator.getNumCourses(); i++) {
        Course course = course_list[i];
        cout << course.getCourseName() << " (" << course.getCreditHours() << " credits): " << course.getGrade() << endl;
    }
}