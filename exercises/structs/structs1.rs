// structs1.rs
// Address all the TODOs to make the tests pass!

//ohhhh I'm supposed to put something here... I get it now
struct ColorClassicStruct {
    //I'm gonna be honest here, I'm following someone elses solution cause I have no experience with c lang,is this person opting for classic strings cause they are more memmory efficient? or just laziness?
    name: &'static str,
    hex: &'static str,
}
//lmaooo I accidentally solved this one by just implementing a tuple, I'll comment out that code and do it right :/
struct ColorTupleStruct(&'static str, &'static str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: "green",
            hex: "#00FF00",
        };
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        //   let green = (String::from("green"), "#00FF00");
        let green = ColorTupleStruct("green", "#00FF00");
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct! ??
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
