# Spark images for [localemr](https://github.com/davlum/localemr)

### Supported Versions:

To see which images are available see the [build matrix][1].
The image includes https://github.com/sumitsu/s3_mocktest_demo JAR in
order to communicate with mock instances of S3.

------

### Configuration

Required environment variables:

- `SPARK_MASTER` => Spark Master IP
- `DEPLOY_MODE` => *client* or *cluster*

Per default the `/tmp` folder is configured as path for submitting local files via
Livy Server. It is configurable through `LOCAL_DIR_WHITELIST` environment
variable.

------

### Usage
```bash
docker run -p 8998:8998 davlum/localemr-container:0.5.0-spark2.4.4
```
### Usage for EMR 6.9.0
```bash
docker run -p 8998:8998 sumitzet/localemr-container:0.7.1-spark3.3.0
```

Visit http://localhost:8998

[1]: <https://github.com/davlum/livy-server-docker/blob/master/.github/workflows/main.yaml>
