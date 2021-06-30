# Low resource PC-Axis file conversion

Many of the tools used to manipulate the statistical PC-Axis data files require
significant computational and storage resources to execute even the simplest of
transformations with.

This library, command line tool (`loxis`) and web service (`loxisw`), converts
PC-Axis files to compressed CSV and Apache Parquet files in a manner which makes
it practical to allow access to conversion request endpoints without any more
restrictions than are needed for the user organization's other web services.

## PxWeb

https://github.com/statisticssweden/PxWeb
