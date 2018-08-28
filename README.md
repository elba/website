# website
elba's (currently non-existent) presence on the world wide web

## Install
```
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run
```

## Usage
1. Create a access token from [Github](https://github.com/settings/tokens), with `read:user` and `user:email` permissions. 

2.
```
$ curl -v -L "http://localhost:17000/api/v1/users/login?gh_name=your_account_name&gh_access_token=your_access_token"
```

Response:
```
{"token":"ihP2qJEETheAS7Gx0TuzrmcWs5uh6bFZ"}
```

3.
Prepare a tar file with proper `elba.toml` in it, and then:
```
$ curl -v -L --request POST --data-binary "@your_project.tar" "http://localhost:17000/api/v1/packages/publish?package_group_name=package_group_name&package_name=package_name&semver=semver&token=your_token" 
```

and then responses 200 OK currently.

## Roadmap
- [x] Login
- [x] Publish package
- [x] Store tarballs
- [x] Fetch index
- [x] Update index
- [x] Push index
- [ ] Setup local index for development usage
- [x] Error handling middleware (currently any error represents as 500 Internal Error)
- [ ] Use supervisor actor to recover from fatal
- [ ] Rollback publish transcaton when fs error occured
- [ ] Setup nginx as TLS front-end and static server (hosts static assets and tarballs).
- [ ] Dockerfile
- [ ] Basic search support
- [ ] Yank support