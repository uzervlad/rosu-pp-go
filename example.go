package main

/*
#cgo CFLAGS: -I./lib
#cgo LDFLAGS: -L./lib -lrosu_pp_go -lm
#include <rosu_pp_go.h>
#include <stdlib.h>
*/
import "C"

import (
	"fmt"
	"os"
	"unsafe"
)

func CreateCalculator(mapData string) (RosuCalculator, bool) {
	cMapData := C.CString(mapData)
	defer C.free(unsafe.Pointer(cMapData))

	var calculator *C.calculator
	err := C.calculator_from_data(&calculator, cMapData)
	if err != 0 {
		return RosuCalculator{}, false
	}

	return RosuCalculator{
		inner: calculator,
	}, true
}

func (rosu RosuCalculator) Calculate(score *ScoreParams) (C.calculateresult, bool) {
	var scoreParams *C.scoreparams
	err := C.score_params_new(&scoreParams)

	if err != 0 {
		return C.calculateresult{}, false
	}

	C.score_params_mode(scoreParams, C.mode(score.mode))
	if score.mods > 0 {
		C.score_params_mods(scoreParams, C.uint(score.mods))
	}
	if score.acc > 0 {
		C.score_params_acc(scoreParams, C.double(score.acc))
	}
	if score.n300 > 0 {
		C.score_params_n300(scoreParams, C.uint(score.n300))
	}
	if score.n100 > 0 {
		C.score_params_n100(scoreParams, C.uint(score.n100))
	}
	if score.n50 > 0 {
		C.score_params_n50(scoreParams, C.uint(score.n50))
	}
	if score.combo > 0 {
		C.score_params_combo(scoreParams, C.uint(score.combo))
	}
	if score.nMisses > 0 {
		C.score_params_n_misses(scoreParams, C.uint(score.nMisses))
	}
	if score.nKatu > 0 {
		C.score_params_n_katu(scoreParams, C.uint(score.nKatu))
	}
	if score.passedObjects > 0 {
		C.score_params_passed_objects(scoreParams, C.uint(score.nKatu))
	}
	if score.clockRate > 0 {
		C.score_params_clock_rate(scoreParams, C.double(score.clockRate))
	}
	defer C.score_params_destroy(&scoreParams)

	calculationResult := C.calculator_calculate(rosu.inner, scoreParams)
	return calculationResult, true
}

func (rosu RosuCalculator) Destroy() {
	C.calculator_destroy(&rosu.inner)
}

type ScoreParams struct {
	mode          uint
	mods          uint
	acc           float64
	n300          uint
	n100          uint
	n50           uint
	nMisses       uint
	nKatu         uint
	combo         uint
	passedObjects uint
	clockRate     float64
}

type RosuCalculator struct {
	inner *C.calculator
}

func main() {
	data, err := os.ReadFile("beatmap.osu")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	calculator, ok := CreateCalculator(string(data))
	if !ok {
		fmt.Println("create calculator can't :(")
		os.Exit(1)
	}

	score := ScoreParams{
		mode: 3,
		acc:  95,
		mods: 0,
	}

	result, ok := calculator.Calculate(&score)
	if !ok {
		fmt.Println("calculate pp can't :(")
		os.Exit(1)
	}
	fmt.Printf("%f stars\n", result.stars)
	fmt.Printf("%f pp\n", result.pp)
}
