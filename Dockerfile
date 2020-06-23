FROM ubuntu:20.10

ARG SPARK_VERSION
ARG LIVY_VERSION
ARG APACHE
ENV LOCAL_DIR_WHITELIST /tmp/
ENV HADOOP_VERSION 3.2.1
ENV S3_ENDPOINT=""
ENV AWS_ACCESS_KEY_ID=""
ENV AWS_SECRET_ACCESS_KEY=""
ENV SPARK_MASTER local[*]
ENV DEPLOY_MODE client


RUN apt-get update -y && apt-get install -y \
    openjdk-8-jre-headless \
    unzip \
    wget \
  && apt-get clean \
  # Apache Livy
  && wget -q -O /tmp/livy.zip https://archive.apache.org/dist/incubator/livy/$LIVY_VERSION-incubating/${APACHE}livy-$LIVY_VERSION-incubating-bin.zip \
  && unzip /tmp/livy.zip -d /opt/ \
  && mv /opt/${APACHE}livy-$LIVY_VERSION-incubating-bin /opt/livy \
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
  # S3 Integration
  && wget -q -P /opt/spark/jars/ https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-s3/1.11.784/aws-java-sdk-s3-1.11.784.jar \
  && wget -q -P /opt/spark/jars/ https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-core/1.11.784/aws-java-sdk-core-1.11.784.jar \
  && wget -q -P /opt/spark/jars/ https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-dynamodb/1.11.784/aws-java-sdk-dynamodb-1.11.784.jar \
  && wget -q -P /opt/spark/jars/ https://repo1.maven.org/maven2/org/apache/hadoop/hadoop-aws/$HADOOP_VERSION/hadoop-aws-$HADOOP_VERSION.jar \
  && rm -r /tmp/*

COPY init /opt/docker-init
COPY conf/livy.conf /opt/livy/conf/livy.conf
COPY conf/core-site.xml /opt/hadoop/etc/hadoop/core-site.xml
COPY conf/s3_mocktest_demo_2.11-0.0.1.jar /opt/spark/jars/s3_mocktest_demo_2.11-0.0.1.jar

EXPOSE 8998

ENV PATH=${HADOOP_HOME}/bin:${SPARK_HOME}/bin:$PATH

WORKDIR /opt/docker-init
ENTRYPOINT ["./entrypoint"]

