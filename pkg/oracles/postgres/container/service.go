package container

import (
	"fmt"
	"log"
	"os/exec"
	"time"
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
			log.Fatal(err)
		}
		service.name = &name
	}
	return *service.name
}

func (service *Service) Dsn() string {
	if service.dsn == nil {
		dsn := fmt.Sprintf(
			"host=0.0.0.0 user=postgres password=password port=500%s",
			service.version)
		service.dsn = &dsn
	}
	return *service.dsn
}

func (service *Service) Start() error {
	return StartService(service.Name())
}

func (service *Service) Close() error {
	return CloseService(service.Name())
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

func InitService(version string) *Service {
	service := Service{version: version, name: nil, dsn: nil}
	return &service
}

func StartService(serviceName string) error {
	isReady := func() bool {
		cmd := exec.Command("docker-compose", "exec", serviceName, "pg_isready")
		err := cmd.Wait()
		return err == nil
	}
	cmd := exec.Command("docker-compose", "up", "-d", serviceName)
	err := cmd.Wait()
	if err != nil {
		return err
	}

	// wait for the database server
	ticker := time.NewTicker(time.Second)
	for i := 0; i <= 10; i++ {
		<-ticker.C // wait for a tick

		if isReady() {
			return nil
		} else {
			fmt.Printf(".")
		}
	}
	return fmt.Errorf("%s service startup timed out", serviceName)
}

func CloseService(service string) error {
	return exec.
		Command("docker-compose", "down", service).
		Wait()
}
