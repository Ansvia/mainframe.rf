#!/usr/bin/env bash
#
# Ini adalah script untuk deploy/update server, script ini akan otomatis meng-upload
# source code ke server di path $$name_shout_snake_case$_DEST_SERVER_PATH (konfigurasinya bisa 
# di lihat di ./ansible/group_vars/all.yml).
#
# Setelah upload selesai, kode Rust akan dicompile di sisi server lalu akan menginstall
# servis menggunakan Systemd, semua prosedur ini bisa dilihat di ansible playbook file api.yml
#
# Kemudia servis akan dijalankan apabila belum jalan dan atau akan direstart apabila 
# servis sedang berjalan. Sehingga service akan otomatis jalan dengan versi terbaru.
#
# Troubleshooting:
#
#   1. Pastikan server target telah terinstall Python agar Ansible bisa bekerja di sisi server.
#   2. Pastikan Rust compiler telah terinstall di server target.
#   3. Pastikan Diesel CLI telah terinstall di server target agar migration script bisa dijalankan otomatis.
#
#

export CURDIR=`dirname $0`
. $CURDIR/includes.sh

export ANSIBLE_CONFIG=ansible.cfg

GIT_REV=$(git rev-parse HEAD)

echo -n "Deploy $name$ $VERSION to remote server? [y/yy/n] "
read confirm

if [ "$confirm" == "yy" ]; then
    yes_for_all="y"
fi

if [ "$confirm" == "y" ] || [ "$yes_for_all" == "y" ]; then
    echo "Updating API service..."
    echo $GIT_REV > $PROJDIR/etc/ansible/GIT_REV
    ansible-playbook -v -i etc/ansible/hosts -e "server=api" etc/ansible/api.yml
fi

if [ "$yes_for_all" != "y" ]; then
    echo -n "Deploy web? [y/n] "
    read confirm
fi

if [ "$confirm" == "y" ] || [ "$yes_for_all" == "y" ]; then
    echo "Updating web frontends..."
    make build-web-frontend
    ansible-playbook -v -i etc/ansible/hosts -e "server=web" etc/ansible/web.yml
fi
