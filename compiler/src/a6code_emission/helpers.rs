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
{state} (ABsep,EndB) *     {dir} {state}
{state} StMark *           L {state}_2",
            ));
            //             if dir == "L" {
            //                 string.push_str(&format!(
            //                     "
            // {state}_1 StMark        *  {state}_2
            // {state}_1 (0,1,H0,H1)   * L {state}_1
            // {state}_1 StSep         * R {state}_2",
            //                 ))
            //             } else {
            //                 string.push_str(&format!("\n{state}_1 StMark * L {state}_2"))
            //             }
        }

        string.push_str(&format!("\n{0} (Middle,StSep,H0,H1) * {1} {0}", state, dir));

        string.push_str(&format!(
            "
# Move a {i} to the left until the first H0, and replace it
{state}_2 (0,1) *            L {state}_2
{state}_2 H0 {i}             S {next_state}",
        ));

        instructions.push_str(&string);
    }

    instructions
}

pub fn go_to_storage(address: u32, next_state: String, mark: bool) -> String {
    let mut string = format!("#GO_TO_Sn: {address}\n");

    let mark = if mark { "StMark" } else { "StSep" };

    let first_state = format!("TO_{address}_S1");

    if address == 1 {
        string.push_str(&format!("\n{first_state} StSep {mark} L {next_state}"));
    } else {
        string.push_str(&format!(
            "\n{first_state} (StSep,StMark) * L TO_{address}_S2"
        ));

        for i in 2..=address {
            let i_str = format!("S{}", i.to_string());
            let next_i_str = format!("S{}", (i + 1).to_string());

            let state = format!("TO_{address}_{i_str}");

            string.push_str(&format!("\n{0} (0,1,H0,H1) * L {0}", state));

            if i == address {
                string.push_str(&format!("\n{state} (StSep,StMark) {mark} L {next_state}",));
            } else {
                let next_state = format!("TO_{address}_{next_i_str}");
                string.push_str(&format!("\n{state} (StSep,StMark) * L {next_state}",));
            };
        }
    }

    string
}
