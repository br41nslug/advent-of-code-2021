
const getInput = () => new Promise((resolve) => {
    process.stdin.resume();
    process.stdin.setEncoding('utf8');
    let lingeringLine = "", result = [];
    process.stdin.on('data', function(chunk) {
        lines = chunk.split("\n");
        lines[0] = lingeringLine + lines[0];
        lingeringLine = lines.pop();
        lines.forEach(line => result.push(line));
    });
    process.stdin.on('end', function() {
        lines.push(lingeringLine);
        resolve(lines);
    });
});

function Board(rows) {
    const self = this || {};
    const _evaluate = (draws) => (row) =>
        Math.max(...row.map(x => draws.indexOf(x)));
    self.rows = rows;
    self.cols = rows.reduce((a, c) => {
        for (let i = 0; i < c.length; i++) {
            if ( ! a[i]) a[i] = [];
            a[i].push(c[i]);
        }
        return a;
    }, []);
    self.all = rows.reduce((a, c) => {
        c.forEach(x => {
            a.push(x);
        });
        return a;
    }, []);
    self.score = (draws, index) => {
        const _draws = draws.slice(0, index + 1);
        const unmarked = this.all
            .filter(val => !_draws.includes(val))
            .reduce((a, c) => (a + c), 0);
        return unmarked * draws[index];
    };
    self.evaluate = (draws) => Math.min(
        Math.min(...self.rows.map(_evaluate(draws))),
        Math.min(...self.cols.map(_evaluate(draws)))
    );
    return self;
}

async function processInput() {
    const input = await getInput();
    const draws = input.shift().split(',').map(x => parseInt(x));
    let boardIndex = -1;
    const boards = input.reduce((b, line) => {
        if (line === '') {
            boardIndex++;
        } else {
            if ( ! b[boardIndex]) b[boardIndex] = [];
            b[boardIndex].push(line.trim().split(/\s+/).map((x) => parseInt(x)));
        }
        return b;
    }, []).map(b => new Board(b));
    return { draws, boards };
}


async function main() {
    const input = await processInput();
    let winner, w = Number.MAX_SAFE_INTEGER;
    input.boards.forEach(board => {
        const val = board.evaluate(input.draws);
        if (val < w) {
            w = val;
            winner = board;
        }
    });
    console.log('Winner score: ', winner.score(input.draws, w));
}
main();