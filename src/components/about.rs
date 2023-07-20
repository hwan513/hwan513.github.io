use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view!(cx,
        section (id="about", style="min-height: 40vh;") {
            h1 { "About me" }
            p { "My programming journey began in the latter half of 2021 when I was introduced to the basics of Matlab and C through an engineering course. Prior to that, I had a surface-level understanding of Python from self-learning. During that time, I thoroughly enjoyed the learning process and the systematic approach that came with writing code and using its surrounding technologies. I immediately jumped into the deep end by learning Vim as my text editor. It was after that course that I decided to pursue Software Engineering." }
            br {}
            p {"My first major pursuit was completing Advent of Code 2021, which involved solving a difficult set of programmatic problems using the only languages I knew at the time: Python, C, and Matlab. As I progressed in my software engineering studies the following year, I learned Java (with a focus on mainly OOP and design) through the courses I was taking and also self-learned Rust, Lua (for configuring Neovim and Hammerspoon), HTML, CSS, and JavaScript. Strengthening my knowledge in algorithms and code design, I successfully completed Advent of Code 2022 solely in Rust." }
            br {}
            p {"In my spare time, I enjoy playing video games—especially 2D platformers like Celeste—and I also love exercising through activities such as running, yoga, badminton, and going to the gym. My passion for learning is evident in my various programming ventures. Additionally, my drive to learn has led me to become proficient in a few esoteric skills, such as achieving grade 8 proficiency in both Piano and Flute. I've also mastered using the Dvorak keyboard layout and experimented with writing in Teeline shorthand."}
        }
    )
}
