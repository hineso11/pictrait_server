terraform {
  required_providers {
    digitalocean = {
      source = "digitalocean/digitalocean"
      version = "2.16.0"
    }
  }
}

variable "ssh_key_id" {
  type = string
  default = "32796192"
}

resource "digitalocean_droplet" "server" {
  image  = "ubuntu-21-10-x64"
  name   = "pictrait-server"
  region = "nyc1"
  size   = "s-1vcpu-1gb"
  backups = false
  monitoring = false
  ipv6 = false
  ssh_keys = [var.ssh_key_id]
  resize_disk = false
  droplet_agent = true
}

resource "digitalocean_firewall" "firewall" {
  name = "firewall"
  droplet_ids = [digitalocean_droplet.server.id]

  inbound_rule {
    protocol = "tcp"
    port_range = "22"
    source_addresses = ["0.0.0.0/0", "::/0"]
  }

  outbound_rule {
    protocol = "tcp"
    destination_addresses = ["0.0.0.0/0", "::/0"]
  }
}

output "server_ip" {
  value = digitalocean_droplet.server.ipv4_address
  description = "Server IP address"
}