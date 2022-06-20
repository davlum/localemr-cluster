FROM ubuntu:22.10

# Build time env vars
ARG SPARK_VERSION
ENV HADOOP_VERSION 3.2.1
ENV LIVY_VERSION 0.5.0

RUN apt-get update -y && apt-get install -y \
    openjdk-8-jre-headless \
    unzip \
    wget \
  && apt-get clean \
  # Apache Livy
  && wget -q -O /tmp/livy.zip https://archive.apache.org/dist/incubator/livy/$LIVY_VERSION-incubating/livy-$LIVY_VERSION-incubating-bin.zip \
  && unzip /tmp/livy.zip -d /opt/ \
  && mv /opt/livy-$LIVY_VERSION-incubating-bin /opt/livy \
  && mkdir /opt/livy/logs \
  # Apache Hadoop
  && wget -q -O /tmp/hadoop.tgz http://archive.apache.org/dist/hadoop/common/hadoop-$HADOOP_VERSION/hadoop-$HADOOP_VERSION.tar.gz \
  && tar -xzf /tmp/hadoop.tgz -C /opt/ \
  && mv /opt/hadoop-$HADOOP_VERSION /opt/hadoop \
  && rm -rf /opt/hadoop/share/doc \
  # Apache Spark 
  && wget -q -O /tmp/spark.tgz https://archive.apache.org/dist/spark/spark-$SPARK_VERSION/spark-$SPARK_VERSION-bin-without-hadoop.tgz \
  && tar -xzf /tmp/spark.tgz -C /opt/ \
  && mv /opt/spark-$SPARK_VERSION-bin-without-hadoop /opt/spark \
  && mkdir -p /tmp/spark-events \
  && mv /opt/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-1.11.375.jar /opt/hadoop/share/hadoop/common/aws-java-sdk-bundle-1.11.375.jar \
  && mv /opt/hadoop/share/hadoop/tools/lib/hadoop-aws-3.2.1.jar /opt/hadoop/share/hadoop/common/hadoop-aws-3.2.1.jar \
  && rm -r /opt/hadoop/share/hadoop/tools \
  && rm -r /tmp/*

COPY init /opt/docker-init
COPY conf/livy.conf /opt/livy/conf/livy.conf
COPY conf/core-site.xml /opt/hadoop/etc/hadoop/core-site.xml
COPY conf/s3_mocktest_demo_2.11-0.0.1.jar /opt/hadoop/share/hadoop/common/s3_mocktest_demo_2.11-0.0.1.jar

# Runtime env vars
ENV S3_ENDPOINT=""
ENV AWS_ACCESS_KEY_ID=TESTING
ENV AWS_SECRET_ACCESS_KEY=TESTING
ENV LOCAL_DIR_WHITELIST=/tmp/

EXPOSE 8998

WORKDIR /opt/docker-init
ENTRYPOINT ["./entrypoint"]
CMD ["livy"]

