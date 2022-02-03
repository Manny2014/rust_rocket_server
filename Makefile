
build:
	@cargo build

dbuild:
	@docker built -t manny87/rust_rocket_server .

skaffold:
	@skaffold dev --default-repo=manny87 -p default

kind:
	@kind create cluster --config cluster.yaml