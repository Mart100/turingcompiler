pub fn move_bits(destination: &str, direction: &str, next_state: &str) -> String {
    let mut instructions = "".to_string();

    let dir = direction;
    let opp_dir = if dir == "R" { "L" } else { "R" };

    for i in [0, 1] {
        // bit to move ZERO or ONE
        let mut string = "".to_string();
        let state = if i == 0 { "MOVE_ZERO" } else { "MOVE_ONE" };
        string.push_str(&format!("\n# Move a {} to number {}", i, destination));
        string.push_str(&format!("\n{0} (0,1,StartA) * {1} {0}", state, dir));

        if destination == "A" {
            string.push_str(&format!(
                "
{0} ABsep *             {2} {0}_2
{0} EndB *              {1} {0}",
                state, dir, opp_dir
            ));
        } else if destination == "B" {
            string.push_str(&format!(
                "
{0} ABsep *             {1} {0}
{0} EndB *              {2} {0}_2",
                state, dir, opp_dir
            ));
        } else if destination == "C" {
            string.push_str(&format!(
                "
{0} (ABsep,EndB) *     {1} {0}
{0} EndC *              {2} {0}_2",
                state, dir, opp_dir
            ));
        } else if destination == "StMark" {
            string.push_str(&format!(
                "
{0} (ABsep,EndB) *     {1} {0}
{0} StMark *           {2} {0}_2",
                state, dir, opp_dir
            ));
        }

        string.push_str(&format!(
            "\n{0} (Middle,StSep,MovH0,MovH1) * {1} {0}",
            state, dir
        ));

        string.push_str(&format!(
            "
# Move a {1} to the left until the first MovH0, and replace it
{0}_2 (0,1) *               {2} {0}_2
{0}_2 MovH0 {1}             S {3}",
            state, i, opp_dir, next_state
        ));

        instructions.push_str(&string);
    }

    instructions
}

pub fn go_to_storage(address: u32, next_state: String) -> String {
    let mut string = "".to_string();

    if address == 1 {
        string.push_str(&format!(
            "# To end of S1\nTO_S1_END StSep * L {}",
            next_state,
        ));
    } else {
        string.push_str("# To end of S1\nTO_S1_END StSep * L TO_S2_END");

        for i in 2..=address {
            let i_str = format!("S{}", i.to_string());
            let next_i_str = format!("S{}", (i + 1).to_string());

            string.push_str(&format!(
                "\n# To end of {0}\n TO_{0}_END (0,1) * L TO_{0}_END",
                i_str
            ));

            if i == address {
                string.push_str(&format!("\nTO_{}_END StSep * L {}\n", i_str, next_state));
            } else {
                string.push_str(&format!(
                    "\nTO_{}_END StSep * L TO_{}_END",
                    i_str, next_i_str
                ));
            }
        }
    }

    string
}
