# cityboulder
Crawler of the cityboulder counter to plot visitor diagram.

## Deployment
The following commands generate a release-$version.tar.gz file which contains all docker images ready to be run. Currently these are:
- cityboulder_web
- cityboulder_crawler

Both images need to mount an sqlite database to /sqlite.db to work. Because they're based on the 'scratch' container, the entire release is currently <10MB in size!

### Package
```
export CITYBOULDER_VERSION=x.x.x
docker-compose build
docker save $(docker image ls cityboulder_*:$CITYBOULDER_VERSION --format "{{.Repository}}:{{.Tag}}") | gzip > release-${CITYBOULDER_VERSION}.tar.gz
```

### Run
```
docker run --rm -v $PWD/sqlite.db:/sqlite.db crawler:x.x.x
docker run --rm -v $PWD/sqlite.db:/sqlite.db -p 8080:8080 cityboulder_web:x.x.x
```