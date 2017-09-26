
# rust-builder
FROM scorpil/rust:nightly
MAINTAINER ivan.zderadicka@gmail.com

# TODO: Rename the builder environment variable to inform users about application you provide them
ENV BUILDER_VERSION 1.0

# Set labels used in OpenShift to describe the builder image
LABEL io.k8s.description="Rust Build for langgen" \
      io.k8s.display-name="Rust Builder 1.0" \
      io.openshift.expose-services="8080:http" \
      io.openshift.tags="builder,rust"


# Install required packages here:
RUN apt-get update &&\
    apt-get install -y wget &&\
    apt-get clean


# Copy the S2I scripts to /usr/local/bin, 
LABEL io.openshift.s2i.scripts-url=image:///usr/local/bin
COPY ./s2i/bin/ /usr/local/bin

# Drop the root user and make the content of /opt/app-root owned by user 1001

RUN adduser --uid 1001 appuser &&\
 mkdir /opt/app &&\
 chown -R 1001:1001 /opt/app

WORKDIR /opt/app

# This default user is created in the openshift/base-centos7 image
USER 1001

# Set the default port for applications built using this image
EXPOSE 8080

# Set the default CMD for the image
CMD ["/usr/local/bin/usage"]
