package main

import (
	"encoding/json"
	"fmt"
	"os"
	"syscall"
	"time"
)

func main() {
	argv := os.Args
	dir := "."
	if len(argv) >= 2 {
		dir = argv[1]
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		panic(err)
	}
	// Emits a JSON array
	fmt.Print("[")
	first := true
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			panic(err)
		}
		perm := info.Mode().String()
		modTime := info.ModTime().UTC().Format(time.RFC3339)
		var uid, gid *uint32
		if info.Sys() != nil {
			if stat, ok := info.Sys().(*syscall.Stat_t); ok {
				uid = &stat.Uid
				gid = &stat.Gid
			}
		}

		s, err := json.Marshal(struct {
			Name         string  `json:"name"`
			Perm         string  `json:"permission"`
			Size         int64   `json:"size"` // TODO: fall back to string when size >= 2^53
			ModifiedTime string  `json:"modified_time"`
			UID          *uint32 `json:"uid,omitempty"`
			GID          *uint32 `json:"gid,omitempty"`
		}{Name: entry.Name(), Perm: perm, Size: info.Size(), ModifiedTime: modTime, UID: uid, GID: gid})
		if first {
			first = false
		} else {
			fmt.Print(",")
		}
		if err != nil {
			fmt.Print(err)
		}
		fmt.Print(string(s))
	}
	fmt.Print("]")
}
