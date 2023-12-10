import * as fs from "fs";

function part1() {
    const content = fs.readFileSync("./input8.txt", "utf-8");
    
    const instructions = content.split("\r\n")[0].split("");
    console.log(instructions);
    
    const nodeData = {};
    content.split("\r\n").slice(2).forEach((line) => {
        const [node, metadata] = line.split(" = ");
        const [left, right] = metadata.split(", ")
        .map((x) => x.replace(")", "").replace("(", "").trim());
        
        nodeData[node] = {left, right};
    });
    
    console.log(nodeData);
    
    let node = "AAA";
    let steps = 0;
    
    for (let i = 0; i < instructions.length; i++) {
        const dir = instructions[i];
        
        if (dir === 'L') node = nodeData[node].left;
        else node = nodeData[node].right;
        
        console.log(dir, node);
        steps += 1;
        
        if (node === 'ZZZ') break;
        if (i === instructions.length - 1) i = -1;
    }

    console.log(`Steps to ZZZ: ${steps}`);
}


function part2() {
    const content = fs.readFileSync("./input8.txt", "utf-8");
    
    const instructions = content.split("\r\n")[0].split("");
    console.log(instructions);
    
    const nodeData = {};
    content.split("\r\n").slice(2).forEach((line) => {
        const [node, metadata] = line.split(" = ");
        const [left, right] = metadata.split(", ")
        .map((x) => x.replace(")", "").replace("(", "").trim());
        
        nodeData[node] = {left, right};
    });
    
    console.log(nodeData);

    const startNodes = Object.keys(nodeData).filter((x) => x.endsWith("A"));
    const all_steps = [];

    for (const startNode of startNodes) {
        let node = startNode;
        let steps = 0;

        for (let i = 0; i < instructions.length; i++) {
            const dir = instructions[i];
            
            if (dir === 'L') node = nodeData[node].left;
            else node = nodeData[node].right;
            
            steps += 1;
            
            if (node.endsWith('Z')) break;
            if (i === instructions.length - 1) i = -1;
        } 

        console.log(`Steps from ${startNode} to end: ${steps}`)
        all_steps.push(steps);
    }

    const gcd = (a, b) => a ? gcd(b % a, a) : b;
    const lcm = (a, b) => a * b / gcd(a, b);

    console.log('Steps for each path:', all_steps);

    const result = all_steps.reduce(lcm);
    console.log(`Steps to end: ${result}`);
}

part2();