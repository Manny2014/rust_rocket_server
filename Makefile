
build:
	@cargo build

dbuild:
	@docker built -t manny87/rust_rocket_server .

skaffold-local:
	@skaffold dev --default-repo=manny87 -p local

skaffold-gcp:
	@skaffold dev --default-repo=us-central1-docker.pkg.dev/shared-network-19f4/main -p gcp-dev

kind:
	@kind create cluster --config cluster.yaml