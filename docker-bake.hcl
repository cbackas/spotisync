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
    "ghcr.io/cbackas/spotisync:sync/latest",
    "ghcr.io/cbackas/spotisync:sync/${VERSION}",
  ]
}

target "spotidownload" {
  inherits = ["common"]
  target = "spotidownload_runtime"
  tags = [
    "ghcr.io/cbackas/spotisync:download/latest",
    "ghcr.io/cbackas/spotisync:download/${VERSION}",
  ]
}

target "common" {
  context = "."
  dockerfile = "Dockerfile"
}
