# select operating system
FROM amazoncorretto:8

ARG SPARK_VERSION
ARG LIVY_VERSION
ARG APACHE

RUN yum update -y && yum install -y \
    gzip \
    unzip \
    procps \
    wget \
    tar \
  && yum clean all \
  && rm -rf /var/cache/yum

ENV LOCAL_DIR_WHITELIST /tmp/

# apache livy
RUN wget https://archive.apache.org/dist/incubator/livy/$LIVY_VERSION-incubating/${APACHE}livy-$LIVY_VERSION-incubating-bin.zip -O /tmp/livy.zip && \
  unzip /tmp/livy.zip -d /opt/ && \
  mv /opt/${APACHE}livy-$LIVY_VERSION-incubating-bin /opt/livy && \
  rm /tmp/livy.zip && \
  mkdir /opt/livy/logs

# apache spark
RUN wget https://archive.apache.org/dist/spark/spark-$SPARK_VERSION/spark-$SPARK_VERSION-bin-hadoop2.7.tgz -O /tmp/spark.tgz && \
  tar -xvzf /tmp/spark.tgz -C /opt/ && \
  mv /opt/spark-$SPARK_VERSION-bin-hadoop2.7 /opt/spark && \
  rm /tmp/spark.tgz

COPY init /opt/docker-init
COPY conf/livy.conf /opt/livy/conf/livy.conf

# expose ports
EXPOSE 8998

# start from init folder
WORKDIR /opt/docker-init
ENTRYPOINT ["./entrypoint"]

