# Spark images for [localemr](https://github.com/davlum/localemr)

### Supported Versions:

To see which images are available see the [build matrix](https://github.com/davlum/livy-server-docker/blob/master/.github/workflows/main.yaml).

------

### Configuration

Required environment variables:

- `SPARK_MASTER` => Spark Master IP
- `DEPLOY_MODE` => *client* or *cluster*

Per default the `/tmp` folder is configured as path for submitting local files via
Livy Server. It is configurable through `LOCAL_DIR_WHITELIST` environment
variable.

------

### Usage:

```bash
docker run -p 8998:8998 -e SPARK_MASTER="local[*]" -e DEPLOY_MODE=client davlum/localemr-spark2.4.4
```

Visit http://localhost:8998

