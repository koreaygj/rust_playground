// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

// * Use an enum to represent all types of employees
enum Position {
    Maintenance_crew,
    Marketing_department,
    Manager,
    Line_supervisor,
    Kitchen_staff,
    Assembly_technician,
}
enum Status {
    Activated,
    Terminated,
}
// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    position: Position,
    status: Status,
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_access(man: &Employee) -> Result<(), String> {
    if let Status::Terminated = man.status {
        return Err("Can't access building(terminated)".to_owned());
    }
    match man.position {
        Position::Maintenance_crew => Ok(()),
        Position::Marketing_department => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Can't access building(invalid position)".to_owned()),
    }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_access(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let man = &Employee {
        position: Position::Manager,
        status: Status::Activated,
    };
    let access = print_access(&man);
    match access {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
