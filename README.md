# build images
```
nohup docker build -t baidang201/exchange-node:release  . &
```

# push images
```
docker push baidang201/exchange-node:release
```

# run docker
```
chmod +x ./up.sh
./up.sh
```