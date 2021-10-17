// meant to be run at the root of the repo
package main

import (
	"fmt"
	"io/fs"
	"os"
	"regexp"
	"strings"
)

var md5Regex = regexp.MustCompile(`^[a-z0-9]{32}$`)
var snakeCaseRegex = regexp.MustCompile(`^[a-z0-9](_[a-z0-9]|[a-z0-9])*[a-z0-9]$`)
var versionRegex = regexp.MustCompile(`^\d{3}(\.\d)?$`)

func failIf(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "%+v\n", err)
	}
}

func isSymLink(file fs.FileInfo) bool {
	return file.Mode()&os.ModeSymlink != 0
}

// a test suite should be a dir of symlinks
func validateTestSuite(
	path string, suite fs.DirEntry, errors []string, refs map[string][]string,
) ([]string, map[string][]string) {
	suitePath := fmt.Sprintf("%s/%s", path, suite.Name())
	if !snakeCaseRegex.Match([]byte(suite.Name())) {
		errors = append(errors, fmt.Sprintf("%s should be snake_case", suitePath))
	}
	if suite.IsDir() {
		entries, err := os.ReadDir(suitePath)
		failIf(err)
		for _, entry := range entries {
			entryPath := fmt.Sprintf("%s/%s", suitePath, entry.Name())
			if !snakeCaseRegex.Match([]byte(entry.Name())) {
				errors = append(errors, fmt.Sprintf("%s should be snake_case", entryPath))
			}
			if entry.IsDir() {
				tests, err := os.ReadDir(entryPath)
				failIf(err)
				for _, test := range tests {
					testPath := fmt.Sprintf("%s/%s", entryPath, test.Name())
					if !snakeCaseRegex.Match([]byte(test.Name())) {
						errors = append(errors, fmt.Sprintf("%s should be snake_case", testPath))
					}
					info, err := test.Info()
					failIf(err)
					if isSymLink(info) {
						linkname, err := os.Readlink(testPath)
						if err != nil {
							errors = append(errors, fmt.Sprintf("%s error: %+v", testPath, err))
						}
						if linkname[0] != '.' {
							errors = append(errors, fmt.Sprintf("%s -> %s should be a relative link", testPath, linkname))
						}
						info, err = os.Stat(linkname + "/input.sql")
						if err != nil {
							errors = append(errors, fmt.Sprintf("%s -> %s %+v", testPath, linkname, err))
						} else {
							linkedFile := info.Name()
							if !md5Regex.Match([]byte(linkedFile)) {
								errors = append(errors, fmt.Sprintf("%s -> %s is not linked to a md5-named dir", testPath, linkname))
							} else {
								refs[linkedFile] = append(refs[linkedFile], testPath)
							}
						}
					} else {
						errors = append(errors, fmt.Sprintf("%s should be a symlink to fixtures/data/{some md5}", testPath))
					}
				}
			} else {
				errors = append(errors, fmt.Sprintf("%s should be a directory", entryPath))
			}
		}
	} else {
		errors = append(errors, fmt.Sprintf("%s should be a directory", suitePath))
	}
	return errors, refs
}

func main() {
	errors := []string{}
	dataEntries, err := os.ReadDir("./fixtures/data")
	failIf(err)
	versions, err := os.ReadDir("./fixtures/versions")
	failIf(err)
	refs := make(map[string][]string, len(dataEntries))

	for _, datum := range dataEntries {
		refs[datum.Name()] = []string{}
		if !datum.IsDir() {
			errors = append(errors, fmt.Sprintf(
				"./fixtures/data/%s should be a directory", datum.Name()))
		}
		if !md5Regex.Match([]byte(datum.Name())) {
			errors = append(errors, fmt.Sprintf(
				"./fixtures/data/%s should be a lower-hex-encoded md5", datum.Name()))
		}
	}
	for _, version := range versions {
		if !versionRegex.Match([]byte(version.Name())) {
			errors = append(errors,
				fmt.Sprintf(
					"./fixtures/versions/%s should be 3 0-padded digits with one optional decimal",
					version.Name(),
				),
			)
		}
		if !version.IsDir() {
			errors = append(errors, fmt.Sprintf(
				"./fixtures/versions/%s should be a directory", version.Name()))
		} else {
			versionPath := fmt.Sprintf("./fixtures/versions/%s", version.Name())
			testSuites, err := os.ReadDir(versionPath)
			failIf(err)
			for _, suite := range testSuites {
				errors, refs = validateTestSuite(versionPath, suite, errors, refs)
			}
			// each version should contain doctests/ and regress/, each of which should be
			// a dir of dirs of symlinks
		}
	}
	for target, links := range refs {
		if len(links) == 0 {
			errors = append(errors, fmt.Sprintf("%s has 0 references", target))
		}
	}

	if nErrors := len(errors); nErrors > 0 {
		fmt.Fprintf(
			os.Stderr, "%d errors:\n  %s\n",
			nErrors, strings.Join(errors, "\n  "),
		)
		os.Exit(1)
	}
}
