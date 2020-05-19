FROM ubuntu:20.10

ARG SPARK_VERSION
ARG LIVY_VERSION
ARG APACHE
ENV LOCAL_DIR_WHITELIST /tmp/

RUN apt-get update -y && apt-get install -y \
    default-jre-headless \  
    unzip \
    wget \
  && apt-get clean \
  # Apache Livy
  && wget -q -O /tmp/livy.zip https://archive.apache.org/dist/incubator/livy/$LIVY_VERSION-incubating/${APACHE}livy-$LIVY_VERSION-incubating-bin.zip \
  && unzip /tmp/livy.zip -d /opt/ \
  && mv /opt/${APACHE}livy-$LIVY_VERSION-incubating-bin /opt/livy \
  && mkdir /opt/livy/logs \
  # Apache Spark 
  && wget -q -O /tmp/spark.tgz https://archive.apache.org/dist/spark/spark-$SPARK_VERSION/spark-$SPARK_VERSION-bin-hadoop2.7.tgz \
  && tar -xvzf /tmp/spark.tgz -C /opt/ \
  && mv /opt/spark-$SPARK_VERSION-bin-hadoop2.7 /opt/spark \
  && mkdir -p /tmp/spark-events \
  # S3 Integration
  && wget -q -O /opt/spark/jars/aws-java-sdk.jar https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk/1.7.4/aws-java-sdk-1.7.4.jar \
  && wget -q -O /opt/spark/jars/hadoop-aws.jar https://repo1.maven.org/maven2/org/apache/hadoop/hadoop-aws/2.7.3/hadoop-aws-2.7.3.jar \
  && rm -r /tmp/*

COPY init /opt/docker-init
COPY conf/livy.conf /opt/livy/conf/livy.conf

EXPOSE 8998

WORKDIR /opt/docker-init
ENTRYPOINT ["./entrypoint"]

