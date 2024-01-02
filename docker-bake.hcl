variable "VERSION" {
  default = "latest"
}

group "default" {
  targets = ["spotisync", "spotidownload"]
}

target "spotisync" {
  inherits = ["common"]
  target = "spotisync_runtime"
  tags = [
    "ghcr.io/cbackas/spotisync:latest",
    "ghcr.io/cbackas/spotisync:${VERSION}",
  ]
}

target "spotidownload" {
  inherits = ["common"]
  target = "spotidownload_runtime"
  tags = [
    "ghcr.io/cbackas/spotidownload:latest",
    "ghcr.io/cbackas/spotidownload:${VERSION}",
  ]
}

target "common" {
  context = "."
  dockerfile = "Dockerfile"
}
