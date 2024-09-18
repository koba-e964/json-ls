# json-ls
`ls`, but it emits JSON.

```console
$ go run . | jq
[
  {
    "name": ".git",
    "permission": "drwxr-xr-x",
    "size": 384,
    "modified_time": "2024-09-18T23:43:34Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": ".github",
    "permission": "drwxr-xr-x",
    "size": 96,
    "modified_time": "2024-09-18T23:41:03Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": ".gitignore",
    "permission": "-rw-r--r--",
    "size": 9,
    "modified_time": "2024-09-18T23:41:53Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": "README.md",
    "permission": "-rw-r--r--",
    "size": 1514,
    "modified_time": "2024-09-18T23:45:18Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": "go.mod",
    "permission": "-rw-r--r--",
    "size": 45,
    "modified_time": "2024-09-18T22:46:39Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": "main.go",
    "permission": "-rw-r--r--",
    "size": 1200,
    "modified_time": "2024-09-18T23:32:41Z",
    "uid": 502,
    "gid": 20
  },
  {
    "name": "schema.json",
    "permission": "-rw-r--r--",
    "size": 561,
    "modified_time": "2024-09-18T23:39:35Z",
    "uid": 502,
    "gid": 20
  }
]
```

```console
$ go run ./main.go | jq '.[]|{name,size}'
{
  "name": ".git",
  "size": 288
}
{
  "name": "go.mod",
  "size": 45
}
{
  "name": "json-ls",
  "size": 2619170
}
{
  "name": "main.go",
  "size": 874
}
```

# Schema
schema.json defines the schema.

```console
$ check-jsonschema <(go run .) --schemafile schema.json
ok -- validation done
```
