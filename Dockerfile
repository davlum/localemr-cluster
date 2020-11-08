# This is a hack to persist env vars through multi layer stages
FROM ubuntu:20.10 AS base

ARG SPARK_VERSION
ENV SPARK_VERSION=$SPARK_VERSION

ENV HADOOP_VERSION=3.2.1

ENV JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64

ENV HADOOP_HOME=/opt/hadoop
ENV HADOOP_CONF_DIR=$HADOOP_HOME/etc/hadoop

ENV SPARK_MASTER=local[*]
ENV DEPLOY_MODE=client
ENV SPARK_HOME=/opt/spark

ENV PATH=${HADOOP_HOME}/bin:${SPARK_HOME}/bin:$PATH

ENV AWS_ACCESS_KEY_ID=TESTING
ENV AWS_SECRET_ACCESS_KEY=TESTING
ENV S3_ENDPOINT=http://s3:2000

################
# Build Hadoop #
################
FROM base AS buildHadoop

RUN apt-get update -y && apt-get install -y openjdk-8-jdk wget && apt-get clean && apt-get autoclean

# Apache Hadoop
RUN wget -q -O /tmp/hadoop.tgz http://archive.apache.org/dist/hadoop/common/hadoop-$HADOOP_VERSION/hadoop-$HADOOP_VERSION.tar.gz
RUN tar -xzf /tmp/hadoop.tgz -C /opt/
RUN mv /opt/hadoop-$HADOOP_VERSION /opt/hadoop
RUN rm -rf /opt/hadoop/share/doc
RUN mv /opt/hadoop/share/hadoop/tools/lib/aws-java-sdk-bundle-1.11.375.jar /opt/hadoop/share/hadoop/common/aws-java-sdk-bundle-1.11.375.jar
RUN mv /opt/hadoop/share/hadoop/tools/lib/hadoop-aws-$HADOOP_VERSION.jar /opt/hadoop/share/hadoop/common/hadoop-aws-$HADOOP_VERSION.jar
RUN rm -r /opt/hadoop/share/hadoop/tools
RUN wget -q -O /tmp/spark.tgz https://archive.apache.org/dist/spark/spark-$SPARK_VERSION/spark-${SPARK_VERSION}-bin-without-hadoop.tgz
RUN tar -xzf /tmp/spark.tgz -C /opt/
RUN mv /opt/spark-${SPARK_VERSION}-bin-without-hadoop /opt/spark
## Compile WordCount.java for Mapreduce test
#COPY test/fixtures/WordCount.java /opt/WordCount.java
#RUN mkdir /opt/wordcount_classes
#RUN javac -classpath $(hadoop classpath) -d /opt/wordcount_classes /opt/WordCount.java
#RUN jar -cvf /opt/wc-mapreduce.jar -C /opt/wordcount_classes/ .
# Compile NonChunkedDefaultS3ClientFactory.java for local S3
COPY conf/NonChunkedDefaultS3ClientFactory.java /opt/NonChunkedDefaultS3ClientFactory.java
RUN mkdir /opt/s3_client_factory_classes
RUN javac -classpath $(hadoop classpath) -d /opt/s3_client_factory_classes /opt/NonChunkedDefaultS3ClientFactory.java
RUN jar -cvf /opt/non-chunked-default-s3-clientfactory.jar -C /opt/s3_client_factory_classes/ .

#############
# Build Rust #
#############

FROM rust:1.47 AS buildRust

WORKDIR /usr/src/app
COPY . .
RUN cargo test && cargo install --path .

########
# App #
########
FROM base AS app

RUN apt-get update -y \
 && apt-get install -y openjdk-8-jre-headless \
 && apt-get clean

WORKDIR /opt/localemr

COPY --from=buildHadoop /opt/hadoop /opt/hadoop/
COPY --from=buildHadoop /opt/spark /opt/spark/
COPY --from=buildHadoop /opt/non-chunked-default-s3-clientfactory.jar /opt/hadoop/share/hadoop/common/
COPY conf/core-site.xml /opt/hadoop/etc/hadoop/core-site.xml
COPY --from=buildRust /usr/local/cargo/bin/localemr-container /usr/local/bin/localemr-container
COPY entrypoint.sh .

ENTRYPOINT ["./entrypoint.sh"]

CMD ["livy"]
