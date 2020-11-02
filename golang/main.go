package main

import (
	"bufio"
	"fmt"
	"math/rand"
	"os"
	"strconv"
)

func main() {
	fmt.Println("猜数字 ( 1 - 100 ) - 你猜猜是多少 ? ")
	correct := 1 + rand.Intn(100)

	sc := bufio.NewScanner(os.Stdin)
	for sc.Scan() {
		input, err := strconv.Atoi(sc.Text())
		if err != nil {
			fmt.Println("请输入数字: ")
		}

		if input == correct {
			fmt.Println("你猜对了 !")
			break
		}

		if input < correct {
			fmt.Println("小了, 再猜: ")
			continue
		}

		if input > correct {
			fmt.Println("大了, 再猜: ")
			continue
		}
	}
}
