import { get, writable } from "svelte/store"

export interface TapeSection {
    start: number;
    end: number;
    name: string;
    color: string;
    value: string;
}

export interface TuringInstruction {
    write: number
    action: 'L' | 'R' | 'S'
    next_state: string
    asm: number
}

export interface AssemblyInstruction {
    instruction: string
    status: string
    sections: TapeSection[]
}

export class TuringMachine {
    instructions = writable<Map<string, TuringInstruction>>(new Map())
    sections = writable<TapeSection[]>([])
    tape = writable<number[]>([])
    head = writable<number>(0)
    state = writable<string>("START")
    steps = writable<number>(0)
    ready = writable<boolean>(false)
    speed = writable<number>(10)
    originalProgram = writable<string>("")
    error = writable<string>("")
    assemblyInstructions = writable<AssemblyInstruction[]>([])
    status = writable<'running' | 'paused'>('paused')
    currentAsmInstruction = writable<number>(0)
    currentInstruction = writable<string>("")
    fileString = ""
    interval = 0

    load_file(file: string) {

        this.clear()

        this.fileString = file

        let instructions = new Map<string, TuringInstruction>()

        let current_asm_instruction_idx = 0
        let current_asm_instruction = ""

        const lines = file.split('\n')
        for (const idx in lines) {

            // first line is tape
            if (idx == "0") {
                this.tape.set(lines[idx].split(' ').map((x, idx) => {
                    if (x.startsWith("!")) {
                        x = x.replace("!", "")
                        this.head.set(idx)
                    }
                    return parseInt(x)
                }))
                continue;
            }

            const line = lines[idx].trim()

            if (line == '') continue
            if (line.startsWith('#')) {
                if (line.startsWith('###')) {
                    current_asm_instruction = line.replace('###', '').trim()
                    this.assemblyInstructions.update((instructions) => {
                        instructions.push({
                            instruction: current_asm_instruction,
                            status: current_asm_instruction_idx == 0 ? "current" : "pending",
                            sections: []
                        })
                        return instructions
                    })
                    current_asm_instruction_idx += 1
                }
                if (line.startsWith('#p')) {
                    this.originalProgram.update((value) => value += line.replace('#p', '').trim() + '\n')
                }
                continue
            }

            const [state, read, write, action, next_state] = line.split(' ')
            instructions.set(`${state}-${parseInt(read)}`, {
                write: parseInt(write),
                action: action as 'L' | 'R' | 'S',
                next_state,
                asm: current_asm_instruction_idx
            });
        }

        this.instructions.set(instructions)
        this.create_sections()
        this.ready.set(true)
    }

    update_sections() {
        const tape = get(this.tape)
        this.sections.update((sections) => {
            sections.forEach((section, index) => {
                let values = "";
                let value = "";

                for (let i = section.start; i <= section.end; i++) {
                    values += tape[i];
                    if (tape[i] != 0 && tape[i] != 1) {
                        value = "x";
                    }
                }

                // transform binary values to decimal
                if (value != "x") value = parseInt(values, 2).toString();
                sections[index].value = value;
            });
            return sections;
        })
    }

    create_sections() {
        let sections = [];
        const tape = get(this.tape)

        sections.push({
            start: 1,
            end: 0,
            name: "S0",
            color: "blue",
            value: "x",
        });
        for (let i = 1; i < tape.length - 1; i++) {
            const section = sections.at(-1)!;
            const value = tape[i];

            if ([5].includes(value)) continue;

            if ([6, 2, 3, 4].includes(value)) {
                let name: string = `S${sections.length}`;
                let color = "blue";

                if (tape[i + 1] == 5) continue;

                if (value == 2) {
                    name = "A";
                    color = "red";
                }
                if (value == 3) {
                    name = "B";
                    color = "green";
                }
                if (value == 4) {
                    name = "C";
                    color = "darkorchid";
                }
                sections.push({
                    start: i + 1,
                    end: i + 1,
                    color,
                    name,
                    value: "x",
                });
            } else {
                section.end = i;
            }
        }

        // switch up section names, S1 -> S4, S2 -> S1, S3 -> S2, S4 -> S3, etc
        let total_storage_sections = sections.filter((section) =>
            section.name.startsWith("S"),
        ).length;
        sections = sections.map((section, index) => {
            if (section.name.startsWith("S")) {
                section.name = `S${total_storage_sections}`;
                total_storage_sections -= 1;
            }
            return section;
        });

        this.sections.set(sections);
    }

    step() {
        const state = get(this.state)
        let head = get(this.head)
        const tape = get(this.tape)
        const read = tape[head]
        const instructions = get(this.instructions)
        const instruction = instructions.get(`${state}-${read}`)

        if (instruction) {
            tape[head] = instruction.write
            let next_state = instruction.next_state
            this.tape.set(tape)

            if (instruction.action == 'L') {
                head--
            } else if (instruction.action == 'R') {
                head++
            }

            this.head.set(head)
            this.state.set(next_state)
            this.steps.update(n => n + 1)
            this.currentInstruction.set(`${next_state}-${tape[head]}`)
            this.update_sections()

            if (instruction.asm != get(this.currentAsmInstruction)) {
                this.updateAssemblyInstructions()
            }
        } else {
            this.error.set(`No instruction for state ${state} and read ${read}`)
            console.error(`No instruction for state ${state} and read ${read}`)
            this.pause()
        }
    }

    updateAssemblyInstructions() {
        this.assemblyInstructions.update((instructions) => {
            let sectionsString = ""
            let asmIdx = get(this.currentAsmInstruction)

            if (asmIdx == 0) return instructions

            let sections = get(this.sections)
            if (instructions[asmIdx - 1]) {
                instructions[asmIdx - 1].sections = JSON.parse(JSON.stringify(sections))
                instructions[asmIdx - 1].status = "completed"
            }

            if (instructions[asmIdx]) {
                instructions[asmIdx].status = "current"
            }
            return instructions
        })

        this.currentAsmInstruction.update(n => n + 1)
    }

    run() {
        let speed = get(this.speed)
        this.status.set('running')
        this.interval = setInterval(() => {
            if (speed == 1000) {
                for (let i = 0; i < 100; i++) {
                    if (get(this.state) == "END") {
                        clearInterval(this.interval)
                        this.end()
                    }
                    else this.step()
                }
            } else {
                if (get(this.state) == "END") {
                    clearInterval(this.interval)
                    this.end()
                } else this.step()
            }
        }, 1000 / speed)
    }

    end() {
        this.updateAssemblyInstructions()
        this.status.set('paused')
    }

    pause() {
        clearInterval(this.interval)
        this.status.set('paused')
    }

    setSpeed(speed: number) {
        let status = get(this.status)
        this.speed.set(speed)
        if (status == 'running') {
            this.pause()
            this.run()
        }
    }

    clear() {
        this.pause()
        this.instructions.set(new Map())
        this.tape.set([])
        this.head.set(0)
        this.speed.set(10)
        this.state.set('START')
        this.steps.set(0)
        this.error.set("")
        this.originalProgram.set("")
        clearInterval(this.interval)
        this.assemblyInstructions.set([])
        this.currentAsmInstruction.set(0)
    }

    delete() {
        this.clear()
        this.ready.set(false)
        this.fileString = ""
    }

    reset(): void {
        this.clear()
        this.load_file(this.fileString)

    }
}

export let interpreter = new TuringMachine();