package languages

// an enumeration of all the allowed languages
var Languages = map[string]int64{
	"other":     -1,
	"pgsql":     0,
	"plpgsql":   1,
	"psql":      2,
	"plperl":    3,
	"pltcl":     4,
	"plpython2": 5,
	"plpython3": 6,
	"sqlite":    7,
}

func LookupId(language string) int64 {
	if id, ok := Languages[language]; ok {
		return id
	} else {
		// definitely other
		return -1
	}
}
