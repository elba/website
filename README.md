# website
elba's (currently non-existent) presence on the world wide web

## Install
```
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run
```

## Deploy
1. Clone this repo into your server.
2. Edit `docker-compose.yml`: fill in enviroment varibles, 
`REMOTE_INDEX_USER` and `REMOTE_INDEX_PWD`.
3. Place your ssl certs to `/var/lib/nginx/certs/cert.csr` and 
`/var/lib/nginx/certs/cert.key`.
4. Make sure you exposed port 80 and port 443 if you have a firewall.
5. Run `docker-compose up`

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
$ curl -v -L --request POST --data-binary "@your_project.tar" "http://localhost:17000/api/v1/packages/publish?group_name=group_name&package_name=package_name&semver=semver&token=your_token" 
```

and then responses 200 OK currently.

## Back-end Roadmap
- [x] Login
- [x] Publish package
- [x] Store tarballs
- [x] Fetch index
- [x] Update index
- [x] Push index
- [x] Error handling middleware (currently any error represents as 500 Internal Error)
- [x] Rollback publish transcaton when fs error occured
- [x] Setup nginx as TLS front-end and static server (hosts static assets and tarballs).
- [x] Dockerfile
- [x] Yank support
- [x] Add description/readme/homepage
- [x] Add authors
- [x] Improve `PackageName` / `PackageVersion` consistency
- [ ] Forced gzip compression
- [ ] Basic search support

### Front-end Roadmap

- [ ] Initially implement front-end
- [ ] Github OAuth
- [ ] Display token
