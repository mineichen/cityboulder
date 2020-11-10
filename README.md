# cityboulder
Crawler of the cityboulder counter to plot visitor diagram

## Required build steps
export CITYBOULDER_VERSION=0.0.3
docker-compose build
docker save $(docker image ls cityboulder_*:$CITYBOULDER_VERSION --format "{{.Repository}}:{{.Tag}}") | gzip > release-${CITYBOULDER_VERSION}.tar.gz

## deploy
docker load -i release-0.0.4.tar.gz