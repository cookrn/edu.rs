# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = '2'

Vagrant.configure VAGRANTFILE_API_VERSION do | config |
  config.vm.box = 'ubuntu/trusty64'

  # If true, then any SSH connections made will enable agent forwarding.
  # Default value: false
  # config.ssh.forward_agent = true

  config.vm.provider 'virtualbox' do | vb |
    # Use VBoxManage to customize the VM. For example to change memory:
    vb.customize(
      [
        'modifyvm',
        :id,
        '--memory',
        '1024'
      ]
    )
  end

  config.vm.provision 'ansible' do | a |
    a.playbook = 'rust-dev.yml'
  end
end
