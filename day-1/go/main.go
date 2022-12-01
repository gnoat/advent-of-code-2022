package main

import (
    "fmt"
    "os"
    "strings"
    "strconv"
    "sort"
)

func main() {
    content_bytes, err := os.ReadFile("data.txt")
    if err != nil {
        fmt.Println(err)
    }

    blocks := strings.Split(string(content_bytes), "\n\n")

    // Now to calculate everything by nesting it in ugly for-loops because Go sucks
    max_elf_cals := 0
    for _, elf_block := range blocks {
        elf_cals := 0
        for _, elf_block := range strings.Split(elf_block, "\n") {
            if elf_block != "" {
                elf_cal_block, err := strconv.Atoi(elf_block)
                if err != nil {
                    fmt.Println(err)
                }
                elf_cals = elf_cals + elf_cal_block
                if elf_cals > max_elf_cals {
                    max_elf_cals = elf_cals
                }
            }
        }
    }

    fmt.Println("The Elf with the most cals is carrying", max_elf_cals, "calories.")

    elf_cal_counter := []int{}
    for _, elf_block := range blocks {
        elf_cals := 0
        for _, elf_block := range strings.Split(elf_block, "\n") {
            if elf_block != "" {
                elf_cal_block, err := strconv.Atoi(elf_block)
                if err != nil {
                    fmt.Println(err)
                }
                elf_cals = elf_cals + elf_cal_block
            }
        }
        elf_cal_counter = append(elf_cal_counter, elf_cals)
    }
    sort.Ints(elf_cal_counter)

    // Next we calculate the sum using *another* for-loop because Go sucks
    top_three_cal_total := 0
    for _, cals := range elf_cal_counter[len(elf_cal_counter)-3:] {
        top_three_cal_total = top_three_cal_total + cals
    }

    fmt.Println("The top three calorie carrying elves have a total", top_three_cal_total, "calories.")

}
