// Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0

package options

import (
	"strconv"

	"github.com/valkey-io/valkey-glide/go/v2/constants"
)

// This base option struct represents the common set of optional arguments for the SCAN family of commands.
// Concrete implementations of this class are tied to specific SCAN commands (`SCAN`, `SSCAN`, `HSCAN`).
type BaseScanOptions struct {
	Match string
	Count int64
}

func NewBaseScanOptions() *BaseScanOptions {
	return &BaseScanOptions{}
}

/*
The match filter is applied to the result of the command and will only include
strings that match the pattern specified. If the sorted set is large enough for scan commands to return
only a subset of the sorted set then there could be a case where the result is empty although there are
items that match the pattern specified. This is due to the default `COUNT` being `10` which indicates
that it will only fetch and match `10` items from the list.
*/
func (scanOptions *BaseScanOptions) SetMatch(m string) *BaseScanOptions {
	scanOptions.Match = m
	return scanOptions
}

/*
`COUNT` is a just a hint for the command for how many elements to fetch from the
sorted set. `COUNT` could be ignored until the sorted set is large enough for the `SCAN` commands to
represent the results as compact single-allocation packed encoding.
*/
func (scanOptions *BaseScanOptions) SetCount(c int64) *BaseScanOptions {
	scanOptions.Count = c
	return scanOptions
}

func (opts *BaseScanOptions) ToArgs() ([]string, error) {
	args := []string{}
	var err error
	if opts.Match != "" {
		args = append(args, constants.MatchKeyword, opts.Match)
	}

	if opts.Count != 0 {
		args = append(args, constants.CountKeyword, strconv.FormatInt(opts.Count, 10))
	}

	return args, err
}
