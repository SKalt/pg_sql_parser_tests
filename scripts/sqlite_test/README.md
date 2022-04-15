Note that not all of the sqlite tests are present. Some are trapped in tcl lists and variables, and templated strings, which we can't currently follow.
Also, the sqlite test suite includes SQL grammar fuzzing which isn't captured here either.

It wasn't feasible to add mix a [trace](https://tcl.tk/man/tcl8.7/TclCmd/trace.html) callback directly into the tcl test suite.
While there're faster subsets of the sqlite test suite, those take ~3m and ignore a large fraction of the sql in the full test suite.
The full test suite is slow and includes fuzz tests, which could lead to undesired randomness in the collected test-statements.
