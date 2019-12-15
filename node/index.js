const { createInterface } = require("readline");
const rl = createInterface(process.stdin, process.stdout);

const correct = Number.parseInt(1 + 100 * Math.random());

console.log("猜数字 ( 1 - 100 ) - 你猜猜是多少 ? ");
rl.prompt();

rl.on("line", answer => {
  if (!Number(answer)) {
    console.log("请输入数字: ");
    return rl.prompt();
  }

  if (answer == correct) {
    console.log("你猜对了 !");
    rl.close();
  }

  if (answer < correct) {
    console.log("小了, 再猜: ");
    rl.prompt();
  }

  if (answer > correct) {
    console.log("大了, 再猜: ");
    rl.prompt();
  }
});
