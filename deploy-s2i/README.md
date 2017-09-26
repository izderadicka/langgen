
# Deploying in Openshift

This directory enables to create so called Source to Image (S2I) deployment in [Openshift](http://opeshift.org)

1. Fork https://github.com/izderadicka/langgen in github
1. Build builder image localy with `make`
2. Tag it with your Docker Hub namespace `docker tag rust-builder your_namespace/rust-builder` and push to Docker hub `docker push your_namespace/rust-builder`
3. Login to opeshift command line `oc`
4. Import it to openshift registry and create image stream `oc import-image izderadicka/rust-builder --confirm`
5. Create application `oc new-app rust-builder~https://github.com/your_github_user/langgen --name=langgen`
6. Might need to create route to access application (`oc expose service langgegn`)
7. Create a webhook in forked repo (`oc describe bc langgen` to see webhook payload URL). Push changes to see automatic rebuild.
