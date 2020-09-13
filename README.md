# A server to run commands submitted by [localemr][1]

This server runs arbitrary commands.
This is heavily unsafe and should not be used in a production environment.

To see which version of Apache Spark are available see the [build matrix][1].
This was refactored from using [Apache Livy][3], as Apache Livy doesn't
support the full `spark-submit` API, for example `--packages`,
and doesn't support [Apache MaprReduce][6] jobs either. In  order to communicate
with mock instances of S3 there is a copy of [NonChunkedDefaultS3ClientFactory.java][4]
from [sumitsu/s3_mocktest_demo][5].

The version of Hadoop used is stuck a 3.2.1, as this version allows the usage
of the aforementioned `NonChunkedDefaultS3ClientFactory.java` class.

------

### Configuration

Some environment variables can be overriden to communicate with an unmocked S3.
```.env
AWS_ACCESS_KEY_ID=TESTING
AWS_SECRET_ACCESS_KEY=TESTING
S3_ENDPOINT=http://s3:2000
```

------

### Usage:

```bash
docker run -p 8998:8998 davlum/localemr-container:latest-spark2.4.4
```


[1]: <https://github.com/davlum/livy-server-docker/blob/master/.github/workflows/main.yaml>
[2]: <https://github.com/davlum/localemr>
[3]: <https://livy.apache.org/>
[4]: <conf/NonChunkedDefaultS3ClientFactory.java>
[5]: <https://github.com/sumitsu/s3_mocktest_demo/blob/master/src/test/java/dev/sumitsu/s3mocktest/testutil/NonChunkedDefaultS3ClientFactory.java>
[6]: <https://hadoop.apache.org/docs/current/hadoop-mapreduce-client/hadoop-mapreduce-client-core/MapReduceTutorial.html>
