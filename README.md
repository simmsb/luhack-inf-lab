# luhack-inf-lab
My lab for practicing pwning infra

## Running

```sh
docker build -t luhack-inf-lab-bof --build-arg FLAG="not_a_flag{nope}" bof/.
docker build -t luhack-inf-lab-vuln-py \
         --build-arg FLAG_0="not_a_flag{nope}" \
         --build-arg FLAG_1="not_a_flag{nope}" \
         --build-arg ROOT_PW="root" \
         vuln-py/.

docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    --net=host \
    -d \
    -it nitros12/container-per-ip \
    luhack-inf-lab-bof -p 5000 -p 1234

docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    --net=host \
    -d \
    -it nitros12/container-per-ip \
    luhack-inf-lab-vuln-py -p 4321
```

## Attempting it yourself

### Bof

1. Start it up:
```
docker run --rm -it -p 5000:5000 -p 1234:1234 nitros12/luhack-inf-lab-bof
```

2. Poke `localhost:1234`


### Vuln-py

1. Start it up:
```
docker run --rm -it -p 4321:4321 nitros12/luhack-inf-lab-vuln-py
```

2. Poke `localhost:4321`
