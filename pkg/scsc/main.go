/*
	strconv spooked cat

	A simple wrapper for the stdlib's strconv that panics if any errors occur.
	It also has better function names.
*/
package scsc

import "strconv"

/*
	Int to String
*/

// Converts int i to string
func IntStr(i int) string {
	return strconv.FormatInt(
		int64(i),
		10,
	)
}

// Converts int64 i64 to string
func Int64Str(i64 int64) string {
	return strconv.FormatInt(
		i64,
		10,
	)
}

/*
	String to Int
*/

// Converts string s to int
func StrInt(s string) int {
	i64, err := strconv.ParseInt(
		s,
		10,
		0,
	)
	if err != nil {
		panic(err)
	}

	return int(i64)
}

// Converts string s to int64
func StrInt64(s string) int64 {
	i64, err := strconv.ParseInt(
		s,
		10,
		0,
	)
	if err != nil {
		panic(err)
	}

	return i64
}
