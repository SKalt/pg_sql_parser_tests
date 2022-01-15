// not a oracle in-and-of-itself, but a component of other oracles.
// The functions here rely on the service-definitions in the top-level
// docker-compose.yaml.
package container

import (
	"database/sql"
	_ "database/sql"
	"fmt"
	"log"
	"time"

	_ "github.com/lib/pq"
)

type Service struct {
	version string
	name    *string
	dsn     *string
}

func (service *Service) Name() string {
	if service.name == nil {
		name, err := DeriveServiceName(service.version)
		if err != nil {
			log.Panic(err)
		}
		service.name = &name
	}
	return *service.name
}

func (service *Service) Dsn() string {
	if service.dsn == nil {
		dsn := fmt.Sprintf(
			"host=0.0.0.0 user=postgres password=password port=500%s sslmode=disable",
			service.version)
		service.dsn = &dsn
	}
	return *service.dsn
}

func (service *Service) isReady() bool {
	db, err := sql.Open("postgres", service.Dsn())
	if err != nil {
		return false
	}
	_, err = db.Exec("SELECT 1;")
	return err == nil
}

func (service *Service) Await() error {
	// wait for the database server
	ticker := time.NewTicker(time.Second)
	for i := 0; i <= 15; i++ {
		<-ticker.C // wait for a tick

		if service.isReady() {
			return nil
		} else {
			fmt.Printf(".")
		}
	}
	return fmt.Errorf("%s service startup timed out", service.Name())
}

func DeriveServiceName(version string) (string, error) {
	switch version {
	case "10":
	case "11":
	case "12":
	case "13":
	case "14":
		break
	default:
		return "", fmt.Errorf("unsupported postgres+psql version %s", version)
	}
	return fmt.Sprintf("pg-%s", version), nil
}

// NOTE: this creates a service-struct, it doesn't actually start the service.
func InitService(version string) *Service {
	service := Service{version: version, name: nil, dsn: nil}
	return &service
}
