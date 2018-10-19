package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
)

type Repos struct {
	Type           string `json:"@type"`
	Href           string `json:"@href"`
	Representation string `json:"@representation"`
	Pagination     struct {
		Limit   int         `json:"limit"`
		Offset  int         `json:"offset"`
		Count   int         `json:"count"`
		IsFirst bool        `json:"is_first"`
		IsLast  bool        `json:"is_last"`
		Next    interface{} `json:"next"`
		Prev    interface{} `json:"prev"`
		First   struct {
			Href   string `json:"@href"`
			Offset int    `json:"offset"`
			Limit  int    `json:"limit"`
		} `json:"first"`
		Last struct {
			Href   string `json:"@href"`
			Offset int    `json:"offset"`
			Limit  int    `json:"limit"`
		} `json:"last"`
	} `json:"@pagination"`
	Repositories []struct {
		Type           string `json:"@type"`
		Href           string `json:"@href"`
		Representation string `json:"@representation"`
		Permissions    struct {
			Read          bool `json:"read"`
			Admin         bool `json:"admin"`
			Activate      bool `json:"activate"`
			Deactivate    bool `json:"deactivate"`
			Migrate       bool `json:"migrate"`
			Star          bool `json:"star"`
			Unstar        bool `json:"unstar"`
			CreateCron    bool `json:"create_cron"`
			CreateEnvVar  bool `json:"create_env_var"`
			CreateKeyPair bool `json:"create_key_pair"`
			DeleteKeyPair bool `json:"delete_key_pair"`
			CreateRequest bool `json:"create_request"`
		} `json:"@permissions"`
		ID             int         `json:"id"`
		Name           string      `json:"name"`
		Slug           string      `json:"slug"`
		Description    string      `json:"description"`
		GithubID       int         `json:"github_id"`
		GithubLanguage interface{} `json:"github_language"`
		Active         bool        `json:"active"`
		Private        bool        `json:"private"`
		Owner          struct {
			Type  string `json:"@type"`
			ID    int    `json:"id"`
			Login string `json:"login"`
			Href  string `json:"@href"`
		} `json:"owner"`
		DefaultBranch struct {
			Type           string `json:"@type"`
			Href           string `json:"@href"`
			Representation string `json:"@representation"`
			Name           string `json:"name"`
		} `json:"default_branch"`
		Starred               bool `json:"starred"`
		ManagedByInstallation bool `json:"managed_by_installation"`
		ActiveOnOrg           bool `json:"active_on_org"`
	} `json:"repositories"`
}

func main() {

	token := os.Getenv("TRAVIS_PERSONAL_TOKEN")
	client := http.Client{}

	req, err := http.NewRequest(
		"GET",
		"https://api.travis-ci.org/repos",
		nil,
	)
	req.Header.Set("Travis-API-Version", "3")
	req.Header.Set("Authorization", "token "+token)
	if err != nil {
		log.Fatalln(err)
	}

	resp, err := client.Do(req)
	if err != nil {
		log.Fatalln(err)
	}
	if resp.StatusCode != http.StatusOK {
		log.Fatalln("Should receive a status code of 200 ok")
	}

	repos := new(Repos)

	json.NewDecoder(resp.Body).Decode(&repos)

	json, err := json.MarshalIndent(repos, "", "    ")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(string(json))
}
