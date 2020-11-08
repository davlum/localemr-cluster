#!/bin/bash

set -o errexit -o nounset -o pipefail

function set_env() {
  export HADOOP_CLASSPATH
  HADOOP_CLASSPATH="/opt/hadoop/share/hadoop/tools/lib/*:$(hadoop classpath)"
  export SPARK_DIST_CLASSPATH=
  SPARK_DIST_CLASSPATH=$(hadoop classpath)
}

function write_config_options(){
    echo "write config options to $1"
    eval "echo \"$(cat $1)\"" > $1
}

function main(){
  set_env

  case "$1" in
    livy)
      write_config_options /opt/hadoop/etc/hadoop/core-site.xml
      localemr-container
      ;;
    *)
      exec "$@"
      ;;
  esac
}

main "$@"
